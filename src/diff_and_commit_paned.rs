use crate::gui_element_provider::GuiElementProvider;
use crate::settings::Settings;
use crate::paned::Paned;

const DEFAULT_POSITION: i32 = 450;


pub fn setupDiffAndCommitPaned(guiElementProvider: &GuiElementProvider, settings: &mut Settings)
{
    Paned::setup(guiElementProvider, settings, "Diff and commit paned", DEFAULT_POSITION);
}