use crate::file_change::FileChange;

const FORMATTING_SUCCEEDED: bool = true;


pub struct DiffFormatter
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
        match line.origin() {
            ' ' => self.addNormalLine(&lineContent),
            '+' => self.addAddedLine(&lineContent),
            '-' => self.addRemovedLine(&lineContent),
            'F' => self.handleFileHeader(&lineContent),
             _  => self.addHunkInfo(&lineContent)
        };
        FORMATTING_SUCCEEDED
    }

    #[allow(clippy::missing_const_for_fn)] // buggy - self cannot be destructed in const fn
    pub fn takeOutput(self) -> FormattedDiff
    {
        self.output
    }


    // private

    fn addNormalLine(&mut self, line: &str)
    {
        self.output.text.push_str(&format!(" {}", line));
        self.output.lineFormats.push(LineFormat::NormalLine);
    }

    fn addAddedLine(&mut self, line: &str)
    {
        self.output.text.push_str(&format!("+{}", line));
        self.output.lineFormats.push(LineFormat::AddedLine);
    }

    fn addRemovedLine(&mut self, line: &str)
    {
        self.output.text.push_str(&format!("-{}", line));
        self.output.lineFormats.push(LineFormat::RemovedLine);
    }

    fn handleFileHeader(&mut self, line : &str)
    {
        match self.mode {
            FormatterMode::Commit => {
                self.output.text.push_str(line);
                self.output.lineFormats.extend(vec![LineFormat::FileHeader; line.lines().count()]);
            },
            FormatterMode::FileChange => ()
        }
    }

    fn addHunkInfo(&mut self, line : &str)
    {
        self.output.text.push_str(line);
        self.output.lineFormats.push(LineFormat::HunkHeader);
    }
}

enum FormatterMode
{
    Commit,
    FileChange
}

pub struct FormattedDiff
{
    pub text: String,
    pub lineFormats: Vec<LineFormat>
}

impl Default for FormattedDiff
{
    fn default() -> Self
    {
        Self{text: String::new(), lineFormats: vec![]}
    }
}

#[derive(Clone, Debug)]
pub enum LineFormat
{
    TopHeader,
    FileHeader,
    HunkHeader,
    NormalLine,
    AddedLine,
    RemovedLine
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
