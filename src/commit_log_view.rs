use crate::commit_log::CommitLog;
use crate::commit_log_column::CommitLogColumn;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::tree_view::TreeView;
use crate::tree_model_utils::toRow;

use gtk::prelude::TreeSelectionExt as _;

pub struct CommitLogView
{
    commitLog: CommitLog,
    sender: Sender
}

impl IEventHandler for CommitLogView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::SelectionChanged(selection) => self.handleSelectionChanged(selection),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogView
{
    pub fn new(commitLog: CommitLog, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        TreeView::new(
            guiElementProvider,
            "Commit log view",
            sender.clone(),
            Source::CommitLogViewWidget,
            &CommitLogColumn::asArrayOfI32());
        Self{commitLog, sender}
    }


    // private

    fn handleSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        let (rows, _model) = selection.selected_rows();
        match rows.get(0) {
            Some(rowPath) => {
                let commitId = self.commitLog.getCommit(toRow(rowPath)).unwrap().id;
                self.sender.send((Source::CommitLogView, Event::CommitSelected(commitId))).unwrap();
            }
            None => self.sender.send((Source::CommitLogView, Event::CommitUnselected)).unwrap()
        }
    }
}
