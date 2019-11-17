use crate::file_change::FileChange;

pub trait FileChangesGetter
{
    fn getFileChange(&self, row: &gtk::TreePath) -> &FileChange;
}