use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use crate::common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, renameFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn loadRepositoryWithNewRenamedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let oldFilePath = PathBuf::from("some_file");
    makeNewStagedFile(&oldFilePath, "some file content\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    let newFilePath = PathBuf::from("renamed_file");
    renameFile(&oldFilePath, &newFilePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    // note: my trials to force git status command to detect a rename were unfruitful,
    // instead it shows separate deleted and new files
    assertRepositoryStatusIs(
        &[Entry{path: oldFilePath.clone(), workTreeStatus: Deleted, indexStatus: Unmodified},
          Entry{path: newFilePath.clone(), workTreeStatus: Untracked, indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Renamed", &newFilePath)], &gui);
    assertDiffViewContains("renamed file\nold path: some_file\nnew path: renamed_file\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}
}

const REPOSITORY_LOG: &str =
r"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 some_file | 1 +
 1 file changed, 1 insertion(+)

diff --git a/some_file b/some_file
new file mode 100644
index 0000000..c2e7a8d
--- /dev/null
+++ b/some_file
@@ -0,0 +1 @@
+some file content
";
