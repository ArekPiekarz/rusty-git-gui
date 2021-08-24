use crate::common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitButtonTooltipIsEmpty};
use crate::common::gui_interactions::setCommitMessage;
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn allowCommittingWhenChangeIsStagedAndCommitMessageIsFilled()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    assertCommitButtonIsDisabled(&gui);

    setCommitMessage("some commit message", &gui);

    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);
}
}
