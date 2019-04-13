use gtk::TextBufferExt as _;
use gtk::TextViewExt as _;


pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "gtk::TextView::get_buffer() returned None.")]
    NoneTextViewBuffer
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