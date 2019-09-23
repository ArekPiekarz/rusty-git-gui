#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{selectStagedChange};
use common::setup::{makeGui, makeNewStagedFile, modifyFile, setupTest};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn loadRepositoryWithModifiedUnstagedFileAndSameNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    modifyFile(&filePath, "staged file content\nmodified unstaged line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertUnstagedChangesViewContains(&[makeFileChange("WT_MODIFIED", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified unstaged line\n", &gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &filePath)], &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &gui);
}