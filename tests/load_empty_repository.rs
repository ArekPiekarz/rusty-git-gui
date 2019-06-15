#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewIsEmpty,
};
use common::setup::{getWindow, setupTest};
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::rc::Rc;


#[test]
fn loadEmptyRepository()
{
    let repositoryDir = setupTest();

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(repositoryDir.path())));

        let window = getWindow();
        assertUnstagedFilesViewIsEmpty(&window);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}