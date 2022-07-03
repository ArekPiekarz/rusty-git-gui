use crate::config::Config;
use crate::event::{Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::pane::setupPane;


pub(crate) fn setupDiffAndCommitPane(guiElementProvider: &GuiElementProvider, config: &Config, sender: Sender)
{
    setupPane(
        guiElementProvider,
        "Diff and commit pane",
        config.diffAndCommitPane.position,
        Source::DiffAndCommitPane, sender);
}
