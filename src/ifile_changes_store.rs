use crate::file_change::FileChange;
use crate::file_path::FilePathStr;


pub trait IFileChangesStore
{
    fn getFileChange(&self, row: usize) -> &FileChange;
    fn getFilePath(&self, row: usize) -> &FilePathStr;
    fn findFilePath(&self, path: &FilePathStr) -> Option<usize>;
    fn connectOnRefreshed(&mut self, handler: OnRefreshedHandler);
}

pub type OnRefreshedHandler = Box<dyn Fn(()) -> glib::Continue>;
