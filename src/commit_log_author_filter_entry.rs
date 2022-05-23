use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::EditableSignals;
use gtk::traits::EntryExt;


pub fn setupCommitLogAuthorFilterEntry(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let widget = guiElementProvider.get::<gtk::Entry>("Commit log author filter entry");
    widget.connect_changed(move |widget| {
        sender.send((Source::CommitLogAuthorFilterEntry, Event::TextEntered(widget.text().into()))).unwrap();
    });
}
