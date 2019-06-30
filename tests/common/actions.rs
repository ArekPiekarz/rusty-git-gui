use crate::common::accessors::getCell;

use rusty_git_gui::gui_setup::{FileChangesView, Gui};
use rusty_git_gui::gui_definitions::{CONTINUE_ITERATING_MODEL, FileChangesColumn, STOP_ITERATING_MODEL};

use gtk::{
    ButtonExt as _,
    TextBufferExt as _,
    TextViewExt as _,
    TreeModelExt as _,
    TreeSelectionExt as _,
    TreeViewExt as _};
use std::path::Path;


pub fn selectUnstagedFile(filePath: &Path, gui: &Gui)
{
    selectFile(filePath, &gui.unstagedChangesView);
}

pub fn selectStagedFile(filePath: &Path, gui: &Gui)
{
    selectFile(filePath, &gui.stagedChangesView);
}

fn selectFile(filePath: &Path, fileChangesView: &FileChangesView)
{
    invokeForRowInFilesView(
        filePath,
        fileChangesView,
        |treeView, _row, iter| { treeView.get_selection().select_iter(iter); });
}

pub fn activateUnstagedFile(filePath: &Path, gui: &Gui)
{
    activateFile(filePath, &gui.unstagedChangesView);
}

pub fn activateStagedFile(filePath: &Path, gui: &Gui)
{
    activateFile(filePath, &gui.stagedChangesView);
}

fn activateFile(filePath: &Path, fileChangesView: &FileChangesView)
{
    invokeForRowInFilesView(
        filePath,
        fileChangesView,
        |treeView, row, _iter| { treeView.row_activated(row, &getFilePathColumn(treeView)); });
}

fn invokeForRowInFilesView(
    filePath: &Path,
    fileChangesView: &FileChangesView,
    action: impl Fn(&gtk::TreeView, &gtk::TreePath, &gtk::TreeIter))
{
    let filePath = filePath.to_str().unwrap();
    let model = (*fileChangesView).get_model().unwrap();
    let mut rowFound = false;
    model.foreach(|model, row, iter| {
        if getCell(model, iter, FileChangesColumn::Path) != filePath {
            return CONTINUE_ITERATING_MODEL; }
        rowFound = true;
        action(&fileChangesView, &row, &iter);
        STOP_ITERATING_MODEL });
    assert_eq!(true, rowFound);
}

fn getFilePathColumn(treeView: &gtk::TreeView) -> gtk::TreeViewColumn
{
    treeView.get_column(FileChangesColumn::Path as i32).unwrap()
}

pub fn setCommitMessage(message: &str, gui: &Gui)
{
    let buffer = gui.commitMessageView.get_buffer().unwrap();
    buffer.insert(&mut buffer.get_start_iter(), message);
}

pub fn clickCommitButton(gui: &Gui)
{
    gui.commitButton.clicked();
}