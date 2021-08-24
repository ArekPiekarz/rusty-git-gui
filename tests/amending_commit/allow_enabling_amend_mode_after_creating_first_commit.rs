use crate::common::gui_assertions::{
    assertCommitAmendCheckboxIsDisabled,
    assertCommitAmendCheckboxIsEnabled,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitAmendCheckboxTooltipIs,
    assertCommitAmendCheckboxTooltipIsEmpty};
use crate::common::gui_interactions::{clickCommitButton, setCommitMessage};
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use std::path::PathBuf;
use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn allowEnablingAmendModeAfterCreatingFirstCommit()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    let gui = makeGui(&repositoryDir);
    setCommitMessage("some commit message", &gui);
    assertCommitAmendCheckboxIsDisabled(&gui);
    assertCommitAmendCheckboxIsUnselected(&gui);
    assertCommitAmendCheckboxTooltipIs("No commit found to amend.", &gui);

    clickCommitButton(&gui);

    assertCommitAmendCheckboxIsEnabled(&gui);
    assertCommitAmendCheckboxIsUnselected(&gui);
    assertCommitAmendCheckboxTooltipIsEmpty(&gui);
}
}
