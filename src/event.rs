use crate::commit_message::CommitMessage;
use crate::config::{AuthorFilter, CommitLogFilters};
use crate::file_change::{FileChange, FileChangeUpdate};
use crate::pane::PanePosition;

use gtk::{gdk, glib};


#[derive(Debug)]
pub(crate) enum Event
{
    // application window
    MaximizationChanged(IsMaximized),
    QuitRequested,

    // repository
    AddedToStaged(FileChange),
    AddedToUnstaged(FileChange),
    AmendedCommit,
    Committed,
    Refreshed,
    RemovedFromStaged(FileChange),
    RemovedFromUnstaged(FileChange),
    UpdatedInStaged(FileChangeUpdate),
    UpdatedInUnstaged(FileChangeUpdate),

    // requests to repository
    AmendCommitRequested(CommitMessage),
    CommitRequested(CommitMessage),
    RefreshRequested,
    StageRequested(FileChange),
    UnstageRequested(FileChange),

    // button
    Clicked,

    // checkbox, tool button
    Toggled(IsEnabled),

    // commit amend mode
    CommitAmendEnabled,
    CommitAmendDisabled,

    // commit log
    CommitSelected(git2::Oid),
    CommitUnselected,

    // text view
    BufferChanged,
    Filled,
    Emptied,
    ZoomRequested(gdk::EventScroll),

    // file changes view
    FileChangeSelected(FileChange),
    FileChangeUnselected,
    FileChangeRefreshed(Option<FileChange>),

    // tree view
    RowActivated(gtk::TreePath),
    RightClicked(gdk::EventButton),

    // tree selection
    SelectionChanged(gtk::TreeSelection),

    // model filter
    RefilterRequested,
    RefilterEnded,

    // pane
    PositionChanged(PanePosition),

    // stack
    ActivePageChanged(String),

    // text entry
    TextEntered(String),
    InvalidTextInputted(regex::Error),
    ValidTextInputted,

    // commit log filters
    ActiveFilterChosen(FilterIndex),
    ActiveFilterDataSwitched(AuthorFilter),
    ActiveFilterSwitched(FilterIndex),
    FilterNameChosen(String),
    FilterAdded(String),
    FiltersUpdated(CommitLogFilters),
    OpenDialogRequested,
    DialogResponded(gtk::ResponseType),

    // commit log filters text entries
    InvalidSummaryTextInputted(regex::Error),
    InvalidAuthorTextInputted(regex::Error),
    ValidSummaryTextInputted,
    ValidAuthorTextInputted,
}

type IsEnabled = bool;
type IsMaximized = bool;
pub(crate) type FilterIndex = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Source
{
    ApplicationWindow,
    CommitAmendCheckbox,
    CommitButton,
    CommitDiffViewWidget,
    CommitLogAuthorFilterEntry,
    CommitLogAuthorFilterCaseButton,
    CommitLogAuthorFilterRegexButton,
    CommitLogFilters,
    CommitLogFiltersComboBox,
    CommitLogModelFilter,
    CommitLogSaveFilterButton,
    CommitLogSaveFilterDialog,
    CommitLogSaveFilterDialogWidget,
    CommitLogShowFilterButton,
    CommitLogSummaryFilterCaseButton,
    CommitLogSummaryFilterEntry,
    CommitLogSummaryFilterRegexButton,
    CommitLogView,
    CommitLogViewWidget,
    CommitMessageView,
    DiffAndCommitPane,
    DiffView,
    FileChangesPane,
    MainPane,
    MainStack,
    RefreshButton,
    Repository,
    StagedChangesStore,
    StagedChangesView,
    UnstagedChangesStore,
    UnstagedChangesView
}

pub(crate) trait IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event);
}

pub(crate) type Sender = glib::Sender<(Source, Event)>;
pub(crate) type Receiver = glib::Receiver<(Source, Event)>;

#[track_caller]
pub(crate) fn handleUnknown(source: Source, event: &Event)
{
    panic!("Unknown source and event: {:?}, {:?}", source, event);
}
