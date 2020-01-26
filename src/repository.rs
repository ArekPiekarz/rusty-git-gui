use crate::error_handling::{exit, showErrorDialog};
use crate::file_change::{FileChange, FileChangeUpdate};
use crate::grouped_file_changes::GroupedFileChanges;
use crate::main_context::{attach, makeChannel};
use crate::staged_changes::StagedChanges;
use crate::unstaged_changes::UnstagedChanges;

use glib::Sender;
use itertools::Itertools;
use std::path::Path;

const NO_INDEX : Option<&git2::Index> = None;
const UNSTAGED_STATUSES : [git2::Status; 5] = [
    git2::Status::WT_NEW,
    git2::Status::WT_MODIFIED,
    git2::Status::WT_DELETED,
    git2::Status::WT_TYPECHANGE,
    git2::Status::WT_RENAMED];
const STAGED_STATUSES : [git2::Status; 5] = [
    git2::Status::INDEX_NEW,
    git2::Status::INDEX_MODIFIED,
    git2::Status::INDEX_DELETED,
    git2::Status::INDEX_TYPECHANGE,
    git2::Status::INDEX_RENAMED];
const STATUS_FOUND : bool = true;
const STATUS_NOT_FOUND : bool = false;
const NO_AUTHOR_UPDATE: Option<&git2::Signature> = None;
const NO_COMMITTER_UPDATE: Option<&git2::Signature> = None;
const NO_MESSAGE_ENCODING_UPDATE: Option<&str> = None;


pub struct Repository
{
    gitRepo: git2::Repository,
    fileChanges: GroupedFileChanges,
    senders: Senders
}

struct Senders
{
    onAddedToStaged: Vec<Sender<FileChange>>,
    onUpdatedInStaged: Vec<Sender<FileChangeUpdate>>,
    onRemovedFromStaged: Vec<Sender<FileChange>>,
    onAddedToUnstaged: Vec<Sender<FileChange>>,
    onUpdatedInUnstaged: Vec<Sender<FileChangeUpdate>>,
    onRemovedFromUnstaged: Vec<Sender<FileChange>>,
    onCommitted: Vec<Sender<()>>,
    onAmendedCommit: Vec<Sender<()>>,
    onRefreshed: Vec<Sender<()>>
}

impl Repository
{
    #[must_use]
    pub fn new(path: &Path) -> Self
    {
        let mut newSelf = Self{
            gitRepo: openRepository(path),
            fileChanges: GroupedFileChanges::new(),
            senders: Senders{
                onAddedToStaged: vec![],
                onUpdatedInStaged: vec![],
                onRemovedFromStaged: vec![],
                onAddedToUnstaged: vec![],
                onUpdatedInUnstaged: vec![],
                onRemovedFromUnstaged: vec![],
                onCommitted: vec![],
                onAmendedCommit: vec![],
                onRefreshed: vec![]
            },
        };
        newSelf.collectCurrentFileChanges();
        newSelf
    }

    pub fn collectCurrentFileChanges(&mut self) -> &GroupedFileChanges
    {
        let mut unstaged = UnstagedChanges::new();
        let mut staged = StagedChanges::new();
        for fileStatusEntry in self.collectFileStatuses().iter() {
            let mut statusFound = false;
            statusFound |= maybeAddToUnstaged(&fileStatusEntry, &mut unstaged);
            statusFound |= maybeAddToStaged(&fileStatusEntry, &mut staged);

            if !statusFound {
                exit(&format!("Failed to handle git status flag {:?} for file {}",
                  fileStatusEntry.status(), getFilePath(&fileStatusEntry)));
            }
        }
        self.fileChanges = GroupedFileChanges{unstaged, staged};
        &self.fileChanges
    }

    #[must_use]
    pub const fn getUnstagedChanges(&self) -> &UnstagedChanges
    {
        &self.fileChanges.unstaged
    }

