use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeGui, makeNewStagedFile, setupTest};
    let gui = makeGui(&repositoryDir);
    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Added}],
        &repositoryDir);
    assertRepositoryLogIs(REPOSITORY_LOG_AFTER_COMMIT, &repositoryDir);
}

const REPOSITORY_LOG_AFTER_COMMIT: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: some commit message
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
"#;