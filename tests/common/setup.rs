use gtk::WidgetExt as _;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::ops::Deref;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::{tempdir, TempDir};

pub fn setupTest() -> TempDir
{
    color_backtrace::install();
    let repositoryDir = makeTemporaryDirectory();
    initializeGitRepository(repositoryDir.path());
    repositoryDir
}

fn makeTemporaryDirectory() -> TempDir
{
    tempdir().unwrap_or_else(|e| panic!("Failed to create temporary directory: {}", e))
}

fn initializeGitRepository(repositoryDir: &Path)
{
    initializeGitRepositoryWith(&["git", "init"], repositoryDir);
    initializeGitRepositoryWith(&["git", "config", "user.name", "John Smith"], repositoryDir);
    initializeGitRepositoryWith(&["git", "config", "user.email", "john.smith@example.com"], repositoryDir);
}

fn initializeGitRepositoryWith(commandParts: &[&str], repositoryDir: &Path)
{
    let mut command = Command::new(commandParts[0]);
    command.args(&commandParts[1..]).current_dir(&repositoryDir).stdout(Stdio::null());
    let status = command.status().unwrap();
    assert_eq!(true, status.success(),
               "Failed to initialize git repository.\nPath: {}\nCommand: {:?}\nCommand status: {}",
               repositoryDir.to_string_lossy(), command, status);
}

pub fn makeNewUnstagedFile(filePath: &Path, content: &str, repositoryDir: &Path)
{
    let mut file = makeNewWritableFile(&repositoryDir.join(filePath));
    file.write(content.as_bytes()).unwrap();
}

fn makeNewWritableFile(filePath: &Path) -> File
{
    OpenOptions::new().write(true).create_new(true).open(filePath).unwrap()
}

pub fn makeNewUnstagedEmptyFile(filePath: &Path, repositoryDir: &Path)
{
    makeNewWritableFile(&repositoryDir.join(filePath));
}

pub fn makeNewStagedFile(filePath: &Path, content: &str, repositoryDir: &Path)
{
    makeNewUnstagedFile(filePath, content, repositoryDir);
    stageFile(filePath, repositoryDir);
}

pub fn stageFile(filePath: &Path, repositoryDir: &Path)
{
    let status = Command::new("git").args(&["add", filePath.to_str().unwrap()])
        .current_dir(&repositoryDir).status().unwrap();
    assert_eq!(true, status.success(),
               r#"Failed to stage file "{}", command finished with {}"#, filePath.to_string_lossy(), status);
}

pub fn makeCommit(message: &str, repositoryDir: &Path)
{
    let status = Command::new("git").args(&["commit", "-m", message])
        .current_dir(&repositoryDir).stdout(Stdio::null()).status().unwrap();
    assert_eq!(true, status.success(),
               r#"Failed to create a commit with message "{}", command finished with {}"#, message, status);
}

pub fn modifyFile(filePath: &Path, newContent: &str, repositoryDir: &Path)
{
    let mut file = openExistingFileForWriting(&repositoryDir.join(filePath));
    file.write(newContent.as_bytes()).unwrap();
}

fn openExistingFileForWriting(filePath: &Path) -> File
{
    OpenOptions::new().write(true).create_new(false).open(filePath).unwrap()
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