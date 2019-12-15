use crate::error_handling::exit;

use gtk::prelude::BuilderExtManual as _;


pub struct GuiElementProvider
{
    provider: gtk::Builder
}

impl GuiElementProvider
{
    pub fn new(guiDescription: &str) -> Self
    {
        Self{provider: gtk::Builder::new_from_string(guiDescription)}
    }

    pub fn get<T: glib::IsA<glib::Object>>(&self, name: &str) -> T
    {
        self.provider.get_object::<T>(name)
            .unwrap_or_else(|| exit(&format!(r#"Failed to get object named "{}" from gtk::Builder."#, name)))
    }
}