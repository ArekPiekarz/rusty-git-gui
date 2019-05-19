#![allow(non_snake_case)]

mod common;

use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains,
};
use common::setup::{getWindow, makeNewFile, setupTest};
use common::utils::{FileInfo, getFileName};
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewUnstagedFile()
{
    let repositoryDir = setupTest();
    let newUnstagedFile = makeNewFile(repositoryDir.path(), "unstaged file content");

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(repositoryDir.path())));

        let window = getWindow();
        assertUnstagedFilesViewContains(
            &[FileInfo{status: "WT_NEW".to_string(), name: getFileName(&newUnstagedFile)}],
            &window);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n\\ No newline at end of file\n", &window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}