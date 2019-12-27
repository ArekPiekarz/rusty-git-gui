use crate::line_count::LineCount;

use std::convert::TryInto;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Clone, Copy)]
pub struct LineNumber(pub usize);

impl Add::<LineCount> for LineNumber
{
    type Output = Self;

    fn add(self, rhs: LineCount) -> Self
    {
        LineNumber(self.0 + rhs.0)
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
    fn from(value: usize) -> LineNumber
    {
        LineNumber(value)
    }
}

impl From::<LineNumber> for i32
{
    fn from(value: LineNumber) -> i32
    {
        value.0.try_into().unwrap()
    }
}