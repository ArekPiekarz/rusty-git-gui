use std::convert::TryInto as _;


pub type Row = usize;

pub fn toRow(rowPath: &gtk::TreePath) -> Row
{
    rowPath.get_indices()[0].try_into().unwrap()
}