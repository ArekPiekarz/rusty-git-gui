use crate::event::{Event, handleUnknown, IEventHandler, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::WidgetExt;


pub(crate) struct CommitLogFiltersView
{
    widget: gtk::Grid
}

impl IEventHandler for CommitLogFiltersView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::Toggled => self.onToggled(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogFiltersView
{
    pub(crate) fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        let widget = guiElementProvider.get::<gtk::Grid>("Commit log filters grid");
        Self{widget}
    }

    fn onToggled(&self)
    {
        self.widget.set_visible(!self.widget.is_visible());
    }
}
