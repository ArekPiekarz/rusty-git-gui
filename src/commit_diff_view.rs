use crate::commit_diff::{makeCommitSummary, makeFormattedDiff};
use crate::diff_colorizer::DiffColorizer;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::text_view::{Notifications, TextView};

use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitDiffView
{
    textView: TextView,
    diffColorizer: DiffColorizer,
    repository: Rc<RefCell<Repository>>
}

impl IEventHandler for CommitDiffView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::CommitSelected(id) => self.onCommitSelected(id),
            Event::CommitUnselected   => self.onCommitUnselected(),
            Event::ZoomRequested(_)   => self.onZoomRequested(source, event),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitDiffView
{
    pub fn new(repository: Rc<RefCell<Repository>>, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let textView = TextView::new(
            guiElementProvider, "Commit diff view", sender, Source::CommitDiffViewWidget, Notifications::Disabled);
        let diffColorizer = DiffColorizer::new(&textView);
        Self{
            textView,
            diffColorizer,
            repository
        }
    }


    // private

    fn onCommitSelected(&mut self, commitId: &git2::Oid)
    {
        let repository = self.repository.borrow();
        let commit = repository.findCommit(*commitId).unwrap();
        let commitDiff = repository.makeDiffOfCommitAndParent(&commit);
        let textDiff = makeCommitSummary(&commit) + &makeFormattedDiff(&commitDiff);
        self.diffColorizer.colorize(&self.textView, &textDiff);
    }

    fn onCommitUnselected(&self)
    {
        self.textView.clear();
    }

    fn onZoomRequested(&mut self, source: Source, event: &Event)
    {
        self.textView.handle(source, event);
    }
}
