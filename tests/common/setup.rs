use gtk::WidgetExt as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::ops::Deref;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;


pub const NO_FILE_CONTENT : &str = "";


pub fn setupTest() -> tempfile::TempDir
{
    color_backtrace::install();
    let repositoryDir = makeTemporaryDirectory();
    initializeGitRepository(repositoryDir.path());
    repositoryDir
}

fn makeTemporaryDirectory() -> tempfile::TempDir
{
    tempfile::tempdir().unwrap_or_else(|e| panic!("Failed to create temporary directory: {}", e))
}

fn initializeGitRepository(repositoryDir: &Path)
{
    let status = Command::new("git").arg("init")
        .current_dir(&repositoryDir).stdout(Stdio::null()).status().unwrap();
    assert_eq!(true, status.success(),
               r#"Failed to initialize git repository in path "{}", command finished with {}"#,
               repositoryDir.to_string_lossy(), status);
}

pub fn makeNewFile(directory: &Path, content: &str) -> NamedTempFile
{
    let mut file = NamedTempFile::new_in(directory).unwrap();
    file.write(content.as_bytes()).unwrap();
    file
}

pub fn makeNewStagedFile(directory: &Path, content: &str) -> NamedTempFile
{
    let file = makeNewFile(directory, content);
    stageFile(file.path(), directory);
    file
}

fn stageFile(filePath: &Path, repositoryDir: &Path)
{
    let status = Command::new("git").args(&["add", filePath.to_str().unwrap()])
        .current_dir(&repositoryDir).status().unwrap();
    assert_eq!(true, status.success(),
               r#"Failed to stage file "{}", command finished with {}"#, filePath.to_string_lossy(), status);
}

pub fn makeRelativePath(file: &NamedTempFile, repositoryDir: &Path) -> String
{
    file.path().strip_prefix(repositoryDir).unwrap().to_str().unwrap().to_string()
}

pub fn makeCommit(message: &str, repositoryDir: &Path)
{
    let status = Command::new("git").args(&["commit", "-m", message])
        .current_dir(&repositoryDir).stdout(Stdio::null()).status().unwrap();
    assert_eq!(true, status.success(),
               r#"Failed to create a commit with message "{}", command finished with {}"#, message, status);
}

pub fn modifyFile(filePath: &str, newContent: &str, repositoryDir: &Path)
{
    let mut file = OpenOptions::new().write(true).create_new(false).open(repositoryDir.join(filePath)).unwrap();
    file.write(newContent.as_bytes()).unwrap();
}

pub fn getWindow() -> ScopedWindow
{
    let mut topLevelWindows = gtk::Window::list_toplevels();
    assert_eq!(topLevelWindows.len(), 1);
    ScopedWindow::new(topLevelWindows.remove(0))
}

pub struct ScopedWindow
{
    window: gtk::Widget
}

impl ScopedWindow
{
    fn new(window: gtk::Widget) -> Self
    {
        ScopedWindow{window}
    }
}

impl Drop for ScopedWindow
{
    fn drop(&mut self)
    {
        self.window.destroy();
    }
}

impl Deref for ScopedWindow
{
    type Target = gtk::Widget;

    fn deref(&self) -> &gtk::Widget
    {
        &self.window
    }
}