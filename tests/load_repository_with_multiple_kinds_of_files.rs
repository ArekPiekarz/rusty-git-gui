#![allow(non_snake_case)]

mod common;

use common::actions::{selectStagedFile, selectUnstagedFile};
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewContains};
use common::setup::{
    makeCommit,
    makeNewStagedFile,
    makeNewUnstagedFile,
    modifyFile,
    setupTest,
    stageFile};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


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

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));
    gui.show();

    assertUnstagedFilesViewContains(
        &[FileInfo::new("WT_NEW", &newUnstagedFilePath),
          FileInfo::new("WT_MODIFIED", &modifiedUnstagedFilePath)],
        &gui);
    assertStagedFilesViewContains(
        &[FileInfo::new("INDEX_MODIFIED", &modifiedStagedFilePath),
          FileInfo::new("INDEX_NEW", &newStagedFilePath)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+new unstaged file content\n", &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectUnstagedFile(&modifiedUnstagedFilePath, &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n new staged file content\n+modified unstaged line\n", &gui);
    selectStagedFile(&modifiedStagedFilePath, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
    selectStagedFile(&newStagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+new staged file content\n", &gui);
}