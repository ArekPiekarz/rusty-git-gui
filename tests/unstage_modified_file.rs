use common::file_change_view_utils::makeFileChange;
use common::gui_interactions::{activateStagedChangeToUnstageIt, selectUnstagedChange};
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};
    let gui = makeGui(&repositoryDir);
    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Modified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
    activateStagedChangeToUnstageIt(&filePath, &gui);
    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Modified, indexStatus: Unmodified}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG, &repositoryDir);
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