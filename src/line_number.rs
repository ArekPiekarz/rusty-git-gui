use crate::line_count::LineCount;

use std::ops::Add;
use std::ops::AddAssign;


#[derive(Clone, Copy)]
pub(crate) struct LineNumber(pub usize);

impl Add::<LineCount> for LineNumber
{
    type Output = Self;

    fn add(self, rhs: LineCount) -> Self
    {
        Self(self.0 + rhs.0)
    }
}

impl Add::<usize> for LineNumber
{
    type Output = Self;

    fn add(self, rhs: usize) -> Self
    {
        Self(self.0 + rhs)
    }
}

impl AddAssign::<usize> for LineNumber
{
    fn add_assign(&mut self, rhs: usize)
    {
        self.0 += rhs;
    }
}

impl From<usize> for LineNumber
{
    fn from(value: usize) -> Self
    {
        Self(value)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From::<LineNumber> for i32
{
    fn from(value: LineNumber) -> Self
    {
        value.0.try_into().unwrap()
    }
}
