use crate::color::Color;
use crate::text_view::TextView;

const IGNORE_FILE_HEADER: () = ();
const PRINTING_SUCCEEDED : bool = true;


pub struct DiffLinePrinter<'a>
{
    textView: &'a TextView
}

impl<'a> DiffLinePrinter<'a>
{
    pub fn new(textView: &'a TextView) -> Self
    {
        textView.clear();
        Self{textView}
    }

    pub fn printDiff(&self, line: &git2::DiffLine) -> bool
    {
        let lineContent = String::from_utf8_lossy(line.content());
        match line.origin() {
            '+' => self.appendAddedLineDiff(&lineContent),
            '-' => self.appendRemovedLineDiff(&lineContent),
            ' ' => self.appendKeptLineDiff(&lineContent),
            'F' => IGNORE_FILE_HEADER,
             _  => self.appendDiffLine(&lineContent)
        };
        PRINTING_SUCCEEDED
    }

    fn appendAddedLineDiff(&self, line : &str)
    {
        self.textView.appendColored(Color("green"), &format!("+{}", line));
    }

    fn appendRemovedLineDiff(&self, line : &str)
    {
        self.textView.appendColored(Color("red"), &format!("-{}", line));
    }

    fn appendKeptLineDiff(&self, line : &str)
    {
        self.textView.append(&format!(" {}", line));
    }

    fn appendDiffLine(&self, line : &str)
    {
        self.textView.append(line);
    }
}