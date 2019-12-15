use crate::color::Color;

const FORMATTING_SUCCEEDED: bool = true;
const IGNORE_FILE_HEADER: () = ();
const GREEN: Color = Color("green");
const RED: Color = Color("red");
const ADDED_PREFIX: &str = "+";
const REMOVED_PREFIX: &str = "-";
const KEPT_PREFIX: &str = " ";
const NO_PREFIX: &str = "";


pub struct DiffFormatter
{
    text: String
}

impl DiffFormatter
{
    pub fn new() -> Self
    {
        Self{text: "".into()}
    }

    pub fn format(&mut self, line: &git2::DiffLine) -> bool
    {
        let lineContent = String::from_utf8_lossy(line.content());
        match line.origin() {
            '+' => self.appendAddedLine(&lineContent),
            '-' => self.appendRemovedLine(&lineContent),
            ' ' => self.appendKeptLine(&lineContent),
            'F' => IGNORE_FILE_HEADER,
             _  => self.appendContextLine(&lineContent)
        };
        FORMATTING_SUCCEEDED
    }

    pub fn getText(&self) -> &str
    {
        &self.text
    }


    // private

    fn appendAddedLine(&mut self, line : &str)
    {
        self.appendColoredLine(GREEN, ADDED_PREFIX, line);
    }

    fn appendRemovedLine(&mut self, line : &str)
    {
        self.appendColoredLine(RED, REMOVED_PREFIX, line);
    }

    fn appendKeptLine(&mut self, line : &str)
    {
        self.appendNormalLine(KEPT_PREFIX, line);
    }

    fn appendContextLine(&mut self, line : &str)
    {
        self.appendNormalLine(NO_PREFIX, line);
    }

    fn appendNormalLine(&mut self, prefix: &str, line : &str)
    {
        self.text.push_str(&format!("<span>{}{}</span>", prefix, glib::markup_escape_text(line)));
    }

    fn appendColoredLine(&mut self, color: Color, prefix: &str, line: &str)
    {
        self.text.push_str(&format!("<span color='{}'>{}{}</span>", color, prefix, glib::markup_escape_text(line)));
    }
}