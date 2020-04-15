#![allow(non_snake_case)]

use common::file_changes_view_utils::makeRenamedFileChange;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, renameFile, setupTest};

use std::path::PathBuf;


#[test]
fn loadRepositoryWithNewRenamedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let oldFilePath = PathBuf::from("file");
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
    assertUnstagedChangesViewContains(&[makeRenamedFileChange("Renamed", &oldFilePath, &newFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}

const REPOSITORY_LOG: &str =
r"Author: John Smith
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
";