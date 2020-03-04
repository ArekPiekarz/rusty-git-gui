use crate::file_path::FilePathString;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileChange
{
    pub status: String,
    pub path: FilePathString,
    pub oldPath: Option<String>
}

#[derive(Clone, Debug)]
pub struct FileChangeUpdate
{
    pub old: FileChange,
    pub new: FileChange
}