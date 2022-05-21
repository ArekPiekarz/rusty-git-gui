use crate::event::{Event, handleUnknown, IEventHandler, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::StackExt;


pub(crate) struct ToolBarStack
{
    widget: gtk::Stack
}

impl IEventHandler for ToolBarStack
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::StackChildChanged(name) => self.onStackChildChanged(name),
            _ => handleUnknown(source, event)
        }
    }
}

impl ToolBarStack
{
    pub(crate) fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        let widget = guiElementProvider.get::<gtk::Stack>("Tool bar stack");
        Self{widget}
    }

    fn onStackChildChanged(&self, name: &str)
    {
        match name {
            "Current changes" => self.widget.set_visible_child_name("Current changes tool bar"),
            "Commit log" => self.widget.set_visible_child_name("Commit log tool bar"),
            _ => panic!("Unknown stack child name: {}", name)
        }
    }
}
