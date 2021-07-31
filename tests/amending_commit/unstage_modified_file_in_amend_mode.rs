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
use crate::common::repository_assertions::{
    assertGitDiffStagedIs,
    assertGitDiffUnstagedIs,
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn unstageModifiedFileInAmendMode()
{
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire().unwrap();
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    modifyFile(&filePath, "modified file content\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    makeCommit(COMMIT_MESSAGE2, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    selectCommitAmendCheckbox(&gui);
    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains(MODIFIED_FILE_DIFF, &gui);

    activateStagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Modified, indexStatus: Modified}],
        &repositoryDir);
    assertGitDiffUnstagedIs(GIT_DIFF_UNSTAGED, &repositoryDir);
    assertGitDiffStagedIs(GIT_DIFF_STAGED, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    selectUnstagedChangeInRow(0, &gui);
    assertDiffViewContains(MODIFIED_FILE_DIFF, &gui);
}
}

const COMMIT_MESSAGE: &str = "Initial commit\n";
const COMMIT_MESSAGE2: &str = "Second commit\n";

const MODIFIED_FILE_DIFF: &str =
"@@ -1 +1 @@
-some file content
+modified file content
";

const GIT_DIFF_UNSTAGED: &str =
"diff --git a/file b/file
index c2e7a8d..5683396 100644
--- a/file
+++ b/file
@@ -1 +1 @@
-some file content
+modified file content
";

const GIT_DIFF_STAGED: &str =
"diff --git a/file b/file
index 5683396..c2e7a8d 100644
--- a/file
+++ b/file
@@ -1 +1 @@
-modified file content
+some file content
";
