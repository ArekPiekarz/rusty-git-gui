use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::{
    assertRepositoryHasNoCommits,
    assertRepositoryLogIs,
    assertRepositoryStatusIs};
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
fn refreshRepositoryWithNewStagedFileAfterItChangesToModified()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Unmodified), IndexStatus(Added))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "modified file content\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Unmodified), IndexStatus(Modified))],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertUnstagedChangesViewIsEmpty(&gui);
}
}

const REPOSITORY_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 file | 1 +
 1 file changed, 1 insertion(+)

diff --git a/file b/file
new file mode 100644
index 0000000..84f83e4
--- /dev/null
+++ b/file
@@ -0,0 +1 @@
+staged file content
"#;
