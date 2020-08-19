use crate::common::file_changes_model_utils::PATH_COLUMN;
use crate::common::test_gui::TestGui;

use rusty_git_gui::gui::Gui;
use rusty_git_gui::tree_model_utils::Row;

use gtk::ButtonExt as _;
use gtk::TextBufferExt as _;
use gtk::TextViewExt as _;
use gtk::ToggleButtonExt as _;
use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::TreeViewExt as _;
use gtk::WidgetExt as _;
use std::convert::TryFrom as _;

const NO_PARENT: Option<&gtk::TreeIter> = None;


pub fn show(gui: &Gui)
{
    gui.show();
    gui.setOpacity(0.0);
    processEvents();
}

pub fn selectUnstagedChangeInRow(row: Row, gui: &TestGui)
{
    selectFileChange(row, &gui.findUnstagedChangesView());
}

pub fn selectStagedChangeInRow(row: Row, gui: &TestGui)
{
    selectFileChange(row, &gui.findStagedChangesView());
}

pub fn activateUnstagedChangeInRow(row: Row, gui: &TestGui)
{
    activateFileChangeInRow(row, &gui.findUnstagedChangesView());
}

pub fn activateStagedChangeInRow(row: Row, gui: &TestGui)
{
    activateFileChangeInRow(row, &gui.findStagedChangesView());
}

pub fn setCommitMessage(message: &str, gui: &TestGui)
{
    let view = gui.findCommitMessageView();
    view.get_buffer().unwrap().set_text(message);
    processEvents();
}

pub fn clickCommitButton(gui: &TestGui)
{
    clickButton(&gui.findCommitButton());
}

pub fn clickRefreshButton(gui: &TestGui)
{
    clickButton(&gui.findRefreshButton());
}

pub fn selectCommitAmendCheckbox(gui: &TestGui)
{
    let checkbox = gui.findCommitAmendCheckbox();
    assert!(checkbox.is_sensitive());
    assert!(!checkbox.get_active());
    checkbox.clicked();
    processEvents();
}

pub fn unselectCommitAmendCheckbox(gui: &TestGui)
{
    let checkbox = gui.findCommitAmendCheckbox();
    assert!(checkbox.is_sensitive());
    assert!(checkbox.get_active());
    checkbox.clicked();
    processEvents();
}


// private

fn processEvents()
{
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}

fn selectFileChange(row: Row, view: &gtk::TreeView)
{
    let model = view.get_model().unwrap();
    let row = i32::try_from(row).unwrap();
    let iter = model.iter_nth_child(NO_PARENT, row).unwrap();
    view.get_selection().select_iter(&iter);
    processEvents();
}

fn activateFileChangeInRow(row: Row, view: &gtk::TreeView)
{
    let model = view.get_model().unwrap();
    let row = i32::try_from(row).unwrap();
    let iter = model.iter_nth_child(NO_PARENT, row).unwrap();
    view.get_selection().select_iter(&iter);
    let rowPath = model.get_path(&iter).unwrap();
    let column = view.get_column(PATH_COLUMN).unwrap();
    view.row_activated(&rowPath, &column);
    processEvents();
}

fn clickButton(button: &gtk::Button)
{
    assert!(button.is_sensitive());
    button.clicked();
    processEvents();
}
