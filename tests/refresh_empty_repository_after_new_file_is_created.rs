#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertGuiIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{clickRefreshButton, selectUnstagedChange};
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryIsEmpty, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewUnstagedFile, setupTest};

use std::path::PathBuf;


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
        &[Entry{path: newUnstagedFilePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &newUnstagedFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);

    selectUnstagedChange(&newUnstagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
}