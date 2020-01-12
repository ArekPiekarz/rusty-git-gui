#![allow(non_snake_case)]

mod common;

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{activateStagedChangeToUnstageIt, selectUnstagedChange};
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};

use std::path::PathBuf;


#[test]
fn unstageModifiedChangeGivenUnstagedModifiedChangeOfSameFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);
    modifyFile(&filePath, "some modified file content\nmodified second line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Modified, indexStatus: Modified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewContains(DIFF_OF_UNSTAGED_CHANGE_BEFORE_UNSTAGING, &gui);

    activateStagedChangeToUnstageIt(&filePath, &gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Modified, indexStatus: Unmodified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedChange(&filePath, &gui);
    assertDiffViewContains(DIFF_OF_UNSTAGED_CHANGE_AFTER_UNSTAGING, &gui);
}

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

const DIFF_OF_UNSTAGED_CHANGE_BEFORE_UNSTAGING: &str =
r#"@@ -1,2 +1,2 @@
-some file content
+some modified file content
 modified second line
"#;

const DIFF_OF_UNSTAGED_CHANGE_AFTER_UNSTAGING: &str =
    r#"@@ -1,2 +1,2 @@
-some file content
-second line
+some modified file content
+modified second line
"#;