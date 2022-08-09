use crate::config::Config;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::prelude::ComboBoxExtManual;
use gtk::traits::{ComboBoxExt, ComboBoxTextExt};
use itertools::Itertools;
use to_trait::To;


pub(crate) struct CommitLogFiltersComboBox
{
    widget: gtk::ComboBoxText,
    filterNames: Vec<String>
}

impl IEventHandler for CommitLogFiltersComboBox
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::FilterAdded(name) => self.onFilterAdded(name),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogFiltersComboBox
{
    pub fn new(guiElementProvider: &GuiElementProvider, config: &Config, sender: Sender) -> Self
    {
        let widget = guiElementProvider.get::<gtk::ComboBoxText>("Commit log filters combo box text");
        let activeFilter = config.commitLogFilters.active;
        let filterNames = config.commitLogFilters.filters.iter().map(|filter| filter.name.clone()).collect_vec();
        for name in &filterNames {
            widget.append_text(name);
        }
        widget.connect_changed(
            move |comboBox|
                if let Some(index) = comboBox.active() {
                    sender.send((
                        Source::CommitLogFiltersComboBox,
                        Event::ActiveFilterChosen(index.try_into().unwrap())))
                        .unwrap();
                });
        widget.set_active(Some(activeFilter.try_into().unwrap()));
        Self{widget, filterNames}
    }

    fn onFilterAdded(&mut self, name: &str)
    {
        self.filterNames.push(name.into());
        self.widget.append_text(name);
        self.widget.set_active(Some(self.filterNames.len().try_to::<u32>().unwrap()-1))
    }
}
