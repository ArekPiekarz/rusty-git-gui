use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::{activateUnstagedChangeInRow, selectStagedChangeInRow};
use crate::common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn stageModifiedFileGivenItWasCommittedBefore()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir,);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Modified, indexStatus: Unmodified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);

    activateUnstagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Modified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("Modified", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChangeInRow(0, &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
}
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