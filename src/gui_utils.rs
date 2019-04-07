use gtk::TextViewExt as _;

pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "Failed to get text view buffer, the value was empty.")]
    NoTextViewBuffer
}

pub fn getBuffer(textView: &gtk::TextView) -> Result<gtk::TextBuffer>
{
    textView.get_buffer().ok_or_else(|| Error::from(ErrorKind::NoTextViewBuffer))
}