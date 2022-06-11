use crate::commit_amend_checkbox::CommitAmendCheckbox;
use crate::application_window::ApplicationWindow;
use crate::commit_button::CommitButton;
use crate::commit_diff_view::CommitDiffView;
use crate::commit_log::CommitLog;
use crate::commit_log_author_filter_entry::{setupCommitLogAuthorFilterEntry, setupCommitLogAuthorFilterRegexButton};
use crate::commit_log_filters_view::CommitLogFiltersView;
use crate::commit_log_model::CommitLogModel;
use crate::commit_log_model_filter::CommitLogModelFilter;
use crate::commit_log_view::CommitLogView;
use crate::commit_message_reader::CommitMessageReader;
use crate::commit_message_view::CommitMessageView;
use crate::diff_and_commit_paned::setupDiffAndCommitPaned;
use crate::diff_view::DiffView;
use crate::event::{Event, handleUnknown, IEventHandler, Receiver, Sender, Source};
use crate::file_changes_paned::setupFileChangesPaned;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::attach;
use crate::main_paned::setupMainPaned;
use crate::main_stack::setupMainStack;
use crate::refresh_button::RefreshButton;
use crate::repository::Repository;
use crate::settings::Settings;
use crate::show_commit_log_filters_button::setupShowCommitLogFiltersButton;
use crate::staged_changes_store::StagedChangesStore;
use crate::staged_changes_view::{makeStagedChangesView, StagedChangesView};
use crate::tool_bar_stack::ToolBarStack;
use crate::unstaged_changes_store::UnstagedChangesStore;
use crate::unstaged_changes_view::{makeUnstagedChangesView, UnstagedChangesView};

use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;


pub struct Gui
{
    applicationWindow: Rc<ApplicationWindow>,
}

struct GuiObjects
{
    toolBarStack: ToolBarStack,
    unstagedChangesView: UnstagedChangesView,
    stagedChangesView: StagedChangesView,
    diffView: DiffView,
    refreshButton: RefreshButton,
    commitMessageView: CommitMessageView,
    commitButton: CommitButton,
    commitAmendCheckbox: CommitAmendCheckbox,
    unstagedChangesStore: Rc<RefCell<UnstagedChangesStore>>,
    stagedChangesStore: Rc<RefCell<StagedChangesStore>>,
    commitLogFiltersView: CommitLogFiltersView,
    commitLogModelFilter: CommitLogModelFilter,
    commitLogView: CommitLogView,
    commitDiffView: CommitDiffView,
}

impl Gui
{
    pub fn new(repository: Rc<RefCell<Repository>>, sender: Sender, receiver: Receiver) -> Self
    {
        let guiElementProvider = GuiElementProvider::new(include_str!("main_window.glade"));

        let unstagedChangesStore = Rc::new(RefCell::new(UnstagedChangesStore::new(
            &guiElementProvider, sender.clone(), &repository)));
        let unstagedChangesView = makeUnstagedChangesView(
            &guiElementProvider, sender.clone(), Rc::clone(&unstagedChangesStore));
        let stagedChangesStore = Rc::new(RefCell::new(StagedChangesStore::new(
            &guiElementProvider, sender.clone(), &repository)));
        let stagedChangesView = makeStagedChangesView(
            &guiElementProvider, sender.clone(), Rc::clone(&stagedChangesStore));
        let diffView = DiffView::new(
            &guiElementProvider, Rc::clone(&repository), sender.clone());
        let refreshButton = RefreshButton::new(&guiElementProvider, sender.clone());
        let commitAmendCheckbox = CommitAmendCheckbox::new(
            &guiElementProvider, &mut repository.borrow_mut(), sender.clone());
        let commitMessageView = CommitMessageView::new(
            &guiElementProvider, Rc::clone(&repository), sender.clone());
        let commitMessageReader = CommitMessageReader::new(&guiElementProvider);
        let commitButton = CommitButton::new(
            &guiElementProvider, commitMessageReader, Rc::clone(&repository), sender.clone());

        let commitLogFiltersView = CommitLogFiltersView::new(&guiElementProvider);
        let commitLogModelFilter = CommitLogModelFilter::new(&guiElementProvider, sender.clone());
        let commitLog = CommitLog::new(&repository.borrow());
        let _commitLogModel = CommitLogModel::new(&commitLog, &guiElementProvider);
        let commitLogView = CommitLogView::new(commitLog, &guiElementProvider, sender.clone());
        let commitDiffView = CommitDiffView::new(Rc::clone(&repository), &guiElementProvider, sender.clone());

        setupMainStack(&guiElementProvider, sender.clone());
        let toolBarStack = ToolBarStack::new(&guiElementProvider);

        setupShowCommitLogFiltersButton(&guiElementProvider, sender.clone());
        setupCommitLogAuthorFilterEntry(&guiElementProvider, sender.clone());
        setupCommitLogAuthorFilterRegexButton(&guiElementProvider, sender);

        let mut settings = Settings::new();
        setupPanes(&guiElementProvider, &mut settings);
        let applicationWindow = ApplicationWindow::new(&guiElementProvider, settings);
        showFirstFileChange(&unstagedChangesView);

        let newSelf = Self{applicationWindow};
        let guiObjects = GuiObjects{
            unstagedChangesView,
            stagedChangesView,
            diffView,
            refreshButton,
            commitMessageView,
            commitButton,
            commitAmendCheckbox,
            unstagedChangesStore,
            stagedChangesStore,
            commitLogFiltersView,
            commitLogModelFilter,
            commitLogView,
            commitDiffView,
            toolBarStack
        };
        setupDispatching(guiObjects, repository, receiver);
        newSelf
    }

