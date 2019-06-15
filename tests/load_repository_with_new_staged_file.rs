#![allow(non_snake_case)]

mod common;

use common::actions::selectStagedFile;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty,
};
use common::setup::{getWindow, makeNewStagedFile, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newStagedFilePath = PathBuf::from("stagedFile");
    makeNewStagedFile(&newStagedFilePath, "staged file content\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &newStagedFilePath)], &window);
        assertDiffViewIsEmpty(&window);
        assertUnstagedFilesViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);

        selectStagedFile(&newStagedFilePath, &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}