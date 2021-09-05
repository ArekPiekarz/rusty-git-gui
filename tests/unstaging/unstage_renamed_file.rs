use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use crate::common::gui_interactions::{activateStagedChangeInRow, selectUnstagedChangeInRow};
use crate::common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, renameFile, setupTest, stageFile};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn unstageRenamedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let oldFilePath = PathBuf::from("some_file");
    makeNewStagedFile(&oldFilePath, "some file content\n", &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    let newFilePath = PathBuf::from("renamed_file");
    renameFile(&oldFilePath, &newFilePath, &repositoryDir);
    stageFile(&oldFilePath, &repositoryDir);
    stageFile(&newFilePath, &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIs(
        &[Entry::renamed("some_file -> renamed_file", WorkTreeStatus(Unmodified), IndexStatus(Renamed))],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("Renamed", &newFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    activateStagedChangeInRow(0, &gui);

    assertRepositoryStatusIs(
        &[Entry::new(&oldFilePath, WorkTreeStatus(Deleted), IndexStatus(Unmodified)),
          Entry::new(&newFilePath, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    assertUnstagedChangesViewContains(&[makeFileChange("Renamed", &newFilePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);

    selectUnstagedChangeInRow(0, &gui);
    assertDiffViewContains(DIFF, &gui);
}
}

const REPOSITORY_LOG: &str =
r#"Author: John Smith
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
"#;

const DIFF: &str =
r"renamed file
old path: some_file
new path: renamed_file
";
