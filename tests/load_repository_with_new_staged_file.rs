#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::selectStagedChange;
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;


#[test]
fn loadRepositoryWithNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newStagedFilePath = PathBuf::from("stagedFile");
    makeNewStagedFile(&newStagedFilePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: newStagedFilePath.clone(), workTreeStatus: Unmodified, indexStatus: Added}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertStagedChangesViewContains(&[makeFileChange("New", &newStagedFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChange(&newStagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &gui);
}