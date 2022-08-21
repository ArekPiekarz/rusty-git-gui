use crate::commit_log_column::{CommitLogColumn};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::text_filter::TextFilter;

use anyhow::Result;
use gtk::traits::TreeModelExt;
use gtk::traits::TreeModelFilterExt;
use std::cell::RefCell;
use std::rc::Rc;


pub(crate) struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    summaryFilter: SummaryFilter,
    authorFilter: AuthorFilter,
    summaryError: Option<regex::Error>,
    authorError: Option<regex::Error>,
    sender: Sender,
    state: State
}

impl IEventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match (source, event) {
            (_,                                         Event::RefilterRequested)     => self.onRefilterRequested(),
            (Source::CommitLogAuthorFilterEntry,        Event::TextEntered(text))     => self.onAuthorTextChanged(text),
            (Source::CommitLogAuthorFilterCaseButton,   Event::Toggled(shouldEnable)) => self.onAuthorCaseSensitivityToggled(*shouldEnable),
            (Source::CommitLogAuthorFilterRegexButton,  Event::Toggled(shouldEnable)) => self.onAuthorRegexToggled(*shouldEnable),
            (Source::CommitLogSummaryFilterEntry,       Event::TextEntered(text))     => self.onSummaryTextChanged(text),
            (Source::CommitLogSummaryFilterCaseButton,  Event::Toggled(shouldEnable)) => self.onSummaryCaseSensitivityToggled(*shouldEnable),
            (Source::CommitLogSummaryFilterRegexButton, Event::Toggled(shouldEnable)) => self.onSummaryRegexToggled(*shouldEnable),
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
        let summaryFilter = Rc::new(RefCell::new(TextFilter::new()));
        let authorFilter = Rc::new(RefCell::new(TextFilter::new()));
        setupFilterFunction(&modelFilter, Rc::clone(&summaryFilter), Rc::clone(&authorFilter));
        Self{
            modelFilter,
            summaryFilter,
            authorFilter,
            summaryError: None,
            authorError: None,
            sender,
            state: State::Success
        }
    }


    // private

    fn onAuthorTextChanged(&mut self, text: &str)
    {
        let result = self.authorFilter.borrow_mut().setText(text);
        self.handleChangeResult(result, FilterKind::Author);
    }

    fn onAuthorCaseSensitivityToggled(&mut self, shouldEnable: bool)
    {
        let result = self.authorFilter.borrow_mut().setCaseSensitivityEnabled(shouldEnable);
        self.handleChangeResult(result, FilterKind::Author);
    }

    fn onAuthorRegexToggled(&mut self, shouldEnable: bool)
    {
        let result = self.authorFilter.borrow_mut().setRegexEnabled(shouldEnable);
        self.handleChangeResult(result, FilterKind::Author);
    }

    fn onSummaryTextChanged(&mut self, text: &str)
    {
        let result = self.summaryFilter.borrow_mut().setText(text);
        self.handleChangeResult(result, FilterKind::Summary);
    }

    fn onSummaryCaseSensitivityToggled(&mut self, shouldEnable: bool)
    {
        let result = self.summaryFilter.borrow_mut().setCaseSensitivityEnabled(shouldEnable);
        self.handleChangeResult(result, FilterKind::Summary);
    }

    fn onSummaryRegexToggled(&mut self, shouldEnable: bool)
    {
        let result = self.summaryFilter.borrow_mut().setRegexEnabled(shouldEnable);
        self.handleChangeResult(result, FilterKind::Summary);
    }

    fn handleChangeResult(&mut self, result: Result<(), regex::Error>, filterKind: FilterKind)
    {
        match result {
            Ok(()) => self.onChangeSucceeded(filterKind),
            Err(e) => self.onChangeFailed(e, filterKind)
        }
    }

    fn onChangeSucceeded(&mut self, filterKind: FilterKind)
    {
        if self.state == State::Failure {
            self.state = State::Success;
            match filterKind {
                FilterKind::Summary => {
                    self.summaryError = None;
                    self.sendValidSummaryTextInputted();
                },
                FilterKind::Author => {
                    self.authorError = None;
                    self.sendValidAuthorTextInputted();
                }
            }
        }
        self.requestRefilter();
    }

    fn onChangeFailed(&mut self, error: regex::Error, filterKind: FilterKind)
    {
        if self.state == State::Success {
            self.state = State::Failure;
        }

        match filterKind {
            FilterKind::Summary => {
                if self.summaryError.as_ref() == Some(&error) {
                    return;
                }
                self.summaryError = Some(error.clone());
                self.sendInvalidSummaryTextInputted(error);
            },
            FilterKind::Author => {
                if self.authorError.as_ref() == Some(&error) {
                    return;
                }
                self.authorError = Some(error.clone());
                self.sendInvalidAuthorTextInputted(error);
            }
        };
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

    fn sendValidSummaryTextInputted(&self)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::ValidSummaryTextInputted)).unwrap();
    }

    fn sendValidAuthorTextInputted(&self)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::ValidAuthorTextInputted)).unwrap();
    }

    fn sendInvalidSummaryTextInputted(&self, error: regex::Error)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::InvalidSummaryTextInputted(error))).unwrap();
    }

    fn sendInvalidAuthorTextInputted(&self, error: regex::Error)
    {
        self.sender.send((Source::CommitLogModelFilter, Event::InvalidAuthorTextInputted(error))).unwrap();
    }
}

fn setupFilterFunction(modelFilter: &gtk::TreeModelFilter, summaryFilter: SummaryFilter, authorFilter: AuthorFilter)
{
    modelFilter.set_visible_func(move |model, iter| {
        isMatch(&summaryFilter, CommitLogColumn::Summary, model, iter)
            && isMatch(&authorFilter, CommitLogColumn::Author, model, iter)
    });
}

fn isMatch(filter: &Rc<RefCell<TextFilter>>, column: CommitLogColumn, model: &gtk::TreeModel, iter: &gtk::TreeIter)
    -> bool
{
    let filter = &*filter.borrow();
    if filter.isEmpty() {
        return true;
    }

    let value = model.value(iter, column.into());
    let text = value.get::<&str>().unwrap();
    filter.isMatch(text)
}

type SummaryFilter = Rc<RefCell<TextFilter>>;
type AuthorFilter = Rc<RefCell<TextFilter>>;

#[derive(Eq, PartialEq)]
enum State
{
    Success,
    Failure
}

enum FilterKind
{
    Summary,
    Author
}
