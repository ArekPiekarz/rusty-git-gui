use crate::error_handling::exit;
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

#[derive(Debug)]
pub struct FileInfo
{
    pub path: String,
    pub status: String
}

pub struct Repository
{
    gitRepo: git2::Repository
}

pub struct FileInfos
{
    pub unstaged: Vec<FileInfo>,
    pub staged: Vec<FileInfo>
}

impl Repository
{
    pub fn new(path: &Path) -> Self
    {
        Self{gitRepo: openRepository(path)}
    }

    pub fn collectFileInfos(&self) -> FileInfos
    {
        let mut unstaged = vec![];
        let mut staged = vec![];
        for fileStatusEntry in self.collectFileStatuses().iter() {
            let mut statusFound = false;
            statusFound |= maybeAddToUnstaged(&fileStatusEntry, &mut unstaged);
            statusFound |= maybeAddToStaged(&fileStatusEntry, &mut staged);

            if !statusFound {
                exit(&format!("Failed to handle git status flag {:?} for file {}",
                  fileStatusEntry.status(), getFilePath(&fileStatusEntry)));
            }
        }
        FileInfos {unstaged, staged}
    }

    fn collectFileStatuses(&self) -> git2::Statuses
    {
        self.gitRepo.statuses(Some(&mut makeStatusOptions()))
            .unwrap_or_else(|e| exit(&format!("Failed to get statuses: {}", e)))
    }

    pub fn makeDiffOfIndexToWorkdir(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = makeDiffOptions(path);
        self.gitRepo.diff_index_to_workdir(NO_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(
                &format!("Failed to get index-to-workdir diff for path {}: {}", path, e)))
    }

    pub fn makeDiffOfTreeToIndex(&self, path: &str) -> git2::Diff
    {
        let mut diffOptions = makeDiffOptions(path);
        let tree = self.findCurrentTree();
        self.gitRepo.diff_tree_to_index(tree.as_ref(), NO_INDEX, Some(&mut diffOptions))
            .unwrap_or_else(|e| exit(&format!("Failed to get tree-to-index diff for path {}: {}", path, e)))
    }

    pub fn stageFile(&self, path: &str)
    {
        let mut index = self.gitRepo.index()
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {}, because index could not be acquired: {}", path, e)));
        index.add_path(Path::new(path))
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {}, because adding path to index failed: {}", path, e)));
        index.write()
            .unwrap_or_else(|e| exit(&format!(
                "Failed to stage file {}, because writing the index to disk failed: {}", path, e)));
    }

    pub fn unstageFile(&self, path: &str)
    {
        let commitObject = match self.findHeadCommit() {
            Some(commit) => Some(commit.into_object()),
            None => None };
        self.gitRepo.reset_default(commitObject.as_ref(), &[path])
            .unwrap_or_else(|e| exit(&format!("Failed to unstage file {}, error: {}", path, e)));
    }

    pub fn commitChanges(&self, message: &str)
    {
        let author = self.gitRepo.signature()
            .unwrap_or_else(|e| exit(&format!("Failed to get a name and/or email of the commit author: {}", e)));
        let commiter = &author;

        let mut index = self.gitRepo.index()
            .unwrap_or_else(|e| exit(&format!("Failed to acquire repository index: {}", e)));
        let treeId = index.write_tree()
            .unwrap_or_else(|e| exit(&format!("Failed to write repository index as tree to disk: {}", e)));
        let tree = self.gitRepo.find_tree(treeId)
            .unwrap_or_else(|e| exit(&format!("Failed to find tree for id {}: {}", treeId, e)));
        let parentCommits = self.findParentCommits();
        let parentCommits = parentCommits.iter().collect::<Vec<&_>>();

        self.gitRepo.commit(Some("HEAD"), &author, &commiter, message, &tree, &parentCommits)
            .unwrap_or_else(|e| exit(&format!("Failed to commit changes: {}", e)));
    }

    fn findHeadCommit(&self) -> Option<git2::Commit>
    {
        if self.isRepositoryEmpty() {
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

    fn isRepositoryEmpty(&self) -> bool
    {
        self.gitRepo.is_empty()
            .unwrap_or_else(|e| exit(&format!("Failed to check if repository is empty: {}", e)))
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
    options.include_ignored(false).include_untracked(true);
    options
}

fn maybeAddToUnstaged(fileStatusEntry: &git2::StatusEntry, unstaged: &mut Vec<FileInfo>) -> bool
{
    maybeAddToFileInfos(fileStatusEntry, unstaged, &UNSTAGED_STATUSES)
}

fn maybeAddToStaged(fileStatusEntry: &git2::StatusEntry, staged: &mut Vec<FileInfo>) -> bool
{
    maybeAddToFileInfos(fileStatusEntry, staged, &STAGED_STATUSES)
}

fn maybeAddToFileInfos(
    fileStatusEntry: &git2::StatusEntry,
    fileInfos: &mut Vec<FileInfo>,
    statusTypes: &[git2::Status]) -> bool
{
    let statusFlag = fileStatusEntry.status();
    for statusType in statusTypes {
        if statusFlag.intersects(*statusType) {
            fileInfos.push(makeFileInfo(&fileStatusEntry, *statusType));
            return STATUS_FOUND;
        }
    }
    STATUS_NOT_FOUND
}

fn makeFileInfo(statusEntry: &git2::StatusEntry, status: git2::Status) -> FileInfo
{
    FileInfo { path: getFilePath(statusEntry), status: format!("{:?}", status) }
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
    diffOptions.pathspec(path).show_untracked_content(true);
    diffOptions
}