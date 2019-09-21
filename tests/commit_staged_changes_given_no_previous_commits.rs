#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipIs,
    assertCommitMessageViewIsEmpty,
    assertStagedChangesViewIsEmpty};
use common::gui_interactions::{clickCommitButton, setCommitMessage};
use common::repository_assertions::{
    assertRepositoryHasNoCommits,
    assertRepositoryLogIs,
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;


#[test]
fn commitStagedChangesGivenNoPreviousCommits()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

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
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("No changes are staged for commit.", &gui);
}