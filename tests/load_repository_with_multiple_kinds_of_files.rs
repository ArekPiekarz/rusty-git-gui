#![allow(non_snake_case)]

mod common;

use common::actions::{selectStagedFile, selectUnstagedFile};
use common::assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewContains,
};
use common::setup::{
    getWindow,
    makeCommit,
    makeNewStagedFile,
    makeNewUnstagedFile,
    makeRelativePath,
    modifyFile,
    setupTest,
    stageFile,
};
use common::utils::FileInfo;
use rusty_git_gui::app_setup::{makeGtkApp, NO_APP_ARGUMENTS};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use std::path::PathBuf;
use std::rc::Rc;

#[test]
fn loadRepositoryWithMultipleKindsOfFiles()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let file = makeNewStagedFile(&repositoryDir, "some file content\nsecond line\n");
    let file = makeRelativePath(&file, &repositoryDir);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&file, "some file content\nmodified second line\n", &repositoryDir);
    stageFile(&PathBuf::from(&file), &repositoryDir);
    let modifiedStagedFile = file;

    let newUnstagedFile = makeNewUnstagedFile(&repositoryDir, "new unstaged file content\n");
    let newUnstagedFile = makeRelativePath(&newUnstagedFile, &repositoryDir);

    let newStagedFile = makeNewStagedFile(&repositoryDir, "new staged file content\n");
    let newStagedFile = makeRelativePath(&newStagedFile, &repositoryDir);
    modifyFile(&newStagedFile, "new staged file content\nmodified unstaged line\n", &repositoryDir);
    let modifiedUnstagedFile = newStagedFile.clone();

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir)));
        let window = getWindow();

        assertUnstagedFilesViewContains(
            &[FileInfo{status: "WT_MODIFIED", name: &modifiedUnstagedFile},
              FileInfo{status: "WT_NEW", name: &newUnstagedFile}],
            &window);
        assertStagedFilesViewContains(
            &[FileInfo{status: "INDEX_MODIFIED", name: &modifiedStagedFile},
              FileInfo{status: "INDEX_NEW", name: &newStagedFile}],
            &window);
        assertDiffViewContains("@@ -1 +1,2 @@\n new staged file content\n+modified unstaged line\n", &window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);

        selectUnstagedFile(&newUnstagedFile, &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+new unstaged file content\n", &window);
        selectStagedFile(&modifiedStagedFile, &window);
        assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &window);
        selectStagedFile(&newStagedFile, &window);
        assertDiffViewContains("@@ -0,0 +1 @@\n+new staged file content\n", &window);
    });
    gtkApp.run(&NO_APP_ARGUMENTS);
}