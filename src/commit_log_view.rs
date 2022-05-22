use crate::commit_log::CommitLog;
use crate::commit_log_column::CommitLogColumn;
use crate::commit_log_selections_comparer::CommitLogSelectionsComparer;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::original_row::OriginalRow;
use crate::tree_view::TreeView;

use gtk::traits::TreeModelExt;
use gtk::traits::TreeSelectionExt;


pub struct CommitLogView
{
    widget: TreeView,
    commitLog: CommitLog,
    sender: Sender
}

impl IEventHandler for CommitLogView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::RefilterRequested           => self.onRefilterRequested(),
            Event::RefilterEnded               => self.onRefilterEnded(),
            Event::SelectionChanged(selection) => self.onSelectionChanged(selection),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogView
{
    pub fn new(commitLog: CommitLog, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let widget = TreeView::new(
            guiElementProvider,
            "Commit log view",
            Some(Box::new(CommitLogSelectionsComparer::new())),
            sender.clone(),
            Source::CommitLogViewWidget,
            &CommitLogColumn::asArrayOfI32());
        Self{widget, commitLog, sender}
    }


    // private

    fn onRefilterRequested(&mut self)
    {
        self.widget.getSelectionMut().blockSignals();
    }

    fn onRefilterEnded(&mut self)
    {
        self.widget.getSelectionMut().unblockSignals();
    }

    fn onSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        match selection.selected() {
            Some((model, iter)) => {
                let row = model.value(&iter, CommitLogColumn::OriginalRow.into()).get::<OriginalRow>().unwrap()
                    .try_into().unwrap();
                let commitId = self.commitLog.getCommit(row).unwrap().id;
                self.sender.send((Source::CommitLogView, Event::CommitSelected(commitId))).unwrap();
            },
            None => self.sender.send((Source::CommitLogView, Event::CommitUnselected)).unwrap()
        }
    }
}
