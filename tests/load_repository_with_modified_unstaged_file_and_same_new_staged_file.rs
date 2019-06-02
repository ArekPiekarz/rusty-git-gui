#![allow(non_snake_case)]

mod common;

use common::actions::selectStagedFile;
use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewContains,
};
use common::setup::{getWindow, makeNewStagedFile, makeRelativePath, modifyFile, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::rc::Rc;


#[test]
fn loadRepositoryWithModifiedUnstagedFileAndSameNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let file = makeNewStagedFile(&repositoryDir, "staged file content\n");
    let file = makeRelativePath(&file, &repositoryDir);
    modifyFile(&file, "staged file content\nmodified unstaged line\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));

        let window = getWindow();
        assertUnstagedFilesViewContains(
            &[FileInfo{status: "WT_MODIFIED", name: &file}],
            &window);
        assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified unstaged line\n", &window);

        assertStagedFilesViewContains(
            &[FileInfo{status: "INDEX_NEW", name: &file}],
            &window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);

        selectStagedFile(&file, &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}