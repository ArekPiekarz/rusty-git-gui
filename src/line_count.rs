use derive_more::From;
use std::ops::AddAssign;


#[derive(From)]
pub struct LineCount(pub usize);

impl AddAssign::<usize> for LineCount
{
    fn add_assign(&mut self, rhs: usize)
    {
        self.0 += rhs;
    }
}