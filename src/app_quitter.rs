use crate::event::{Event, handleUnknown, IEventHandler, Source};

pub(crate) struct AppQuitter
{
}

impl IEventHandler for AppQuitter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::QuitRequested => quitApp(),
            _ => handleUnknown(source, event)
        }
    }
}

impl AppQuitter
{
    pub fn new() -> Self
    {
        Self{}
    }
}

fn quitApp()
{
    if gtk::main_level() > 0 {
        gtk::main_quit();
    }
}
