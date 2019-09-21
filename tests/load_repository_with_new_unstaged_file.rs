#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains};
use common::gui_interactions::show;
use common::setup::{makeGui, makeNewUnstagedFile, setupTest};
use common::utils::makeFileChange;

use std::path::PathBuf;


#[test]
fn loadRepositoryWithNewUnstagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newUnstagedFilePath = PathBuf::from("unstagedFile");
    makeNewUnstagedFile(&newUnstagedFilePath, "unstaged file content\n", &repositoryDir);

    let gui = makeGui(&repositoryDir);
    show(&gui);

    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &newUnstagedFilePath)], &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+unstaged file content\n", &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}