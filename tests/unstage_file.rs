#![allow(non_snake_case)]

mod common;

use common::actions::{activateStagedFile, selectUnstagedFile};
use common::assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty
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
fn unstageFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "file content\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertUnstagedFilesViewIsEmpty(&window);
        assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &window);
        assertDiffViewIsEmpty(&window);

        activateStagedFile(&filePath, &window);

        assertUnstagedFilesViewContains(&[FileInfo::new("WT_NEW", &filePath)], &window);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewIsEmpty(&window);

        selectUnstagedFile(&filePath, &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}