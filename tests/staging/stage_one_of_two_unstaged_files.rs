use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::{activateUnstagedChangeInRow, selectStagedChangeInRow};
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{makeGui, makeNewUnstagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn stageOneOfTwoNewFiles()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath1 = PathBuf::from("fileName1");
    let filePath2 = PathBuf::from("fileName2");
    makeNewUnstagedFile(&filePath1, "file content 1\n", &repositoryDir);
    makeNewUnstagedFile(&filePath2, "file content 2\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath1, WorkTreeStatus(Untracked), IndexStatus(Untracked)),
          Entry::new(&filePath2, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &filePath1),
          makeFileChange("New", &filePath2)],
        &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content 1\n", &gui);

    activateUnstagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath1, WorkTreeStatus(Unmodified), IndexStatus(Added)),
          Entry::new(&filePath2, WorkTreeStatus(Untracked),  IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath2)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath1)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content 2\n", &gui);

    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content 1\n", &gui);
}
}
