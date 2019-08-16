use crate::repository::Repository;

use std::rc::Rc;


pub trait DiffMaker
{
    fn makeDiff<'a>(&'a self, path: &str) -> git2::Diff<'a>;
}

pub struct UnstagedDiffMaker
{
    repository: Rc<Repository>
}

pub struct StagedDiffMaker
{
    repository: Rc<Repository>
}

impl UnstagedDiffMaker
{
    pub fn new(repository: Rc<Repository>) -> Self
    {
        Self{repository}
    }
}

impl StagedDiffMaker
{
    pub fn new(repository: Rc<Repository>) -> Self
    {
        Self{repository}
    }
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