use crate::gui_element_provider::GuiElementProvider;
use crate::settings::Settings;

use gtk::GtkWindowExt as _;
use gtk::WidgetExt as _;
use std::rc::Rc;

const SECTION: &str = "Application window";
const IS_MAXIMIZED_KEY: &str = "isMaximized";
const MAXIMIZE_BY_DEFAULT: bool = true;
const PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER : gtk::Inhibit = gtk::Inhibit(true);


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
        self.window.show_all();
    }


    // private

    fn loadSettings(&self, settings: &Settings)
    {
        let isMaximized = settings.get(SECTION, IS_MAXIMIZED_KEY, MAXIMIZE_BY_DEFAULT);
        match isMaximized {
            true => self.window.maximize(),
            false => self.window.unmaximize()
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
            gtk::main_quit();
            PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER });
    }
}