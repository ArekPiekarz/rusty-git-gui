use crate::file_change::FileChange;


pub trait FileChangeViewObserver
{
    fn onSelected(&self, _: &FileChange) {}
    fn onDeselected(&self) {}
    fn onActivated(&self, _: &FileChange) {}
}