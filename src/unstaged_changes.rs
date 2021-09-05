#![allow(clippy::new_without_default)]

use crate::file_change::FileChange;

use std::ops::{Deref, DerefMut};


#[derive(Debug)]
pub struct UnstagedChanges(pub Vec<FileChange>);

impl UnstagedChanges
{
    pub const fn new() -> Self
    {
        Self{0: vec![]}
    }
}

impl Deref for UnstagedChanges
{
    type Target = Vec<FileChange>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
impl DerefMut for UnstagedChanges
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}
