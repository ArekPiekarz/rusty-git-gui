use crate::commit_log_column::{CommitLogColumn};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::text_filter::TextFilter;

use gtk::traits::TreeModelExt;
use gtk::traits::TreeModelFilterExt;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    authorFilter: Rc<RefCell<TextFilter>>,
    sender: Sender
}

impl IEventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match (source, event) {
            (_, Event::RefilterRequested)   => self.onRefilterRequested(),
            (_, Event::TextEntered(filter)) => self.onCommitAuthorFilterChanged(filter),
            (Source::CommitLogAuthorFilterRegexButton, Event::Toggled(isEnabled)) => self.onRegexToggled(*isEnabled),
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
        let authorFilter = Rc::new(RefCell::new(TextFilter::new()));
        setupFilterFunction(&modelFilter, Rc::clone(&authorFilter));
        Self{modelFilter, authorFilter, sender}
    }


    // private

    fn onCommitAuthorFilterChanged(&self, text: &str)
    {
        match self.authorFilter.borrow_mut().setText(text) {
            Ok(()) => self.requestRefilter(),
            Err(e) => eprintln!("Invalid author filter regex: {}", e)
        }

    }

    fn onRegexToggled(&self, isEnabled: bool)
    {
        match self.authorFilter.borrow_mut().enableRegex(isEnabled) {
            Ok(()) => self.requestRefilter(),
            Err(e) => eprintln!("Invalid author filter regex: {}", e)
        }
    }

    fn onRefilterRequested(&self)
    {
        self.modelFilter.refilter();
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterEnded)).unwrap();
    }

    fn requestRefilter(&self)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterRequested)).unwrap();
    }
}

fn setupFilterFunction(modelFilter: &gtk::TreeModelFilter, authorFilter: Rc<RefCell<TextFilter>>)
{
    modelFilter.set_visible_func(move |model, iter| {
        let authorFilter = &*authorFilter.borrow();
        if authorFilter.isEmpty() {
            return true;
        }

        let author = model.value(iter, CommitLogColumn::Author.into());
        let author = author.get::<&str>().unwrap();
        authorFilter.isMatch(author)
    });
}
