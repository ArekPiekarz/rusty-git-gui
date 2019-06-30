#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewContains};
use common::setup::{makeNewUnstagedEmptyFile, setupTest};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewUnstagedEmptyFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newUnstagedFilePath = PathBuf::from("unstagedFile");
    makeNewUnstagedEmptyFile(&newUnstagedFilePath, &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertUnstagedFilesViewContains(&[FileInfo::new("WT_NEW", &newUnstagedFilePath)], &gui);
    assertStagedFilesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}