#![allow(non_snake_case)]

mod common;

use common::actions::selectStagedFile;
use common::gui_assertions::{
    assertCommitButtonIsDisabled,
    assertCommitMessageViewIsEmpty,
    assertDiffViewContains,
    assertStagedFilesViewContains,
    assertUnstagedFilesViewContains};
use common::setup::{makeNewStagedFile, modifyFile, setupTest};
use common::utils::FileInfo;

use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[test]
fn loadRepositoryWithModifiedUnstagedFileAndSameNewStagedFile()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let filePath = PathBuf::from("fileName");
    makeNewStagedFile(&filePath, "staged file content\n", &repositoryDir);
    modifyFile(&filePath, "staged file content\nmodified unstaged line\n", &repositoryDir);

    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));
    gui.show();

    assertUnstagedFilesViewContains(&[FileInfo::new("WT_MODIFIED", &filePath)], &gui);
    assertDiffViewContains("@@ -1 +1,2 @@\n staged file content\n+modified unstaged line\n", &gui);
    assertStagedFilesViewContains(&[FileInfo::new("INDEX_NEW", &filePath)], &gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);

    selectStagedFile(&filePath, &gui);
    assertDiffViewContains("@@ -0,0 +1 @@\n+staged file content\n", &gui);
}