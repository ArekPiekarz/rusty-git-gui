use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::{
    assertRepositoryHasNoCommits,
    assertRepositoryLogIs,
    assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeCommit, makeGui, makeNewUnstagedFile, modifyFile, setupTest, stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn refreshRepositoryWithUntrackedFileAfterItIsCommittedAndModified()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("unstagedFile");
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
    modifyFile(&filePath, "unstaged file content\nwith second line\n", &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Modified, indexStatus: Unmodified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n unstaged file content\n+with second line\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}
}

const REPOSITORY_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 unstagedFile | 1 +
 1 file changed, 1 insertion(+)

diff --git a/unstagedFile b/unstagedFile
new file mode 100644
index 0000000..e2166a5
--- /dev/null
+++ b/unstagedFile
@@ -0,0 +1 @@
+unstaged file content
"#;