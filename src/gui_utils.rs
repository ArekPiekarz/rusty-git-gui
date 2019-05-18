use crate::gui_definitions::EXCLUDE_HIDDEN_CHARACTERS;
use gtk::{
    TextBufferExt as _,
    TextViewExt as _,
    TreeModelExt as _,
};


pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "gtk::TextView::get_buffer() returned None.")]
    NoneTextViewBuffer,
    #[fail(display = "gtk::TextBuffer::get_text() returned None.")]
    NoneTextBufferContent
}

impl failchain::ChainErrorKind for ErrorKind
{
    type Error = Error;
}


pub fn getBuffer(textView: &gtk::TextView) -> Result<gtk::TextBuffer>
{
    textView.get_buffer().ok_or(ErrorKind::NoneTextViewBuffer.into())
}

pub fn clearBuffer(buffer: &gtk::TextBuffer)
{
    buffer.delete(&mut buffer.get_start_iter(), &mut buffer.get_end_iter());
}

pub fn getText(buffer: &gtk::TextBuffer) -> Result<String>
{
    match buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), EXCLUDE_HIDDEN_CHARACTERS) {
        Some(text) => Ok(text.into()),
        None => Err(ErrorKind::NoneTextBufferContent.into())
    }
}

pub fn isTextBufferEmpty(buffer: &gtk::TextBuffer) -> bool
{
    buffer.get_char_count() == 0
}

pub fn isModelEmpty(model: &gtk::TreeModel) -> bool
{
    model.get_iter_first() == None
}