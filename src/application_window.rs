use crate::event_constants::FORWARD_EVENT;
use crate::gui_element_provider::GuiElementProvider;
use crate::settings::Settings;

use gtk::prelude::GtkWindowExt as _;
use gtk::prelude::WidgetExt as _;
use std::rc::Rc;

const SECTION: &str = "Application window";
const IS_MAXIMIZED_KEY: &str = "isMaximized";
const MAXIMIZE_BY_DEFAULT: bool = true;


pub struct ApplicationWindow
{
    window: gtk::ApplicationWindow
}

impl ApplicationWindow
{
    pub fn new(guiElementProvider: &GuiElementProvider, mut settings: Settings) -> Rc<Self>
    {
        let newSelf = Rc::new(Self{window: guiElementProvider.get::<gtk::ApplicationWindow>("Main window")});
        newSelf.loadSettings(&settings);
        newSelf.setupSavingSettings(&mut settings);
        newSelf.connectToWindowDeletion(settings);
        newSelf
    }

    pub fn show(&self)
    {
        self.window.show();
    }

    pub fn setOpacity(&self, value: f64)
    {
        self.window.set_opacity(value);
    }

    // private

    fn loadSettings(&self, settings: &Settings)
    {
        let isMaximized = settings.get(SECTION, IS_MAXIMIZED_KEY, MAXIMIZE_BY_DEFAULT);
        if isMaximized {
            self.window.maximize();
        } else {
            self.window.unmaximize();
        }
    }

    fn saveSettings(&self, settings: &Settings)
    {
        settings.set(SECTION, IS_MAXIMIZED_KEY, self.window.is_maximized());
    }

    fn setupSavingSettings(self: &Rc<Self>, settings: &mut Settings)
    {
        let weakSelf = Rc::downgrade(self);
        settings.addSaver(Box::new(
            move |settings| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.saveSettings(settings);
                }
            }
        ));
    }

    fn connectToWindowDeletion(&self, settings: Settings)
    {
        self.window.connect_delete_event(move |_window, _event| {
            settings.save();
            if gtk::main_level() > 0 {
                gtk::main_quit();
            }
            FORWARD_EVENT
        });
    }
}
