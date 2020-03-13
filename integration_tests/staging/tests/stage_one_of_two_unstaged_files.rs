#![allow(non_snake_case)]

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{activateUnstagedChangeToStageIt, selectStagedChange};
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewUnstagedFile, setupTest};

use std::path::PathBuf;


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
        &[Entry{path: filePath1.clone(), workTreeStatus: Untracked, indexStatus: Untracked},
          Entry{path: filePath2.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &filePath1),
          makeFileChange("New", &filePath2)],
        &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content 1\n", &gui);

    activateUnstagedChangeToStageIt(&filePath1, &gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath1.clone(), workTreeStatus: Unmodified, indexStatus: Added},
          Entry{path: filePath2.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath2)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath1)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content 2\n", &gui);

    selectStagedChange(&filePath1, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content 1\n", &gui);
}