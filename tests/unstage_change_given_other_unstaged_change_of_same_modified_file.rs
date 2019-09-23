#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{activateStagedChangeToUnstageIt, selectUnstagedChange};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn unstageChangeGivenOtherUnstagedChangeOfSameModifiedFile()
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

    assertUnstagedChangesViewContains(&[makeFileChange("WT_MODIFIED", &filePath)], &gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_MODIFIED", &filePath)], &gui);
    assertDiffViewContains(DIFF_OF_UNSTAGED_CHANGE_BEFORE_UNSTAGING, &gui);

    activateStagedChangeToUnstageIt(&filePath, &gui);

    assertUnstagedChangesViewContains(&[makeFileChange("WT_MODIFIED", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedChange(&filePath, &gui);
    assertDiffViewContains(DIFF_OF_UNSTAGED_CHANGE_AFTER_UNSTAGING, &gui);
}

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