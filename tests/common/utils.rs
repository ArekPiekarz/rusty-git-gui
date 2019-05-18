use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use gtk::TreeModelExt as _;
use tempfile::NamedTempFile;

pub struct FileInfo
{
    pub status: String,
    pub name: String
}

pub fn getFileName(file: &NamedTempFile) -> String
{
    file.path().file_name().unwrap().to_str().unwrap().to_string()
}

pub fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: FileStatusModelColumn) -> String
{
    model.get_value(iter, column as i32).downcast_ref::<String>().unwrap().get().unwrap()
}