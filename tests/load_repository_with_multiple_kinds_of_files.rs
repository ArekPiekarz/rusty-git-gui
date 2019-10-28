use common::file_change_view_utils::makeFileChange;
use common::repository_assertions::{assertRepositoryLogIs, assertRepositoryStatusIs};
use common::repository_status_utils::{FileChangeStatus::*, RepositoryStatusEntry as Entry};
    makeGui,
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
          makeFileChange("WT_MODIFIED", &newStagedAndModifiedUnstagedFilePath)],
          makeFileChange("INDEX_NEW", &newStagedAndModifiedUnstagedFilePath)],
    selectUnstagedChange(&newStagedAndModifiedUnstagedFilePath, &gui);
    selectStagedChange(&newStagedAndModifiedUnstagedFilePath, &gui);
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