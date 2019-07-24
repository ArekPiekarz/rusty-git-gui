use common::setup::{makeNewStagedFile, setupTest};
use rusty_git_gui::gui_setup::makeGui;
    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertRepositoryHasNoCommits(&repositoryDir);
    assertRepositoryStatusIs("A  file\n", &repositoryDir);

    setCommitMessage("some commit message", &gui);
    clickCommitButton(&gui);

    assertRepositoryLogIs(
        "Author: John Smith\n\
        Email: john.smith@example.com\n\
        Subject: some commit message\n\
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
        +some file content\n",
        &repositoryDir);
    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertStagedFilesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipContains("No changes are staged for commit.", &gui);