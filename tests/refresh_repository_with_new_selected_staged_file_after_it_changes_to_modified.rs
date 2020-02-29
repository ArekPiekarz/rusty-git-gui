#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{assertDiffViewContains, assertStagedChangesViewContains, assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::{clickRefreshButton, selectStagedChange};
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};

use std::path::PathBuf;


#[test]
fn refreshRepositoryWithNewSelectedStagedFileAfterItChangesToModified()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Added}],
        &repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertStagedChangesViewContains(&[makeFileChange("New", &filePath)], &gui);
    assertUnstagedChangesViewIsEmpty(&gui);
    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &gui);

    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "modified file content\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    clickRefreshButton(&gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Modified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1 @@\n-staged file content\n+modified file content\n", &gui);
    assertUnstagedChangesViewIsEmpty(&gui);
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