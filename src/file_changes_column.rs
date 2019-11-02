use strum_macros::EnumCount;


#[derive(EnumCount)]
pub enum FileChangesColumn
{
    Status,
    Path
}

impl FileChangesColumn
{
    pub const fn asArrayOfI32() -> [i32; FILECHANGESCOLUMN_COUNT]
    {
        [0, 1]
    }

    pub const fn asArrayOfU32() -> [u32; FILECHANGESCOLUMN_COUNT]
    {
        [0, 1]
    }
}