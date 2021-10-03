use crate::diff_colorizer::DiffColorizer;
use crate::diff_formatter::DiffFormatter;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::error_handling::exit;
use crate::file_change::FileChange;
use crate::gui_element_provider::GuiElementProvider;
use crate::line_diff::LineDiff;
use crate::repository::Repository;
use crate::text_view::{Notifications, TextView};

use std::cell::RefCell;
use std::rc::Rc;


pub struct DiffView
{
    widget: TextView,
    repository: Rc<RefCell<Repository>>,
    diffColorizer: DiffColorizer,
    displayState: DisplayedFileChange,
    stagedChangeDiffMaker: DiffMaker
}

#[derive(Eq, PartialEq)]
enum DisplayedFileChange
{
    None,
    Unstaged,
    Staged
}

type DiffMaker = for <'a> fn(&FileChange, &'a Repository) -> git2::Diff<'a>;

impl IEventHandler for DiffView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        use crate::event::{Source as S, Event as E};
        match (source, event) {
            (S::CommitAmendCheckbox, E::CommitAmendDisabled)                => self.onCommitAmendDisabled(),
            (S::CommitAmendCheckbox, E::CommitAmendEnabled)                 => self.onCommitAmendEnabled(),
            (S::DiffView,            E::ZoomRequested(_))                   => self.onZoomRequested(source, event),
            (S::StagedChangesView,   E::FileChangeRefreshed(fileChangeOpt)) => self.onStagedOptionalChangeRefreshed(fileChangeOpt),
            (S::StagedChangesView,   E::FileChangeSelected(fileChange))     => self.onStagedChangeSelected(fileChange),
            (S::StagedChangesView,   E::FileChangeUnselected)               => self.onStagedChangeUnselected(),
            (S::UnstagedChangesView, E::FileChangeRefreshed(fileChangeOpt)) => self.onUnstagedOptionalChangeRefreshed(fileChangeOpt),
            (S::UnstagedChangesView, E::FileChangeSelected(fileChange))     => self.onUnstagedChangeSelected(fileChange),
            (S::UnstagedChangesView, E::FileChangeUnselected)               => self.onUnstagedChangeUnselected(),
            _ => handleUnknown(source, event)
        }
    }
}

