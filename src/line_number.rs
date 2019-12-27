use crate::line_count::LineCount;

use derive_more::From;
use std::convert::TryInto;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Clone, Copy, From)]
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

impl Into::<i32> for LineNumber
{
    fn into(self) -> i32
    {
        self.0.try_into().unwrap()
    }
}