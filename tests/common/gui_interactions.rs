use rusty_git_gui::gui::Gui;
use std::path::Path;


pub fn selectUnstagedChange(filePath: &Path, gui: &Gui)
{
    assert!(gui.unstagedChangesView.select(filePath.to_str().unwrap()));
}

pub fn selectStagedChange(filePath: &Path, gui: &Gui)
{
    assert!(gui.stagedChangesView.select(filePath.to_str().unwrap()));
}

pub fn activateUnstagedChange(filePath: &Path, gui: &Gui)
{
    assert!(gui.unstagedChangesView.activate(filePath.to_str().unwrap()));
}

pub fn activateStagedChange(filePath: &Path, gui: &Gui)
{
    assert!(gui.stagedChangesView.activate(filePath.to_str().unwrap()));
}

pub fn setCommitMessage(message: &str, gui: &Gui)
{
    gui.commitMessageView.setText(message);
}

pub fn clickCommitButton(gui: &Gui)
{
    gui.commitButton.click();
}