    #[must_use]
    pub const fn getStagedChanges(&self) -> &StagedChanges
    {
        &self.fileChanges.staged
    }

    #[must_use]
    pub fn hasStagedChanges(&self) -> bool
    {
        !self.fileChanges.staged.is_empty()
    }

    #[must_use]
    pub fn isEmpty(&self) -> bool
    {
        self.gitRepo.is_empty()
            .unwrap_or_else(|e| exit(&format!("Failed to check if repository is empty: {}", e)))
    }

    pub fn getLastCommitMessage(&self) -> Result<Option<String>,()>
    {
        match self.findHeadCommit() {
            Some(commit) => match commit.message() {
                Some(message) => Ok(Some(message.into())),
                None => Err(())
            }
            None => Ok(None)
        }
    }

    #[must_use]
    pub fn makeDiffOfIndexToWorkdir(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = makeDiffOptions(path);
        self.gitRepo.diff_index_to_workdir(NO_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(
                &format!("Failed to get index-to-workdir diff for path {}: {}", path, e)))
    }

    #[must_use]
    pub fn makeDiffOfTreeToIndex(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = makeDiffOptions(path);
        let tree = self.findCurrentTree();
        self.gitRepo.diff_tree_to_index(tree.as_ref(), NO_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(&format!("Failed to get tree-to-index diff for path {}: {}", path, e)))
    }

    #[must_use]
    pub fn makeDiffOfIndexToWorkdirForRenamedFile(&self, oldPath: &str, newPath: &str) -> git2::Diff
    {
        let mut diffOptions = makeDiffOptions(oldPath);
        diffOptions.pathspec(newPath);
        let mut diff = self.gitRepo.diff_index_to_workdir(NO_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(
                &format!("Failed to get index-to-workdir diff for path {}: {}", oldPath, e)));
        let mut diffFindOptions = git2::DiffFindOptions::new();
        diffFindOptions.for_untracked(true);
        diff.find_similar(Some(&mut diffFindOptions)).unwrap();
        diff
    }

    pub fn stageFileChange(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "WT_DELETED" => self.removePathFromIndex(&fileChange.path),
            _ => self.addPathToIndex(&fileChange.path)
        }

        let oldStagedFileChange = self.fileChanges.staged.iter().find(
            |stagedFileChange| stagedFileChange.path == fileChange.path).cloned();
        self.collectCurrentFileChanges();
        let newStagedFileChange = self.fileChanges.staged.iter().find(
            |stagedFileChange| stagedFileChange.path == fileChange.path);

        self.notifyOnRemovedFromUnstaged(fileChange);

