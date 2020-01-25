use rusty_git_gui::gui::Gui;
use std::path::Path;


pub fn show(gui: &Gui)
{
    gui.show();
    processEvents();
}

pub fn selectUnstagedChange(filePath: &Path, gui: &Gui)
{
    assert!(gui.unstagedChangesView.borrow().select(filePath.to_str().unwrap()));
    processEvents();
}

pub fn selectStagedChange(filePath: &Path, gui: &Gui)
{
    assert!(gui.stagedChangesView.borrow().select(filePath.to_str().unwrap()));
    processEvents();
}

pub fn activateUnstagedChangeToStageIt(filePath: &Path, gui: &Gui)
{
    assert!(gui.unstagedChangesView.borrow().activate(filePath.to_str().unwrap()));
    processEvents();
}

pub fn activateStagedChangeToUnstageIt(filePath: &Path, gui: &Gui)
{
    assert!(gui.stagedChangesView.borrow().activate(filePath.to_str().unwrap()));
    processEvents();
}

pub fn setCommitMessage(message: &str, gui: &Gui)
{
    gui.commitMessageView.borrow().setText(message);
    processEvents();
}

pub fn clickCommitButton(gui: &Gui)
{
    gui.commitButton.borrow().click().unwrap();
    processEvents();
}

pub fn clickRefreshButton(gui: &Gui)
{
    gui.refreshButton.click();
    processEvents();
}

pub fn selectCommitAmendCheckbox(gui: &Gui)
{
    gui.commitAmendCheckbox.borrow().select();
    processEvents();
}

pub fn unselectCommitAmendCheckbox(gui: &Gui)
{
    gui.commitAmendCheckbox.borrow().unselect();
    processEvents();
}

// private

fn processEvents()
{
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}