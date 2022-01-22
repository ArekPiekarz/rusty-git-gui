pub enum CommitLogColumn
{
    Summary,
    Date,
    Author,
    Email
}

impl CommitLogColumn
{
    pub const fn asArrayOfI32() -> [i32; 4]
    {
        [0, 1, 2, 3]
    }
}

#[allow(clippy::as_conversions)]
impl From<CommitLogColumn> for u32
{
    fn from(value: CommitLogColumn) -> Self
    {
        value as Self
    }
}