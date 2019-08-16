use rusty_git_gui::file_change::FileChange;

use std::path::Path;


pub fn makeFileChange(status: &str, path: &Path) -> FileChange
{
    FileChange{status: status.into(), path: path.to_str().unwrap().into()}
}