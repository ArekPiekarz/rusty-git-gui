pub(crate) enum FileChangesColumn
{
    Status,
    Path
}

const FILE_CHANGES_COLUMN_COUNT: usize = 2;

impl FileChangesColumn
{
    pub const fn asArrayOfI32() -> [i32; FILE_CHANGES_COLUMN_COUNT]
    {
        [0, 1]
    }
}

#[allow(clippy::as_conversions)]
impl From<FileChangesColumn> for i32
{
    fn from(value: FileChangesColumn) -> Self
    {
        value as Self
    }
}

#[allow(clippy::as_conversions)]
impl From<FileChangesColumn> for u32
{
    fn from(value: FileChangesColumn) -> Self
    {
        value as Self
    }
}
