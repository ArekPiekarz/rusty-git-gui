use crate::commit_message::CommitMessage;
use crate::file_change::{FileChange, FileChangeUpdate};


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

    // checkbox
    Toggled,

    // commit amend mode
    CommitAmendEnabled,
    CommitAmendDisabled,

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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source
{
    CommitAmendCheckbox,
    CommitButton,
    CommitMessageView,
    DiffView,
    RefreshButton,
    Repository,
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
    panic!("Unknown source and event: {:?}, {:?}", source, event)
}