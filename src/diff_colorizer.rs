use crate::line_diff::LineDiff;
use crate::line_number::LineNumber;
use crate::text_view::TextView;

use gtk::prelude::TextTagExt as _;


pub struct DiffColorizer
{
    addedLineTag: gtk::TextTag,
    removedLineTag: gtk::TextTag,
    hunkHeaderTag: gtk::TextTag,
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
        let addedLineTag = makeTag("green");
        let removedLineTag = makeTag("red");
        let hunkHeaderTag = makeTag("silver");
        textView.registerTags(&[&addedLineTag, &removedLineTag, &hunkHeaderTag]);
        Self{
            addedLineTag,
            removedLineTag,
            hunkHeaderTag,
            tagStartLine: 0.into(),
            state: State::Normal}
    }

    pub fn colorize(&mut self, textView: &TextView, diff: &str)
    {
        textView.setText(diff);
        self.applyTags(textView, diff);
    }

    pub fn update(&mut self, textView: &TextView, differences: Vec<LineDiff>)
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
                let lineNumber: LineNumber = lineNumber.into();
                match character {
                    '+' => self.applyTagToAddedLine(textView, lineNumber),
                    '-' => self.applyTagToRemovedLine(textView, lineNumber),
                    '@' => self.applyTagToHunkHeader(textView, lineNumber),
                     _  => self.applyTagToNormalLine(textView, lineNumber)
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
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
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
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            }
            State::Removed => (),
        }
    }

    fn applyTagToHunkHeader(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => (),
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
        }
        // hunk headers are in the form of "@@ -24,12 +25,14 @@ struct Foo"
        textView.applyTagUntilMatchEnd(&self.hunkHeaderTag, lineNumber, " @@");
    }

    fn applyTagToNormalLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => (),
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            },
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
        }
    }

    fn closeLastOpenTag(&self, textView: &TextView)
    {
        match self.state {
            State::Normal => (),
            State::Added => textView.applyTagUntilEnd(&self.addedLineTag, self.tagStartLine),
            State::Removed => textView.applyTagUntilEnd(&self.removedLineTag, self.tagStartLine)
        }
    }
}

fn updateDiff(textView: &TextView, differences: Vec<LineDiff>)
{
    let mut currentLine = 0.into();
    for difference in differences {
        match difference {
            LineDiff::Equal(text) => {
                currentLine += text.lines().count();
            },
            LineDiff::Insert(text) => {
                let text = ensureTextEndsWithNewLine(&text);
                textView.insertTextAt(&text, currentLine);
                currentLine += text.lines().count();
            }
            LineDiff::Delete(text) => {
                textView.removeTextAt(currentLine, text.lines().count().into());
            }
        }
    }
}

fn makeTag(name: &str) -> gtk::TextTag
{
    let tag = gtk::TextTag::new(Some(name));
    tag.set_foreground(Some(name));
    tag
}

fn diffRequiresUpdating(differences: &[LineDiff]) -> bool
{
    if differences.is_empty() {
        return false;
    }

    for change in differences {
        match change {
            LineDiff::Equal(_) => continue,
            LineDiff::Delete(_) => return true,
            LineDiff::Insert(_) => return true
        }
    }

    false
}

fn ensureTextEndsWithNewLine(text: &str) -> String
{
    if text.ends_with('\n') {
        text.into()
    } else {
        text.to_string()+ "\n"
    }
}
