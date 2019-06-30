#![allow(non_snake_case)]

mod common;

use common::actions::{activateUnstagedFile, selectStagedFile};
use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty};
use common::setup::{makeNewStagedFile, modifyFile, setupTest};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

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

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));
    gui.show();

    assertUnstagedFilesViewContains(&[FileInfo::new("WT_MODIFIED", &filePath)], &gui);
    assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified line\n", &gui);

    activateUnstagedFile(&filePath, &gui);

    assertUnstagedFilesViewIsEmpty(&gui);
    assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedFile(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1,2 @@\n+staged file content\n+modified line\n", &gui);
}