use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::activateStagedChangeInRow;
use crate::common::repository_assertions::{
    assertRepositoryLogIs,
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
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
fn unstageDeletedFileGivenItWasRestored()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, FILE_CONTENT, &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    removeFile(&filePath, &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    makeNewUnstagedFile(&filePath, FILE_CONTENT, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Deleted},
          Entry{path: filePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("Deleted", &filePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1,2 @@\n+some file content\n+second line\n", &gui);

    activateStagedChangeInRow(0, &gui);

    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
}
}

const FILE_CONTENT: &str = "some file content\nsecond line\n";

const REPOSITORY_LOG: &str =
    r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 fileName | 2 ++
 1 file changed, 2 insertions(+)

diff --git a/fileName b/fileName
new file mode 100644
index 0000000..1820ab1
--- /dev/null
+++ b/fileName
@@ -0,0 +1,2 @@
+some file content
+second line
"#;
