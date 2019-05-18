use crate::gui_utils::getBuffer;
use failure::ResultExt as _;
use gtk::TextBufferExt as _;

pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to make diff line printer.")]
    NewDiffLinePrinter
}

const PRINTING_SUCCEEDED : bool = true;

pub struct DiffLinePrinter
{
    buffer: gtk::TextBuffer
}

impl DiffLinePrinter
{
    pub fn new(textView: &gtk::TextView) -> Result<Self>
    {
        let buffer = getBuffer(textView).context(ErrorKind::NewDiffLinePrinter)?;
        buffer.delete(&mut buffer.get_start_iter(), &mut buffer.get_end_iter());
        Ok(Self { buffer })
    }

    pub fn printDiff(&self, line: &git2::DiffLine) -> bool
    {
        let lineContent = String::from_utf8_lossy(line.content());
        match line.origin() {
            '+' => self.insertAddedLineDiff(&lineContent),
            '-' => self.insertRemovedLineDiff(&lineContent),
            ' ' => self.insertKeptLineDiff(&lineContent),
            'F' => (), // ignore file header
             _  => self.insertDiffLine(&lineContent)
        };
        PRINTING_SUCCEEDED
    }

    fn insertAddedLineDiff(&self, line : &str)
    {
        self.insertColoredLineDiff("green", "+", line);
    }

    fn insertRemovedLineDiff(&self, line : &str)
    {
        self.insertColoredLineDiff("red", "-", line);
    }

    fn insertColoredLineDiff(&self, color: &str, linePrefix: &str, line : &str)
    {
        self.buffer.insert_markup(
            &mut self.buffer.get_end_iter(),
            &format!("<span color='{}'>{}{}</span>", color, linePrefix, glib::markup_escape_text(line)));
    }

    fn insertKeptLineDiff(&self, line : &str)
    {
        self.buffer.insert(&mut self.buffer.get_end_iter(), &format!(" {}", line));
    }

    fn insertDiffLine(&self, line : &str)
    {
        self.buffer.insert(&mut self.buffer.get_end_iter(), line);
    }
}
