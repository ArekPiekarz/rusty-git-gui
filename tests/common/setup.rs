use crate::common::gui_interactions::show;
use crate::common::test_gui::TestGui;

use rusty_git_gui::app_setup::{setupGtk, setupPanicHandler};
use rusty_git_gui::gui::Gui;

use gtk::glib::object::Cast as _;
use gtk::glib::ObjectExt as _;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::{tempdir, TempDir};


pub fn setupTest() -> TempDir
{
    setupPanicHandler();
    setupGtk();
    let repositoryDir = makeTemporaryDirectory();
    initializeGitRepository(repositoryDir.path());
    repositoryDir
}

pub fn makeGui(repositoryDir: &Path) -> TestGui
{
    let gui = Gui::new(repositoryDir);
    show(&gui);
    TestGui::new(getAppWindow())
}

pub fn makeNewUnstagedFile(filePath: &Path, content: &str, repositoryDir: &Path)
{
    let mut file = makeNewWritableFile(&repositoryDir.join(filePath));
    file.write(content.as_bytes()).unwrap();
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

pub fn makeSubdirectory(subdir: &Path, repositoryDir: &Path)
{
    std::fs::create_dir(repositoryDir.join(subdir)).unwrap()
}

pub fn removeFile(filePath: &Path, repositoryDir: &Path)
{
    std::fs::remove_file(repositoryDir.join(filePath)).unwrap();
}

pub fn renameFile(oldFilePath: &Path, newFilePath: &Path, repositoryDir: &Path)
{
    std::fs::rename(repositoryDir.join(oldFilePath), repositoryDir.join(newFilePath)).unwrap();
}


// private

fn getAppWindow() -> gtk::ApplicationWindow
{
    let mut topLevelWindows = gtk::Window::list_toplevels();

    match topLevelWindows.len() {
        1 => topLevelWindows.remove(0).downcast::<gtk::ApplicationWindow>().unwrap(),
        2 => {
            let tooltipWindow = topLevelWindows[1].downcast_ref::<gtk::Window>().unwrap();
            assert_eq!(tooltipWindow.type_().name(), "GtkTooltipWindow");
            topLevelWindows.remove(0).downcast::<gtk::ApplicationWindow>().unwrap()
        },
        count => panic!("Wrong number of windows, expected 1 or 2, got {}: {:?}", count, topLevelWindows)
    }
}

fn makeTemporaryDirectory() -> TempDir
{
    tempdir().unwrap_or_else(|e| panic!("Failed to create temporary directory: {}", e))
}

fn initializeGitRepository(repositoryDir: &Path)
{
    initializeGitRepositoryWith(&["git", "init", "--initial-branch", "main"], repositoryDir);
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

fn makeNewWritableFile(filePath: &Path) -> File
{
    OpenOptions::new().write(true).create_new(true).open(filePath).unwrap()
}

fn openExistingFileForWriting(filePath: &Path) -> File
{
    OpenOptions::new().write(true).create_new(false).open(filePath).unwrap()
}
