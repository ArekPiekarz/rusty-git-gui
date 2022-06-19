use anyhow::{bail, Result};
use regex::{Regex, RegexBuilder};


pub(crate) struct TextFilter
{
    regexState: RegexState
}

enum RegexState
{
    Disabled{text: String},
    Invalid{text: String},
    Valid{regex: Regex}
}

impl TextFilter
{
    pub(crate) fn new() -> Self
    {
        Self{regexState: RegexState::Disabled{text: String::new()}}
    }

    pub(crate) fn setText(&mut self, newText: &str) -> Result<()>
    {
        match &mut self.regexState {
            RegexState::Disabled{text} => *text = newText.into(),
            RegexState::Invalid{text} => {
                match RegexBuilder::new(newText).case_insensitive(true).build() {
                    Ok(newRegex) => self.regexState = RegexState::Valid{regex: newRegex},
                    Err(e) => {
                        *text = newText.into();
                        bail!(e)
                    }
                }
            },
            RegexState::Valid{regex} => {
                match RegexBuilder::new(newText).case_insensitive(true).build() {
                    Ok(newRegex) => *regex = newRegex,
                    Err(e) => {
                        self.regexState = RegexState::Invalid{text: newText.into()};
                        bail!(e)
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn setRegexEnabled(&mut self, shouldEnable: bool) -> Result<()>
    {
        if shouldEnable {
            match &mut self.regexState {
                RegexState::Disabled{text} => {
                    match RegexBuilder::new(&text).case_insensitive(true).build() {
                        Ok(regex) => self.regexState = RegexState::Valid{regex},
                        Err(e) => {
                            self.regexState = RegexState::Invalid{text: text.clone()};
                            bail!(e)
                        }
                    }
                },
                RegexState::Invalid{..} => (),
                RegexState::Valid{..} => ()
            }
        } else {
            match &mut self.regexState {
                RegexState::Disabled{..} => (),
                RegexState::Invalid{text} => self.regexState = RegexState::Disabled{text: text.clone()},
                RegexState::Valid{regex} => self.regexState = RegexState::Disabled{text: regex.as_str().into()}
            }
        }
        Ok(())
    }

    pub(crate) fn isEmpty(&self) -> bool
    {
        match &self.regexState {
            RegexState::Disabled{text} => text.is_empty(),
            RegexState::Invalid{..} => true,
            RegexState::Valid{regex} => regex.as_str().is_empty()
        }
    }

    pub(crate) fn isMatch(&self, input: &str) -> bool
    {
        match &self.regexState {
            RegexState::Disabled{text} => input.to_lowercase().contains(text),
            RegexState::Invalid{..} => false,
            RegexState::Valid{regex} => regex.is_match(input)
        }
    }
}
