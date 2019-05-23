#![allow(non_snake_case)]

mod common;

use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty,
};
use common::setup::{getWindow, makeNewFile, makeNewStagedFile, setupTest};
use common::utils::{FileInfo, getFileName};
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewStagedFile()
{
    let repositoryDir = setupTest();
    let newStagedFile = makeNewStagedFile(repositoryDir.path(), "staged file content");

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(repositoryDir.path())));

        let window = getWindow();
        assertStagedFilesViewContains(
            &[FileInfo{status: "INDEX_NEW".to_string(), name: getFileName(&newStagedFile)}],
            &window);
        assertDiffViewIsEmpty(&window);
        assertUnstagedFilesViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}