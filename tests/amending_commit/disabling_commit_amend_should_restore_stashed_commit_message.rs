use crate::common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonIsEnabled,
    assertCommitMessageViewTextIs};
use crate::common::gui_interactions::{selectCommitAmendCheckbox, setCommitMessage, unselectCommitAmendCheckbox};
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn disablingCommitAmendShouldRestoreStashedCommitMessage()
{
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire().unwrap();
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE1, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    setCommitMessage(COMMIT_MESSAGE2, &gui);
    selectCommitAmendCheckbox(&gui);
    assertCommitMessageViewTextIs(COMMIT_MESSAGE1, &gui);
    assertCommitButtonIsEnabled(&gui);

    unselectCommitAmendCheckbox(&gui);

    assertCommitMessageViewTextIs(COMMIT_MESSAGE2, &gui);
    assertCommitButtonIsDisabled(&gui);
}
}

const COMMIT_MESSAGE1: &str = "Initial commit\n";
const COMMIT_MESSAGE2: &str = "Second commit\n";
