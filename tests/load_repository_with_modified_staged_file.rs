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
use common::setup::{getWindow, makeCommit, makeNewStagedFile, makeRelativePath, modifyFile, setupTest, stageFile};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithModifiedStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let file = makeNewStagedFile(&repositoryDir, "some file content\nsecond line\n");
    let file = makeRelativePath(&file, &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&file, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&PathBuf::from(&file), &repositoryDir);

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));

        let window = getWindow();
        assertUnstagedFilesViewIsEmpty(&window);
        assertStagedFilesViewContains(&[FileInfo{status: "INDEX_MODIFIED".to_string(), name: file.clone()}], &window);
        assertDiffViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);

        selectStagedFile(&file, &window);
        assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}