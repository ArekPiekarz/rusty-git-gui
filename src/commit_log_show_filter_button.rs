use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::ToggleToolButtonExt;


pub(crate) fn setupCommitLogShowFilterButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleToolButton>("Commit log show filter button");
    button.connect_toggled(
        move |button| sender.send((Source::CommitLogShowFilterButton, Event::Toggled(button.is_active()))).unwrap());
}
