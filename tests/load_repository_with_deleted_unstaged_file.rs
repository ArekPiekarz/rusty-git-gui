#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains};
use common::gui_interactions::selectStagedChange;
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewStagedFile, removeFile, setupTest};

use std::path::PathBuf;


#[test]
fn loadRepositoryWithDeletedUnstagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("someFile");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    removeFile(&filePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Deleted, indexStatus: Added}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Deleted", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +0,0 @@\n-some file content\n", &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+some file content\n", &gui);
}