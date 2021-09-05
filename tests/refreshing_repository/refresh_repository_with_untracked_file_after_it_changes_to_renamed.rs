use crate::common::file_changes_view_utils::makeFileChange;
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
    let filePath = PathBuf::from("some_file");
        &[Entry::new(&filePath, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
    let tempRenamedFilePath = PathBuf::from("temp_renamed_file");
        &[Entry::new(&tempRenamedFilePath, WorkTreeStatus(Deleted),   IndexStatus(Unmodified)),
          Entry::new(&filePath,            WorkTreeStatus(Untracked), IndexStatus(Untracked))],
    assertUnstagedChangesViewContains(&[makeFileChange("Renamed", &filePath)], &gui);
    assertDiffViewContains("renamed file\nold path: temp_renamed_file\nnew path: some_file\n", &gui);
 some_file => temp_renamed_file | 0
diff --git a/some_file b/temp_renamed_file
rename from some_file
rename to temp_renamed_file
 some_file | 1 +
diff --git a/some_file b/some_file
+++ b/some_file