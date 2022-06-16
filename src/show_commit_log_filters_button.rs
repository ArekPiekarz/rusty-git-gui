use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::ToggleToolButtonExt;


pub(crate) fn setupShowCommitLogFiltersButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleToolButton>("Show commit log filters button");
    button.connect_toggled(
        move |button| sender.send((Source::ShowCommitLogFiltersButton, Event::Toggled(button.is_active()))).unwrap());
}
