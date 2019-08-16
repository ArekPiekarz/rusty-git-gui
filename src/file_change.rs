use shrinkwraprs::Shrinkwrap;


#[derive(Debug, PartialEq)]
pub struct FileChange
{
    pub status: String,
    pub path: String
}

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct UnstagedFileChanges(pub Vec<FileChange>);

impl UnstagedFileChanges
{
    pub fn new() -> Self
    {
        Self{0: vec![]}
    }
}

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct StagedFileChanges(pub Vec<FileChange>);

impl StagedFileChanges
{
    pub fn new() -> Self
    {
        Self{0: vec![]}
    }
}

pub struct GroupedFileChanges
{
    pub unstaged: UnstagedFileChanges,
    pub staged: StagedFileChanges
}
