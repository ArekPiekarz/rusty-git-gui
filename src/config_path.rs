use std::path::{Path, PathBuf};


#[derive(Debug)]
pub(crate) struct ConfigPath
{
    dirPath: PathBuf,
    filePath: PathBuf
}

impl ConfigPath
{
    pub fn getDirPath(&self) -> &Path
    {
        &self.dirPath
    }

    pub fn getFilePath(&self) -> &Path
    {
        &self.filePath
    }
}

impl Default for ConfigPath
{
    fn default() -> Self
    {
        let mut dirPath = dirs::config_dir().unwrap();
        dirPath.push("rusty-git-gui");
        let mut filePath = dirPath.clone();
        filePath.push("config.toml");
        Self{dirPath, filePath}
    }
}
