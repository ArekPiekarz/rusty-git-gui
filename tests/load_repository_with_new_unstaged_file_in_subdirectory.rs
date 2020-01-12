#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewUnstagedFile, makeSubdirectory, setupTest};

use std::path::PathBuf;


#[test]
fn loadRepositoryWithNewUnstagedFileInSubdirectory()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let subdir = PathBuf::from("subdir");
    makeSubdirectory(&subdir, &repositoryDir);
    let newUnstagedFilePath = subdir.join("unstagedFile");
    makeNewUnstagedFile(&newUnstagedFilePath, "unstaged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: newUnstagedFilePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &newUnstagedFilePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}