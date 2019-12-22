use crate::diff_formatter::DiffFormatter;
use crate::error_handling::exit;
use crate::file_change::FileChange;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes_view::StagedChangesView;
use crate::text_view::{Notifications, TextView};
use crate::unstaged_changes_view::UnstagedChangesView;

use std::cell::RefCell;
use std::rc::Rc;


pub struct DiffView
{
    widget: Rc<RefCell<TextView>>,
    repository: Rc<RefCell<Repository>>,
    displayState: DisplayedFileChange
}

#[derive(PartialEq)]
enum DisplayedFileChange
{
    None,
    Unstaged,
    Staged
}

impl DiffView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        unstagedChangesView: &mut UnstagedChangesView,
        stagedChangesView: &mut StagedChangesView,
        repository: Rc<RefCell<Repository>>)
        -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{
            widget: TextView::new(guiElementProvider, "Diff view", Notifications::Disabled),
            repository,
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
                rcSelf.borrow_mut().onUnstagedChangeRefreshed(&fileChangeOpt);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToStagedChangeRefreshed(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        view.connectOnRefreshed(Box::new(move |fileChangeOpt| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onStagedChangeRefreshed(&fileChangeOpt);
            }
            glib::Continue(true)
        }));
    }

    fn onUnstagedChangeSelected(&mut self, fileChange: &FileChange)
    {
        match fileChange.status.as_str() {
            "WT_RENAMED" => self.onFileChangeSelected(
                fileChange, Self::makeDiffForUnstagedRenamedFile, DisplayedFileChange::Unstaged),
            _ => self.onFileChangeSelected(
                fileChange, Self::makeDiffForUnstagedChange, DisplayedFileChange::Unstaged)
        }
    }

    fn onStagedChangeSelected(&mut self, fileChange: &FileChange)
    {
        self.onFileChangeSelected(fileChange, Self::makeDiffForStagedChange, DisplayedFileChange::Staged);
    }

    fn onFileChangeSelected(
        &mut self,
        fileChange: &FileChange,
        diffMaker: for <'a> fn(&FileChange, &'a Repository) -> git2::Diff<'a>,
        newDisplayState: DisplayedFileChange)
    {
        let mut diffFormatter = DiffFormatter::new();
        let repository = self.repository.borrow();
        let diff = (diffMaker)(&fileChange, &repository);
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffFormatter.format(&line))
            .unwrap_or_else(|e| exit(&format!("Failed to format diff: {}", e)));
        diffFormatter.finish();
        self.widget.borrow().setRichText(diffFormatter.getText());
        self.displayState = newDisplayState;
    }

    fn onUnstagedChangeUnselected(&mut self)
    {
        if self.displayState == DisplayedFileChange::Unstaged {
            self.widget.borrow().clear();
            self.displayState = DisplayedFileChange::None;
        }
    }

    fn onStagedChangeUnselected(&mut self)
    {
        if self.displayState == DisplayedFileChange::Staged {
            self.widget.borrow().clear();
            self.displayState = DisplayedFileChange::None;
        }
    }

    fn onUnstagedChangeRefreshed(&mut self, fileChangeOpt: &Option<FileChange>)
    {
        match fileChangeOpt {
            Some(fileChange) => self.onUnstagedChangeSelected(fileChange),
            None => self.onUnstagedChangeUnselected()
        }
    }

    fn onStagedChangeRefreshed(&mut self, fileChangeOpt: &Option<FileChange>)
    {
        match fileChangeOpt {
            Some(fileChange) => self.onStagedChangeSelected(fileChange),
            None => self.onStagedChangeUnselected()
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
        repository.makeDiffOfIndexToWorkdirForRenamedFile(&fileChange.oldPath.as_ref().unwrap(), &fileChange.path)
    }
}