use super::utils::{FileInfo, getCell};
use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use rusty_git_gui::gui_utils::getText;
use glib::object::Cast as _;
use gtk::{TextViewExt as _, TreeModelExt as _, TreeViewExt as _, WidgetExt as _};

// https://developer.gnome.org/gtk3/stable/GtkTreeModel.html#gtk-tree-model-foreach
const CONTINUE_ITERATING: bool = false;


pub fn assertUnstagedFilesViewIsEmpty(window: &gtk::Widget)
{
    assertFilesViewIsEmpty(window, "Unstaged files view");
}

pub fn assertStagedFilesViewIsEmpty(window: &gtk::Widget)
{
    assertFilesViewIsEmpty(window, "Staged files view");
}

fn assertFilesViewIsEmpty(window: &gtk::Widget, name: &str)
{
    let widget = gtk_test::find_widget_by_name(window, name).unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    assert_eq!(None, model.get_iter_first());
}

pub fn assertDiffViewIsEmpty(window: &gtk::Widget)
{
    assertTextViewIsEmpty(window, "Diff view");
}

pub fn assertCommitMessageViewIsEmpty(window: &gtk::Widget)
{
    assertTextViewIsEmpty(window, "Commit message view");
}

fn assertTextViewIsEmpty(window: &gtk::Widget, name: &str)
{
    let widget = gtk_test::find_widget_by_name(window, name).unwrap();
    let textView = widget.downcast::<gtk::TextView>().unwrap();
    let buffer = textView.get_buffer().unwrap();
    let textViewContent = getText(&buffer).unwrap();
    assert_eq!("", textViewContent.as_str());
}

pub fn assertCommitButtonIsDisabled(window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(window, "Commit button").unwrap();
    let button = widget.downcast::<gtk::Button>().unwrap();
    assert_eq!(false, button.is_sensitive());
}

pub fn assertUnstagedFilesViewContains(window: &gtk::Widget, files: &[FileInfo])
{
    let widget = gtk_test::find_widget_by_name(window, "Unstaged files view").unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    model.foreach(|model, row, iter| {
        let row = row.to_string().parse::<usize>().unwrap();
        assert_eq!(files[row].status, getCell(model, iter, FileStatusModelColumn::Status));
        assert_eq!(files[row].name, getCell(model, iter, FileStatusModelColumn::Path));
        CONTINUE_ITERATING});
}