use crate::file_change::FileChange;


pub trait FileChangeViewObserver
{
    fn onFilled(&self) {}
    fn onEmptied(&self) {}
    fn onSelected(&self, _: &FileChange) {}
    fn onDeselected(&self) {}
    fn onActivated(&self, _: &FileChange) {}
}