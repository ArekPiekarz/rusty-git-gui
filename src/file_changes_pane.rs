use crate::config::Config;
use crate::event::{Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::pane::setupPane;


pub(crate) fn setupFileChangesPane(guiElementProvider: &GuiElementProvider, config: &Config, sender: Sender)
{
    setupPane(
        guiElementProvider,
        "File changes pane",
        config.fileChangesPane.position,
        Source::FileChangesPane,
        sender);
}
