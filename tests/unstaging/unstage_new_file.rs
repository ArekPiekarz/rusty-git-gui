use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::{activateStagedChangeInRow, selectUnstagedChangeInRow};
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn unstageNewFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Added}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    activateStagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);
}
}