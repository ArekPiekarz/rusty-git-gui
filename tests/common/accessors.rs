use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use gtk::TreeModelExt as _;


pub fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: FileStatusModelColumn) -> String
{
    model.get_value(iter, column as i32).get::<String>().unwrap()
}

pub fn getFirstRowCell(model: &gtk::TreeModel, column: FileStatusModelColumn) -> String
{
    model.get_value(&model.get_iter_first().unwrap(), column as i32).get::<String>().unwrap()
}