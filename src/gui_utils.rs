use crate::std_utils::getOption;
use failchain::ResultExt as _;
use gtk::TextViewExt as _;

pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "Failed to get text view buffer.")]
    NoTextViewBuffer
}

impl failchain::ChainErrorKind for ErrorKind
{
    type Error = Error;
}

pub fn getBuffer(textView: &gtk::TextView) -> Result<gtk::TextBuffer>
{
    getOption(textView.get_buffer()).chain_err(|| ErrorKind::NoTextViewBuffer)
}