#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains,
};
use common::setup::{getWindow, makeNewUnstagedFile, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewUnstagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newUnstagedFilePath = PathBuf::from("unstagedFile");
    makeNewUnstagedFile(&newUnstagedFilePath, "unstaged file content\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertUnstagedFilesViewContains(&[FileInfo::new("WT_NEW", &newUnstagedFilePath)], &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &window);
        assertStagedFilesViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}