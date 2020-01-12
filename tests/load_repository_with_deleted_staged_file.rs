#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::selectStagedChange;
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, removeFile, setupTest, stageFile};

use std::path::PathBuf;


#[test]
fn loadRepositoryWithDeletedStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("someFile");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    removeFile(&filePath, &repositoryDir);
    stageFile(&filePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Deleted}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("Deleted", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -1 +0,0 @@\n-some file content\n", &gui);
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