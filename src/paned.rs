use crate::gui_element_provider::GuiElementProvider;
use crate::settings::Settings;

use gtk::PanedExt as _;

const POSITION_KEY: &str = "position";


pub struct Paned
{
    widget: gtk::Paned,
    name: &'static str,
}

impl Paned
{
    pub fn setup(
        guiElementProvider: &GuiElementProvider,
        settings: &mut Settings,
        name: &'static str,
        defaultPosition: i32)
    {
        let widget = guiElementProvider.get::<gtk::Paned>(name);
        widget.set_position(settings.get(name, POSITION_KEY, defaultPosition));
        let newSelf = Self{widget, name};
        newSelf.setupSavingSettings(settings);
    }


    // private

    fn setupSavingSettings(self, settings: &mut Settings)
    {
        settings.addSaver(Box::new(
            move |settings| {
                self.save(settings);
            }
        ));
    }

    fn save(&self, settings: &Settings)
    {
        settings.set(self.name, POSITION_KEY, self.widget.get_position());
    }
}