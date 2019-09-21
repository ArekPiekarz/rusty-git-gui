use rusty_git_gui::file_change::FileChange;
use rusty_git_gui::gui::Gui;


pub fn assertUnstagedChangesViewIsEmpty(gui: &Gui)
{
    assert!(gui.unstagedChangesView.borrow().isEmpty());
}

pub fn assertStagedChangesViewIsEmpty(gui: &Gui)
{
    assert!(gui.stagedChangesView.borrow().isEmpty());
}

pub fn assertDiffViewIsEmpty(gui: &Gui)
{
    assert!(gui.diffView.borrow().isEmpty());
}

pub fn assertCommitMessageViewIsEmpty(gui: &Gui)
{
    assert!(gui.commitMessageView.borrow().isEmpty());
}

pub fn assertCommitButtonIsEnabled(gui: &Gui)
{
    assert!(gui.commitButton.borrow().isEnabled());
}

pub fn assertCommitButtonIsDisabled(gui: &Gui)
{
    assert!(gui.commitButton.borrow().isDisabled());
}

pub fn assertCommitButtonTooltipIs(tooltip: &str, gui: &Gui)
{
    assert_eq!(tooltip, gui.commitButton.borrow().getTooltip(), "\nExpected commit button tooltip does not match actual.");
}
pub fn assertCommitButtonTooltipIsEmpty(gui: &Gui)
{
    assert_eq!("", gui.commitButton.borrow().getTooltip());
}

pub fn assertUnstagedChangesViewContains(changes: &[FileChange], gui: &Gui)
{
    assert_eq!(changes, &gui.unstagedChangesView.borrow().getData()[..]);
}

pub fn assertStagedChangesViewContains(changes: &[FileChange], gui: &Gui)
{
    assert_eq!(changes, &gui.stagedChangesView.borrow().getData()[..]);
}

pub fn assertDiffViewContains(content: &str, gui: &Gui)
{
    assert_eq!(content, gui.diffView.borrow().getText(),
           "\nExpected diff view content differs from actual.");
}