use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::error_handling::{exit, showErrorDialog};
use crate::file_change::{FileChange, FileChangeUpdate};
use crate::grouped_file_changes::GroupedFileChanges;
use crate::settings::Settings;
use crate::staged_changes::StagedChanges;
use crate::unstaged_changes::UnstagedChanges;

use itertools::Itertools;
use std::path::Path;

const CURRENT_INDEX : Option<&git2::Index> = None;
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
const DEFAULT_DIFF_CONTEXT_SIZE: u32 = 3;


pub struct Repository
{
    gitRepo: git2::Repository,
    fileChanges: GroupedFileChanges,
    sender: Sender,
    stager: Stager,
    unstager: Unstager,
    diffContextSize: u32
}

impl IEventHandler for Repository
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::AmendCommitRequested(message) => self.amendCommit(message),
            Event::CommitAmendDisabled           => self.disableCommitAmendMode(),
            Event::CommitAmendEnabled            => self.enableCommitAmendMode(),
            Event::CommitRequested(message)      => self.commit(message),
            Event::RefreshRequested              => self.refresh(),
            Event::StageRequested(fileChange)    => self.stage(fileChange),
            Event::UnstageRequested(fileChange)  => self.unstage(fileChange),
            _ => handleUnknown(source, event)
        }
    }
}

