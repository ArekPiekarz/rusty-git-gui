#![allow(clippy::new_without_default)]

use crate::file_change::FileChange;

use shrinkwraprs::Shrinkwrap;


#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct StagedChanges(pub Vec<FileChange>);

impl StagedChanges
{
    pub fn new() -> Self
    {
        Self{0: vec![]}
    }
}