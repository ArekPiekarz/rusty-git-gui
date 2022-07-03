use crate::config::Config;
use crate::event::{Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::pane::setupPane;


pub(crate) fn setupMainPane(guiElementProvider: &GuiElementProvider, config: &Config, sender: Sender)
{
    setupPane(guiElementProvider, "Main pane", config.mainPane.position, Source::MainPane, sender);
}
