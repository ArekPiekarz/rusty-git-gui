use crate::common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipIs,
    assertCommitMessageViewIsEmpty};
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
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
}