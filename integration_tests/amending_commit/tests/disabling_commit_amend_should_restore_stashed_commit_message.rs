#![allow(non_snake_case)]

use common::gui_assertions::{assertCommitButtonIsDisabled, assertCommitButtonIsEnabled, assertCommitMessageViewIs};
use common::gui_interactions::{selectCommitAmendCheckbox, setCommitMessage, unselectCommitAmendCheckbox};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;

#[test]
fn disablingCommitAmendShouldRestoreStashedCommitMessage()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE1, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    setCommitMessage(COMMIT_MESSAGE2, &gui);
    selectCommitAmendCheckbox(&gui);
    assertCommitMessageViewIs(COMMIT_MESSAGE1, &gui);
    assertCommitButtonIsEnabled(&gui);

    unselectCommitAmendCheckbox(&gui);

    assertCommitMessageViewIs(COMMIT_MESSAGE2, &gui);
    assertCommitButtonIsDisabled(&gui);
}

const COMMIT_MESSAGE1: &str = "Initial commit\n";
const COMMIT_MESSAGE2: &str = "Second commit\n";