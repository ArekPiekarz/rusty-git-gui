use crate::common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitButtonTooltipIs,
    assertCommitButtonTooltipIsEmpty};
use crate::common::gui_interactions::setCommitMessage;
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn forbidCommittingAfterMessageIsCleared()
{
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire().unwrap();
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    setCommitMessage("some commit message", &gui);
    assertCommitButtonIsEnabled(&gui);
    assertCommitButtonTooltipIsEmpty(&gui);

    setCommitMessage("", &gui);

    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipIs("The commit message is empty.", &gui);
}
}
