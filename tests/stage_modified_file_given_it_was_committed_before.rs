#![allow(non_snake_case)]

mod common;

use common::actions::{activateUnstagedFile, selectStagedFile};
use common::assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty};
use common::setup::{getWindow, makeCommit, makeNewStagedFile, modifyFile, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn stageModifiedFileGivenItWasCommittedBefore()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir,);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertUnstagedFilesViewContains(&[FileInfo::new("WT_MODIFIED", &filePath)], &window);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &window);

        activateUnstagedFile(&filePath, &window);

        assertUnstagedFilesViewIsEmpty(&window);
        assertStagedFilesViewContains(&[FileInfo::new("INDEX_MODIFIED", &filePath)], &window);
        assertDiffViewIsEmpty(&window);

        selectStagedFile(&filePath, &window);
        assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}