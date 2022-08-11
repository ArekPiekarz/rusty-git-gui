pub(crate) type Row = usize;

#[must_use]
pub(crate) fn toRow(rowPath: &gtk::TreePath) -> Row
{
    rowPath.indices()[0].try_into().unwrap()
}
