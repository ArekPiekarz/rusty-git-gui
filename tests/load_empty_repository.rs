#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewIsEmpty};
use common::setup::{makeGui, setupTest};


#[test]
fn loadEmptyRepository()
{
    let repositoryDir = setupTest();

    let gui = makeGui(repositoryDir.path());

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}