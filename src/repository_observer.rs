use crate::file_change::FileChange;


pub trait RepositoryObserver
{
    fn onStaged(&self, _: &FileChange) {}
    fn onUnstaged(&self, _: &FileChange) {}
    fn onCommitted(&self) {}
}