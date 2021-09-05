use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertGuiIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::{clickRefreshButton, selectUnstagedChangeInRow};
use crate::common::repository_assertions::{
    assertRepositoryHasNoCommits,
    assertRepositoryIsEmpty,
    assertRepositoryStatusIs};
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
fn refreshEmptyRepositoryAfterNewFileIsCreated()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let gui = makeGui(&repositoryDir);
    assertRepositoryIsEmpty(&repositoryDir);
    assertGuiIsEmpty(&gui);

    let newUnstagedFilePath = PathBuf::from("unstagedFile");
    makeNewUnstagedFile(&newUnstagedFilePath, "unstaged file content\n", &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry::new(&newUnstagedFilePath, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &newUnstagedFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);

    selectUnstagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
}
}
