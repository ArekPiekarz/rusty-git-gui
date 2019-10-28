use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, modifyFile, setupTest, stageFile};
    let gui = makeGui(&repositoryDir);
    assertRepositoryStatusIs(
        &[Entry{path: filePath.clone(), workTreeStatus: Unmodified, indexStatus: Modified}],
        &repositoryDir);
    assertRepositoryLogIs(FIRST_COMMIT_LOG, &repositoryDir);
    assertRepositoryLogIs(&(SECOND_COMMIT_LOG.to_string() + FIRST_COMMIT_LOG), &repositoryDir);
}

const FIRST_COMMIT_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: initial commit
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

const SECOND_COMMIT_LOG: &str =
r#"Author: John Smith
Email: john.smith@example.com
Subject: second commit
---
 file | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)

diff --git a/file b/file
index c2e7a8d..5683396 100644
--- a/file
+++ b/file
@@ -1 +1 @@
-some file content
+modified file content
"#;