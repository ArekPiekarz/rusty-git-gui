use crate::gui_element_provider::GuiElementProvider;
use crate::text_view::EXCLUDE_HIDDEN_CHARACTERS;

use gtk::TextBufferExt as _;
use gtk::TextViewExt as _;


pub struct CommitMessageReader
{
    buffer: gtk::TextBuffer
}

impl CommitMessageReader
{
    pub fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        Self{buffer: guiElementProvider.get::<gtk::TextView>("Commit message view").get_buffer().unwrap()}
    }

    pub fn hasText(&self) -> bool
    {
        !self.getText().is_empty()
    }

    pub fn getText(&self) -> String
    {
        self.buffer.get_text(&self.buffer.get_start_iter(), &self.buffer.get_end_iter(), EXCLUDE_HIDDEN_CHARACTERS)
            .unwrap().into()
    }
}