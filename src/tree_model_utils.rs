use std::convert::TryInto as _;


pub type Row = usize;

#[must_use]
pub fn toRow(rowPath: &gtk::TreePath) -> Row
{
    rowPath.indices()[0].try_into().unwrap()
}
