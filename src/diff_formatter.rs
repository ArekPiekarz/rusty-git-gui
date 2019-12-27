const FORMATTING_SUCCEEDED: bool = true;
const IGNORE_FILE_HEADER: () = ();


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
            prefix if isContentPrefix(prefix) => self.addContent(prefix, &lineContent),
            'F' => IGNORE_FILE_HEADER,
             _  => self.addHunkInfo(&lineContent)
        };
        FORMATTING_SUCCEEDED
    }

    pub fn takeText(self) -> String
    {
        self.text
    }


    // private

    fn addContent(&mut self, prefix: char, line : &str)
    {
        self.text.push_str(&format!("{}{}", prefix, line));
    }

    fn addHunkInfo(&mut self, line : &str)
    {
        self.text.push_str(&line);
    }
}

fn isContentPrefix(prefix: char) -> bool
{
    prefix == '+' || prefix == '-' || prefix == ' '
}