use rusty_git_gui::file_changes_view_entry::FileChangesViewEntry;

use std::path::Path;


pub fn makeFileChange(status: &str, path: &Path) -> FileChangesViewEntry
{
    FileChangesViewEntry{status: status.into(), path: path.to_str().unwrap().into()}
}

pub fn makeRenamedFileChange(status: &str, oldPath: &Path, newPath: &Path) -> FileChangesViewEntry
{
    FileChangesViewEntry{
        status: status.into(),
        path: format!("{} -> {}", oldPath.to_str().unwrap(), newPath.to_str().unwrap())}
}