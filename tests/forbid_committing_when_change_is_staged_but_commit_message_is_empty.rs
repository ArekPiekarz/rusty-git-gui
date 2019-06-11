#![allow(non_snake_case)]

mod common;

use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipContains,
    assertCommitMessageViewIsEmpty};
use common::setup::{getWindow, makeNewStagedFile, setupTest};
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;

use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn forbidCommittingWhenChangeIsStagedButCommitMessageIsEmpty()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
        assertCommitButtonTooltipContains("The commit message is empty.", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}