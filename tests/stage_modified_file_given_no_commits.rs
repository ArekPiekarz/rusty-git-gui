#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::{activateUnstagedChange, selectStagedChange};
use common::setup::{makeNewStagedFile, modifyFile, setupTest};
use common::utils::makeFileChange;

use rusty_git_gui::gui::Gui;
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

    let gui = Gui::new(Rc::new(Repository::new(&repositoryDir)));
    gui.show();

    assertUnstagedChangesViewContains(&[makeFileChange("WT_MODIFIED", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified line\n", &gui);

    activateUnstagedChange(&filePath, &gui);

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1,2 @@\n+staged file content\n+modified line\n", &gui);
}