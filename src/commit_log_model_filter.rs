use crate::commit_log_column::{CommitLogColumn};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::text_filter::TextFilter;

use anyhow::{Error, Result};
use gtk::traits::TreeModelExt;
use gtk::traits::TreeModelFilterExt;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    authorFilter: Rc<RefCell<TextFilter>>,
    sender: Sender,
    state: State
}

impl IEventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match (source, event) {
            (_, Event::RefilterRequested)   => self.onRefilterRequested(),
            (_, Event::TextEntered(filter)) => self.onCommitAuthorFilterChanged(filter),
            (Source::CommitLogAuthorFilterCaseButton,  Event::Toggled(shouldEnable)) => self.onCaseSensitivityToggled(*shouldEnable),
            (Source::CommitLogAuthorFilterRegexButton, Event::Toggled(shouldEnable)) => self.onRegexToggled(*shouldEnable),
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
        Self{modelFilter, authorFilter, sender, state: State::Success}
    }


    // private

    fn onCommitAuthorFilterChanged(&mut self, text: &str)
    {
        let result = self.authorFilter.borrow_mut().setText(text);
        self.handleChangeResult(result);
    }

    fn onCaseSensitivityToggled(&mut self, shouldEnable: bool)
    {
        let result = self.authorFilter.borrow_mut().setCaseSensitivityEnabled(shouldEnable);
        self.handleChangeResult(result);
    }

    fn onRegexToggled(&mut self, shouldEnable: bool)
    {
        let result = self.authorFilter.borrow_mut().setRegexEnabled(shouldEnable);
        self.handleChangeResult(result);
    }

    fn handleChangeResult(&mut self, result: Result<()>)
    {
        match result {
            Ok(()) => self.onChangeSucceeded(),
            Err(e) => self.onChangeFailed(e)
        }
    }

    fn onChangeSucceeded(&mut self)
    {
        if self.state == State::Failure {
            self.state = State::Success;
            self.sendValidTextInputted();
        }
        self.requestRefilter();
    }

    fn onChangeFailed(&mut self, error: Error)
    {
        if self.state == State::Success {
            self.state = State::Failure;
            self.sendInvalidTextInputted(error);
        }
    }

    fn requestRefilter(&self)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterRequested)).unwrap();
    }

    fn onRefilterRequested(&self)
    {
        self.modelFilter.refilter();
        self.sender.send((Source::CommitLogModelFilter, Event::RefilterEnded)).unwrap();
    }

    fn sendValidTextInputted(&self)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::ValidTextInputted)).unwrap();
    }

    fn sendInvalidTextInputted(&self, error: Error)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::InvalidTextInputted(error))).unwrap();
    }
}

#[derive(Eq, PartialEq)]
enum State
{
    Success,
    Failure
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
