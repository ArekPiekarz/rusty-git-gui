use crate::diff_line_printer::DiffLinePrinter;
use crate::error_handling::exit;
use crate::file_change::FileChange;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes_view::StagedChangesView;
use crate::text_view::TextView;
use crate::unstaged_changes_view::UnstagedChangesView;

use std::cell::RefCell;
use std::rc::Rc;


pub struct DiffView
{
    widget: Rc<RefCell<TextView>>,
    repository: Rc<RefCell<Repository>>,
    displayState: DisplayState
}

#[derive(PartialEq)]
enum DisplayState
{
    NoFileChange,
    UnstagedFileChange,
    StagedFileChange
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
            widget: TextView::new(guiElementProvider, "Diff view"),
            repository,
            displayState: DisplayState::NoFileChange
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
    }

    fn connectSelfToStagedChangesView(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        Self::connectSelfToStagedChangeSelected(rcSelf, view);
        Self::connectSelfToStagedChangeUnselected(rcSelf, view);
    }

    fn connectSelfToUnstagedChangeSelected(rcSelf: &Rc<RefCell<Self>>, view: &mut UnstagedChangesView)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        view.connectOnSelected(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUnstagedChangeSelected(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToStagedChangeSelected(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        view.connectOnSelected(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onStagedChangeSelected(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToUnstagedChangeUnselected(rcSelf: &Rc<RefCell<Self>>, view: &mut UnstagedChangesView)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        view.connectOnUnselected(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUnstagedChangeUnselected();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToStagedChangeUnselected(rcSelf: &Rc<RefCell<Self>>, view: &mut StagedChangesView)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        view.connectOnUnselected(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onStagedChangeUnselected();
            }
            glib::Continue(true)
        }));
    }

    fn onUnstagedChangeSelected(&mut self, fileChange: &FileChange)
    {
        self.onFileChangeSelected(fileChange, Self::makeDiffForUnstagedChange, DisplayState::UnstagedFileChange);
    }

    fn onStagedChangeSelected(&mut self, fileChange: &FileChange)
    {
        self.onFileChangeSelected(fileChange, Self::makeDiffForStagedChange, DisplayState::StagedFileChange);
    }

    fn onFileChangeSelected(
        &mut self,
        fileChange: &FileChange,
        diffMaker: for <'a> fn(&Self, &str, &'a Repository) -> git2::Diff<'a>,
        newDisplayState: DisplayState)
    {
        let widget = self.widget.borrow();
        let diffLinePrinter = DiffLinePrinter::new(&widget);
        let repository = self.repository.borrow();
        let diff = (diffMaker)(self, &fileChange.path, &repository);
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffLinePrinter.printDiff(&line))
            .unwrap_or_else(|e| exit(&format!("Failed to print diff: {}", e)));
        self.displayState = newDisplayState;
    }

    fn onUnstagedChangeUnselected(&mut self)
    {
        if self.displayState == DisplayState::UnstagedFileChange {
            self.widget.borrow().clear();
            self.displayState = DisplayState::NoFileChange;
        }
    }

    fn onStagedChangeUnselected(&mut self)
    {
        if self.displayState == DisplayState::StagedFileChange {
            self.widget.borrow().clear();
            self.displayState = DisplayState::NoFileChange;
        }
    }

    fn makeDiffForUnstagedChange<'a>(&self, path: &str, repository: &'a Repository) -> git2::Diff<'a>
    {
        repository.makeDiffOfIndexToWorkdir(path)
    }

    fn makeDiffForStagedChange<'a>(&self, path: &str, repository: &'a Repository) -> git2::Diff<'a>
    {
        repository.makeDiffOfTreeToIndex(path)
    }
}