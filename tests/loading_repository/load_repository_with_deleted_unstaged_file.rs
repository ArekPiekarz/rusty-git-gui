use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::selectStagedChangeInRow;
use crate::common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeGui, makeNewStagedFile, removeFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn loadRepositoryWithDeletedUnstagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("someFile");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    removeFile(&filePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Deleted, indexStatus: Added}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Deleted", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +0,0 @@\n-some file content\n", &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+some file content\n", &gui);
}
}