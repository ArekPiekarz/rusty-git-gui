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
use common::setup::{makeCommit, makeNewStagedFile, modifyFile, setupTest, stageFile};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn unstageModifiedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertUnstagedFilesViewIsEmpty(&gui);
    assertStagedFilesViewContains(&[FileInfo::new("INDEX_MODIFIED", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    activateStagedFile(&filePath, &gui);

    assertUnstagedFilesViewContains(&[FileInfo::new("WT_MODIFIED", &filePath)], &gui);
    assertStagedFilesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedFile(&filePath, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
}