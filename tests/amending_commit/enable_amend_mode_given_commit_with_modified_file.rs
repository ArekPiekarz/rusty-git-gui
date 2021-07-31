use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertCommitMessageViewIsEmpty,
    assertCommitMessageViewTextIs,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty};
use crate::common::gui_interactions::{selectCommitAmendCheckbox, selectStagedChangeInRow};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn enableAmendModeGivenCommitWithModifiedFile()
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
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectCommitAmendCheckbox(&gui);

    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertCommitMessageViewTextIs(COMMIT_MESSAGE2, &gui);
    assertDiffViewIsEmpty(&gui);
    selectStagedChangeInRow(0, &gui);
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
