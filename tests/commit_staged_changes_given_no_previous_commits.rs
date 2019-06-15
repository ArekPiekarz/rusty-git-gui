#![allow(non_snake_case)]

mod common;

use common::actions::{clickCommitButton, setCommitMessage};
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipContains,
    assertCommitMessageViewIsEmpty,
    assertStagedFilesViewIsEmpty};
use common::repository_assertions::{
    assertRepositoryHasNoCommits,
    assertRepositoryLogIs,
    assertRepositoryStatusIs,
    assertRepositoryStatusIsEmpty};
use common::setup::{getWindow, makeNewStagedFile, setupTest};

use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;

use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn commitStagedChangesGivenNoPreviousCommits()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertRepositoryHasNoCommits(&repositoryDir);
        assertRepositoryStatusIs("A  file\n", &repositoryDir);

        setCommitMessage("some commit message", &window);
        clickCommitButton(&window);

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
        assertStagedFilesViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
        assertCommitButtonTooltipContains("No changes are staged for commit.", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}