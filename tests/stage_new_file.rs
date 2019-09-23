#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::{activateUnstagedChangeToStageIt, selectStagedChange, show};
use common::setup::{makeGui, makeNewUnstagedFile, setupTest};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn stageNewFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewUnstagedFile(&filePath, "file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    show(&gui);

    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);

    activateUnstagedChangeToStageIt(&filePath, &gui);

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);
}