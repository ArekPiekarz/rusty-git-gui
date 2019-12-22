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
use common::setup::{makeGui, makeNewUnstagedFile, modifyFile, setupTest};

use std::path::PathBuf;


#[test]
fn refreshRepositoryWithUntrackedFileAfterItIsModified()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("unstagedFile");
    makeNewUnstagedFile(&filePath, "unstaged file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &filePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    modifyFile(&filePath, "unstaged file content\nwith second line\n", &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &filePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1,2 @@\n+unstaged file content\n+with second line\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}