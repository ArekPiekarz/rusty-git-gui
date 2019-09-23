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
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn stageModifiedFileGivenItWasCommittedBefore()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir,);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    show(&gui);

    assertUnstagedChangesViewContains(&[makeFileChange("WT_MODIFIED", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);

    activateUnstagedChangeToStageIt(&filePath, &gui);

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_MODIFIED", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
}