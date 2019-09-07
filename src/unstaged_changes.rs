use crate::file_change::FileChange;

use shrinkwraprs::Shrinkwrap;


#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct UnstagedChanges(pub Vec<FileChange>);

impl UnstagedChanges
{
    pub fn new() -> Self
    {
        Self{0: vec![]}
    }
}