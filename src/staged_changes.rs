#![allow(clippy::new_without_default)]

use crate::file_change::FileChange;

use std::ops::{Deref, DerefMut};


pub struct StagedChanges(pub Vec<FileChange>);

impl StagedChanges
{
    pub fn new() -> Self
    {
        Self{0: vec![]}
    }
}

impl Deref for StagedChanges
{
    type Target = Vec<FileChange>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
impl DerefMut for StagedChanges
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}