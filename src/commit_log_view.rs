use crate::commit_log_column::CommitLogColumn;
use crate::event::{Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::tree_view::TreeView;


pub struct CommitLogView
{}

impl CommitLogView
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        TreeView::new(
            guiElementProvider,
            "Commit log view",
            sender,
            Source::CommitLogViewWidget,
            &CommitLogColumn::asArrayOfI32());
        Self{}
    }
}