        match oldStagedFileChange {
            Some(oldStagedFileChange) => self.finishStagingWhenFileWasAlreadyStaged(
                &oldStagedFileChange, newStagedFileChange),
            None => self.finishStagingWhenFileWasNotYetStaged(newStagedFileChange)
        }
    }

    pub fn unstageFileChange(&mut self, fileChange: &FileChange)
    {
        {
            let commitObject = match self.findHeadCommit() {
                Some(commit) => Some(commit.into_object()),
                None => None };
            self.gitRepo.reset_default(commitObject.as_ref(), &[&fileChange.path])
                .unwrap_or_else(|e| exit(&format!("Failed to unstage file {}, error: {}", fileChange.path, e)));
        }

        self.notifyOnRemovedFromStaged(fileChange);
        let oldUnstagedFileChange = self.fileChanges.unstaged.iter().find(
            |unstagedFileChange| unstagedFileChange.path == fileChange.path).cloned();
        self.collectCurrentFileChanges();
        let newUnstagedFileChange = self.fileChanges.unstaged.iter().find(
            |unstagedFileChange| unstagedFileChange.path == fileChange.path);

        match oldUnstagedFileChange {
            Some(oldUnstagedFileChange) => self.finishUnstagingWhenFileWasAlreadyUnstaged(
                &oldUnstagedFileChange, newUnstagedFileChange),
            None => self.finishUnstagingWhenFileWasNotYetUnstaged(newUnstagedFileChange)
        }
    }

    pub fn commit(&mut self, message: &str)
    {
        {
            let author = self.gitRepo.signature()
                .unwrap_or_else(|e| exit(&format!("Failed to get a name and/or email of the commit author: {}", e)));
            let commiter = &author;
            let tree = self.storeIndexAsTree();
            let parentCommits = self.findParentCommits();
            let parentCommits = parentCommits.iter().collect_vec();

            self.gitRepo.commit(Some("HEAD"), &author, commiter, message, &tree, &parentCommits)
                .unwrap_or_else(|e| exit(&format!("Failed to commit changes: {}", e)));
        }

        self.collectCurrentFileChanges();
        self.notifyOnCommitted();
    }

    pub fn amendCommit(&mut self, newMessage: &str)
    {
        match self.tryAmendCommit(newMessage) {
            Ok(_) => {
                self.collectCurrentFileChanges();
                self.notifyOnAmendedCommit();
            },
            Err(e) => showErrorDialog(e)
        }
    }

    pub fn refresh(&mut self)
    {
        self.collectCurrentFileChanges();
        self.notifyOnRefreshed();
    }

    pub fn connectOnAddedToStaged(&mut self, handler: Box<dyn Fn(FileChange) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onAddedToStaged.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnUpdatedInStaged(&mut self, handler: Box<dyn Fn(FileChangeUpdate) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onUpdatedInStaged.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnRemovedFromStaged(&mut self, handler: Box<dyn Fn(FileChange) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onRemovedFromStaged.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnAddedToUnstaged(&mut self, handler: Box<dyn Fn(FileChange) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onAddedToUnstaged.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnUpdatedInUnstaged(&mut self, handler: Box<dyn Fn(FileChangeUpdate) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onUpdatedInUnstaged.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnRemovedFromUnstaged(&mut self, handler: Box<dyn Fn(FileChange) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onRemovedFromUnstaged.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnCommitted(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onCommitted.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnAmendedCommit(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onAmendedCommit.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnRefreshed(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onRefreshed.push(sender);
        attach(receiver, handler);
    }


    // private

    fn collectFileStatuses(&self) -> git2::Statuses
    {
        self.gitRepo.statuses(Some(&mut makeStatusOptions()))
            .unwrap_or_else(|e| exit(&format!("Failed to get statuses: {}", e)))
    }

    fn addPathToIndex(&self, filePath: &str)
    {
        let mut index = self.gitRepo.index()
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {}, because index could not be acquired: {}", filePath, e)));
        index.add_path(Path::new(filePath))
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {}, because adding path to index failed: {}", filePath, e)));
        index.write()
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {}, because writing the index to disk failed: {}", filePath, e)));
    }

    fn removePathFromIndex(&self, filePath: &str)
    {
        let mut index = self.gitRepo.index()
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {} for removal, because index could not be acquired: {}", filePath, e)));
        index.remove_path(Path::new(filePath))
            .unwrap_or_else(|e| exit(&format!("Failed to staged file {} for removal: {}", filePath, e)));
        index.write()
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {} for removal, because writing the index to disk failed: {}", filePath, e)));
    }

    fn findHeadCommit(&self) -> Option<git2::Commit>
    {
        if self.isEmpty() {
            return None;
        }

        let head = self.gitRepo.head()
            .unwrap_or_else(|e| exit(&format!("Failed to get reference to HEAD: {}", e)));
        let commit = head.peel_to_commit()
            .unwrap_or_else(|e| exit(&format!("Failed to turn a reference to HEAD into a commit: {}", e)));
        Some(commit)
    }

    fn findParentCommits(&self) -> Vec<git2::Commit>
    {
        match self.findHeadCommit() {
            Some(commit) => vec![commit],
            None => vec![]
        }
    }

    fn findCurrentTree(&self) -> Option<git2::Tree>
    {
        match self.gitRepo.head() {
            Ok(head) => Some(head.peel_to_tree()
                .unwrap_or_else(|e| exit(&format!("Failed to turn a reference to HEAD into a tree: {}", e)))),
            Err(ref e) if e.class() == git2::ErrorClass::Reference && e.code() == git2::ErrorCode::UnbornBranch => None,
            Err(e) => exit(&format!("Failed to get reference to HEAD: {}", e))
        }
    }

    fn finishStagingWhenFileWasAlreadyStaged(&self, oldFileChange: &FileChange, newFileChange: Option<&FileChange>)
    {
        match newFileChange {
            Some(newFileChange) => {
                if oldFileChange.status != newFileChange.status {
                    self.notifyOnUpdatedInStaged(
                        &FileChangeUpdate {old: oldFileChange.clone(), new: (*newFileChange).clone()})
                }
            }
            None => self.notifyOnRemovedFromStaged(oldFileChange)
        }
    }

    fn finishStagingWhenFileWasNotYetStaged(&self, fileChange: Option<&FileChange>)
    {
        if let Some(fileChange) = fileChange {
            self.notifyOnAddedToStaged(fileChange)
        }
    }

    fn finishUnstagingWhenFileWasAlreadyUnstaged(&self, oldFileChange: &FileChange, newFileChange: Option<&FileChange>)
    {
        match newFileChange {
            Some(newFileChange) => {
                if oldFileChange.status != newFileChange.status {
                    self.notifyOnUpdatedInUnstaged(
                        &FileChangeUpdate {old: oldFileChange.clone(), new: (*newFileChange).clone()})
                }
            }
            None => self.notifyOnRemovedFromUnstaged(oldFileChange)
        }
    }

    fn finishUnstagingWhenFileWasNotYetUnstaged(&self, fileChange: Option<&FileChange>)
    {
        if let Some(fileChange) = fileChange {
            self.notifyOnAddedToUnstaged(fileChange)
        }
    }

    fn tryAmendCommit(&self, newMessage: &str) -> Result<(), &str>
    {
        match self.findHeadCommit() {
            Some(headCommit) => {
                let newTree = self.storeIndexAsTree();
                headCommit.amend(
                    Some("HEAD"),
                    NO_AUTHOR_UPDATE,
                    NO_COMMITTER_UPDATE,
                    NO_MESSAGE_ENCODING_UPDATE,
                    Some(newMessage),
                    Some(&newTree))
                    .unwrap();
                Ok(())
            },
            None => Err("Failed to amend commit - no HEAD commit was found.")
        }
    }

    fn storeIndexAsTree(&self) -> git2::Tree
    {
        let mut index = self.gitRepo.index()
            .unwrap_or_else(|e| exit(&format!("Failed to acquire repository index: {}", e)));
        let treeId = index.write_tree()
            .unwrap_or_else(|e| exit(&format!("Failed to write repository index as tree to disk: {}", e)));
        self.gitRepo.find_tree(treeId)
            .unwrap_or_else(|e| exit(&format!("Failed to find tree for id {}: {}", treeId, e)))
    }

    fn notifyOnAddedToStaged(&self, fileChange: &FileChange)
    {
        for sender in &self.senders.onAddedToStaged {
            sender.send(fileChange.clone()).unwrap();
        }
    }

    fn notifyOnUpdatedInStaged(&self, updatedFileChange: &FileChangeUpdate)
    {
        for sender in &self.senders.onUpdatedInStaged {
            sender.send(updatedFileChange.clone()).unwrap();
        }
    }

    fn notifyOnUpdatedInUnstaged(&self, updatedFileChange: &FileChangeUpdate)
    {
        for sender in &self.senders.onUpdatedInUnstaged {
            sender.send(updatedFileChange.clone()).unwrap();
        }
    }

    fn notifyOnRemovedFromStaged(&self, fileChange: &FileChange)
    {
        for sender in &self.senders.onRemovedFromStaged {
            sender.send(fileChange.clone()).unwrap();
        }
    }

    fn notifyOnAddedToUnstaged(&self, fileChange: &FileChange)
    {
        for sender in &self.senders.onAddedToUnstaged {
            sender.send(fileChange.clone()).unwrap();
        }
    }

    fn notifyOnRemovedFromUnstaged(&self, fileChange: &FileChange)
    {
        for sender in &self.senders.onRemovedFromUnstaged {
            sender.send(fileChange.clone()).unwrap();
        }
    }

    fn notifyOnCommitted(&self)
    {
        for sender in &self.senders.onCommitted {
            sender.send(()).unwrap();
        }
    }

    fn notifyOnAmendedCommit(&self)
    {
        for sender in &self.senders.onAmendedCommit {
            sender.send(()).unwrap();
        }
    }

    fn notifyOnRefreshed(&self)
    {
        for sender in &self.senders.onRefreshed {
            sender.send(()).unwrap();
        }
    }
}

fn openRepository(repositoryDir: &Path) -> git2::Repository
{
    git2::Repository::open(repositoryDir)
        .unwrap_or_else(|e| exit(&format!("Failed to open repository: {}", e)))
}

fn makeStatusOptions() -> git2::StatusOptions
{
    let mut options = git2::StatusOptions::new();
    options
        .include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .renames_index_to_workdir(true);
    options
}

fn maybeAddToUnstaged(fileStatusEntry: &git2::StatusEntry, unstaged: &mut UnstagedChanges) -> bool
{
    maybeAddToFileChanges(fileStatusEntry, unstaged, &UNSTAGED_STATUSES)
}

fn maybeAddToStaged(fileStatusEntry: &git2::StatusEntry, staged: &mut StagedChanges) -> bool
{
    maybeAddToFileChanges(fileStatusEntry, staged, &STAGED_STATUSES)
}

fn maybeAddToFileChanges(
    fileStatusEntry: &git2::StatusEntry,
    fileChanges: &mut Vec<FileChange>,
    statusTypes: &[git2::Status]) -> bool
{
    let statusFlags = fileStatusEntry.status();
    for statusType in statusTypes {
        if statusFlags.intersects(*statusType) {
            match *statusType {
                git2::Status::WT_RENAMED => fileChanges.push(makeRenamedFileChange(fileStatusEntry, *statusType)),
                _ => fileChanges.push(makeFileChange(fileStatusEntry, *statusType))
            }
            return STATUS_FOUND;
        }
    }
    STATUS_NOT_FOUND
}

fn makeFileChange(statusEntry: &git2::StatusEntry, status: git2::Status) -> FileChange
{
    FileChange{status: format!("{:?}", status), path: getFilePath(statusEntry), oldPath: None}
}

fn makeRenamedFileChange(statusEntry: &git2::StatusEntry, status: git2::Status) -> FileChange
{
    let fileChange = makeFileChange(statusEntry, status);
    FileChange{
        status: fileChange.status,
        path: statusEntry.index_to_workdir().unwrap().new_file().path().unwrap().to_str().unwrap().into(),
        oldPath: Some(fileChange.path)
    }
}

fn getFilePath(statusEntry: &git2::StatusEntry) -> String
{
    statusEntry.path().unwrap_or_else(
        || exit(&format!("Failed to convert status entry file path to UTF-8: {}",
             String::from_utf8_lossy(statusEntry.path_bytes())))).to_string()
}

fn makeDiffOptions(path: &str) -> git2::DiffOptions
{
    let mut diffOptions = git2::DiffOptions::new();
    diffOptions.pathspec(path).show_untracked_content(true).recurse_untracked_dirs(true);
    diffOptions
}