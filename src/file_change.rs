use crate::file_path::FilePathString;


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileChange
{
    pub status: String,
    pub path: FilePathString,
    pub oldPath: Option<String>
}

#[derive(Clone)]
pub struct FileChangeUpdate
{
    pub old: FileChange,
    pub new: FileChange
}