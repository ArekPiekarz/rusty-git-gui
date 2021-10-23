use crate::common::file_changes_model_utils::PATH_COLUMN;
use crate::common::test_gui::TestGui;

use rusty_git_gui::gui::Gui;
use rusty_git_gui::tree_model_utils::Row;

use gtk::prelude::ButtonExt as _;
use gtk::prelude::TextBufferExt as _;
use gtk::prelude::TextViewExt as _;
use gtk::prelude::ToggleButtonExt as _;
use gtk::prelude::TreeModelExt as _;
use gtk::prelude::TreeSelectionExt as _;
use gtk::prelude::TreeViewExt as _;
use gtk::prelude::WidgetExt as _;

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
    view.buffer().unwrap().set_text(message);
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
    assert!(!checkbox.is_active());
    checkbox.clicked();
    processEvents();
}

pub fn unselectCommitAmendCheckbox(gui: &TestGui)
{
    let checkbox = gui.findCommitAmendCheckbox();
    assert!(checkbox.is_sensitive());
    assert!(checkbox.is_active());
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
    let model = view.model().unwrap();
    let row = i32::try_from(row).unwrap();
    let iter = model.iter_nth_child(NO_PARENT, row).unwrap();
    view.selection().select_iter(&iter);
    processEvents();
}

fn activateFileChangeInRow(row: Row, view: &gtk::TreeView)
{
    let model = view.model().unwrap();
    let row = i32::try_from(row).unwrap();
    let iter = model.iter_nth_child(NO_PARENT, row).unwrap();
    view.selection().select_iter(&iter);
    let rowPath = model.path(&iter).unwrap();
    let column = view.column(PATH_COLUMN).unwrap();
    view.row_activated(&rowPath, &column);
    processEvents();
}

fn clickButton(button: &gtk::Button)
{
    assert!(button.is_sensitive());
    button.clicked();
    processEvents();
}
