#![allow(non_snake_case)]

use common::file_change_view_utils::makeFileChange;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewContains};
use common::gui_interactions::{selectStagedChange, selectUnstagedChange};
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{
    makeCommit,
    makeGui,
    makeNewStagedFile,
    makeNewUnstagedFile,
    modifyFile,
    setupTest,
    stageFile};

use std::path::PathBuf;


#[test]
fn loadRepositoryWithMultipleKindsOfFiles()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let modifiedStagedFilePath = PathBuf::from("fileName1");
    makeNewStagedFile(&modifiedStagedFilePath, "some file content\nsecond line\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&modifiedStagedFilePath, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&modifiedStagedFilePath, &repositoryDir);

    let newUnstagedFilePath = PathBuf::from("fileName2");
    makeNewUnstagedFile(&newUnstagedFilePath, "new unstaged file content\n", &repositoryDir);

    let newStagedAndModifiedUnstagedFilePath = PathBuf::from("fileName3");
    makeNewStagedFile(&newStagedAndModifiedUnstagedFilePath, "new staged file content\n", &repositoryDir);
    modifyFile(&newStagedAndModifiedUnstagedFilePath, "new staged file content\nmodified unstaged line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: modifiedStagedFilePath.clone(),               workTreeStatus: Unmodified, indexStatus: Modified},
          Entry{path: newStagedAndModifiedUnstagedFilePath.clone(), workTreeStatus: Modified,   indexStatus: Added},
          Entry{path: newUnstagedFilePath.clone(),                  workTreeStatus: Untracked,  indexStatus: Untracked}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(
        &[makeFileChange("New", &newUnstagedFilePath),
          makeFileChange("Modified", &newStagedAndModifiedUnstagedFilePath)],
        &gui);
    assertStagedChangesViewContains(
        &[makeFileChange("Modified", &modifiedStagedFilePath),
          makeFileChange("New", &newStagedAndModifiedUnstagedFilePath)],
        &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+new unstaged file content\n", &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectUnstagedChange(&newStagedAndModifiedUnstagedFilePath, &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n new staged file content\n+modified unstaged line\n", &gui);
    selectStagedChange(&modifiedStagedFilePath, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
    selectStagedChange(&newStagedAndModifiedUnstagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+new staged file content\n", &gui);
}

const REPOSITORY_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: Initial commit
---
 fileName1 | 2 ++
 1 file changed, 2 insertions(+)

diff --git a/fileName1 b/fileName1
new file mode 100644
index 0000000..1820ab1
--- /dev/null
+++ b/fileName1
@@ -0,0 +1,2 @@
+some file content
+second line
"#;