use crate::line_diff::LineDiff;
use crate::line_number::LineNumber;
use crate::text_view::TextView;

use gtk::prelude::TextTagExt as _;
use crate::diff_formatter::{FormattedDiff, LineFormat};


pub(crate) struct DiffColorizer
{
    addedLineTag: gtk::TextTag,
    removedLineTag: gtk::TextTag,
    hunkHeaderTag: gtk::TextTag,
    fileHeaderTag: gtk::TextTag,
    tagStartLine: LineNumber,
    state: State
}

enum State
{
    Normal,
    Added,
    Removed,
    FileHeader
}

impl DiffColorizer
{
    pub fn new(textView: &TextView) -> Self
    {
        let addedLineTag = makeTag("green");
        let removedLineTag = makeTag("red");
        let hunkHeaderTag = makeTag("silver");
        let fileHeaderTag = makeTag("dodgerblue");
        textView.registerTags(&[&addedLineTag, &removedLineTag, &hunkHeaderTag, &fileHeaderTag]);
        Self{
            addedLineTag,
            removedLineTag,
            hunkHeaderTag,
            fileHeaderTag,
            tagStartLine: 0.into(),
            state: State::Normal}
    }

    pub fn colorize(&mut self, textView: &TextView, diff: &FormattedDiff)
    {
        textView.setText(&diff.text);
        self.applyTags(textView, &diff.lineFormats);
    }

    pub fn update(&mut self, textView: &TextView, differences: Vec<LineDiff>, lineFormats: &[LineFormat])
    {
        if !diffRequiresUpdating(&differences) {
            return;
        }

        textView.removeTags();
        updateDiff(textView, differences);
        self.applyTags(textView, lineFormats);
    }


    // private

    fn applyTags(&mut self, textView: &TextView, lineFormats: &[LineFormat])
    {
        self.state = State::Normal;
        self.tagStartLine = 0.into();
        self.applyTagsBasedOnLineTypes(textView, lineFormats);
        self.closeLastOpenTag(textView);
    }

    fn applyTagsBasedOnLineTypes(&mut self, textView: &TextView, lineFormats: &[LineFormat])
    {
        for (lineNumber, lineFormat) in lineFormats.iter().enumerate() {
            let lineNumber: LineNumber = lineNumber.into();
            match lineFormat {
                LineFormat::TopHeader           => self.applyTagToNormalLine(textView, lineNumber),
                LineFormat::FileHeader          => self.applyTagToFileHeader(textView, lineNumber),
                LineFormat::HunkHeader          => self.applyTagToHunkHeader(textView, lineNumber),
                LineFormat::ContextLine         => self.applyTagToNormalLine(textView, lineNumber),
                LineFormat::AddedLine           => self.applyTagToAddedLine(textView, lineNumber),
                LineFormat::RemovedLine         => self.applyTagToRemovedLine(textView, lineNumber),
                LineFormat::SameNoNewLineAtEnd  => self.applyTagToSameNoNewLineAtEnd(textView, lineNumber),
                LineFormat::AddedNewLineAtEnd   => self.applyTagToAddedNewLineAtEnd(textView, lineNumber),
                LineFormat::RemovedNewLineAtEnd => self.applyTagToRemovedNewLineAtEnd(textView, lineNumber),
                LineFormat::BinaryLine          => self.applyTagToBinaryLine(textView, lineNumber)
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
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
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
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            }
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
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
        }
        // hunk headers are in the form of "@@ -24,12 +25,14 @@ struct Foo"
        textView.applyTagUntilMatchEnd(&self.hunkHeaderTag, lineNumber, " @@");
    }

    fn applyTagToFileHeader(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::FileHeader;
            },
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::FileHeader;
            },
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            },
            State::FileHeader => ()
        }
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
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
        }
    }

    fn applyTagToSameNoNewLineAtEnd(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        self.applyTagToMetaLine(MetaTagKind::Context, textView, lineNumber);
    }

    fn applyTagToAddedNewLineAtEnd(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        self.applyTagToMetaLine(MetaTagKind::Addition, textView, lineNumber);
    }

    fn applyTagToRemovedNewLineAtEnd(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        self.applyTagToMetaLine(MetaTagKind::Deletion, textView, lineNumber);
    }

    fn applyTagToBinaryLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        self.applyTagToMetaLine(MetaTagKind::Context, textView, lineNumber);
    }

    fn applyTagToMetaLine(&mut self, tagKind: MetaTagKind, textView: &TextView, lineNumber: LineNumber)
    {
        // For some reason the meta information from git2 for addition and deletion of new lines at the end of files
        // seems to be switched, so we switch colors for them.
        let tag = match tagKind {
            MetaTagKind::Context  => &self.hunkHeaderTag,
            MetaTagKind::Addition => &self.removedLineTag,
            MetaTagKind::Deletion => &self.addedLineTag
        };

        match self.state {
            State::Normal => {
                textView.applyTagUntilLineEnd(tag, lineNumber);
            },
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                textView.applyTagUntilLineEnd(tag, lineNumber);
                self.state = State::Normal;
            },
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                textView.applyTagUntilLineEnd(tag, lineNumber);
                self.state = State::Normal;
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                textView.applyTagUntilLineEnd(tag, lineNumber);
                self.state = State::Normal;
            }
        }
    }

    fn closeLastOpenTag(&self, textView: &TextView)
    {
        match self.state {
            State::Normal => (),
            State::Added => textView.applyTagUntilEnd(&self.addedLineTag, self.tagStartLine),
            State::Removed => textView.applyTagUntilEnd(&self.removedLineTag, self.tagStartLine),
            State::FileHeader => textView.applyTagUntilEnd(&self.fileHeaderTag, self.tagStartLine)
        }
    }
}

#[derive(Clone, Copy)]
enum MetaTagKind
{
    Context,
    Addition,
    Deletion
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
