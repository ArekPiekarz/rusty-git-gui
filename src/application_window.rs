use crate::gui_element_provider::GuiElementProvider;
use crate::settings::Settings;

use gtk::WidgetExt as _;

const PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER : gtk::Inhibit = gtk::Inhibit(true);


pub struct ApplicationWindow
{
    window: gtk::ApplicationWindow
}

impl ApplicationWindow
{
    pub fn new(guiElementProvider: &GuiElementProvider, settings: Settings) -> Self
    {
        let newSelf = Self{window: guiElementProvider.get::<gtk::ApplicationWindow>("Main window")};
        newSelf.connectToWindowDeletion(settings);
        newSelf
    }

    pub fn show(&self)
    {
        self.window.show_all();
    }


    // private

    fn connectToWindowDeletion(&self, settings: Settings)
    {
        self.window.connect_delete_event(move |_window, _event| {
            settings.save();
            gtk::main_quit();
            PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER });
    }
}