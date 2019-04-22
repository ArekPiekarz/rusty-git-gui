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
