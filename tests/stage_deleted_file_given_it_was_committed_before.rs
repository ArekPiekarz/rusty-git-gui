#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::{activateUnstagedChangeToStageIt, selectStagedChange};
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, removeFile, setupTest};

use std::path::PathBuf;


#[test]
fn stageDeletedFileGivenItWasCommittedBefore()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("someFile");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    removeFile(&filePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Deleted, indexStatus: Unmodified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("WT_DELETED", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains(DIFF, &gui);

    activateUnstagedChangeToStageIt(&filePath, &gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Deleted}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_DELETED", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains(DIFF, &gui);
}

const REPOSITORY_LOG: &str =
    r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 someFile | 1 +
 1 file changed, 1 insertion(+)

diff --git a/someFile b/someFile
new file mode 100644
index 0000000..c2e7a8d
--- /dev/null
+++ b/someFile
@@ -0,0 +1 @@
+some file content
"#;

const DIFF: &str = "@@ -1 +0,0 @@\n-some file content\n";