use crate::common::gui_assertions::{
    assertCommitButtonIsEnabled,
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipIs,
    assertCommitButtonTooltipIsEmpty};
use crate::common::gui_interactions::{activateStagedChangeInRow, setCommitMessage};
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn forbidCommittingWhenLastStagedChangeIsUnstagedAndMessageIsFilled()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    setCommitMessage("some commit message", &gui);
    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);

    activateStagedChangeInRow(0, &gui);

    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("No changes are staged for commit.", &gui);
}
}
