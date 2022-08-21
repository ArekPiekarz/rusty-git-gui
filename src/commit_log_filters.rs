use crate::config::{AuthorFilter, CommitLogFilter, Config, SummaryFilter};
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
            (S::CommitLogAuthorFilterCaseButton,   E::Toggled(isEnabled))        => self.onAuthorCaseButtonToggled(*isEnabled),
            (S::CommitLogAuthorFilterEntry,        E::TextEntered(text))         => self.onAuthorTextEntered(text),
            (S::CommitLogAuthorFilterRegexButton,  E::Toggled(isEnabled))        => self.onAuthorRegexButtonToggled(*isEnabled),
            (S::CommitLogSummaryFilterCaseButton,  E::Toggled(isEnabled))        => self.onSummaryCaseButtonToggled(*isEnabled),
            (S::CommitLogSummaryFilterEntry,       E::TextEntered(text))         => self.onSummaryTextEntered(text),
            (S::CommitLogSummaryFilterRegexButton, E::Toggled(isEnabled))        => self.onSummaryRegexButtonToggled(*isEnabled),
            (_,                                    E::ActiveFilterChosen(index)) => self.onActiveFilterChosen(*index),
            (_,                                    E::FilterNameChosen(name))    => self.onFilterNameChosen(name),
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
        newSelf.notifyActiveFilterDataSwitched();
        newSelf
    }

    fn onActiveFilterChosen(&mut self, index: usize)
    {
        if self.currentFilter.index == index {
            return;
        }
        self.currentFilter = CurrentFilter::new(index);
        self.notifyActiveFilterDataSwitched();
        self.notifyFiltersUpdated();
    }

    fn onAuthorCaseButtonToggled(&mut self, isEnabled: bool)
    {
        if self.filters[self.currentFilter.index].authorFilter.caseSensitive == isEnabled {
            self.currentFilter.authorFilterChanges.caseSensitive = None;
        } else {
            self.currentFilter.authorFilterChanges.caseSensitive = Some(isEnabled);
        }
    }

    fn onAuthorRegexButtonToggled(&mut self, isEnabled: bool)
    {
        if self.filters[self.currentFilter.index].authorFilter.usesRegex == isEnabled {
            self.currentFilter.authorFilterChanges.usesRegex = None;
        } else {
            self.currentFilter.authorFilterChanges.usesRegex = Some(isEnabled);
        }
    }

    fn onAuthorTextEntered(&mut self, text: &str)
    {
        if self.filters[self.currentFilter.index].authorFilter.pattern == text {
            self.currentFilter.authorFilterChanges.pattern = None;
        } else {
            self.currentFilter.authorFilterChanges.pattern = Some(text.into());
        }
    }

    fn onSummaryCaseButtonToggled(&mut self, isEnabled: bool)
    {
        if self.filters[self.currentFilter.index].summaryFilter.caseSensitive == isEnabled {
            self.currentFilter.summaryFilterChanges.caseSensitive = None;
        } else {
            self.currentFilter.summaryFilterChanges.caseSensitive = Some(isEnabled);
        }
    }

    fn onSummaryRegexButtonToggled(&mut self, isEnabled: bool)
    {
        if self.filters[self.currentFilter.index].summaryFilter.usesRegex == isEnabled {
            self.currentFilter.summaryFilterChanges.usesRegex = None;
        } else {
            self.currentFilter.summaryFilterChanges.usesRegex = Some(isEnabled);
        }
    }

    fn onSummaryTextEntered(&mut self, text: &str)
    {
        if self.filters[self.currentFilter.index].summaryFilter.pattern == text {
            self.currentFilter.summaryFilterChanges.pattern = None;
        } else {
            self.currentFilter.summaryFilterChanges.pattern = Some(text.into());
        }
    }

    fn onFilterNameChosen(&mut self, name: &str)
    {
        match self.filters.iter().find_position(|filter| filter.name == name)
        {
            Some((index, _filter)) => {
                let isFilterSwitchNeeded = self.currentFilter.index != index;
                self.updateFilter(index);
                if isFilterSwitchNeeded {
                    self.notifyActiveFilterSwitched(index);
                }
                self.notifyFiltersUpdated();
            },
            None => {
                self.addFilter(name);
                self.notifyFilterAdded(name);
                self.notifyFiltersUpdated();
            }
        }
    }

    fn addFilter(&mut self, name: &str)
    {
        let newFilter = CommitLogFilter{
            name: name.into(),
            summaryFilter: self.mergeOriginalAndChangedSummaryFilter(),
            authorFilter: self.mergeOriginalAndChangedAuthorFilter()};
        self.filters.push(newFilter);
        self.currentFilter = CurrentFilter::new(self.filters.len()-1);
    }

    fn mergeOriginalAndChangedSummaryFilter(&self) -> SummaryFilter
    {
        let baseFilter = &self.filters[self.currentFilter.index].summaryFilter;
        let filterChanges = &self.currentFilter.summaryFilterChanges;
        SummaryFilter{
            pattern: filterChanges.pattern.as_ref().unwrap_or(&baseFilter.pattern).into(),
            caseSensitive: filterChanges.caseSensitive.unwrap_or(baseFilter.caseSensitive),
            usesRegex: filterChanges.usesRegex.unwrap_or(baseFilter.usesRegex)
        }
    }

    fn mergeOriginalAndChangedAuthorFilter(&self) -> AuthorFilter
    {
        let baseFilter = &self.filters[self.currentFilter.index].authorFilter;
        let filterChanges = &self.currentFilter.authorFilterChanges;
        AuthorFilter{
            pattern: filterChanges.pattern.as_ref().unwrap_or(&baseFilter.pattern).into(),
            caseSensitive: filterChanges.caseSensitive.unwrap_or(baseFilter.caseSensitive),
            usesRegex: filterChanges.usesRegex.unwrap_or(baseFilter.usesRegex)
        }
    }

    fn updateFilter(&mut self, index: FilterIndex)
    {
        self.filters[index].summaryFilter = self.mergeOriginalAndChangedSummaryFilter();
        self.filters[index].authorFilter = self.mergeOriginalAndChangedAuthorFilter();
        self.currentFilter = CurrentFilter::new(index);
    }

    fn notifyActiveFilterSwitched(&self, index: FilterIndex)
    {
        self.sender.send((Source::CommitLogFilters, Event::ActiveFilterSwitched(index))).unwrap();
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

    fn notifyActiveFilterDataSwitched(&self)
    {
        self.sender.send((
            Source::CommitLogFilters,
            Event::ActiveFilterDataSwitched(self.filters[self.currentFilter.index].authorFilter.clone())))
            .unwrap();
    }
}

struct CurrentFilter
{
    index: usize,
    summaryFilterChanges: SummaryFilterChanges,
    authorFilterChanges: AuthorFilterChanges,
}

impl CurrentFilter
{
    fn new(index: FilterIndex) -> Self
    {
        Self{index, summaryFilterChanges: SummaryFilterChanges::new(), authorFilterChanges: AuthorFilterChanges::new()}
    }
}

type SummaryFilterChanges = TextFilterChanges;
type AuthorFilterChanges = TextFilterChanges;

struct TextFilterChanges
{
    pattern: Option<String>,
    caseSensitive: Option<bool>,
    usesRegex: Option<bool>
}

impl TextFilterChanges
{
    fn new() -> Self
    {
        Self{pattern: None, caseSensitive: None, usesRegex: None}
    }
}
