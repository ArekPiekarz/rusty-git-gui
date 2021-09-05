// Utilities to parse and check git status porcelain format for scripts.
// For details see https://git-scm.com/docs/git-status#_short_format

use itertools::Itertools;
use std::path::Path;

const POSITION_OF_INDEX_STATUS: usize = 0;
const POSITION_OF_WORK_TREE_STATUS: usize = 1;
const START_OF_FILE_PATH: usize = 3;


#[derive(Debug, PartialEq)]
pub struct RepositoryStatus
{
    pub data: Vec<RepositoryStatusEntry>
}

impl RepositoryStatus
{
    pub fn from(text: &str) -> Self
    {
        Self{data: text.split('\n').dropping_back(1).map(|line| RepositoryStatusEntry::from(&line)).collect_vec()}
    }
}

#[derive(Debug, PartialEq)]
pub struct RepositoryStatusEntry
{
    pub path: String,  // it can be a normal path or for renamed files "old path -> new path"
    pub workTreeStatus: FileChangeStatus,
    pub indexStatus: FileChangeStatus
}

impl RepositoryStatusEntry
{
    pub fn new(path: &Path, workTreeStatus: WorkTreeStatus, indexStatus: IndexStatus) -> Self
    {
        Self{path: path.to_str().unwrap().into(), workTreeStatus: workTreeStatus.0, indexStatus: indexStatus.0}
    }

    pub fn renamed(paths: &str, workTreeStatus: WorkTreeStatus, indexStatus: IndexStatus) -> Self
    {
        Self{path: paths.into(), workTreeStatus: workTreeStatus.0, indexStatus: indexStatus.0}
    }

    pub fn from(line: &str) -> Self
    {
        // Example of text: "AM fileName"
        // 0th letter is status of file change in index (here A means Added),
        // 1st letter is status of file change in work tree (here M means Modified)
        // 3rd letter onwards contains file path
        Self{
            path: line[START_OF_FILE_PATH..].into(),
            workTreeStatus: FileChangeStatus::from(line.chars().nth(POSITION_OF_WORK_TREE_STATUS).unwrap()),
            indexStatus: FileChangeStatus::from(line.chars().nth(POSITION_OF_INDEX_STATUS).unwrap())
        }
    }
}

pub struct IndexStatus(pub FileChangeStatus);
pub struct WorkTreeStatus(pub FileChangeStatus);

#[derive(Debug, PartialEq)]
pub enum FileChangeStatus
{
    Untracked,
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Unmerged
}

impl FileChangeStatus
{
    pub fn from(symbol: char) -> Self
    {
        match symbol {
            '?' => Self::Untracked,
            ' ' => Self::Unmodified,
            'M' => Self::Modified,
            'A' => Self::Added,
            'D' => Self::Deleted,
            'R' => Self::Renamed,
            'C' => Self::Copied,
            'U' => Self::Unmerged,
             _  => panic!("Unknown file change status: {}", symbol)
        }
    }
}
