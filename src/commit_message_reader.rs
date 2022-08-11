use crate::gui_element_provider::GuiElementProvider;
use crate::text_view::EXCLUDE_HIDDEN_CHARACTERS;

use gtk::prelude::TextBufferExt as _;
use gtk::prelude::TextViewExt as _;


pub(crate) struct CommitMessageReader
{
    buffer: gtk::TextBuffer
}

impl CommitMessageReader
{
    pub fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        Self{buffer: guiElementProvider.get::<gtk::TextView>("Commit message view").buffer().unwrap()}
    }

    pub fn hasText(&self) -> bool
    {
        !self.getText().is_empty()
    }

    pub fn getText(&self) -> String
    {
        self.buffer.text(&self.buffer.start_iter(), &self.buffer.end_iter(), EXCLUDE_HIDDEN_CHARACTERS)
            .unwrap().into()
    }
}
