use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::EditableSignals;
use gtk::prelude::IsA;
use gtk::traits::CssProviderExt;
use gtk::traits::EntryExt;
use gtk::traits::StyleContextExt;
use gtk::traits::ToggleButtonExt;
use gtk::traits::WidgetExt;

const INVALID_INPUT_CSS: &[u8] = "entry {color: red;}".as_bytes();
const VALID_INPUT_CSS: &[u8] = "".as_bytes();


pub(crate) struct CommitLogAuthorFilterEntry
{
    cssProvider: gtk::CssProvider
}

impl IEventHandler for CommitLogAuthorFilterEntry
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::InvalidTextInputted => self.onInvalidRegexInputted(),
            Event::ValidTextInputted => self.onValidRegexInputted(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogAuthorFilterEntry
{
    pub(crate) fn new(guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let widget = guiElementProvider.get::<gtk::Entry>("Commit log author filter entry");
        let cssProvider = setupCss(&widget);
        connectWidget(&widget, sender.clone());
        setupRegexButton(guiElementProvider, sender);
        Self{cssProvider}
    }

    fn onInvalidRegexInputted(&self)
    {
        self.cssProvider.load_from_data(INVALID_INPUT_CSS).unwrap();
    }

    fn onValidRegexInputted(&self)
    {
        self.cssProvider.load_from_data(VALID_INPUT_CSS).unwrap();
    }
}

fn setupCss<WidgetType>(widget: &WidgetType) -> gtk::CssProvider
    where WidgetType: IsA<gtk::Widget>
{
    let cssProvider = gtk::CssProvider::new();
    widget.style_context().add_provider(&cssProvider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    cssProvider
}

fn connectWidget(widget: &gtk::Entry, sender: Sender)
{
    widget.connect_changed(move |widget| {
        sender.send((Source::CommitLogAuthorFilterEntry, Event::TextEntered(widget.text().into()))).unwrap();
    });

}

fn setupRegexButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleButton>("Commit log author filter regex button");
    button.connect_toggled(move |button|
        sender.send((Source::CommitLogAuthorFilterRegexButton, Event::Toggled(button.is_active()))).unwrap());
}
