use crate::common::file_changes_view_utils::{makeFileChange, makeRenamedFileChange};
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::{
    assertRepositoryHasNoCommits,
    assertRepositoryLogIs,
    assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeCommit, makeGui, makeNewUnstagedFile, renameFile, setupTest, stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn refreshRepositoryWithUntrackedFileAfterItChangesToRenamed()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewUnstagedFile(&filePath, "unstaged file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    stageFile(&filePath, &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    let tempRenamedFilePath = PathBuf::from("tempRenamedFile");
    renameFile(&filePath, &tempRenamedFilePath, &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    stageFile(&tempRenamedFilePath, &repositoryDir);
    makeCommit("Second commit", &repositoryDir);
    renameFile(&tempRenamedFilePath, &filePath, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: tempRenamedFilePath.clone(), workTreeStatus: Deleted, indexStatus: Unmodified},
          Entry{path: filePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeRenamedFileChange("Renamed", &tempRenamedFilePath, &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
}
}

const REPOSITORY_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: Second commit
---
 file => tempRenamedFile | 0
 1 file changed, 0 insertions(+), 0 deletions(-)

diff --git a/file b/tempRenamedFile
similarity index 100%
rename from file
rename to tempRenamedFile
Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 file | 1 +
 1 file changed, 1 insertion(+)

diff --git a/file b/file
new file mode 100644
index 0000000..e2166a5
--- /dev/null
+++ b/file
@@ -0,0 +1 @@
+unstaged file content
"#;