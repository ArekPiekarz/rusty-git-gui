#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitButtonTooltipContains,
    assertCommitMessageViewIsEmpty};
use common::setup::{makeNewUnstagedFile, setupTest};

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn forbidCommittingWhenNoChangesAreStaged()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("file");
    makeNewUnstagedFile(&filePath, "unstaged file content\n", &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));

    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
    assertCommitButtonTooltipContains("No changes are staged for commit.", &gui);
}