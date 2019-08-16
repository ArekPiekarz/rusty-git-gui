#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertDiffViewContains,
    assertDiffViewIsEmpty,
    assertStagedChangesViewContains,
    assertStagedChangesViewIsEmpty,
    assertUnstagedChangesViewContains,
    assertUnstagedChangesViewIsEmpty};
use common::gui_interactions::{activateUnstagedChange, selectStagedChange};
use common::setup::{makeNewUnstagedFile, setupTest};
use common::utils::makeFileChange;

use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn stageNewFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewUnstagedFile(&filePath, "file content\n", &repositoryDir);

    let gui = Gui::new(Rc::new(Repository::new(&repositoryDir)));
    gui.show();

    assertUnstagedChangesViewContains(&[makeFileChange("WT_NEW", &filePath)], &gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);

    activateUnstagedChange(&filePath, &gui);

    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewContains(&[makeFileChange("INDEX_NEW", &filePath)], &gui);
    assertDiffViewIsEmpty(&gui);

    selectStagedChange(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+file content\n", &gui);
}