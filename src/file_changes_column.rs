pub enum FileChangesColumn
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

    pub const fn asArrayOfU32() -> [u32; FILE_CHANGES_COLUMN_COUNT]
    {
        [0, 1]
    }
}