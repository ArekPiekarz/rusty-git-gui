use crate::common::accessors::{getCell, getFirstRowCell};
use crate::common::utils::FileInfo;

use rusty_git_gui::gui_definitions::{CONTINUE_ITERATING_MODEL, FileChangesColumn};
use rusty_git_gui::gui_setup::{FileChangesView, Gui, TextView};
use rusty_git_gui::gui_utils::getText;

use gtk::{TextViewExt as _, TreeModelExt as _, TreeViewExt as _, WidgetExt as _};
use more_asserts::assert_lt;


const NO_TEXT_CONTENT : &str = "";


pub fn assertUnstagedFilesViewIsEmpty(gui: &Gui)
{
    assertFilesViewIsEmpty(&gui.unstagedChangesView);
}

pub fn assertStagedFilesViewIsEmpty(gui: &Gui)
{
    assertFilesViewIsEmpty(&gui.stagedChangesView);
}

fn assertFilesViewIsEmpty(view: &FileChangesView)
{
    let model = view.get_model().unwrap();
    assert_eq!(None, model.get_iter_first(),
               "{} is not empty, the first row is: [{}, {}]",
               view.name(),
               getFirstRowCell(&model, FileChangesColumn::Status),
               getFirstRowCell(&model, FileChangesColumn::Path));
}

pub fn assertDiffViewIsEmpty(gui: &Gui)
{
    assertDiffViewContains(NO_TEXT_CONTENT, gui);
}

pub fn assertCommitMessageViewIsEmpty(gui: &Gui)
{
    assertTextViewIsEmpty(&gui.commitMessageView);
}

fn assertTextViewIsEmpty(textView: &TextView)
{
    assertTextViewContains(NO_TEXT_CONTENT, textView);
}

fn assertTextViewContains(content: &str, textView: &TextView)
{
    let buffer = textView.get_buffer().unwrap();
    let textViewContent = getText(&buffer).unwrap();
    assert_eq!(content, textViewContent.as_str(),
               "\nExpected {} content differs from actual.", textView.name().to_lowercase());
}

pub fn assertCommitButtonIsEnabled(gui: &Gui)
{
    assert_eq!(true, gui.commitButton.is_sensitive());
}

pub fn assertCommitButtonIsDisabled(gui: &Gui)
{
    assert_eq!(false, gui.commitButton.is_sensitive());
}

pub fn assertCommitButtonTooltipContains(tooltip: &str, gui: &Gui)
{
    assert_eq!(tooltip, gui.commitButton.get_tooltip_text().unwrap().as_str());
}

pub fn assertCommitButtonTooltipIsEmpty(gui: &Gui)
{
    assert_eq!(None, gui.commitButton.get_tooltip_text());
}

pub fn assertUnstagedFilesViewContains(files: &[FileInfo], gui: &Gui)
{
    assertFilesViewContains(files, &gui.unstagedChangesView);
}

pub fn assertStagedFilesViewContains(files: &[FileInfo], gui: &Gui)
{
    assertFilesViewContains(files, &gui.stagedChangesView);
}

fn assertFilesViewContains(files: &[FileInfo], fileChangesView: &FileChangesView)
{
    let model = (*fileChangesView).get_model().unwrap();
    let mut rowCount = 0;
    model.foreach(|model, row, iter| {
        let row = row.to_string().parse::<usize>().unwrap();
        assert_lt!(row, files.len(),
                   "{} has more rows than expected. The unexpected row is: [{}, {}]",
                   fileChangesView.name(),
                   getCell(model, iter, FileChangesColumn::Status),
                   getCell(model, iter, FileChangesColumn::Path));
        assert_eq!(files[row].status, getCell(model, iter, FileChangesColumn::Status),
                   "File status differs at row {} in {}.", row, fileChangesView.name().to_lowercase());
        assert_eq!(files[row].path, getCell(model, iter, FileChangesColumn::Path),
                   "File path differs at row {} in {}.", row, fileChangesView.name().to_lowercase());
        rowCount += 1;
        CONTINUE_ITERATING_MODEL});
    assert_eq!(files.len(), rowCount, "{} contained too few rows.", fileChangesView.name());
}

pub fn assertDiffViewContains(content: &str, gui: &Gui)
{
    assertTextViewContains(content, &gui.diffView);
}