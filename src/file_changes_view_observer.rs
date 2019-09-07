use crate::file_change::FileChange;


pub trait FileChangesViewObserver
{
    fn onSelected(&self, _: &FileChange) {}
    fn onDeselected(&self) {}
    fn onActivated(&self, _: &FileChange) {}
}