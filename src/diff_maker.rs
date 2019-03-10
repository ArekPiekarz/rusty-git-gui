use crate::repository::*;
use std::rc::Rc;

pub trait DiffMaker
{
    fn makeDiff<'a>(&'a self, path: &str) -> git2::Diff<'a>;
}

pub struct UnstagedDiffMaker
{
    pub repository: Rc<Repository>
}

pub struct StagedDiffMaker
{
    pub repository: Rc<Repository>
}

impl DiffMaker for UnstagedDiffMaker
{
    fn makeDiff<'a>(&'a self, path: &str) -> git2::Diff<'a> {
        self.repository.makeDiffOfIndexToWorkdir(path)
    }
}

impl DiffMaker for StagedDiffMaker
{
    fn makeDiff<'a>(&'a self, path: &str) -> git2::Diff<'a> {
        self.repository.makeDiffOfTreeToIndex(path)
    }
}