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
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{makeCommit, makeGui, makeNewUnstagedFile, renameFile, setupTest, stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn refreshRepositoryWithUntrackedFileAfterItChangesToRenamed()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("some_file");
    makeNewUnstagedFile(&filePath, "unstaged file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::new(&filePath, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);

    stageFile(&filePath, &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    let tempRenamedFilePath = PathBuf::from("temp_renamed_file");
    renameFile(&filePath, &tempRenamedFilePath, &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    stageFile(&tempRenamedFilePath, &repositoryDir);
    makeCommit("Second commit", &repositoryDir);
    renameFile(&tempRenamedFilePath, &filePath, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry::new(&tempRenamedFilePath, WorkTreeStatus(Deleted),   IndexStatus(Unmodified)),
          Entry::new(&filePath,            WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Renamed", &filePath)], &gui);
    assertDiffViewContains("renamed file\nold path: temp_renamed_file\nnew path: some_file\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
}
}

const REPOSITORY_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: Second commit
---
 some_file => temp_renamed_file | 0
 1 file changed, 0 insertions(+), 0 deletions(-)

diff --git a/some_file b/temp_renamed_file
similarity index 100%
rename from some_file
rename to temp_renamed_file
Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 some_file | 1 +
 1 file changed, 1 insertion(+)

diff --git a/some_file b/some_file
new file mode 100644
index 0000000..e2166a5
--- /dev/null
+++ b/some_file
@@ -0,0 +1 @@
+unstaged file content
"#;
