use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertCommitMessageViewIsEmpty,
    assertCommitMessageViewTextIs,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty};
use crate::common::gui_interactions::{selectCommitAmendCheckbox, selectStagedChangeInRow};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn enableAmendModeGivenCommitWithNewFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectCommitAmendCheckbox(&gui);

    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertCommitMessageViewTextIs(COMMIT_MESSAGE, &gui);
    assertDiffViewIsEmpty(&gui);
    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains(NEW_FILE_DIFF, &gui);
}
}

const COMMIT_MESSAGE: &str = "Initial commit\n";

const NEW_FILE_DIFF: &str =
"@@ -0,0 +1 @@
+some file content
";
