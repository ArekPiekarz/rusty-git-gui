use crate::config::AuthorFilter;
use crate::event::{Event, handleUnknown, IEventHandler, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::{EntryExt, ToggleButtonExt, WidgetExt};


pub(crate) struct CommitLogFiltersView
{
    grid: gtk::Grid,
    entry: gtk::Entry,
    caseButton: gtk::ToggleButton,
    regexButton: gtk::ToggleButton,
}

impl IEventHandler for CommitLogFiltersView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::ActiveFilterDataSwitched(filter) => self.onActiveFilterSwitched(filter),
            Event::Toggled(isEnabled)               => self.onToggled(*isEnabled),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogFiltersView
{
    pub(crate) fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        let grid = guiElementProvider.get::<gtk::Grid>("Commit log filters grid");
        let entry = guiElementProvider.get::<gtk::Entry>("Commit log author filter entry");
        let caseButton = guiElementProvider.get::<gtk::ToggleButton>("Commit log author filter case button");
        let regexButton = guiElementProvider.get::<gtk::ToggleButton>("Commit log author filter regex button");
        Self{grid, entry, caseButton, regexButton}
    }

    fn onActiveFilterSwitched(&self, filter: &AuthorFilter)
    {
        self.entry.set_text(&filter.pattern);
        self.caseButton.set_active(filter.caseSensitive);
        self.regexButton.set_active(filter.usesRegex);
    }

    fn onToggled(&self, isEnabled: bool)
    {
        self.grid.set_visible(isEnabled);
    }
}
