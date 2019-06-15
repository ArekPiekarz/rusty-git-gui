use crate::common::accessors::{getCell, getFirstRowCell};
use crate::common::utils::FileInfo;
use rusty_git_gui::gui_definitions::{CONTINUE_ITERATING_MODEL, FileStatusModelColumn};
use rusty_git_gui::gui_utils::getText;

use glib::object::Cast as _;
use gtk::{TextViewExt as _, TreeModelExt as _, TreeViewExt as _, WidgetExt as _};
use more_asserts::assert_lt;


const NO_TEXT_CONTENT : &str = "";
const BUTTON_ENABLED : bool = true;
const BUTTON_DISABLED : bool = false;


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
    assert_eq!(None, model.get_iter_first(),
               "{} is not empty, the first row is: [{}, {}]",
               name,
               getFirstRowCell(&model, FileStatusModelColumn::Status),
               getFirstRowCell(&model, FileStatusModelColumn::Path));
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

pub fn assertCommitButtonIsEnabled(window: &gtk::Widget)
{
    assertCommitButtonIsInState(BUTTON_ENABLED, window);
}

pub fn assertCommitButtonIsDisabled(window: &gtk::Widget)
{
    assertCommitButtonIsInState(BUTTON_DISABLED, window);
}

fn assertCommitButtonIsInState(buttonState: bool, window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(window, "Commit button").unwrap();
    let button = widget.downcast::<gtk::Button>().unwrap();
    assert_eq!(buttonState, button.is_sensitive());
}

pub fn assertCommitButtonTooltipContains(tooltip: &str, window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(&*window, "Commit button").unwrap();
    assert_eq!(tooltip, widget.get_tooltip_text().unwrap().as_str());
}

pub fn assertCommitButtonTooltipIsEmpty(window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(&*window, "Commit button").unwrap();
    assert_eq!(None, widget.get_tooltip_text());
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
        assert_lt!(row, files.len(),
                   "{} has more rows than expected. The unexpected row is: [{}, {}]",
                   widgetName,
                   getCell(model, iter, FileStatusModelColumn::Status),
                   getCell(model, iter, FileStatusModelColumn::Path));
        assert_eq!(files[row].status, getCell(model, iter, FileStatusModelColumn::Status),
                   "File status differs at row {} in {}.", row, widgetName.to_lowercase());
        assert_eq!(files[row].path, getCell(model, iter, FileStatusModelColumn::Path),
                   "File path differs at row {} in {}.", row, widgetName.to_lowercase());
        rowCount += 1;
        CONTINUE_ITERATING_MODEL});
    assert_eq!(files.len(), rowCount, "{} contained too few rows.", widgetName);
}

pub fn assertDiffViewContains(content: &str, window: &gtk::Widget)
{
    assertTextViewContains(content, window, "Diff view");
}