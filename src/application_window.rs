use crate::gui_element_provider::GuiElementProvider;

use gtk::WidgetExt as _;

const PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER : gtk::Inhibit = gtk::Inhibit(true);


pub struct ApplicationWindow
{
    window: gtk::ApplicationWindow
}

impl ApplicationWindow
{
    pub fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        let window = guiElementProvider.get::<gtk::ApplicationWindow>("Main window");
        quitOnDelete(&window);
        Self{window}
    }

    pub fn show(&self)
    {
        self.window.show_all();
    }
}

fn quitOnDelete(appWindow: &gtk::ApplicationWindow)
{
    appWindow.connect_delete_event(|_window, _event| {
        gtk::main_quit();
        PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER });
}