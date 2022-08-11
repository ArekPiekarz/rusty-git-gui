use crate::file_change::FileChange;

const FORMATTING_SUCCEEDED: bool = true;


pub(crate) struct DiffFormatter
{
    output: FormattedDiff,
    mode: FormatterMode
}

impl DiffFormatter
{
    pub fn newForCommit() -> Self
    {
        Self{output: FormattedDiff::default(), mode: FormatterMode::Commit}
    }

    pub fn newForFileChange(fileChange: &FileChange) -> Self
    {
        Self{output: formatDiffHeader(fileChange), mode: FormatterMode::FileChange}
    }

    pub fn format(&mut self, line: &git2::DiffLine) -> bool
    {
        let lineContent = String::from_utf8_lossy(line.content());
        match line.origin_value() {
            git2::DiffLineType::Context      => self.formatContextLine(&lineContent),
            git2::DiffLineType::Addition     => self.formatAddedLine(&lineContent),
            git2::DiffLineType::Deletion     => self.formatRemovedLine(&lineContent),
            git2::DiffLineType::ContextEOFNL => self.formatSameNoNewLineAtEnd(&lineContent),
            git2::DiffLineType::AddEOFNL     => self.formatAddedNewLineAtEnd(&lineContent),
            git2::DiffLineType::DeleteEOFNL  => self.formatRemovedNewLineAtEnd(&lineContent),
            git2::DiffLineType::FileHeader   => self.formatFileHeader(&lineContent),
            git2::DiffLineType::HunkHeader   => self.formatHunkHeader(&lineContent),
            git2::DiffLineType::Binary       => self.formatBinaryLine(&lineContent)
        };
        FORMATTING_SUCCEEDED
    }

    #[allow(clippy::missing_const_for_fn)] // buggy - self cannot be destructed in const fn
    pub fn takeOutput(self) -> FormattedDiff
    {
        self.output
    }


    // private

    fn formatContextLine(&mut self, line: &str)
    {
        self.output.text.push_str(&format!(" {}", line));
        self.output.lineFormats.push(LineFormat::ContextLine);
    }

    fn formatAddedLine(&mut self, line: &str)
    {
        self.output.text.push_str(&format!("+{}", line));
        self.output.lineFormats.push(LineFormat::AddedLine);
    }

    fn formatRemovedLine(&mut self, line: &str)
    {
        self.output.text.push_str(&format!("-{}", line));
        self.output.lineFormats.push(LineFormat::RemovedLine);
    }

    fn formatSameNoNewLineAtEnd(&mut self, line: &str)
    {
        self.output.text.push_str(line);
        self.output.lineFormats.push(LineFormat::SameNoNewLineAtEnd);
    }

    fn formatAddedNewLineAtEnd(&mut self, line: &str)
    {
        self.output.text.push_str(line);
        self.output.lineFormats.push(LineFormat::AddedNewLineAtEnd);
    }

    fn formatRemovedNewLineAtEnd(&mut self, line: &str)
    {
        self.output.text.push_str(line);
        self.output.lineFormats.push(LineFormat::RemovedNewLineAtEnd);
    }

    fn formatFileHeader(&mut self, line : &str)
    {
        match self.mode {
            FormatterMode::Commit => {
                self.output.text.push_str(line);
                self.output.lineFormats.extend(vec![LineFormat::FileHeader; line.lines().count()]);
            },
            FormatterMode::FileChange => ()
        }
    }

    fn formatHunkHeader(&mut self, line : &str)
    {
        self.output.text.push_str(line);
        self.output.lineFormats.push(LineFormat::HunkHeader);
    }

    fn formatBinaryLine(&mut self, line: &str)
    {
        self.output.text.push_str(line);
        self.output.lineFormats.push(LineFormat::BinaryLine);
    }
}

enum FormatterMode
{
    Commit,
    FileChange
}

#[derive(Default)]
pub(crate) struct FormattedDiff
{
    pub text: String,
    pub lineFormats: Vec<LineFormat>
}

#[derive(Clone, Debug)]
pub(crate) enum LineFormat
{
    TopHeader,
    FileHeader,
    HunkHeader,
    ContextLine,
    AddedLine,
    RemovedLine,
    SameNoNewLineAtEnd,
    AddedNewLineAtEnd,
    RemovedNewLineAtEnd,
    BinaryLine
}

fn formatDiffHeader(fileChange: &FileChange) -> FormattedDiff
{
    match &fileChange.oldPath {
        Some(oldPath) => {
            let text = format!("renamed file\nold path: {}\nnew path: {}\n", oldPath, fileChange.path);
            let lineFormats = vec![LineFormat::TopHeader; 3];
            FormattedDiff{text, lineFormats}
        },
        None => FormattedDiff::default()
    }
}
