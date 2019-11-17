#[derive(Clone, Debug, PartialEq)]
pub struct FileChange
{
    pub status: String,
    pub path: String,
    pub oldPath: Option<String>
}

#[derive(Clone)]
pub struct FileChangeUpdate
{
    pub old: FileChange,
    pub new: FileChange
}