use crate::common::accessors::{CONTINUE_ITERATING_MODEL, getCell, STOP_ITERATING_MODEL};
use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use glib::object::Cast as _;
use gtk::{TreeModelExt as _, TreeSelectionExt as _, TreeViewExt as _};


pub fn selectStagedFile(filePath: &str, window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(window, "Staged files view").unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    let mut selectedSuccessfully = false;
    model.foreach(|model, _row, iter| {
        if getCell(model, iter, FileStatusModelColumn::Path) == filePath {
            treeView.get_selection().select_iter(iter);
            selectedSuccessfully = true;
            return STOP_ITERATING_MODEL;}
        CONTINUE_ITERATING_MODEL});
    assert_eq!(true, selectedSuccessfully);
}