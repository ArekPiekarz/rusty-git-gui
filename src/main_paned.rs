use crate::gui_element_provider::GuiElementProvider;
use crate::settings::Settings;
use crate::paned::Paned;

const DEFAULT_POSITION: i32 = 200;


pub fn setupMainPaned(guiElementProvider: &GuiElementProvider, settings: &mut Settings)
{
    Paned::setup(guiElementProvider, settings, "Main paned", DEFAULT_POSITION);
}