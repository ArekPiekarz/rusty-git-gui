#![allow(clippy::new_without_default)]

use crate::staged_changes::StagedChanges;
use crate::unstaged_changes::UnstagedChanges;

pub struct GroupedFileChanges
{
    pub unstaged: UnstagedChanges,
    pub staged: StagedChanges
}

impl GroupedFileChanges
{
    pub fn new() -> Self
    {
        Self{unstaged: UnstagedChanges::new(), staged: StagedChanges::new()}
    }
}