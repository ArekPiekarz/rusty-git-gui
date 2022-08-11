use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub(crate) struct Color(pub &'static str);

impl Display for Color
{
    fn fmt(&self, formatter: &mut Formatter) -> Result
    {
        write!(formatter, "{}", self.0)
    }
}
