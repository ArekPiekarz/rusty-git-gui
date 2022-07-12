use crate::commit_amend_checkbox::CommitAmendCheckbox;
use crate::app_quitter::AppQuitter;
use crate::application_window::ApplicationWindow;
use crate::commit_button::CommitButton;
use crate::commit_diff_view::CommitDiffView;
use crate::commit_log::CommitLog;
use crate::commit_log_author_filter_entry::CommitLogAuthorFilterEntry;
use crate::commit_log_filters_view::CommitLogFiltersView;
use crate::commit_log_model::CommitLogModel;
use crate::commit_log_model_filter::CommitLogModelFilter;
use crate::commit_log_view::CommitLogView;
use crate::commit_message_reader::CommitMessageReader;
use crate::commit_message_view::CommitMessageView;
use crate::config::Config;
use crate::config_path::ConfigPath;
use crate::config_store::ConfigStore;
use crate::diff_and_commit_pane::setupDiffAndCommitPane;
use crate::diff_view::DiffView;
use crate::event::{Event, handleUnknown, IEventHandler, Receiver, Sender, Source};
use crate::file_changes_pane::setupFileChangesPane;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::main_pane::setupMainPane;
use crate::main_stack::setupMainStack;
use crate::refresh_button::RefreshButton;
use crate::repository::Repository;
use crate::show_commit_log_filters_button::setupShowCommitLogFiltersButton;
use crate::staged_changes_store::StagedChangesStore;
use crate::staged_changes_view::{makeStagedChangesView, StagedChangesView};
use crate::tool_bar_stack::ToolBarStack;
use crate::unstaged_changes_store::UnstagedChangesStore;
use crate::unstaged_changes_view::{makeUnstagedChangesView, UnstagedChangesView};

use gtk::glib;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;


pub struct Gui
{
    applicationWindow: ApplicationWindow
}

struct GuiObjects
{
    configStore: ConfigStore,
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
    commitLogAuthorFilterEntry: CommitLogAuthorFilterEntry,
    appQuitter: AppQuitter
}

impl Gui
{
    pub fn new(repositoryDir: &Path) -> Self
    {
        let configStore = ConfigStore::new(&ConfigPath::default());
        let config = configStore.getConfig();
        let (sender, receiver) = makeChannel();
        let repository = Rc::new(RefCell::new(Repository::new(repositoryDir, config, sender.clone())));
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
        let commitLogAuthorFilterEntry = CommitLogAuthorFilterEntry::new(&guiElementProvider, sender.clone());

        setupPanes(&guiElementProvider, config, sender.clone());
        let appQuitter = AppQuitter::new();
        let applicationWindow = ApplicationWindow::new(&guiElementProvider, config, sender);
        showFirstFileChange(&unstagedChangesView);

        let newSelf = Self{applicationWindow};
        let guiObjects = GuiObjects{
            configStore,
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
            toolBarStack,
            commitLogAuthorFilterEntry,
            appQuitter
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
    let mut configStore = gui.configStore;
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
    let mut commitLogAuthorFilterEntry = gui.commitLogAuthorFilterEntry;
    let mut appQuitter = gui.appQuitter;

    use Source as S;
    use Event as E;
    attach(receiver, move |(source, event)| { match (source, &event) {
        (S::ApplicationWindow,                E::MaximizationChanged(_))  => configStore.handle(source, &event),
        (S::ApplicationWindow,                E::QuitRequested)           => (&mut configStore, &mut appQuitter).handle(source, &event),
        (S::CommitAmendCheckbox,              E::CommitAmendDisabled)     => (&repository, &mut commitMessageView, &mut commitButton, &mut diffView).handle(source, &event),
        (S::CommitAmendCheckbox,              E::CommitAmendEnabled)      => (&repository, &mut commitMessageView, &mut commitButton, &mut diffView).handle(source, &event),
        (S::CommitAmendCheckbox,              E::Toggled(_))              => commitAmendCheckbox.handle(source, &event),
        (S::CommitButton,                     E::AmendCommitRequested(_)) => repository.handle(source, &event),
        (S::CommitButton,                     E::Clicked)                 => commitButton.handle(source, &event),
        (S::CommitButton,                     E::CommitRequested(_))      => repository.handle(source, &event),
        (S::CommitDiffViewWidget,             E::ZoomRequested(_))        => commitDiffView.handle(source, &event),
        (S::CommitLogAuthorFilterEntry,       E::TextEntered(_))          => commitLogModelFilter.handle(source, &event),
        (S::CommitLogAuthorFilterCaseButton,  E::Toggled(_))              => commitLogModelFilter.handle(source, &event),
        (S::CommitLogAuthorFilterRegexButton, E::Toggled(_))              => commitLogModelFilter.handle(source, &event),
        (S::CommitLogModelFilter,             E::RefilterRequested)       => (&mut commitLogView, &mut commitLogModelFilter).handle(source, &event),
        (S::CommitLogModelFilter,             E::RefilterEnded)           => commitLogView.handle(source, &event),
        (S::CommitLogModelFilter,             E::InvalidTextInputted(_))  => commitLogAuthorFilterEntry.handle(source, &event),
        (S::CommitLogModelFilter,             E::ValidTextInputted)       => commitLogAuthorFilterEntry.handle(source, &event),
        (S::CommitLogView,                    E::CommitSelected(_))       => commitDiffView.handle(source, &event),
        (S::CommitLogView,                    E::CommitUnselected)        => commitDiffView.handle(source, &event),
        (S::CommitLogViewWidget,              E::RightClicked(_))         => (),
        (S::CommitLogViewWidget,              E::RowActivated(_))         => (),
        (S::CommitLogViewWidget,              E::SelectionChanged(_))     => commitLogView.handle(source, &event),
        (S::CommitMessageView,                E::BufferChanged)           => commitMessageView.handle(source, &event),
        (S::CommitMessageView,                E::Emptied)                 => commitButton.handle(source, &event),
        (S::CommitMessageView,                E::Filled)                  => commitButton.handle(source, &event),
        (S::CommitMessageView,                E::ZoomRequested(_))        => commitMessageView.handle(source, &event),
        (S::DiffAndCommitPane,                E::PositionChanged(_))      => configStore.handle(source, &event),
        (S::DiffView,                         E::ZoomRequested(_))        => diffView.handle(source, &event),
        (S::FileChangesPane,                  E::PositionChanged(_))      => configStore.handle(source, &event),
        (S::MainPane,                         E::PositionChanged(_))      => configStore.handle(source, &event),
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
        (S::ShowCommitLogFiltersButton,       E::Toggled(_))              => commitLogFiltersView.handle(source, &event),
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

fn setupPanes(guiElementProvider: &GuiElementProvider, config: &Config, sender: Sender)
{
    setupMainPane(guiElementProvider, config, sender.clone());
    setupFileChangesPane(guiElementProvider, config, sender.clone());
    setupDiffAndCommitPane(guiElementProvider, config, sender);
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
