use crate::line_number::LineNumber;
use crate::text_view::TextView;

use difference::Difference;
use gtk::TextTagExt as _;
use std::cell::RefCell;
use std::rc::Rc;


pub struct DiffColorizer
{
    textView: Rc<RefCell<TextView>>,
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
    pub fn new(textView: Rc<RefCell<TextView>>) -> Self
    {
        let greenTag = makeTag("green");
        let redTag = makeTag("red");
        textView.borrow().registerTags(&[&greenTag, &redTag]);
        Self{
            textView,
            greenTag,
            redTag,
            tagStartLine: 0.into(),
            state: State::Normal}
    }

    pub fn colorize(&mut self, diff: &str)
    {
        self.setText(diff);
        self.applyTags(diff);
    }

    pub fn update(&mut self, differences: Vec<Difference>)
    {
        if !diffRequiresUpdating(&differences) {
            return;
        }

        self.removeTags();
        self.updateDiff(differences);
        self.applyTags(&self.getText());
    }


    // private

    fn updateDiff(&self, differences: Vec<Difference>)
    {
        let textView = self.textView.borrow();
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

    fn applyTags(&mut self, text: &str)
    {
        self.state = State::Normal;
        self.tagStartLine = 0.into();
        self.applyTagsBasedOnLineTypes(text);
        self.closeLastOpenTag();
    }

    fn applyTagsBasedOnLineTypes(&mut self, text: &str)
    {
        for (lineNumber, line) in text.lines().enumerate() {
            if let Some(character) = line.chars().nth(0) {
                match character {
                    '+' => self.applyTagToAddedLine(lineNumber.into()),
                    '-' => self.applyTagToRemovedLine(lineNumber.into()),
                     _  => self.applyTagToNormalLine(lineNumber.into())
                }
            }
        }
    }

    fn applyTagToAddedLine(&mut self, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            },
            State::Added => (),
            State::Removed => {
                self.applyTag(&self.redTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            }
        }
    }

    fn applyTagToRemovedLine(&mut self, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            },
            State::Added => {
                self.applyTag(&self.greenTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            }
            State::Removed => (),
        }
    }

    fn applyTagToNormalLine(&mut self, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => (),
            State::Added => {
                self.applyTag(&self.greenTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            },
            State::Removed => {
                self.applyTag(&self.redTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
        }
    }

    fn closeLastOpenTag(&self)
    {
        match self.state {
            State::Normal => (),
            State::Added => self.applyTagUntilEnd(&self.greenTag, self.tagStartLine),
            State::Removed => self.applyTagUntilEnd(&self.redTag, self.tagStartLine)
        }
    }

    fn applyTag(&self, tag: &gtk::TextTag, startLine: LineNumber, endline: LineNumber)
    {
        self.textView.borrow().applyTag(tag, startLine, endline);
    }

    fn applyTagUntilEnd(&self, tag: &gtk::TextTag, tagStartLine: LineNumber)
    {
        self.textView.borrow().applyTagUntilEnd(tag, tagStartLine);
    }

    fn removeTags(&self)
    {
        self.textView.borrow().removeTags();
    }

    fn getText(&self) -> String
    {
        self.textView.borrow().getText()
    }

    fn setText(&self, text: &str)
    {
        self.textView.borrow().setText(text);
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