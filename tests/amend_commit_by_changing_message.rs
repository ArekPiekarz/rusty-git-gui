#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitAmendCheckboxIsEnabled,
    assertCommitAmendCheckboxIsSelected,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitMessageViewIs,
    assertCommitMessageViewIsEmpty,
    assertStagedChangesViewIsEmpty};
use common::gui_interactions::{clickCommitButton, selectCommitAmendCheckbox, setCommitMessage};
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIsEmpty};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;

#[test]
fn amendCommitByChangingMessage()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE1, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    selectCommitAmendCheckbox(&gui);
    setCommitMessage(COMMIT_MESSAGE2, &gui);

    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG_BEFORE_COMMIT_AMEND, &repositoryDir);
    assertCommitAmendCheckboxIsSelected(&gui);
    assertCommitAmendCheckboxIsEnabled(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIs(COMMIT_MESSAGE2, &gui);
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

const COMMIT_MESSAGE1: &str = "Initial commit";
const COMMIT_MESSAGE2: &str = "Amended commit";

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
Subject: Amended commit
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