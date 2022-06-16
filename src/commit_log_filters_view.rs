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
            Event::Toggled(isEnabled) => self.onToggled(*isEnabled),
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

    fn onToggled(&self, isEnabled: bool)
    {
        self.widget.set_visible(isEnabled);
    }
}
