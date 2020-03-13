#![allow(non_snake_case)]

use common::gui_assertions::{
    assertCommitButtonIsEnabled,
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipIs,
    assertCommitButtonTooltipIsEmpty};
use common::gui_interactions::{activateStagedChangeToUnstageIt, setCommitMessage};
use common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;


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

    activateStagedChangeToUnstageIt(&filePath, &gui);

    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("No changes are staged for commit.", &gui);
}