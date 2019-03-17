use crate::error_handling::exit;
use gtk::TextBufferExt as _;
use gtk::TextViewExt as _;

const PRINTING_SUCCEEDED : bool = true;

pub struct DiffLinePrinter
{
    buffer: gtk::TextBuffer
}

impl DiffLinePrinter
{
    pub fn new(textView: &gtk::TextView) -> Self
    {
        let buffer = textView.get_buffer().unwrap_or_else(|| exit("Failed to get text view buffer"));
        buffer.delete(&mut buffer.get_start_iter(), &mut buffer.get_end_iter());
        Self { buffer }
    }

    pub fn printDiff(&self, line: &git2::DiffLine) -> bool
    {
        let lineContent = String::from_utf8_lossy(line.content());
        match line.origin() {
            '+' => self.insertAddedLineDiff(&lineContent),
            '-' => self.insertRemovedLineDiff(&lineContent),
            ' ' => self.insertKeptLineDiff(&lineContent),
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
            &format!("<span color='{}'>{}{}</span>", color, linePrefix, v_htmlescape::escape(line)));
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
