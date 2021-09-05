use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::{
    activateUnstagedChangeInRow,
    selectCommitAmendCheckbox,
    selectStagedChangeInRow};
use crate::common::repository_assertions::{
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{
    makeCommit,
    makeGui,
    makeNewStagedFile,
    makeNewUnstagedFile,
    removeFile,
    setupTest,
    stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn stageNewFileForAmendWhichInNormalModeWasStagedForRemoval()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    removeFile(&filePath, &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    makeNewUnstagedFile(&filePath, "some file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("Deleted", &filePath)], &gui);
    selectCommitAmendCheckbox(&gui);
    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Unmodified), IndexStatus(Deleted)),
          Entry::new(&filePath, WorkTreeStatus(Untracked),  IndexStatus(Untracked))],
        &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);

    activateUnstagedChangeInRow(0, &gui);

    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains(FILE_DIFF, &gui);
}
}

const COMMIT_MESSAGE: &str = "Initial commit\n";

const FILE_DIFF: &str =
"@@ -0,0 +1 @@
+some file content
";
