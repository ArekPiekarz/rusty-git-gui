#![allow(non_snake_case)]

mod common;

use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
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
fn loadRepositoryWithNewUnstagedEmptyFile()
{
    let repositoryDir = setupTest();
    let newUnstagedFile = makeNewFile(repositoryDir.path());

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(repositoryDir.path())));

        let window = getWindow();
        assertUnstagedFilesViewContains(
            &window,
            &[FileInfo{status: "WT_NEW".to_string(), name: getFileName(&newUnstagedFile)}]);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}