#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipContains,
    assertCommitMessageViewIsEmpty};
use common::setup::{makeNewStagedFile, setupTest};

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn forbidCommittingWhenChangeIsStagedButCommitMessageIsEmpty()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipContains("The commit message is empty.", &gui);
}