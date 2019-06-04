use std::path::Path;

pub struct FileInfo<'a>
{
    pub status: &'a str,
    pub path: &'a str
}

impl<'a> FileInfo<'a>
{
    pub fn new(status: &'a str, path: &'a Path) -> Self
    {
        Self{status: status, path: path.to_str().unwrap()}
    }
}
