use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{makeGui, makeNewUnstagedFile, removeFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn refreshRepositoryWithThreeUntrackedFilesAfterFirstIsRemoved()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath0 = PathBuf::from("unstagedFile0");
    let filePath1 = PathBuf::from("unstagedFile1");
    let filePath2 = PathBuf::from("unstagedFile2");
    makeNewUnstagedFile(&filePath0, "unstaged file content 0\n", &repositoryDir);
    makeNewUnstagedFile(&filePath1, "unstaged file content 1\n", &repositoryDir);
    makeNewUnstagedFile(&filePath2, "unstaged file content 2\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath0, WorkTreeStatus(Untracked), IndexStatus(Untracked)),
          Entry::new(&filePath1, WorkTreeStatus(Untracked), IndexStatus(Untracked)),
          Entry::new(&filePath2, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &filePath0),
          makeFileChange("New", &filePath1),
          makeFileChange("New", &filePath2)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 0\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    removeFile(&filePath0, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath1, WorkTreeStatus(Untracked), IndexStatus(Untracked)),
          Entry::new(&filePath2, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &filePath1),
          makeFileChange("New", &filePath2)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 1\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}
}
