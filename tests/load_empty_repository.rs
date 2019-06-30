#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedFilesViewIsEmpty,
    assertUnstagedFilesViewIsEmpty};
use common::setup::setupTest;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::rc::Rc;


#[test]
fn loadEmptyRepository()
{
    let repositoryDir = setupTest();

    let gui = makeGui(Rc::new(Repository::new(repositoryDir.path())));

    assertUnstagedFilesViewIsEmpty(&gui);
    assertStagedFilesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}