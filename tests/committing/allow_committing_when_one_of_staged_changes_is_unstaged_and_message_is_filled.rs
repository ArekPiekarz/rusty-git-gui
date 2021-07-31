use crate::common::gui_assertions::{assertCommitButtonIsEnabled, assertCommitButtonTooltipIsEmpty};
use crate::common::gui_interactions::{activateStagedChangeInRow, setCommitMessage};
use crate::common::setup::{makeGui, makeNewStagedFile, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn allowCommittingWhenOneOfStagedChangesIsUnstagedAndMessageIsFilled()
{
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire().unwrap();
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
}
