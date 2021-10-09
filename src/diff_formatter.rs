use crate::file_change::FileChange;

const FORMATTING_SUCCEEDED: bool = true;


pub struct DiffFormatter
{
    text: String,
    mode: FormatterMode
}

impl DiffFormatter
{
    pub const fn newForCommit() -> Self
    {
        Self{text: String::new(), mode: FormatterMode::Commit}
    }

    pub fn newForFileChange(fileChange: &FileChange) -> Self
    {
        Self{text: formatDiffHeader(fileChange), mode: FormatterMode::FileChange}
    }

    pub fn format(&mut self, line: &git2::DiffLine) -> bool
    {
        let lineContent = String::from_utf8_lossy(line.content());
        match line.origin() {
            prefix @ ('+' | '-' | ' ') => self.addContent(prefix, &lineContent),
            'F' => self.handleFileHeader(&lineContent),
             _  => self.addHunkInfo(&lineContent)
        };
        FORMATTING_SUCCEEDED
    }

    #[allow(clippy::missing_const_for_fn)] // buggy - self cannot be destructed in const fn
    pub fn takeText(self) -> String
    {
        self.text
    }


    // private

    fn addContent(&mut self, prefix: char, line : &str)
    {
        self.text.push_str(&format!("{}{}", prefix, line));
    }

    fn handleFileHeader(&mut self, line : &str)
    {
        match self.mode {
            FormatterMode::Commit => self.text.push_str(line),
            FormatterMode::FileChange => ()
        }
    }

    fn addHunkInfo(&mut self, line : &str)
    {
        self.text.push_str(line);
    }
}

enum FormatterMode
{
    Commit,
    FileChange
}

fn formatDiffHeader(fileChange: &FileChange) -> String
{
    match &fileChange.oldPath {
        Some(oldPath) => format!("renamed file\nold path: {}\nnew path: {}\n", oldPath, fileChange.path),
        None => String::new()
    }
}
