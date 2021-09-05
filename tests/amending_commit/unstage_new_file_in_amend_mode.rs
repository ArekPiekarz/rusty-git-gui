use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::{
    activateStagedChangeInRow,
    selectCommitAmendCheckbox,
    selectStagedChangeInRow,
    selectUnstagedChangeInRow};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};
use crate::common::repository_assertions::{assertRepositoryStatusIs, assertRepositoryStatusIsEmpty};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


const COMMIT_MESSAGE: &str = "Initial commit\n";

rusty_fork_test! {
#[test]
fn unstageNewFileInAmendMode()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    selectCommitAmendCheckbox(&gui);
    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains(FILE_DIFF, &gui);

    activateStagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Unmodified), IndexStatus(Deleted)),
          Entry::new(&filePath, WorkTreeStatus(Untracked),  IndexStatus(Untracked))],
        &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    selectUnstagedChangeInRow(0, &gui);
    assertDiffViewContains(FILE_DIFF, &gui);
}
}

const FILE_DIFF: &str =
    "@@ -0,0 +1 @@
+some file content
";
