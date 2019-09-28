#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewIsEmpty,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewIsEmpty};
use common::repository_assertions::{assertRepositoryHasNoCommits, assertRepositoryStatusIsEmpty};
use common::setup::{makeGui, setupTest};


#[test]
fn loadEmptyRepository()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let gui = makeGui(&repositoryDir);

    assertRepositoryStatusIsEmpty(&repositoryDir);
    assertRepositoryHasNoCommits(&repositoryDir);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}