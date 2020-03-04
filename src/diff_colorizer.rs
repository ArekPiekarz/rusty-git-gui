use crate::line_number::LineNumber;
use crate::text_view::TextView;

use difference::Difference;
use gtk::TextTagExt as _;


pub struct DiffColorizer
{
    greenTag: gtk::TextTag,
    redTag: gtk::TextTag,
    tagStartLine: LineNumber,
    state: State
}

enum State
{
    Normal,
    Added,
    Removed
}

impl DiffColorizer
{
    pub fn new(textView: &TextView) -> Self
    {
        let greenTag = makeTag("green");
        let redTag = makeTag("red");
        textView.registerTags(&[&greenTag, &redTag]);
        Self{
            greenTag,
            redTag,
            tagStartLine: 0.into(),
            state: State::Normal}
    }

    pub fn colorize(&mut self, textView: &TextView, diff: &str)
    {
        textView.setText(diff);
        self.applyTags(textView, diff);
    }

    pub fn update(&mut self, textView: &TextView, differences: Vec<Difference>)
    {
        if !diffRequiresUpdating(&differences) {
            return;
        }

        textView.removeTags();
        updateDiff(textView, differences);
        self.applyTags(textView, &textView.getText());
    }


    // private

    fn applyTags(&mut self, textView: &TextView, text: &str)
    {
        self.state = State::Normal;
        self.tagStartLine = 0.into();
        self.applyTagsBasedOnLineTypes(textView, text);
        self.closeLastOpenTag(textView);
    }

    fn applyTagsBasedOnLineTypes(&mut self, textView: &TextView, text: &str)
    {
        for (lineNumber, line) in text.lines().enumerate() {
            if let Some(character) = line.chars().next() {
                match character {
                    '+' => self.applyTagToAddedLine(textView, lineNumber.into()),
                    '-' => self.applyTagToRemovedLine(textView, lineNumber.into()),
                     _  => self.applyTagToNormalLine(textView, lineNumber.into())
                }
            }
        }
    }

    fn applyTagToAddedLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            },
            State::Added => (),
            State::Removed => {
                textView.applyTag(&self.redTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            }
        }
    }

    fn applyTagToRemovedLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            },
            State::Added => {
                textView.applyTag(&self.greenTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            }
            State::Removed => (),
        }
    }

    fn applyTagToNormalLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => (),
            State::Added => {
                textView.applyTag(&self.greenTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            },
            State::Removed => {
                textView.applyTag(&self.redTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
        }
    }

    fn closeLastOpenTag(&self, textView: &TextView)
    {
        match self.state {
            State::Normal => (),
            State::Added => textView.applyTagUntilEnd(&self.greenTag, self.tagStartLine),
            State::Removed => textView.applyTagUntilEnd(&self.redTag, self.tagStartLine)
        }
    }
}

fn updateDiff(textView: &TextView, differences: Vec<Difference>)
{
    let mut currentLine = 0.into();
    for difference in differences {
        match difference {
            Difference::Same(text) => {
                currentLine += text.lines().count();
            },
            Difference::Add(text) => {
                let text = ensureTextEndsWithNewLine(text);
                textView.insertTextAt(&text, currentLine);
                currentLine += text.lines().count();
            }
            Difference::Rem(text) => {
                textView.removeTextAt(currentLine, text.lines().count().into());
            }
        }
    }
}

fn makeTag(name: &str) -> gtk::TextTag
{
    let tag = gtk::TextTag::new(Some(name));
    tag.set_property_foreground(Some(name));
    tag
}

fn diffRequiresUpdating(differences: &[Difference]) -> bool
{
    match differences {
        [] | [Difference::Same(_)] => false,
        _ => true
    }
}

fn ensureTextEndsWithNewLine(text: String) -> String
{
    if text.ends_with('\n') {
        text
    } else {
        text + "\n"
    }
}