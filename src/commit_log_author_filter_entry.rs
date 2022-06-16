use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::EditableSignals;
use gtk::traits::EntryExt;
use gtk::traits::ToggleButtonExt;


pub fn setupCommitLogAuthorFilterEntry(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let widget = guiElementProvider.get::<gtk::Entry>("Commit log author filter entry");
    widget.connect_changed(move |widget| {
        sender.send((Source::CommitLogAuthorFilterEntry, Event::TextEntered(widget.text().into()))).unwrap();
    });
}

pub(crate) fn setupCommitLogAuthorFilterRegexButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleButton>("Commit log author filter regex button");
    button.connect_toggled(move |button|
        sender.send((Source::CommitLogAuthorFilterRegexButton, Event::Toggled(button.is_active()))).unwrap());
}
