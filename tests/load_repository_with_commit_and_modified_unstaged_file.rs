#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::setup::{makeCommit, makeNewStagedFile, modifyFile, setupTest};
use common::utils::makeFileChange;

use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithCommitAndModifiedUnstagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "some file content\nsecond line\n", &repositoryDir,);
    makeCommit("Initial commit", &repositoryDir);
    modifyFile(&filePath, "some file content\nmodified second line\n", &repositoryDir);

    let gui = Gui::new(Rc::new(Repository::new(&repositoryDir)));
    gui.show();

    assertUnstagedChangesViewContains(&[makeFileChange("WT_MODIFIED", &filePath)], &gui);
    assertDiffViewContains("@@ -1,2 +1,2 @@\n some file content\n-second line\n+modified second line\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}