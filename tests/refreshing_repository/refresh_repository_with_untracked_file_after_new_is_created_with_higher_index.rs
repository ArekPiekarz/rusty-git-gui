use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeGui, makeNewUnstagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn refreshRepositoryWithUntrackedFileAfterNewIsCreatedWithHigherIndex()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath0 = PathBuf::from("unstagedFile0");
    makeNewUnstagedFile(&filePath0, "unstaged file content 0\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath0.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath0)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 0\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    let filePath1 = PathBuf::from("unstagedFile1");
    makeNewUnstagedFile(&filePath1, "unstaged file content 1\n", &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath0.clone(), workTreeStatus: Untracked, indexStatus: Untracked},
            Entry{path: filePath1.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &filePath0),
          makeFileChange("New", &filePath1)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 0\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}
}
