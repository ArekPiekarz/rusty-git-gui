#![allow(non_snake_case)]

use common::file_changes_view_utils::makeFileChange;
use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::clickRefreshButton;
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryIsEmpty, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewUnstagedFile, removeFile, setupTest};

use std::path::PathBuf;


#[test]
fn refreshRepositoryWithUntrackedFileAfterItIsRemoved()
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
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    removeFile(&filePath, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryIsEmpty(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
}