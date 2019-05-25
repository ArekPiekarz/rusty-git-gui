use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use gtk::TreeModelExt as _;


// https://developer.gnome.org/gtk3/stable/GtkTreeModel.html#gtk-tree-model-foreach
pub const CONTINUE_ITERATING_MODEL: bool = false;
pub const STOP_ITERATING_MODEL: bool = true;


pub fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: FileStatusModelColumn) -> String
{
    model.get_value(iter, column as i32).downcast_ref::<String>().unwrap().get().unwrap()
}