    pub fn show(&self)
    {
        self.applicationWindow.show();
    }

    pub fn setOpacity(&self, value: f64)
    {
        self.applicationWindow.setOpacity(value);
    }
}

#[allow(clippy::items_after_statements)]
#[allow(clippy::match_same_arms)]
fn setupDispatching(gui: GuiObjects, mut repository: Rc<RefCell<Repository>>, receiver: Receiver)
{
    let mut toolBarStack = gui.toolBarStack;
    let mut unstagedChangesView = gui.unstagedChangesView;
    let mut stagedChangesView = gui.stagedChangesView;
    let mut diffView = gui.diffView;
    let mut refreshButton = gui.refreshButton;
    let mut commitMessageView = gui.commitMessageView;
    let mut commitButton = gui.commitButton;
    let mut commitAmendCheckbox = gui.commitAmendCheckbox;
    let mut unstagedChangesStore = Rc::clone(&gui.unstagedChangesStore);
    let mut stagedChangesStore = Rc::clone(&gui.stagedChangesStore);
    let mut commitLogFiltersView = gui.commitLogFiltersView;
    let mut commitLogModelFilter = gui.commitLogModelFilter;
    let mut commitLogView = gui.commitLogView;
    let mut commitDiffView = gui.commitDiffView;

    use Source as S;
    use Event as E;
    attach(receiver, move |(source, event)| { match (source, &event) {
        (S::CommitAmendCheckbox,              E::CommitAmendDisabled)     => (&repository, &mut commitMessageView, &mut commitButton, &mut diffView).handle(source, &event),
        (S::CommitAmendCheckbox,              E::CommitAmendEnabled)      => (&repository, &mut commitMessageView, &mut commitButton, &mut diffView).handle(source, &event),
        (S::CommitAmendCheckbox,              E::Toggled)                 => commitAmendCheckbox.handle(source, &event),
        (S::CommitButton,                     E::AmendCommitRequested(_)) => repository.handle(source, &event),
        (S::CommitButton,                     E::Clicked)                 => commitButton.handle(source, &event),
        (S::CommitButton,                     E::CommitRequested(_))      => repository.handle(source, &event),
        (S::CommitDiffViewWidget,             E::ZoomRequested(_))        => commitDiffView.handle(source, &event),
        (S::CommitLogAuthorFilterEntry,       E::TextEntered(_))          => commitLogModelFilter.handle(source, &event),
        (S::CommitLogAuthorFilterRegexButton, E::Toggled)                 => commitLogModelFilter.handle(source, &event),
        (S::CommitLogModelFilter,             E::RefilterRequested)       => (&mut commitLogView, &mut commitLogModelFilter).handle(source, &event),
        (S::CommitLogModelFilter,             E::RefilterEnded)           => commitLogView.handle(source, &event),
        (S::CommitLogView,                    E::CommitSelected(_))       => commitDiffView.handle(source, &event),
        (S::CommitLogView,                    E::CommitUnselected)        => commitDiffView.handle(source, &event),
        (S::CommitLogViewWidget,              E::RightClicked(_))         => (),
        (S::CommitLogViewWidget,              E::RowActivated(_))         => (),
        (S::CommitLogViewWidget,              E::SelectionChanged(_))     => commitLogView.handle(source, &event),
        (S::CommitMessageView,                E::BufferChanged)           => commitMessageView.handle(source, &event),
        (S::CommitMessageView,                E::Emptied)                 => commitButton.handle(source, &event),
        (S::CommitMessageView,                E::Filled)                  => commitButton.handle(source, &event),
        (S::CommitMessageView,                E::ZoomRequested(_))        => commitMessageView.handle(source, &event),
        (S::DiffView,                         E::ZoomRequested(_))        => diffView.handle(source, &event),
        (S::MainStack,                        E::StackChildChanged(_))    => toolBarStack.handle(source, &event),
        (S::RefreshButton,                    E::Clicked)                 => refreshButton.handle(source, &event),
        (S::RefreshButton,                    E::RefreshRequested)        => repository.handle(source, &event),
        (S::Repository,                       E::AddedToStaged(_))        => (&stagedChangesStore, &mut commitButton).handle(source, &event),
        (S::Repository,                       E::AddedToUnstaged(_))      => unstagedChangesStore.handle(source, &event),
        (S::Repository,                       E::AmendedCommit)           => (&stagedChangesStore, &mut commitAmendCheckbox).handle(source, &event),
        (S::Repository,                       E::Committed)               => (&stagedChangesStore, &mut commitMessageView, &mut commitAmendCheckbox).handle(source, &event),
        (S::Repository,                       E::RemovedFromStaged(_))    => (&stagedChangesStore, &mut commitButton).handle(source, &event),
        (S::Repository,                       E::RemovedFromUnstaged(_))  => unstagedChangesStore.handle(source, &event),
        (S::Repository,                       E::Refreshed)               => (&unstagedChangesStore, &stagedChangesStore).handle(source, &event),
        (S::Repository,                       E::UpdatedInStaged(_))      => stagedChangesStore.handle(source, &event),
        (S::Repository,                       E::UpdatedInUnstaged(_))    => unstagedChangesStore.handle(source, &event),
        (S::ShowCommitLogFiltersButton,       E::Toggled)                 => commitLogFiltersView.handle(source, &event),
        (S::StagedChangesStore,               E::Refreshed)               => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,                E::FileChangeRefreshed(_))  => diffView.handle(source, &event),
        (S::StagedChangesView,                E::FileChangeSelected(_))   => (&mut diffView, &mut unstagedChangesView).handle(source, &event),
        (S::StagedChangesView,                E::FileChangeUnselected)    => diffView.handle(source, &event),
        (S::StagedChangesView,                E::RightClicked(_))         => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,                E::RowActivated(_))         => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,                E::SelectionChanged(_))     => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,                E::UnstageRequested(_))     => repository.handle(source, &event),
        (S::UnstagedChangesStore,             E::Refreshed)               => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,              E::FileChangeRefreshed(_))  => diffView.handle(source, &event),
        (S::UnstagedChangesView,              E::FileChangeSelected(_))   => (&mut diffView, &mut stagedChangesView).handle(source, &event),
        (S::UnstagedChangesView,              E::FileChangeUnselected)    => diffView.handle(source, &event),
        (S::UnstagedChangesView,              E::RightClicked(_))         => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,              E::RowActivated(_))         => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,              E::SelectionChanged(_))     => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,              E::StageRequested(_))       => repository.handle(source, &event),
        (source, event) => handleUnknown(source, event) }

        glib::Continue(true)
    });
}

