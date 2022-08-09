use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::ToolButtonExt;


pub(crate) fn setupCommitLogSaveFilterButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToolButton>("Commit log save filter button");
    button.connect_clicked(
        move |_button| sender.send((Source::CommitLogSaveFilterButton, Event::OpenDialogRequested)).unwrap());
}
