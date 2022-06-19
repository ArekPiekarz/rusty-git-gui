use crate::commit_message::CommitMessage;
use crate::file_change::{FileChange, FileChangeUpdate};

use gtk::{gdk, glib};


#[derive(Debug)]
pub enum Event
{
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

    // stack
    StackChildChanged(String),

    // text entry
    TextEntered(String),
    InvalidTextInputted,
    ValidTextInputted,
}

type IsEnabled = bool;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source
{
    CommitAmendCheckbox,
    CommitButton,
    CommitDiffViewWidget,
    CommitLogAuthorFilterEntry,
    CommitLogAuthorFilterRegexButton,
    CommitLogModelFilter,
    CommitLogView,
    CommitLogViewWidget,
    CommitMessageView,
    DiffView,
    MainStack,
    RefreshButton,
    Repository,
    ShowCommitLogFiltersButton,
    StagedChangesStore,
    StagedChangesView,
    UnstagedChangesStore,
    UnstagedChangesView
}

pub trait IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event);
}

pub type Sender = glib::Sender<(Source, Event)>;
pub type Receiver = glib::Receiver<(Source, Event)>;

#[track_caller]
pub fn handleUnknown(source: Source, event: &Event)
{
    panic!("Unknown source and event: {:?}, {:?}", source, event);
}
