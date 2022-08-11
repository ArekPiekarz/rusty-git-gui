use crate::file_path::FilePathString;


#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FileChange
{
    pub status: String,
    pub path: FilePathString,
    pub oldPath: Option<FilePathString>
}

impl FileChange
{
    pub fn from(delta: &git2::DiffDelta) -> Self
    {
        let mut newSelf = Self{
            status: format!("{:?}", delta.status()),
            path: delta.new_file().path().unwrap().to_str().unwrap().into(),
            oldPath: None};

        if let Some(oldPath) = delta.old_file().path() {
            let oldPath = oldPath.to_str().unwrap();
            if oldPath != newSelf.path {
                newSelf.oldPath = Some(oldPath.into());
            }
        };
        newSelf
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FileChangeUpdate
{
    pub old: FileChange,
    pub new: FileChange
}
