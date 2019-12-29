// https://developer.gnome.org/gtk3/stable/GtkTreeModel.html#gtk-tree-model-foreach
pub const CONTINUE_ITERATING_MODEL: bool = false;
pub const STOP_ITERATING_MODEL: bool = true;

pub fn toRow(rowPath: &gtk::TreePath) -> usize
{
    *rowPath.get_indices().get(0).unwrap() as usize
}