#![allow(non_snake_case)]

mod common;

use common::actions::{activateStagedFile, selectUnstagedFile};
use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty};
use common::setup::{makeNewStagedFile, setupTest};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn unstageNewFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "file content\n", &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertUnstagedFilesViewIsEmpty(&gui);
    assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    activateStagedFile(&filePath, &gui);

    assertUnstagedFilesViewContains(&[FileInfo::new("WT_NEW", &filePath)], &gui);
    assertStagedFilesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedFile(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);
}