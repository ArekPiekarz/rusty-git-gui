#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitButtonTooltipIsEmpty};
use common::gui_interactions::setCommitMessage;
use common::setup::{makeNewStagedFile, setupTest};

use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn allowCommittingWhenChangeIsStagedAndCommitMessageIsFilled()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = Gui::new(Rc::new(Repository::new(&repositoryDir)));
    assertCommitButtonIsDisabled(&gui);

    setCommitMessage("some commit message", &gui);

    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);
}