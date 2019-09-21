#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipIs,
    assertCommitMessageViewIsEmpty};
use common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;


#[test]
fn forbidCommittingWhenChangeIsStagedButCommitMessageIsEmpty()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("The commit message is empty.", &gui);
}