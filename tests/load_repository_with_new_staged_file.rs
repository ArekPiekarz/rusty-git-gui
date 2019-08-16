#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::selectStagedChange;
use common::setup::{makeNewStagedFile, setupTest};
use common::utils::makeFileChange;

use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let newStagedFilePath = PathBuf::from("stagedFile");
    makeNewStagedFile(&newStagedFilePath, "staged file content\n", &repositoryDir);

    let gui = Gui::new(Rc::new(Repository::new(&repositoryDir)));

    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &newStagedFilePath)], &gui);
    assertDiffViewIsEmpty(&gui);
    assertUnstagedChangesViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedChange(&newStagedFilePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &gui);
}