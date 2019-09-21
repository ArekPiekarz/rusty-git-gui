#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{selectStagedChange, selectUnstagedChange, show};
use common::setup::{
    makeCommit,
    makeGui,
    makeNewStagedFile,
    makeNewUnstagedFile,
    modifyFile,
    setupTest,
    stageFile};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn loadRepositoryWithMultipleKindsOfFiles()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let modifiedStagedFilePath = PathBuf::from("fileName1");
    makeNewStagedFile(&modifiedStagedFilePath, "some file content\nsecond line\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&modifiedStagedFilePath, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&modifiedStagedFilePath, &repositoryDir);

    let newUnstagedFilePath = PathBuf::from("fileName2");
    makeNewUnstagedFile(&newUnstagedFilePath, "new unstaged file content\n", &repositoryDir);

    let newStagedFilePath = PathBuf::from("fileName3");
    makeNewStagedFile(&newStagedFilePath, "new staged file content\n", &repositoryDir);
    let modifiedUnstagedFilePath = newStagedFilePath.clone();
    modifyFile(&modifiedUnstagedFilePath, "new staged file content\nmodified unstaged line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    show(&gui);

    assertUnstagedChangesViewContains(
        &[makeFileChange("WT_NEW", &newUnstagedFilePath),
          makeFileChange("WT_MODIFIED", &modifiedUnstagedFilePath)],
        &gui);
    assertStagedChangesViewContains(
        &[makeFileChange("INDEX_MODIFIED", &modifiedStagedFilePath),
          makeFileChange("INDEX_NEW", &newStagedFilePath)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+new unstaged file content\n", &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectUnstagedChange(&modifiedUnstagedFilePath, &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n new staged file content\n+modified unstaged line\n", &gui);
    selectStagedChange(&modifiedStagedFilePath, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
    selectStagedChange(&newStagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+new staged file content\n", &gui);
}