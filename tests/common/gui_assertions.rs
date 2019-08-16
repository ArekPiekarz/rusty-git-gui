use rusty_git_gui::file_change::FileChange;
use rusty_git_gui::gui::Gui;


pub fn assertUnstagedChangesViewIsEmpty(gui: &Gui)
{
    assert!(gui.unstagedChangesView.isEmpty());
}

pub fn assertStagedChangesViewIsEmpty(gui: &Gui)
{
    assert!(gui.stagedChangesView.isEmpty());
}

pub fn assertDiffViewIsEmpty(gui: &Gui)
{
    assert!(gui.diffView.isEmpty());
}

pub fn assertCommitMessageViewIsEmpty(gui: &Gui)
{
    assert!(gui.commitMessageView.isEmpty());
}

pub fn assertCommitButtonIsEnabled(gui: &Gui)
{
    assert!(gui.commitButton.isEnabled());
}

pub fn assertCommitButtonIsDisabled(gui: &Gui)
{
    assert!(gui.commitButton.isDisabled());
}

pub fn assertCommitButtonTooltipIs(tooltip: &str, gui: &Gui)
{
    assert_eq!(tooltip, gui.commitButton.getTooltip());
}
pub fn assertCommitButtonTooltipIsEmpty(gui: &Gui)
{
    assert_eq!("", gui.commitButton.getTooltip());
}

pub fn assertUnstagedChangesViewContains(changes: &[FileChange], gui: &Gui)
{
    assert_eq!(changes, &gui.unstagedChangesView.getData()[..]);
}

pub fn assertStagedChangesViewContains(changes: &[FileChange], gui: &Gui)
{
    assert_eq!(changes, &gui.stagedChangesView.getData()[..]);
}

pub fn assertDiffViewContains(content: &str, gui: &Gui)
{
    assert_eq!(content, gui.diffView.getText(),
           "\nExpected diff view content differs from actual.");
}