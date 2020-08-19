use crate::common::gui_assertions::{assertCommitMessageViewIsEmpty, assertCommitMessageViewTextIs};
use crate::common::gui_interactions::selectCommitAmendCheckbox;
use crate::common::setup::{makeCommit, makeGui, makeNewStagedFile, setupTest};

use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn enablingCommitAmendShouldFillCommitMessageFromPreviousCommit()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewStagedFile(&filePath, "some file content\n", &repositoryDir);
    makeCommit(COMMIT_MESSAGE, &repositoryDir);
    let gui = makeGui(&repositoryDir);
    assertCommitMessageViewIsEmpty(&gui);

    selectCommitAmendCheckbox(&gui);

    assertCommitMessageViewTextIs(COMMIT_MESSAGE, &gui);
}
}

const COMMIT_MESSAGE: &str = "Initial commit\n";