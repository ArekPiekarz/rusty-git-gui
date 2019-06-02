#![allow(non_snake_case)]

mod common;

use common::actions::selectStagedFile;
use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty,
};
use common::setup::{getWindow, makeNewStagedFile, makeRelativePath, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newStagedFile = makeNewStagedFile(&repositoryDir, "staged file content");
    let newStagedFile = makeRelativePath(&newStagedFile, &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));

        let window = getWindow();
        assertStagedFilesViewContains(
            &[FileInfo{status: "INDEX_NEW", name: &newStagedFile}],
            &window);
        assertDiffViewIsEmpty(&window);
        assertUnstagedFilesViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);

        selectStagedFile(&newStagedFile, &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n\\ No newline at end of file\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}