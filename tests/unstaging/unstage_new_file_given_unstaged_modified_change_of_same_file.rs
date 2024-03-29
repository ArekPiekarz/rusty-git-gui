use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::{activateStagedChangeInRow, selectUnstagedChangeInRow};
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
fn unstageNewFileGivenUnstagedModifiedChangeOfSameFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Modified), IndexStatus(Added))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);

    activateStagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -0,0 +1,2 @@\n+some file content\n+modified second line\n", &gui);
}
}
