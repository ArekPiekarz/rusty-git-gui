use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use anyhow::Error;
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
    widget: gtk::Entry,
    cssProvider: gtk::CssProvider
}

impl IEventHandler for CommitLogAuthorFilterEntry
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::InvalidTextInputted(error) => self.onInvalidRegexInputted(error),
            Event::ValidTextInputted          => self.onValidRegexInputted(),
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
        connectEntry(&widget, sender.clone());
        setupCaseSensitivityButton(guiElementProvider, sender.clone());
        setupRegexButton(guiElementProvider, sender);
        Self{widget, cssProvider}
    }

    fn onInvalidRegexInputted(&self, error: &Error)
    {
        self.cssProvider.load_from_data(INVALID_INPUT_CSS).unwrap();
        self.widget.set_tooltip_text(Some(&error.to_string()));
    }

    fn onValidRegexInputted(&self)
    {
        self.cssProvider.load_from_data(VALID_INPUT_CSS).unwrap();
        self.widget.set_tooltip_text(None);
    }
}

fn setupCss<WidgetType>(widget: &WidgetType) -> gtk::CssProvider
    where WidgetType: IsA<gtk::Widget>
{
    let cssProvider = gtk::CssProvider::new();
    widget.style_context().add_provider(&cssProvider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    cssProvider
}

fn connectEntry(entry: &gtk::Entry, sender: Sender)
{
    entry.connect_changed(move |entry| {
        sender.send((Source::CommitLogAuthorFilterEntry, Event::TextEntered(entry.text().into()))).unwrap();
    });
}

fn setupCaseSensitivityButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleButton>("Commit log author filter case button");
    button.connect_toggled(move |button|
        sender.send((Source::CommitLogAuthorFilterCaseButton, Event::Toggled(button.is_active()))).unwrap());
}

fn setupRegexButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleButton>("Commit log author filter regex button");
    button.connect_toggled(move |button|
        sender.send((Source::CommitLogAuthorFilterRegexButton, Event::Toggled(button.is_active()))).unwrap());
}
