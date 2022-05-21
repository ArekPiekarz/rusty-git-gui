use crate::commit_log_column::{CommitLogColumn};
use crate::event::{Event, handleUnknown, IEventHandler, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::glib;
use gtk::traits::TreeModelExt;
use gtk::traits::TreeModelFilterExt;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    authorFilter: AuthorFilter,
}

type AuthorFilter = Rc<RefCell<String>>;

impl IEventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::TextEntered(filter) => self.onCommitAuthorFilterChanged(filter),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogModelFilter
{
    pub fn new(guiElementProvider: &GuiElementProvider)
        -> Self
    {
        let modelFilter = guiElementProvider.get::<gtk::TreeModelFilter>("Commit log store filter");
        let authorFilter = Rc::new(RefCell::new(String::new()));
        setupFilterFunction(&modelFilter, Rc::clone(&authorFilter));
        Self{modelFilter, authorFilter}
    }


    // private

    fn onCommitAuthorFilterChanged(&self, filter: &str)
    {
        *self.authorFilter.borrow_mut() = filter.into();
        self.modelFilter.refilter();
    }
}

fn setupFilterFunction(modelFilter: &gtk::TreeModelFilter, authorFilter: AuthorFilter)
{
    modelFilter.set_visible_func(move |model, iter| {
        if isRowEmpty(model, iter) {
            return false;
        }

        let authorFilter = &*authorFilter.borrow();
        if authorFilter.is_empty() {
            return true;
        }
        let author = model.value(iter, CommitLogColumn::Author.into());
        let author = author.get::<&str>().unwrap();
        author == *authorFilter
    });
}

fn isRowEmpty(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> bool
{
    match model.value(iter, CommitLogColumn::Date.into()).get::<&str>() {
        Ok(text) => text.is_empty(),
        Err(error) => match error {
            glib::value::ValueTypeMismatchOrNoneError::WrongValueType(e) => panic!("Wrong value type: {}", e),
            glib::value::ValueTypeMismatchOrNoneError::UnexpectedNone => true
        }
    }
}
