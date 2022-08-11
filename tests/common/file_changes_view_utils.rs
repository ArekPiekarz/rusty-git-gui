use std::path::Path;


#[derive(Debug, Eq, PartialEq)]
pub(crate) struct FileChangesViewEntry
{
    pub status: String,
    pub path: String,
}

pub(crate) fn makeFileChange(status: &str, path: &Path) -> FileChangesViewEntry
{
    FileChangesViewEntry{status: status.into(), path: path.to_str().unwrap().into()}
}
