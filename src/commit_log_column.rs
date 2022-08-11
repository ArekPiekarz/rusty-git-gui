pub(crate) enum CommitLogColumn
{
    Summary,
    Date,
    Author,
    Email,
    OriginalRow
}

impl CommitLogColumn
{
    pub const fn asArrayOfI32() -> [i32; 4]
    {
        [0, 1, 2, 3]
    }
}

#[allow(clippy::as_conversions)]
impl From<CommitLogColumn> for i32
{
    fn from(value: CommitLogColumn) -> Self
    {
        value as Self
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
