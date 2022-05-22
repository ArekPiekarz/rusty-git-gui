pub(crate) trait SelectionsComparer
{
    fn setFirst(&mut self, selection: &gtk::TreeSelection);
    fn setSecond(&mut self, selection: &gtk::TreeSelection);
    fn areDifferent(&self) -> bool;
}
