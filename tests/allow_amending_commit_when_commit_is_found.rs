#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitAmendCheckboxIsEnabled,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitAmendCheckboxTooltipIsEmpty};
use common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;

#[test]
fn allowAmendingCommitWhenCommitIsFound()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit("initial commit", &repositoryDir);

    let gui = makeGui(&repositoryDir);

    assertCommitAmendCheckboxIsEnabled(&gui);
    assertCommitAmendCheckboxIsUnselected(&gui);
    assertCommitAmendCheckboxTooltipIsEmpty(&gui);
}