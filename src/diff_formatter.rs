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
    text: String,
    state: State
}

enum State
{
    Normal,
    Added,
    Removed
}

impl DiffFormatter
{
    pub fn new() -> Self
    {
        Self{text: "<span>".into(), state: State::Normal}
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

    pub fn finish(&mut self)
    {
        match self.state {
            State::Normal => self.text.push_str("</span>"),
            State::Added | State::Removed => self.text.push_str("</span></span>")
        }
    }

    pub fn getText(&self) -> &str
    {
        &self.text
    }


    // private

    fn appendAddedLine(&mut self, line : &str)
    {
        match self.state {
            State::Normal => {
                self.appendColoredLineStart(GREEN, ADDED_PREFIX, line);
                self.state = State::Added;
            },
            State::Added => {
                self.appendNormalLine(ADDED_PREFIX, line)
            },
            State::Removed => {
                self.appendColoredLineEndAndStart(GREEN, ADDED_PREFIX, line);
                self.state = State::Added;
            }
        }
    }

    fn appendRemovedLine(&mut self, line : &str)
    {
        match self.state {
            State::Normal => {
                self.appendColoredLineStart(RED, REMOVED_PREFIX, line);
                self.state = State::Removed;
            },
            State::Added => {
                self.appendColoredLineEndAndStart(RED, REMOVED_PREFIX, line);
                self.state = State::Removed;
            },
            State::Removed => {
                self.appendNormalLine(REMOVED_PREFIX, line)
            }
        }
    }

    fn appendKeptLine(&mut self, line : &str)
    {
        self.appendLine(KEPT_PREFIX, line);
    }

    fn appendContextLine(&mut self, line : &str)
    {
        self.appendLine(NO_PREFIX, line);
    }

    fn appendLine(&mut self, prefix: &str, line : &str)
    {
        match self.state {
            State::Normal => {
                self.appendNormalLine(prefix, line)
            },
            State::Added => {
                self.appendColoredLineEndAndNormalStart(prefix, line);
                self.state = State::Normal;
            },
            State::Removed => {
                self.appendColoredLineEndAndNormalStart(prefix, line);
                self.state = State::Normal;
            }
        }
    }

    fn appendNormalLine(&mut self, prefix: &str, line : &str)
    {
        self.text.push_str(&format!("{}{}", prefix, glib::markup_escape_text(line)));
    }

    fn appendColoredLineStart(&mut self, color: Color, prefix: &str, line: &str)
    {
        self.text.push_str(&format!("<span color='{}'>{}{}", color, prefix, glib::markup_escape_text(line)));
    }

    fn appendColoredLineEndAndStart(&mut self, color: Color, prefix: &str, line: &str)
    {
        self.text.push_str(&format!("</span><span color='{}'>{}{}", color, prefix, glib::markup_escape_text(line)));
    }

    fn appendColoredLineEndAndNormalStart(&mut self, prefix: &str, line: &str)
    {
        self.text.push_str(&format!("</span>{}{}", prefix, glib::markup_escape_text(line)));
    }
}