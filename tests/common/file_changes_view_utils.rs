use rusty_git_gui::file_changes_view_entry::FileChangesViewEntry;

use std::path::Path;


pub fn makeFileChange(status: &str, path: &Path) -> FileChangesViewEntry
{
    FileChangesViewEntry{status: status.into(), path: path.to_str().unwrap().into()}
}
