use crate::config::{AuthorFilter, CommitLogFilter, Config};
use crate::event::{Event, FilterIndex, handleUnknown, IEventHandler, Sender, Source};

use itertools::Itertools;


pub(crate) struct CommitLogFilters
{
    currentFilter: CurrentFilter,
    filters: Vec<CommitLogFilter>,
    sender: Sender
}

impl IEventHandler for CommitLogFilters
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        use Source as S;
        use Event as E;
        match (source, event) {
            (S::CommitLogAuthorFilterCaseButton,  E::Toggled(isEnabled))        => self.onCaseButtonToggled(*isEnabled),
            (S::CommitLogAuthorFilterRegexButton, E::Toggled(isEnabled))        => self.onRegexButtonToggled(*isEnabled),
            (_,                                   E::ActiveFilterChosen(index)) => self.onActiveFilterChosen(*index),
            (_,                                   E::FilterNameChosen(name))    => self.onFilterNameChosen(name),
            (_,                                   E::TextEntered(text))         => self.onTextEntered(text),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogFilters
{
    pub fn new(config: &Config, sender: Sender) -> Self
    {
        let newSelf = Self{
            currentFilter: CurrentFilter::new(config.commitLogFilters.active),
            filters: config.commitLogFilters.filters.clone(),
            sender
        };
        newSelf.notifyActiveFilterSwitched();
        newSelf
    }

    fn onActiveFilterChosen(&mut self, index: usize)
    {
        if self.currentFilter.index == index {
            return;
        }
        self.currentFilter = CurrentFilter::new(index);
        self.notifyActiveFilterSwitched();
        self.notifyFiltersUpdated();
    }

    fn onCaseButtonToggled(&mut self, isEnabled: bool)
    {
        if self.filters[self.currentFilter.index].authorFilter.caseSensitive == isEnabled {
            self.currentFilter.authorFilterChanges.caseSensitive = None;
        } else {
            self.currentFilter.authorFilterChanges.caseSensitive = Some(isEnabled);
        }
    }

    fn onRegexButtonToggled(&mut self, isEnabled: bool)
    {
        if self.filters[self.currentFilter.index].authorFilter.usesRegex == isEnabled {
            self.currentFilter.authorFilterChanges.usesRegex = None;
        } else {
            self.currentFilter.authorFilterChanges.usesRegex = Some(isEnabled);
        }
    }

    fn onFilterNameChosen(&mut self, name: &str)
    {
        match self.filters.iter().find_position(|filter| filter.name == name)
        {
            Some((_index, _filter)) => unimplemented!(),
            None => {
                self.addFilter(name);
                self.notifyFilterAdded(name);
                self.notifyFiltersUpdated();
            }
        }
    }

    fn onTextEntered(&mut self, text: &str)
    {
        if self.filters[self.currentFilter.index].authorFilter.pattern == text {
            self.currentFilter.authorFilterChanges.pattern = None;
        } else {
            self.currentFilter.authorFilterChanges.pattern = Some(text.into());
        }
    }

    fn addFilter(&mut self, name: &str)
    {
        let newFilter = CommitLogFilter{name: name.into(), authorFilter: self.mergeOriginalAndChangedFilter()};
        self.filters.push(newFilter);
        self.currentFilter = CurrentFilter::new(self.filters.len()-1);
    }

    fn mergeOriginalAndChangedFilter(&self) -> AuthorFilter
    {
        let baseFilter = &self.filters[self.currentFilter.index].authorFilter;
        let authorFilterChanges = &self.currentFilter.authorFilterChanges;
        AuthorFilter{
            pattern: authorFilterChanges.pattern.as_ref().unwrap_or(&baseFilter.pattern).into(),
            caseSensitive: authorFilterChanges.caseSensitive.unwrap_or(baseFilter.caseSensitive),
            usesRegex: authorFilterChanges.usesRegex.unwrap_or(baseFilter.usesRegex)
        }
    }

    fn notifyFilterAdded(&self, name: &str)
    {
        self.sender.send((Source::CommitLogFilters, Event::FilterAdded(name.into()))).unwrap();
    }

    fn notifyFiltersUpdated(&self)
    {
        self.sender.send((
            Source::CommitLogFilters,
            Event::FiltersUpdated(
                crate::config::CommitLogFilters{active: self.currentFilter.index, filters: self.filters.clone()})))
            .unwrap();
    }

    fn notifyActiveFilterSwitched(&self)
    {
        self.sender.send((
            Source::CommitLogFilters,
            Event::ActiveFilterSwitched(self.filters[self.currentFilter.index].authorFilter.clone())))
            .unwrap();
    }
}

struct CurrentFilter
{
    index: usize,
    authorFilterChanges: AuthorFilterChanges
}

impl CurrentFilter
{
    fn new(index: FilterIndex) -> Self
    {
        Self{index, authorFilterChanges: AuthorFilterChanges::new()}
    }
}

struct AuthorFilterChanges
{
    pattern: Option<String>,
    caseSensitive: Option<bool>,
    usesRegex: Option<bool>
}

impl AuthorFilterChanges
{
    fn new() -> Self
    {
        Self{pattern: None, caseSensitive: None, usesRegex: None}
    }
}
