#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitButtonTooltipIs,
    assertCommitButtonTooltipIsEmpty};
use common::gui_interactions::setCommitMessage;
use common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;


#[test]
fn forbidCommittingAfterMessageIsCleared()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    setCommitMessage("some commit message", &gui);
    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);

    setCommitMessage("", &gui);

    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("The commit message is empty.", &gui);
}