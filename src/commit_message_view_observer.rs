pub trait CommitMessageViewObserver
{
    fn onFilled(&self);
    fn onEmptied(&self);
}