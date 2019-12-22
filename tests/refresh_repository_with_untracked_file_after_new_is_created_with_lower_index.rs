#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::gui_interactions::clickRefreshButton;
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewUnstagedFile, setupTest};

use std::path::PathBuf;


#[test]
fn refreshRepositoryWithUntrackedFileAfterNewIsCreatedWithLowerIndex()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath1 = PathBuf::from("unstagedFile1");
    makeNewUnstagedFile(&filePath1, "unstaged file content 1\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath1.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &filePath1)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 1\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    let filePath0 = PathBuf::from("unstagedFile0");
    makeNewUnstagedFile(&filePath0, "unstaged file content 0\n", &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath0.clone(), workTreeStatus: Untracked, indexStatus: Untracked},
          Entry{path: filePath1.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("WT_NEW", &filePath0),
          makeFileChange("WT_NEW", &filePath1)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 1\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}