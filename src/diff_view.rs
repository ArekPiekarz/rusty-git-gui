use crate::diff_colorizer::DiffColorizer;
use crate::diff_formatter::DiffFormatter;
use crate::error_handling::exit;
use crate::file_change::FileChange;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes_view::StagedChangesView;
use crate::text_view::{Notifications, TextView};
use crate::unstaged_changes_view::UnstagedChangesView;

use difference::Changeset;
use std::cell::RefCell;
use std::rc::Rc;


pub struct DiffView
{
    widget: Rc<RefCell<TextView>>,
    repository: Rc<RefCell<Repository>>,
    diffColorizer: DiffColorizer,
    displayState: DisplayedFileChange
}

#[derive(Eq, PartialEq)]
enum DisplayedFileChange
{
    None,
    Unstaged,
    Staged
}

type DiffMaker = for <'a> fn(&FileChange, &'a Repository) -> git2::Diff<'a>;

impl DiffView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        unstagedChangesView: &mut UnstagedChangesView,
        stagedChangesView: &mut StagedChangesView,
        repository: Rc<RefCell<Repository>>)
        -> Rc<RefCell<Self>>
    {
        let widget = TextView::new(guiElementProvider, "Diff view", Notifications::Disabled);
        let diffColorizer = DiffColorizer::new(Rc::clone(&widget));
        let newSelf = Rc::new(RefCell::new(Self{
            widget,
            repository,
            diffColorizer,
            displayState: DisplayedFileChange::None
        }));
        Self::connectSelfToUnstagedChangesView(&newSelf, unstagedChangesView);
        Self::connectSelfToStagedChangesView(&newSelf, stagedChangesView);
        newSelf
    }

    pub fn getText(&self) -> String
    {
        self.widget.borrow().getText()
    }

    pub fn isEmpty(&self) -> bool
    {
        self.widget.borrow().isEmpty()
    }


    // private

    fn connectSelfToUnstagedChangesView(rcSelf: &Rc<RefCell<Self>>, view: &mut UnstagedChangesView)
    {
        Self::connectSelfToUnstagedChangeSelected(rcSelf, view);
        Self::connectSelfToUnstagedChangeUnselected(rcSelf, view);
        Self::connectSelfToUnstagedChangeRefreshed(rcSelf, view);
    }

    fn connectSelfToStagedChangesView(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        Self::connectSelfToStagedChangeSelected(rcSelf, view);
        Self::connectSelfToStagedChangeUnselected(rcSelf, view);
        Self::connectSelfToStagedChangeRefreshed(rcSelf, view);
    }

    fn connectSelfToUnstagedChangeSelected(rcSelf: &Rc<RefCell<Self>>, view: &mut UnstagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnSelected(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUnstagedChangeSelected(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToStagedChangeSelected(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnSelected(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onStagedChangeSelected(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToUnstagedChangeUnselected(rcSelf: &Rc<RefCell<Self>>, view: &mut UnstagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnUnselected(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUnstagedChangeUnselected();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToStagedChangeUnselected(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnUnselected(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onStagedChangeUnselected();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToUnstagedChangeRefreshed(rcSelf: &Rc<RefCell<Self>>, view: &mut UnstagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnRefreshed(Box::new(move |fileChangeOpt| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUnstagedOptionalChangeRefreshed(&fileChangeOpt);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToStagedChangeRefreshed(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnRefreshed(Box::new(move |fileChangeOpt| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onStagedOptionalChangeRefreshed(&fileChangeOpt);
            }
            glib::Continue(true)
        }));
    }

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
        self.onFileChangeSelected(fileChange, makeDiffForStagedChange, DisplayedFileChange::Staged);
    }

    fn onFileChangeSelected(
        &mut self,
        fileChange: &FileChange,
        diffMaker: DiffMaker,
        newDisplayState: DisplayedFileChange)
    {
        let diff = self.makeFormattedDiff(fileChange, diffMaker);
        self.diffColorizer.colorize(&diff);
        self.displayState = newDisplayState;
    }

    fn onFileChangeRefreshed(
        &mut self,
        fileChange: &FileChange,
        diffMaker: DiffMaker,
        newDisplayState: DisplayedFileChange)
    {
        let oldDiff = self.widget.borrow().getText();
        let newDiff = self.makeFormattedDiff(fileChange, diffMaker);
        let changeset = Changeset::new(&oldDiff, &newDiff, "\n");
        self.diffColorizer.update(changeset.diffs);
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
        self.onFileChangeRefreshed(fileChange, makeDiffForStagedChange, DisplayedFileChange::Staged);
    }

    fn clear(&mut self)
    {
        self.widget.borrow().clear();
        self.displayState = DisplayedFileChange::None;
    }

    fn makeFormattedDiff(&self, fileChange: &FileChange, diffMaker: DiffMaker) -> String
    {
        let mut diffFormatter = DiffFormatter::new();
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

fn makeDiffForUnstagedRenamedFile<'a>(fileChange: &FileChange, repository: &'a Repository) -> git2::Diff<'a>
{
    repository.makeDiffOfIndexToWorkdirForRenamedFile(fileChange.oldPath.as_ref().unwrap(), &fileChange.path)
}