pub trait FileChangeStoreObserver
{
    fn onFilled(&self);
    fn onEmptied(&self);
}