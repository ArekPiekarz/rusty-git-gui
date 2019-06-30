use crate::gui_setup::FileChangesStore;

use std::rc::Rc;
use strum_macros::EnumCount;


#[derive(EnumCount)]
pub enum FileChangesColumn
{
    Status,
    Path
}

pub struct StagingSwitchStores
{
    pub source: Rc<FileChangesStore>,
    pub target: Rc<FileChangesStore>
}


pub const EXCLUDE_HIDDEN_CHARACTERS : bool = false;

pub const FILE_CHANGES_COLUMNS_I32: [i32; FILECHANGESCOLUMN_COUNT] = [0, 1];
pub const FILE_CHANGES_COLUMNS_U32: [u32; FILECHANGESCOLUMN_COUNT] = [0, 1];

// https://developer.gnome.org/gtk3/stable/GtkTreeModel.html#gtk-tree-model-foreach
pub const CONTINUE_ITERATING_MODEL: bool = false;
pub const STOP_ITERATING_MODEL: bool = true;