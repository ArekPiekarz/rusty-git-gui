use crate::config::Config;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::event_constants::FORWARD_EVENT;
use crate::gui_element_provider::GuiElementProvider;

use gtk::prelude::GtkWindowExt as _;
use gtk::prelude::WidgetExt as _;


pub(crate) struct ApplicationWindow
{
    window: gtk::ApplicationWindow
}

impl IEventHandler for ApplicationWindow
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::QuitRequested => self.onQuitRequested(),
            _ => handleUnknown(source, event)
        }
    }
}

impl ApplicationWindow
{
    pub fn new(guiElementProvider: &GuiElementProvider, config: &Config, sender: Sender) -> Self
    {
        let newSelf = Self{window: guiElementProvider.get::<gtk::ApplicationWindow>("Main window")};
        newSelf.loadConfig(config);
        newSelf.connectToMaximizationChanged(sender.clone());
        newSelf.connectToWindowDeletion(sender);
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

    fn loadConfig(&self, config: &Config)
    {
        if config.applicationWindow.isMaximized {
            self.window.maximize();
        } else {
            self.window.unmaximize();
        }
    }

    fn connectToMaximizationChanged(&self, sender: Sender)
    {
        self.window.connect_is_maximized_notify(move |window|
            sender.send((Source::ApplicationWindow, Event::MaximizationChanged(window.is_maximized()))).unwrap());
    }

    fn connectToWindowDeletion(&self, sender: Sender)
    {
        self.window.connect_delete_event(move |_window, _event| {
            sender.send((Source::ApplicationWindow, Event::QuitRequested)).unwrap();
            FORWARD_EVENT
        });
    }

    fn onQuitRequested(&self)
    {
        if gtk::main_level() > 0 {
            gtk::main_quit();
        }
    }
}
