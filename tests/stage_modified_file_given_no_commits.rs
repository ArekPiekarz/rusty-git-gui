#![allow(non_snake_case)]

mod common;

use common::actions::{activateUnstagedFile, selectStagedFile};
use common::assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty};
use common::setup::{getWindow, makeNewStagedFile, modifyFile, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn stageModifiedFileGivenNoCommits()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    modifyFile(&filePath, "staged file content\nmodified line\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertUnstagedFilesViewContains(&[FileInfo::new("WT_MODIFIED", &filePath)], &window);
        assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &window);
        assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified line\n", &window);

        activateUnstagedFile(&filePath, &window);

        assertUnstagedFilesViewIsEmpty(&window);
        assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &window);
        assertDiffViewIsEmpty(&window);

        selectStagedFile(&filePath, &window);
        assertDiffViewContains("@@ -0,0 +1,2 @@\n+staged file content\n+modified line\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}