fn setupPanes(guiElementProvider: &GuiElementProvider, settings: &mut Settings)
{
    setupMainPaned(guiElementProvider, settings);
    setupFileChangesPaned(guiElementProvider, settings);
    setupDiffAndCommitPaned(guiElementProvider, settings);
}

fn showFirstFileChange(unstagedChangesView: &UnstagedChangesView)
{
    unstagedChangesView.focus();
    unstagedChangesView.trySelectFirst();
}

impl<T0, T1> IEventHandler for (T0, T1)
    where T0: IEventHandler, T1: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
    }
}

impl<T0, T1, T2> IEventHandler for (T0, T1, T2)
    where T0: IEventHandler, T1: IEventHandler, T2: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
        self.2.handle(source, event);
    }
}

impl<T0, T1, T2, T3> IEventHandler for (T0, T1, T2, T3)
    where T0: IEventHandler, T1: IEventHandler, T2: IEventHandler, T3: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
        self.2.handle(source, event);
        self.3.handle(source, event);
    }
}

impl<T> IEventHandler for Rc<RefCell<T>>
    where T: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.borrow_mut().handle(source, event);
    }
}

impl<T> IEventHandler for &Rc<RefCell<T>>
    where T: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.borrow_mut().handle(source, event);
    }
}

impl<T> IEventHandler for &mut T
    where T: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        (*self).handle(source, event);
    }
}
