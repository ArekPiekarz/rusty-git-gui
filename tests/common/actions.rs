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
    let filePath = filePath.to_str().unwrap();
    let widget = gtk_test::find_widget_by_name(window, widgetName).unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    let mut selectedSuccessfully = false;
    model.foreach(|model, _row, iter| {
        if getCell(model, iter, FileStatusModelColumn::Path) != filePath {
            return CONTINUE_ITERATING_MODEL; }
        treeView.get_selection().select_iter(iter);
        selectedSuccessfully = true;
        STOP_ITERATING_MODEL });
    assert_eq!(true, selectedSuccessfully);
}