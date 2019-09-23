#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::{activateStagedChangeToUnstageIt, selectUnstagedChange};
use common::setup::{makeGui, makeNewStagedFile, setupTest};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn unstageNewFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    activateStagedChangeToUnstageIt(&filePath, &gui);

    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);
}