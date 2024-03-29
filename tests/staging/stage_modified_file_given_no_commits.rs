use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::{activateUnstagedChangeInRow, selectStagedChangeInRow};
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{makeGui, makeNewStagedFile, modifyFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn stageModifiedFileGivenNoCommits()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    modifyFile(&filePath, "staged file content\nmodified line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Modified), IndexStatus(Added))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified line\n", &gui);

    activateUnstagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Unmodified), IndexStatus(Added))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -0,0 +1,2 @@\n+staged file content\n+modified line\n", &gui);
}
}
