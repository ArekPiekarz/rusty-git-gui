use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeGui, makeNewUnstagedFile, removeFile, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn refreshRepositoryWithThreeUntrackedFilesAfterFirstTwoAreRemoved()
{
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire().unwrap();
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
        &[Entry{path: filePath0.clone(), workTreeStatus: Untracked, indexStatus: Untracked},
          Entry{path: filePath1.clone(), workTreeStatus: Untracked, indexStatus: Untracked},
          Entry{path: filePath2.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
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
    removeFile(&filePath1, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath2.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &filePath2)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content 2\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}
}
