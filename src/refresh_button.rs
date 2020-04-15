use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::ButtonExt as _;


pub struct RefreshButton
{
    widget: gtk::Button,
    sender: Sender
}

impl IEventHandler for RefreshButton
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::Clicked => self.refresh(),
            _ => handleUnknown(source, event)
        }
    }
}

impl RefreshButton
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let widget = guiElementProvider.get::<gtk::Button>("Refresh button");
        let newSelf = Self{widget, sender};
        newSelf.connectWidget();
        newSelf
    }


    // private

    fn connectWidget(&self)
    {
        let sender = self.sender.clone();
        self.widget.connect_clicked(move |_button| {
            sender.send((Source::RefreshButton, Event::Clicked)).unwrap();
        });
    }

    fn refresh(&self)
    {
        self.sender.send((Source::RefreshButton, Event::RefreshRequested)).unwrap();
    }
}