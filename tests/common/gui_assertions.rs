use rusty_git_gui::file_changes_view_entry::FileChangesViewEntry;
use rusty_git_gui::gui::Gui;


pub fn assertGuiIsEmpty(gui: &Gui)
{
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}

pub fn assertUnstagedChangesViewIsEmpty(gui: &Gui)
{
    assert!(gui.unstagedChangesView.borrow().isEmpty(),
            "Expected empty Unstaged changes view, instead got: {:?}",
            gui.unstagedChangesView.borrow().getData());
}

pub fn assertStagedChangesViewIsEmpty(gui: &Gui)
{
    assert!(gui.stagedChangesView.borrow().isEmpty(),
            "Expected empty Staged changes view, instead got: {:?}",
            gui.stagedChangesView.borrow().getData());
}

pub fn assertDiffViewIsEmpty(gui: &Gui)
{
    assert!(gui.diffView.borrow().isEmpty(),
            "Expected empty Diff view, instead got: {:?}",
            gui.diffView.borrow().getText());
}

pub fn assertCommitMessageViewIsEmpty(gui: &Gui)
{
    assert!(gui.commitMessageView.borrow().isEmpty(),
            "Expected empty Commit message view, instead got: {:?}",
            gui.commitMessageView.borrow().getText());
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
    assert_eq!(tooltip, gui.commitButton.borrow().getTooltip(),
               "\nExpected commit button tooltip does not match actual.");
}
pub fn assertCommitButtonTooltipIsEmpty(gui: &Gui)
{
    assert_eq!("", gui.commitButton.borrow().getTooltip());
}

pub fn assertUnstagedChangesViewContains(changes: &[FileChangesViewEntry], gui: &Gui)
{
    assert_eq!(changes, &gui.unstagedChangesView.borrow().getData()[..],
               "\nExpected unstaged changes view content differs from actual.");
}

pub fn assertStagedChangesViewContains(changes: &[FileChangesViewEntry], gui: &Gui)
{
    assert_eq!(changes, &gui.stagedChangesView.borrow().getData()[..],
               "\nExpected staged changes view content differs from actual.");
}

pub fn assertDiffViewContains(content: &str, gui: &Gui)
{
    assert_eq!(content, gui.diffView.borrow().getText(),
               "\nExpected diff view content differs from actual.");
}