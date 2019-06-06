use crate::common::accessors::{CONTINUE_ITERATING_MODEL, getCell, STOP_ITERATING_MODEL};
use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use glib::object::Cast as _;
use gtk::{TreeModelExt as _, TreeSelectionExt as _, TreeViewExt as _};
use std::path::Path;


pub fn selectUnstagedFile(filePath: &Path, window: &gtk::Widget)
{
    selectFile(filePath, "Unstaged files view", window);
}

pub fn selectStagedFile(filePath: &Path, window: &gtk::Widget)
{
    selectFile(filePath, "Staged files view", window);
}

fn selectFile(filePath: &Path, widgetName: &str, window: &gtk::Widget)
{
    invokeForRowInFilesView(
        filePath,
        widgetName,
        window,
        |treeView, _row, iter| { treeView.get_selection().select_iter(iter); });
}

pub fn activateUnstagedFile(filePath: &Path, window: &gtk::Widget)
{
    activateFile(filePath, "Unstaged files view", window);
}

pub fn activateStagedFile(filePath: &Path, window: &gtk::Widget)
{
    activateFile(filePath, "Staged files view", window);
}

fn activateFile(filePath: &Path, widgetName: &str, window: &gtk::Widget)
{
    invokeForRowInFilesView(
        filePath,
        widgetName,
        window,
        |treeView, row, _iter| { treeView.row_activated(row, &getFilePathColumn(treeView)); });
}

fn invokeForRowInFilesView(
    filePath: &Path,
    widgetName: &str,
    window: &gtk::Widget,
    action: impl Fn(&gtk::TreeView, &gtk::TreePath, &gtk::TreeIter))
{
    let filePath = filePath.to_str().unwrap();
    let widget = gtk_test::find_widget_by_name(window, widgetName).unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    let mut rowFound = false;
    model.foreach(|model, row, iter| {
        if getCell(model, iter, FileStatusModelColumn::Path) != filePath {
            return CONTINUE_ITERATING_MODEL; }
        rowFound = true;
        action(&treeView, &row, &iter);
        STOP_ITERATING_MODEL });
    assert_eq!(true, rowFound);
}

fn getFilePathColumn(treeView: &gtk::TreeView) -> gtk::TreeViewColumn
{
    treeView.get_column(FileStatusModelColumn::Path as i32).unwrap()
}