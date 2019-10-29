#[derive(Clone, Debug, PartialEq)]
pub struct FileChange
{
    pub status: String,
    pub path: String
}

#[derive(Clone)]
pub struct UpdatedFileChange
{
    pub old: FileChange,
    pub new: FileChange
}