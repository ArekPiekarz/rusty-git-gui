use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewContains};
use crate::common::gui_interactions::{selectCommitAmendCheckbox, selectStagedChangeInRow};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn enableAmendModeGivenSelectedStagedModifiedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    modifyFile(&filePath, "modified file content\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    selectStagedChangeInRow(0, &gui);
    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewContains(MODIFIED_FILE_DIFF, &gui);

    selectCommitAmendCheckbox(&gui);

    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains(AMENDED_FILE_DIFF, &gui);
}
}

const COMMIT_MESSAGE: &str = "Initial commit\n";

const MODIFIED_FILE_DIFF: &str =
"@@ -1 +1 @@
-some file content
+modified file content
";

const AMENDED_FILE_DIFF: &str =
"@@ -0,0 +1 @@
+modified file content
";