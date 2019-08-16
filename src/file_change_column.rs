use strum_macros::EnumCount;


#[derive(EnumCount)]
pub enum FileChangeColumn
{
    Status,
    Path
}

impl FileChangeColumn
{
    pub fn asArrayOfI32() -> [i32; FILECHANGECOLUMN_COUNT]
    {
        [0, 1]
    }

    pub fn asArrayOfU32() -> [u32; FILECHANGECOLUMN_COUNT]
    {
        [0, 1]
    }
}