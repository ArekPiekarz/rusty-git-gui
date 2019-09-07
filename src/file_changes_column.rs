use strum_macros::EnumCount;


#[derive(EnumCount)]
pub enum FileChangesColumn
{
    Status,
    Path
}

impl FileChangesColumn
{
    pub fn asArrayOfI32() -> [i32; FILECHANGESCOLUMN_COUNT]
    {
        [0, 1]
    }

    pub fn asArrayOfU32() -> [u32; FILECHANGESCOLUMN_COUNT]
    {
        [0, 1]
    }
}