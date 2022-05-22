use crate::commit_log_column::{CommitLogColumn};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::TreeModelExt;
use gtk::traits::TreeModelFilterExt;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    authorFilter: AuthorFilter,
    sender: Sender
}

type AuthorFilter = Rc<RefCell<String>>;

impl IEventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::RefilterRequested   => self.onRefilterRequested(),
            Event::TextEntered(filter) => self.onCommitAuthorFilterChanged(filter),
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
        let authorFilter = Rc::new(RefCell::new(String::new()));
        setupFilterFunction(&modelFilter, Rc::clone(&authorFilter));
        Self{modelFilter, authorFilter, sender}
    }


    // private

    fn onCommitAuthorFilterChanged(&self, filter: &str)
    {
        *self.authorFilter.borrow_mut() = filter.into();
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterRequested)).unwrap();
    }

    fn onRefilterRequested(&self)
    {
        self.modelFilter.refilter();
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterEnded)).unwrap();
    }
}

fn setupFilterFunction(modelFilter: &gtk::TreeModelFilter, authorFilter: AuthorFilter)
{
    modelFilter.set_visible_func(move |model, iter| {
        let authorFilter = &*authorFilter.borrow();
        if authorFilter.is_empty() {
            return true;
        }

        let author = model.value(iter, CommitLogColumn::Author.into());
        let author = author.get::<&str>().unwrap();
        author == *authorFilter
    });
}
