#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipIs,
    assertCommitMessageViewIsEmpty,
    assertStagedChangesViewIsEmpty};
use common::gui_interactions::{clickCommitButton, setCommitMessage};
use common::repository_assertions::{
    assertRepositoryLogIs,
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use common::setup::{makeCommit, makeNewStagedFile, modifyFile, setupTest, stageFile};

use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn commitStagedChangesGivenOnePreviousCommit()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit("initial commit", &repositoryDir);
    modifyFile(&filePath, "modified file content\n", &repositoryDir);
    stageFile(&filePath, &repositoryDir);

    let gui = Gui::new(Rc::new(Repository::new(&repositoryDir)));

    let firstCommitLog =
        "Author: John Smith\n\
         Email: john.smith@example.com\n\
         Subject: initial commit\n\
         ---\n \
          file | 1 +\n \
          1 file changed, 1 insertion(+)\n\
         \n\
         diff --git a/file b/file\n\
         new file mode 100644\n\
         index 0000000..c2e7a8d\n\
         --- /dev/null\n\
         +++ b/file\n\
         @@ -0,0 +1 @@\n\
         +some file content\n";
    assertRepositoryLogIs(firstCommitLog, &repositoryDir);
    assertRepositoryStatusIs("M  file\n", &repositoryDir);

    setCommitMessage("second commit", &gui);
    clickCommitButton(&gui);

    assertRepositoryLogIs(
        &("Author: John Smith\n\
        Email: john.smith@example.com\n\
        Subject: second commit\n\
        ---\n \
         file | 2 +-\n \
         1 file changed, 1 insertion(+), 1 deletion(-)\n\
        \n\
        diff --git a/file b/file\n\
        index c2e7a8d..5683396 100644\n\
        --- a/file\n\
        +++ b/file\n\
        @@ -1 +1 @@\n\
        -some file content\n\
        +modified file content\n".to_string()
        + firstCommitLog),
        &repositoryDir);
    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("No changes are staged for commit.", &gui);
}