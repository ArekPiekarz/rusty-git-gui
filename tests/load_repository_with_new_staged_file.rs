#![allow(non_snake_case)]

mod common;

use common::actions::selectStagedFile;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewIsEmpty};
use common::setup::{makeNewStagedFile, setupTest};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newStagedFilePath = PathBuf::from("stagedFile");
    makeNewStagedFile(&newStagedFilePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &newStagedFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertUnstagedFilesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedFile(&newStagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &gui);
}