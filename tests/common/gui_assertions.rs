use crate::common::file_changes_model_utils::{PATH_COLUMN, STATUS_COLUMN};
use crate::common::file_changes_view_utils::FileChangesViewEntry;
use crate::common::test_gui::TestGui;


use gtk::prelude::TextBufferExt as _;
use gtk::prelude::TextViewExt as _;
use gtk::prelude::ToggleButtonExt as _;
use gtk::prelude::TreeModelExt as _;
use gtk::prelude::TreeViewExt as _;
use gtk::prelude::WidgetExt as _;

const CONTINUE_ITERATING_MODEL: bool = false;
const EXCLUDE_HIDDEN_CHARACTERS : bool = false;
const NO_FILE_CHANGES: Vec<FileChangesViewEntry> = vec![];


pub(crate) fn assertGuiIsEmpty(gui: &TestGui)
{
    assertUnstagedChangesViewIsEmpty(&gui);
    assertStagedChangesViewIsEmpty(&gui);
    assertDiffViewIsEmpty(&gui);
    assertCommitMessageViewIsEmpty(&gui);
    assertCommitButtonIsDisabled(&gui);
}

pub(crate) fn assertUnstagedChangesViewIsEmpty(gui: &TestGui)
{
    assert_eq!(NO_FILE_CHANGES, getFileChanges(&gui.findUnstagedChangesView()),
               "\nExpected empty unstaged changes view, but it is filled.");
}

pub(crate) fn assertStagedChangesViewIsEmpty(gui: &TestGui)
{
    assert_eq!(NO_FILE_CHANGES, getFileChanges(&gui.findStagedChangesView()),
               "\nExpected empty staged changes view, but it is filled.");
}

pub(crate) fn assertDiffViewIsEmpty(gui: &TestGui)
{
    assert_eq!("", getText(&gui.findDiffView()),
               "\nExpected empty diff view, but it is filled.");
}

pub(crate) fn assertCommitMessageViewIsEmpty(gui: &TestGui)
{
    assert_eq!("", getText(&gui.findCommitMessageView()),
               "\nExpected empty commit message view, but it is filled.");
}

pub(crate) fn assertCommitMessageViewTextIs(text: &str, gui: &TestGui)
{
    assert_eq!(text, getText(&gui.findCommitMessageView()),
               "\nExpected content of commit message view differs from actual.");
}

pub(crate) fn assertCommitButtonIsEnabled(gui: &TestGui)
{
    assert!(gui.findCommitButton().is_sensitive(),
            "Expected commit button to be enabled, but it is disabled.");
}

pub(crate) fn assertCommitButtonIsDisabled(gui: &TestGui)
{
    assert!(!gui.findCommitButton().is_sensitive(),
            "Expected commit button to be disabled, but it is enabled.");
}

pub(crate) fn assertCommitButtonTooltipIs(tooltip: &str, gui: &TestGui)
{
    assert_eq!(tooltip, gui.findCommitButton().tooltip_text().unwrap().as_str(),
               "\nExpected commit button tooltip differs from actual.");
}

pub(crate) fn assertCommitButtonTooltipIsEmpty(gui: &TestGui)
{
    assert_eq!(None, gui.findCommitButton().tooltip_text(),
               "\nExpected empty commit button tooltip, but it is filled.");
}

pub(crate) fn assertUnstagedChangesViewContains(changes: &[FileChangesViewEntry], gui: &TestGui)
{
    assert_eq!(changes, &getFileChanges(&gui.findUnstagedChangesView())[..],
               "\nExpected content of unstaged changes view differs from actual.");
}

pub(crate) fn assertStagedChangesViewContains(changes: &[FileChangesViewEntry], gui: &TestGui)
{
    assert_eq!(changes, &getFileChanges(&gui.findStagedChangesView())[..],
               "\nExpected content of staged changes view differs from actual.");
}

pub(crate) fn assertDiffViewContains(content: &str, gui: &TestGui)
{
    assert_eq!(content, getText(&gui.findDiffView()),
               "\nExpected content of diff view differs from actual.");
}

pub(crate) fn assertCommitAmendCheckboxIsEnabled(gui: &TestGui)
{
    assert!(gui.findCommitAmendCheckbox().is_sensitive(),
            "\nExpected commit amend checkbox to be enabled, but it is disabled.");
}

pub(crate) fn assertCommitAmendCheckboxIsDisabled(gui: &TestGui)
{
    assert!(!gui.findCommitAmendCheckbox().is_sensitive(),
            "\nExpected commit amend checkbox to be disabled, but it is enabled.");
}

pub(crate) fn assertCommitAmendCheckboxIsSelected(gui: &TestGui)
{
    assert!(gui.findCommitAmendCheckbox().is_active(),
            "\nExpected commit amend checkbox to be selected, but it is unselected.");
}

pub(crate) fn assertCommitAmendCheckboxIsUnselected(gui: &TestGui)
{
    assert!(!gui.findCommitAmendCheckbox().is_active(),
            "\nExpected commit amend checkbox to be unselected, but it is selected.");
}

pub(crate) fn assertCommitAmendCheckboxTooltipIs(tooltip: &str, gui: &TestGui)
{
    assert_eq!(tooltip, gui.findCommitAmendCheckbox().tooltip_text().unwrap(),
               "\nExpected content of commit amend checkbox tooltip differs from actual.");
}

pub(crate) fn assertCommitAmendCheckboxTooltipIsEmpty(gui: &TestGui)
{
    assert_eq!(None, gui.findCommitAmendCheckbox().tooltip_text(),
               "\nExpected empty commit amend checkbox tooltip, but it is filled.");
}


// private

fn getFileChanges(fileChangesView: &gtk::TreeView) -> Vec<FileChangesViewEntry>
{
    let mut content = vec![];
    fileChangesView.model().unwrap().foreach(|model, _row, iter| {
        content.push(FileChangesViewEntry{
            status: getStatusCell(model, iter),
            path: getPathCell(model, iter)});
        CONTINUE_ITERATING_MODEL });
    content
}

fn getStatusCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
{
    getCell(model, iter, STATUS_COLUMN)
}

fn getPathCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
{
    getCell(model, iter, PATH_COLUMN)
}

fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: i32) -> String
{
    model.value(iter, column).get::<String>().unwrap()
}

fn getText(textView: &gtk::TextView) -> String
{
    let buffer = textView.buffer().unwrap();
    buffer.text(&buffer.start_iter(), &buffer.end_iter(), EXCLUDE_HIDDEN_CHARACTERS).unwrap().into()
}
