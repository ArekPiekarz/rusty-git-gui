use crate::common::gui_assertions::{
    assertCommitAmendCheckboxIsEnabled,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitAmendCheckboxTooltipIsEmpty};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
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
}