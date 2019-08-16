pub trait TextViewObserver
{
    fn onFilled(&self);
    fn onEmptied(&self);
}