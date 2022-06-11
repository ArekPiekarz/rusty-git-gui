use crate::commit_log_column::{CommitLogColumn};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::TreeModelExt;
use gtk::traits::TreeModelFilterExt;
use regex::RegexBuilder;
use std::cell::RefCell;
use std::ops::Not;
use std::rc::Rc;


pub struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    authorFilter: Rc<RefCell<AuthorFilter>>,
    sender: Sender
}

impl IEventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match (source, event) {
            (_, Event::RefilterRequested)   => self.onRefilterRequested(),
            (_, Event::TextEntered(filter)) => self.onCommitAuthorFilterChanged(filter),
            (Source::CommitLogAuthorFilterRegexButton, Event::Toggled) => self.onRegexToggled(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogModelFilter
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender)
        -> Self
    {
        let modelFilter = guiElementProvider.get::<gtk::TreeModelFilter>("Commit log store filter");
        let authorFilter = Rc::new(RefCell::new(AuthorFilter::new()));
        setupFilterFunction(&modelFilter, Rc::clone(&authorFilter));
        Self{modelFilter, authorFilter, sender}
    }


    // private

    fn onCommitAuthorFilterChanged(&self, filter: &str)
    {
        self.authorFilter.borrow_mut().text = filter.to_lowercase();
        self.requestRefilter();
    }

    fn onRefilterRequested(&self)
    {
        self.modelFilter.refilter();
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterEnded)).unwrap();
    }

    fn onRegexToggled(&self)
    {
        let mut authorFilter = self.authorFilter.borrow_mut();
        authorFilter.useRegex = authorFilter.useRegex.not();
        self.requestRefilter();
    }

    fn requestRefilter(&self)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterRequested)).unwrap();
    }
}

fn setupFilterFunction(modelFilter: &gtk::TreeModelFilter, authorFilter: Rc<RefCell<AuthorFilter>>)
{
    modelFilter.set_visible_func(move |model, iter| {
        let authorFilter = &*authorFilter.borrow();
        if authorFilter.text.is_empty() {
            return true;
        }

        let author = model.value(iter, CommitLogColumn::Author.into());
        let author = author.get::<&str>().unwrap();
        if authorFilter.useRegex {
            match RegexBuilder::new(&authorFilter.text).case_insensitive(true).build() {
                Ok(regex) => regex.is_match(author),
                Err(_) => false
            }

        } else {
            author.to_lowercase().contains(&authorFilter.text)
        }
    });
}

struct AuthorFilter
{
    text: String,
    useRegex: bool
}

impl AuthorFilter
{
    fn new() -> Self
    {
        Self{text: String::new(), useRegex: false}
    }
}
