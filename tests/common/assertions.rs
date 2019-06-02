use crate::common::accessors::{CONTINUE_ITERATING_MODEL, getCell};
use crate::common::utils::FileInfo;
use rusty_git_gui::gui_definitions::FileStatusModelColumn;
use rusty_git_gui::gui_utils::getText;
use glib::object::Cast as _;
use gtk::{TextViewExt as _, TreeModelExt as _, TreeViewExt as _, WidgetExt as _};


const NO_TEXT_CONTENT : &str = "";


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
    assertDiffViewContains(NO_TEXT_CONTENT, window);
}

pub fn assertCommitMessageViewIsEmpty(window: &gtk::Widget)
{
    assertTextViewIsEmpty(window, "Commit message view");
}

fn assertTextViewIsEmpty(window: &gtk::Widget, name: &str)
{
    assertTextViewContains(NO_TEXT_CONTENT, window, name);
}

fn assertTextViewContains(content: &str, window: &gtk::Widget, name: &str)
{
    let widget = gtk_test::find_widget_by_name(window, name).unwrap();
    let textView = widget.downcast::<gtk::TextView>().unwrap();
    let buffer = textView.get_buffer().unwrap();
    let textViewContent = getText(&buffer).unwrap();
    assert_eq!(content, textViewContent.as_str());
}

pub fn assertCommitButtonIsDisabled(window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(window, "Commit button").unwrap();
    let button = widget.downcast::<gtk::Button>().unwrap();
    assert_eq!(false, button.is_sensitive());
}

pub fn assertUnstagedFilesViewContains(files: &[FileInfo], window: &gtk::Widget)
{
    assertFilesViewContains(files, window, "Unstaged files view");
}

pub fn assertStagedFilesViewContains(files: &[FileInfo], window: &gtk::Widget)
{
    assertFilesViewContains(files, window, "Staged files view");
}

fn assertFilesViewContains(files: &[FileInfo], window: &gtk::Widget, widgetName: &str)
{
    let widget = gtk_test::find_widget_by_name(window, widgetName).unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    let mut rowCount = 0;
    model.foreach(|model, row, iter| {
        let row = row.to_string().parse::<usize>().unwrap();
        assert_eq!(files[row].status, getCell(model, iter, FileStatusModelColumn::Status),
                   "File status differs at row {} in {}.", row, widgetName.to_lowercase());
        assert_eq!(files[row].name, getCell(model, iter, FileStatusModelColumn::Path),
                   "File path differs at row {} in {}.", row, widgetName.to_lowercase());
        rowCount += 1;
        CONTINUE_ITERATING_MODEL});
    assert_eq!(files.len(), rowCount);
}

pub fn assertDiffViewContains(content: &str, window: &gtk::Widget)
{
    assertTextViewContains(content, window, "Diff view");
}