impl Repository
{
    #[must_use]
    pub fn new(path: &Path, sender: Sender, settings: &Settings) -> Self
    {
        let mut newSelf = Self{
            gitRepo: openRepository(path),
            fileChanges: GroupedFileChanges::new(),
            sender,
            stager: Self::stageNormally,
            unstager: Self::unstageNormally,
            diffContextSize: settings.get("Repository", "diffContextSize", DEFAULT_DIFF_CONTEXT_SIZE)
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

    fn collectLastCommitChanges(&self) -> Vec<FileChange>
    {
        let mut fileChanges = vec![];
        if let Some(commit) = self.findHeadCommit() {
            let amendDiff = self.makeDiffToAmend(&commit);
            for delta in amendDiff.deltas() {
                fileChanges.push(FileChange::from(&delta));
            }
        }
        fileChanges
    }

    #[must_use]
    pub fn makeDiffOfIndexToWorkdir(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = self.makeDiffOptions(path);
        self.gitRepo.diff_index_to_workdir(CURRENT_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(
                &format!("Failed to get index-to-workdir diff for path {}: {}", path, e)))
    }

    #[must_use]
    pub fn makeDiffOfTreeToIndex(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = self.makeDiffOptions(path);
        let tree = self.findCurrentTree();
        self.gitRepo.diff_tree_to_index(tree.as_ref(), CURRENT_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(&format!("Failed to get tree-to-index diff for path {}: {}", path, e)))
    }

    #[must_use]
    pub fn makeDiffOfIndexToWorkdirForRenamedFile(&self, oldPath: &str, newPath: &str) -> git2::Diff
    {
        let mut diffOptions = self.makeDiffOptions(oldPath);
        diffOptions.pathspec(newPath);
        let mut diff = self.gitRepo.diff_index_to_workdir(CURRENT_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(
                &format!("Failed to get index-to-workdir diff for path {}: {}", oldPath, e)));
        let mut diffFindOptions = git2::DiffFindOptions::new();
        diffFindOptions.for_untracked(true);
        diff.find_similar(Some(&mut diffFindOptions)).unwrap();
        diff
    }

    #[must_use]
    pub fn makeDiffOfTreeToIndexForRenamedFile(&self, oldPath: &str, newPath: &str) -> git2::Diff
    {
        let mut diffOptions = self.makeDiffOptions(oldPath);
        diffOptions.pathspec(newPath);
        let tree = self.findCurrentTree();
        let mut diff = self.gitRepo.diff_tree_to_index(tree.as_ref(), CURRENT_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(&format!(
                "Failed to get tree-to-index diff for paths: old: {}, new: {}, cause: {}", oldPath, newPath, e)));
        let mut diffFindOptions = git2::DiffFindOptions::new();
        diff.find_similar(Some(&mut diffFindOptions)).unwrap();
        diff
    }

    #[must_use]
    pub fn makeDiffToAmend(&self, commit: &git2::Commit) -> git2::Diff
    {
        let mut diffOptions = git2::DiffOptions::new();
        diffOptions.indent_heuristic(true);
        let tree = findTreeOfParentOfCommit(commit);
        self.gitRepo.diff_tree_to_index(tree.as_ref(), CURRENT_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(&format!("Failed to get diff to amend: {}", e)))
    }

    #[must_use]
    pub fn makeDiffToAmendForPath(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = git2::DiffOptions::new();
        diffOptions
            .pathspec(path)
            .indent_heuristic(true);
        let tree = self.findTreeOfParentOfHeadCommit();
        self.gitRepo.diff_tree_to_index(tree.as_ref(), CURRENT_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(&format!(
                "Failed to get tree-to-index diff to amend for path {}: {}", path, e)))
    }

    pub fn stage(&mut self, fileChange: &FileChange)
    {
        (self.stager)(self, fileChange);
    }

    pub fn unstage(&mut self, fileChange: &FileChange)
    {
        (self.unstager)(self, fileChange);
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


    // private

    fn enableCommitAmendMode(&mut self)
    {
        self.stager = Self::stageToAmend;
        self.unstager = Self::unstageToAmend;
        self.collectCurrentFileChangesToAmend();
        self.notifyOnRefreshed();
    }

    fn disableCommitAmendMode(&mut self)
    {
        self.stager = Self::stageNormally;
        self.unstager = Self::unstageNormally;
        self.collectCurrentFileChanges();
        self.notifyOnRefreshed();
    }

    pub fn stageNormally(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "WT_DELETED" => self.removePathFromIndex(&fileChange.path),
            "WT_RENAMED" => {
                self.addPathToIndex(&fileChange.path);
                self.removePathFromIndex(&fileChange.oldPath.as_ref().unwrap());
            }
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

    pub fn stageToAmend(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "WT_DELETED" => self.removePathFromIndex(&fileChange.path),
            _ => self.addPathToIndex(&fileChange.path)
        }

        let oldStagedFileChange = self.fileChanges.staged.iter().find(
            |stagedFileChange| stagedFileChange.path == fileChange.path).cloned();
        self.collectCurrentFileChangesToAmend();
        let newStagedFileChange = self.fileChanges.staged.iter().find(
            |stagedFileChange| stagedFileChange.path == fileChange.path);

        self.notifyOnRemovedFromUnstaged(fileChange);

        match oldStagedFileChange {
            Some(oldStagedFileChange) => self.finishStagingWhenFileWasAlreadyStaged(
                &oldStagedFileChange, newStagedFileChange),
            None => self.finishStagingWhenFileWasNotYetStaged(newStagedFileChange)
        }
    }

    pub fn unstageNormally(&mut self, fileChange: &FileChange)
    {
        {
            let commitObject = match self.findHeadCommit() {
                Some(commit) => Some(commit.into_object()),
                None => None };
            let mut paths = vec![&fileChange.path];
            if let Some(oldPath) = &fileChange.oldPath {
                paths.push(oldPath);
            }
            self.gitRepo.reset_default(commitObject.as_ref(), &paths)
                .unwrap_or_else(|e| exit(&format!(
                    "Failed to unstage {}: {:?}, cause: {}", getFileWord(&paths), paths, e)));
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

    fn unstageToAmend(&mut self, fileChange: &FileChange)
    {
        {
            let parent = self.findParentOfHeadCommit();
            let parentObject = match parent {
                Some(parent) => Some(parent.into_object()),
                None => None
            };
            self.gitRepo.reset_default(parentObject.as_ref(), &[&fileChange.path])
                .unwrap_or_else(|e| exit(&format!("Failed to unstage file {}, error: {}", fileChange.path, e)));
        }

        self.notifyOnRemovedFromStaged(fileChange);
        let oldUnstagedFileChange = self.fileChanges.unstaged.iter().find(
            |unstagedFileChange| unstagedFileChange.path == fileChange.path).cloned();
        self.collectCurrentFileChangesToAmend();
        let newUnstagedFileChange = self.fileChanges.unstaged.iter().find(
            |unstagedFileChange| unstagedFileChange.path == fileChange.path);

        match oldUnstagedFileChange {
            Some(oldUnstagedFileChange) => self.finishUnstagingWhenFileWasAlreadyUnstaged(
                &oldUnstagedFileChange, newUnstagedFileChange),
            None => self.finishUnstagingWhenFileWasNotYetUnstaged(newUnstagedFileChange)
        }
    }

    fn collectFileStatuses(&self) -> git2::Statuses
    {
        self.gitRepo.statuses(Some(&mut makeStatusOptions()))
            .unwrap_or_else(|e| exit(&format!("Failed to get statuses: {}", e)))
    }

    pub fn collectCurrentFileChangesToAmend(&mut self) -> &GroupedFileChanges
    {
        let mut unstaged = UnstagedChanges::new();
        for fileStatusEntry in self.collectFileStatuses().iter() {
            maybeAddToUnstaged(&fileStatusEntry, &mut unstaged);
        }
        let staged = StagedChanges(self.collectLastCommitChanges());
        self.fileChanges = GroupedFileChanges{unstaged, staged};
        &self.fileChanges
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

    fn findTreeOfParentOfHeadCommit(&self) -> Option<git2::Tree>
    {
        let head = self.findHeadCommit().unwrap();
        findTreeOfParentOfCommit(&head)
    }

    fn findParentOfHeadCommit(&self) -> Option<git2::Commit>
    {
        let head = self.findHeadCommit().unwrap();
        head.parents().next()
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

    fn makeDiffOptions(&self, path: &str) -> git2::DiffOptions
    {
        let mut diffOptions = git2::DiffOptions::new();
        diffOptions
            .pathspec(path)
            .indent_heuristic(true)
            .recurse_untracked_dirs(true)
            .show_untracked_content(true)
            .context_lines(self.diffContextSize);
        diffOptions
    }

    fn notifyOnAddedToStaged(&self, fileChange: &FileChange)
    {
        self.sender.send((Source::Repository, Event::AddedToStaged(fileChange.clone()))).unwrap();
    }

    fn notifyOnUpdatedInStaged(&self, fileChangeUpdate: &FileChangeUpdate)
    {
        self.sender.send((Source::Repository, Event::UpdatedInStaged(fileChangeUpdate.clone()))).unwrap();
    }

    fn notifyOnRemovedFromStaged(&self, fileChange: &FileChange)
    {
        self.sender.send((Source::Repository, Event::RemovedFromStaged(fileChange.clone()))).unwrap();
    }

    fn notifyOnAddedToUnstaged(&self, fileChange: &FileChange)
    {
        self.sender.send((Source::Repository, Event::AddedToUnstaged(fileChange.clone()))).unwrap();
    }

    fn notifyOnUpdatedInUnstaged(&self, fileChangeUpdate: &FileChangeUpdate)
    {
        self.sender.send((Source::Repository, Event::UpdatedInUnstaged(fileChangeUpdate.clone()))).unwrap();
    }

    fn notifyOnRemovedFromUnstaged(&self, fileChange: &FileChange)
    {
        self.sender.send((Source::Repository, Event::RemovedFromUnstaged(fileChange.clone()))).unwrap();
    }

    fn notifyOnCommitted(&self)
    {
        self.sender.send((Source::Repository, Event::Committed)).unwrap();
    }

    fn notifyOnAmendedCommit(&self)
    {
        self.sender.send((Source::Repository, Event::AmendedCommit)).unwrap();
    }

    fn notifyOnRefreshed(&self)
    {
        self.sender.send((Source::Repository, Event::Refreshed)).unwrap();
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
        .renames_head_to_index(true)
        .renames_index_to_workdir(true);
    options
}

fn maybeAddToUnstaged(fileStatusEntry: &git2::StatusEntry, unstaged: &mut UnstagedChanges) -> bool
{
    maybeAddToFileChanges(
        fileStatusEntry, unstaged, &UNSTAGED_STATUSES, git2::Status::WT_RENAMED, extractRenamedPathFromUnstaged)
}

fn maybeAddToStaged(fileStatusEntry: &git2::StatusEntry, staged: &mut StagedChanges) -> bool
{
    maybeAddToFileChanges(
        fileStatusEntry, staged, &STAGED_STATUSES, git2::Status::INDEX_RENAMED, extractRenamedPathFromStaged)
}

fn maybeAddToFileChanges(
    fileStatusEntry: &git2::StatusEntry,
    fileChanges: &mut Vec<FileChange>,
    statusTypes: &[git2::Status],
    renamedStatus: git2::Status,
    renamedPathExtractor: RenamedPathExtractor)
    -> bool
{
    let statusFlags = fileStatusEntry.status();
    for statusType in statusTypes {
        if statusFlags.intersects(*statusType) {
            if *statusType == renamedStatus {
                fileChanges.push(makeRenamedFileChange(fileStatusEntry, *statusType, renamedPathExtractor));
            } else {
                fileChanges.push(makeFileChange(fileStatusEntry, *statusType));
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

fn makeRenamedFileChange(statusEntry: &git2::StatusEntry, status: git2::Status, pathExtractor: RenamedPathExtractor)
    -> FileChange
{
    let fileChange = makeFileChange(statusEntry, status);
    FileChange{
        status: fileChange.status,
        path: pathExtractor(statusEntry),
        oldPath: Some(fileChange.path)
    }
}

fn getFilePath(statusEntry: &git2::StatusEntry) -> String
{
    statusEntry.path().unwrap_or_else(
        || exit(&format!("Failed to convert status entry file path to UTF-8: {}",
             String::from_utf8_lossy(statusEntry.path_bytes())))).to_string()
}

fn findTreeOfParentOfCommit<'a>(commit: &git2::Commit<'a>) -> Option<git2::Tree<'a>>
{
    match commit.parents().next() {
        Some(parent) => Some(parent.tree().unwrap()),
        None => None
    }
}

fn extractRenamedPathFromStaged(statusEntry: &git2::StatusEntry) -> String
{
    statusEntry.head_to_index().unwrap().new_file().path().unwrap().to_str().unwrap().into()
}

fn extractRenamedPathFromUnstaged(statusEntry: &git2::StatusEntry) -> String
{
    statusEntry.index_to_workdir().unwrap().new_file().path().unwrap().to_str().unwrap().into()
}

fn getFileWord(paths: &[&String]) -> &'static str
{
    match paths.len() {
        1 => "file",
        _ => "files"
    }
}

type RenamedPathExtractor = fn(&git2::StatusEntry) -> String;
type Stager = fn(&mut Repository, &FileChange);
type Unstager = fn(&mut Repository, &FileChange);
