use common::setup::{makeCommit, makeNewStagedFile, modifyFile, setupTest, stageFile};
use rusty_git_gui::gui_setup::makeGui;
    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));
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
    assertStagedFilesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipContains("No changes are staged for commit.", &gui);