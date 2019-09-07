use crate::diff_line_printer::DiffLinePrinter;
use crate::diff_maker::{DiffMaker, StagedDiffMaker, UnstagedDiffMaker};
use crate::error_handling::exit;
use crate::file_change::FileChange;
use crate::file_changes_view_observer::FileChangesViewObserver;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes_view::StagedChangesView;
use crate::text_view::TextView;
use crate::unstaged_changes_view::UnstagedChangesView;

use std::rc::Rc;


pub struct DiffView
{
    widget: Rc<TextView>,
    // silence false positives about dead code
    // the observers have to be stored here so the clones in closures can be upgraded from Weak to Rc
    #[allow(dead_code)]
    unstagedChangesViewObserver: Rc<UnstagedChangesViewObserver>,
    #[allow(dead_code)]
    stagedChangesViewObserver: Rc<StagedChangesViewObserver>
}

impl DiffView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        unstagedChangesView: &UnstagedChangesView,
        stagedChangesView: &StagedChangesView,
        repository: Rc<Repository>)
        -> Self
    {
        let widget = TextView::new(guiElementProvider, "Diff view");
        let diffViewHandler = Rc::new(DiffViewHandler::new(Rc::clone(&widget), repository));
        Self{
            widget,
            unstagedChangesViewObserver: UnstagedChangesViewObserver::new(
                Rc::clone(&diffViewHandler), unstagedChangesView),
            stagedChangesViewObserver: StagedChangesViewObserver::new(
                diffViewHandler, stagedChangesView)
        }
    }

    pub fn getText(&self) -> String
    {
        self.widget.getText()
    }

    pub fn isEmpty(&self) -> bool
    {
        self.widget.isEmpty()
    }
}


struct DiffViewHandler
{
    widget: Rc<TextView>,
    unstagedDiffMaker: UnstagedDiffMaker,
    stagedDiffMaker: StagedDiffMaker
}

impl DiffViewHandler
{
    fn new(widget: Rc<TextView>, repository: Rc<Repository>) -> Self
    {
        Self{
            widget,
            unstagedDiffMaker: UnstagedDiffMaker::new(Rc::clone(&repository)),
            stagedDiffMaker: StagedDiffMaker::new(repository)
        }
    }

    fn onUnstagedSelected(&self, fileChange: &FileChange)
    {
        let diffLinePrinter = DiffLinePrinter::new(&self.widget);
        let diff = self.unstagedDiffMaker.makeDiff(&fileChange.path);
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffLinePrinter.printDiff(&line))
            .unwrap_or_else(|e| exit(&format!("Failed to print diff: {}", e)));
    }

    fn onUnstagedDeselected(&self)
    {
        self.widget.clear();
    }

    fn onStagedSelected(&self, fileChange: &FileChange)
    {
        let diffLinePrinter = DiffLinePrinter::new(&self.widget);
        let diff = self.stagedDiffMaker.makeDiff(&fileChange.path);
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffLinePrinter.printDiff(&line))
            .unwrap_or_else(|e| exit(&format!("Failed to print diff: {}", e)));
    }

    fn onStagedDeselected(&self)
    {
        self.widget.clear();
    }
}


struct UnstagedChangesViewObserver
{
    selectionChangedHandler: Rc<DiffViewHandler>
}

impl UnstagedChangesViewObserver
{
    fn new(selectionChangedHandler: Rc<DiffViewHandler>, unstagedChangesView: &UnstagedChangesView) -> Rc<Self>
    {
        let newSelf = Rc::new(Self{selectionChangedHandler});
        unstagedChangesView.connectOnSelected(Rc::downgrade(&(newSelf.clone() as Rc<dyn FileChangesViewObserver>)));
        unstagedChangesView.connectOnDeselected(Rc::downgrade(&(newSelf.clone() as Rc<dyn FileChangesViewObserver>)));
        newSelf
    }
}

impl FileChangesViewObserver for UnstagedChangesViewObserver
{
    fn onSelected(&self, fileChange: &FileChange)
    {
        self.selectionChangedHandler.onUnstagedSelected(fileChange);
    }

    fn onDeselected(&self)
    {
        self.selectionChangedHandler.onUnstagedDeselected();
    }
}


struct StagedChangesViewObserver
{
    selectionChangedHandler: Rc<DiffViewHandler>
}

impl StagedChangesViewObserver
{
    fn new(selectionChangedHandler: Rc<DiffViewHandler>, view: &StagedChangesView) -> Rc<Self>
    {
        let newSelf = Rc::new(Self{selectionChangedHandler});
        view.connectOnSelected(Rc::downgrade(&(newSelf.clone() as Rc<dyn FileChangesViewObserver>)));
        view.connectOnDeselected(Rc::downgrade(&(newSelf.clone() as Rc<dyn FileChangesViewObserver>)));
        newSelf
    }
}

impl FileChangesViewObserver for StagedChangesViewObserver
{
    fn onSelected(&self, fileChange: &FileChange)
    {
        self.selectionChangedHandler.onStagedSelected(fileChange);
    }

    fn onDeselected(&self)
    {
        self.selectionChangedHandler.onStagedDeselected();
    }
}