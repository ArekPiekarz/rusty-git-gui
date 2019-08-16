#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewIsEmpty};
use common::setup::setupTest;

use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use std::rc::Rc;


#[test]
fn loadEmptyRepository()
{
    let repositoryDir = setupTest();

    let gui = Gui::new(Rc::new(Repository::new(repositoryDir.path())));

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}