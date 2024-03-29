use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertCommitAmendCheckboxIsEnabled,
    assertCommitAmendCheckboxIsSelected,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitMessageViewIsEmpty,
    assertCommitMessageViewTextIs,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty};
use crate::common::gui_interactions::{clickCommitButton, selectCommitAmendCheckbox};
use crate::common::repository_assertions::{
    assertRepositoryLogIs,
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn amendCommitByChangingFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    modifyFile(&filePath, "amended file content\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    selectCommitAmendCheckbox(&gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Unmodified), IndexStatus(Modified))],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG_BEFORE_COMMIT_AMEND, &repositoryDir);
    assertCommitAmendCheckboxIsSelected(&gui);
    assertCommitAmendCheckboxIsEnabled(&gui);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertCommitMessageViewTextIs(COMMIT_MESSAGE, &gui);
    assertCommitButtonIsEnabled(&gui);

    clickCommitButton(&gui);

    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG_AFTER_COMMIT_AMEND, &repositoryDir);
    assertCommitAmendCheckboxIsUnselected(&gui);
    assertCommitAmendCheckboxIsEnabled(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}
}

const COMMIT_MESSAGE: &str = "Initial commit\n";

const REPOSITORY_LOG_BEFORE_COMMIT_AMEND: &str =
    r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 file | 1 +
 1 file changed, 1 insertion(+)

diff --git a/file b/file
new file mode 100644
index 0000000..c2e7a8d
--- /dev/null
+++ b/file
@@ -0,0 +1 @@
+some file content
"#;

const REPOSITORY_LOG_AFTER_COMMIT_AMEND: &str =
    r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 file | 1 +
 1 file changed, 1 insertion(+)

diff --git a/file b/file
new file mode 100644
index 0000000..a35faa2
--- /dev/null
+++ b/file
@@ -0,0 +1 @@
+amended file content
"#;
