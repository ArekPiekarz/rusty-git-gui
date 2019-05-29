#![allow(non_snake_case)]

mod common;

use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains,
};
use common::setup::{getWindow, makeCommit, makeNewStagedFile, makeRelativePath, modifyFile, setupTest};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::rc::Rc;


#[test]
fn loadRepositoryWithCommitAndModifiedUnstagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let file = makeNewStagedFile(&repositoryDir, "some file content\nsecond line\n");
    let file = makeRelativePath(&file, &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&file, "some file content\nmodified second line\n", &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));

        let window = getWindow();
        assertUnstagedFilesViewContains(
            &[FileInfo{status: "WT_MODIFIED".to_string(), name: file.clone()}], &window);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}