impl DiffView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        repository: Rc<RefCell<Repository>>,
        sender: Sender)
        -> Self
    {
        let widget = TextView::new(
            guiElementProvider, "Diff view", sender, Source::DiffView, Notifications::Disabled);
        let diffColorizer = DiffColorizer::new(&widget);
        Self{
            widget,
            repository,
            diffColorizer,
            displayState: DisplayedFileChange::None,
            stagedChangeDiffMaker: makeDiffForStagedChange
        }
    }


    // private

    fn onUnstagedChangeSelected(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "WT_RENAMED" => self.onFileChangeSelected(
                fileChange, makeDiffForUnstagedRenamedFile, DisplayedFileChange::Unstaged),
            _ => self.onFileChangeSelected(
                fileChange, makeDiffForUnstagedChange, DisplayedFileChange::Unstaged)
        }
    }

    fn onStagedChangeSelected(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "INDEX_RENAMED" => {
                self.onFileChangeSelected(fileChange, makeDiffForStagedRenamedFile, DisplayedFileChange::Staged)
            },
            _ => self.onFileChangeSelected(fileChange, self.stagedChangeDiffMaker, DisplayedFileChange::Staged)
        }
    }

    fn onFileChangeSelected(
        &mut self,
        fileChange: &FileChange,
        diffMaker: DiffMaker,
        newDisplayState: DisplayedFileChange)
    {
        let diff = self.makeFormattedDiff(fileChange, diffMaker);
        self.diffColorizer.colorize(&self.widget, &diff);
        self.displayState = newDisplayState;
    }

    fn onFileChangeRefreshed(
        &mut self,
        fileChange: &FileChange,
        diffMaker: DiffMaker,
        newDisplayState: DisplayedFileChange)
    {
        let oldDiff = self.widget.getText();
        let newDiff = self.makeFormattedDiff(fileChange, diffMaker);
        let changeset = similar::TextDiff::configure().diff_lines(&oldDiff, &newDiff);
        let changeset: Vec<_> = changeset.iter_all_changes().map(
            |change| match change.tag() {
                similar::ChangeTag::Equal => LineDiff::Equal(change.to_string_lossy()),
                similar::ChangeTag::Delete => LineDiff::Delete(change.to_string_lossy()),
                similar::ChangeTag::Insert => LineDiff::Insert(change.to_string_lossy())
            }).collect();
        self.diffColorizer.update(&self.widget, changeset);
        self.displayState = newDisplayState;
    }

    fn onUnstagedChangeUnselected(&mut self)
    {
        if self.displayState == DisplayedFileChange::Unstaged {
            self.clear();
        }
    }

    fn onStagedChangeUnselected(&mut self)
    {
        if self.displayState == DisplayedFileChange::Staged {
            self.clear();
        }
    }

    fn onUnstagedOptionalChangeRefreshed(&mut self, fileChangeOpt: &Option<FileChange>)
    {
        if self.displayState != DisplayedFileChange::Unstaged {
            return;
        }

        match fileChangeOpt {
            Some(fileChange) => self.onUnstagedChangeRefreshed(fileChange),
            None => self.clear()
        }
    }

    fn onStagedOptionalChangeRefreshed(&mut self, fileChangeOpt: &Option<FileChange>)
    {
        if self.displayState != DisplayedFileChange::Staged {
            return;
        }

        match fileChangeOpt {
            Some(fileChange) => self.onStagedChangeRefreshed(fileChange),
            None => self.clear()
        }
    }

    fn onUnstagedChangeRefreshed(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "WT_RENAMED" => self.onFileChangeRefreshed(
                fileChange, makeDiffForUnstagedRenamedFile, DisplayedFileChange::Unstaged),
            _ => self.onFileChangeRefreshed(
                fileChange, makeDiffForUnstagedChange, DisplayedFileChange::Unstaged)
        }
    }

    fn onStagedChangeRefreshed(&mut self, fileChange: &FileChange)
    {
        self.onFileChangeRefreshed(fileChange, self.stagedChangeDiffMaker, DisplayedFileChange::Staged);
    }

    fn onZoomRequested(&mut self, source: Source, event: &Event)
    {
        self.widget.handle(source, event);
    }

    fn onCommitAmendDisabled(&mut self)
    {
        self.stagedChangeDiffMaker = makeDiffForStagedChange;
    }

    fn onCommitAmendEnabled(&mut self)
    {
        self.stagedChangeDiffMaker = makeDiffForStagedChangeToAmend;
    }

    fn clear(&mut self)
    {
        self.widget.clear();
        self.displayState = DisplayedFileChange::None;
    }

    fn makeFormattedDiff(&self, fileChange: &FileChange, diffMaker: DiffMaker) -> String
    {
        let mut diffFormatter = DiffFormatter::newForFileChange(fileChange);
        let repository = self.repository.borrow();
        let diff = (diffMaker)(fileChange, &repository);
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffFormatter.format(&line))
            .unwrap_or_else(|e| exit(&format!("Failed to format diff: {}", e)));
        diffFormatter.takeText()
    }
}

fn makeDiffForUnstagedChange<'a>(fileChange: &FileChange, repository: &'a Repository) -> git2::Diff<'a>
{
    repository.makeDiffOfIndexToWorkdir(&fileChange.path)
}

fn makeDiffForStagedChange<'a>(fileChange: &FileChange, repository: &'a Repository) -> git2::Diff<'a>
{
    repository.makeDiffOfTreeToIndex(&fileChange.path)
}

fn makeDiffForStagedChangeToAmend<'a>(fileChange: &FileChange, repository: &'a Repository) -> git2::Diff<'a>
{
    repository.makeDiffToAmendForPath(&fileChange.path)
}

fn makeDiffForStagedRenamedFile<'a>(fileChange: &FileChange, repository: &'a Repository) -> git2::Diff<'a>
{
    repository.makeDiffOfTreeToIndexForRenamedFile(fileChange.oldPath.as_ref().unwrap(), &fileChange.path)
}

fn makeDiffForUnstagedRenamedFile<'a>(fileChange: &FileChange, repository: &'a Repository) -> git2::Diff<'a>
{
    repository.makeDiffOfIndexToWorkdirForRenamedFile(fileChange.oldPath.as_ref().unwrap(), &fileChange.path)
}
