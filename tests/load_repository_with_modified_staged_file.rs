#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::selectStagedChange;
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn loadRepositoryWithModifiedStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertStagedChangesViewContains(&[makeFileChange("INDEX_MODIFIED", &filePath)], &gui);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
}