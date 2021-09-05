use crate::common::file_changes_view_utils::makeFileChange;
    assertDiffViewContains,
use crate::common::repository_status_utils::{
    FileChangeStatus::*,
    IndexStatus,
    RepositoryStatusEntry as Entry,
    WorkTreeStatus};
    let oldFilePath = PathBuf::from("some_file");
        &[Entry::new(&oldFilePath, WorkTreeStatus(Deleted), IndexStatus(Unmodified)),
          Entry::new(&newFilePath, WorkTreeStatus(Untracked), IndexStatus(Untracked))],
    assertUnstagedChangesViewContains(&[makeFileChange("Renamed", &newFilePath)], &gui);
    assertDiffViewContains("renamed file\nold path: some_file\nnew path: renamed_file\n", &gui);
 some_file | 1 +
diff --git a/some_file b/some_file
+++ b/some_file