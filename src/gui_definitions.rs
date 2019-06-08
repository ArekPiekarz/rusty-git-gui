use std::rc::Rc;


pub enum FileStatusModelColumn
{
    Status,
    Path
}

pub struct StagingAreaChangeModels
{
    pub source: Rc<gtk::ListStore>,
    pub target: Rc<gtk::ListStore>
}


pub const EXCLUDE_HIDDEN_CHARACTERS : bool = false;
pub const FILE_STATUS_MODEL_COLUMN_INDICES: [u32; 2] = [
    FileStatusModelColumn::Status as u32,
    FileStatusModelColumn::Path as u32];

// https://developer.gnome.org/gtk3/stable/GtkTreeModel.html#gtk-tree-model-foreach
pub const CONTINUE_ITERATING_MODEL: bool = false;
pub const STOP_ITERATING_MODEL: bool = true;