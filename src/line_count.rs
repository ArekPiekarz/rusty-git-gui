use std::ops::AddAssign;


pub struct LineCount(pub usize);

impl AddAssign::<usize> for LineCount
{
    fn add_assign(&mut self, rhs: usize)
    {
        self.0 += rhs;
    }
}

impl From<usize> for LineCount
{
    fn from(value: usize) -> Self
    {
        Self(value)
    }
}