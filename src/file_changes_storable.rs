pub trait FileChangesStorable
{
    fn remove(&self, iterator: &gtk::TreeIter);
}