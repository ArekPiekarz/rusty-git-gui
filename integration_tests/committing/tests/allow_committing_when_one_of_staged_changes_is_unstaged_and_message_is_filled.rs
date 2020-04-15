#![allow(non_snake_case)]

use common::gui_assertions::{assertCommitButtonIsEnabled, assertCommitButtonTooltipIsEmpty};
use common::gui_interactions::{activateStagedChangeInRow, setCommitMessage};
use common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;


#[test]
fn allowCommittingWhenOneOfStagedChangesIsUnstagedAndMessageIsFilled()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file1");
    let filePath2 = PathBuf::from("file2");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    makeNewStagedFile(&filePath2, "second staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    setCommitMessage("some commit message", &gui);
    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);

    activateStagedChangeInRow(0, &gui);

    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);
}