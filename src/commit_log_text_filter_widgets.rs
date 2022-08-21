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


pub(crate) struct CommitLogTextFilterWidgets
{
    entryWidget: gtk::Entry,
    cssProvider: gtk::CssProvider
}

impl IEventHandler for CommitLogTextFilterWidgets
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

impl CommitLogTextFilterWidgets
{
    pub(crate) fn new(
        entryData: &WidgetData,
        caseButtonData: &WidgetData,
        regexButtonData: &WidgetData,
        guiElementProvider: &GuiElementProvider,
        sender: Sender)
        -> Self
    {
        let entryWidget = guiElementProvider.get::<gtk::Entry>(entryData.name);
        let cssProvider = setupCss(&entryWidget);
        connectEntry(&entryWidget, entryData.source, sender.clone());
        setupButton(caseButtonData.name, caseButtonData.source, guiElementProvider, sender.clone());
        setupButton(regexButtonData.name, regexButtonData.source, guiElementProvider, sender);
        Self{entryWidget, cssProvider}
    }

    fn onInvalidRegexInputted(&self, error: &regex::Error)
    {
        self.cssProvider.load_from_data(INVALID_INPUT_CSS).unwrap();
        self.entryWidget.set_tooltip_text(Some(&error.to_string()));
    }

    fn onValidRegexInputted(&self)
    {
        self.cssProvider.load_from_data(VALID_INPUT_CSS).unwrap();
        self.entryWidget.set_tooltip_text(None);
    }
}

fn setupCss<WidgetType>(widget: &WidgetType) -> gtk::CssProvider
    where WidgetType: IsA<gtk::Widget>
{
    let cssProvider = gtk::CssProvider::new();
    widget.style_context().add_provider(&cssProvider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    cssProvider
}

fn connectEntry(entry: &gtk::Entry, source: Source, sender: Sender)
{
    entry.connect_changed(move |entry| sender.send((source, Event::TextEntered(entry.text().into()))).unwrap());
}

fn setupButton(name: &str, source: Source, guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::ToggleButton>(name);
    button.connect_toggled(move |button| sender.send((source, Event::Toggled(button.is_active()))).unwrap());
}

pub(crate) struct WidgetData
{
    pub name: &'static str,
    pub source: Source,
}
