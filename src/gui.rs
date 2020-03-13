use crate::commit_amend_checkbox::CommitAmendCheckbox;
use crate::application_window::ApplicationWindow;
use crate::commit_button::CommitButton;
use crate::commit_message_view::CommitMessageView;
use crate::diff_and_commit_paned::setupDiffAndCommitPaned;
use crate::diff_view::DiffView;
use crate::event::{Event, handleUnknown, IEventHandler, Receiver, Sender, Source};
use crate::file_changes_paned::setupFileChangesPaned;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::attach;
use crate::main_paned::setupMainPaned;
use crate::refresh_button::RefreshButton;
use crate::repository::Repository;
use crate::settings::Settings;
use crate::staged_changes_store::StagedChangesStore;
use crate::staged_changes_view::{makeStagedChangesView, StagedChangesView};
use crate::unstaged_changes_store::UnstagedChangesStore;
use crate::unstaged_changes_view::{makeUnstagedChangesView, UnstagedChangesView};

use std::cell::RefCell;
use std::rc::Rc;


pub struct Gui
{
    pub unstagedChangesView: Rc<RefCell<UnstagedChangesView>>,
    pub stagedChangesView: Rc<RefCell<StagedChangesView>>,
    pub diffView: Rc<RefCell<DiffView>>,
    pub refreshButton: Rc<RefCell<RefreshButton>>,
    pub commitMessageView: Rc<RefCell<CommitMessageView>>,
    pub commitButton: Rc<RefCell<CommitButton>>,
    pub commitAmendCheckbox: Rc<RefCell<CommitAmendCheckbox>>,
    applicationWindow: Rc<ApplicationWindow>,
    unstagedChangesStore: Rc<RefCell<UnstagedChangesStore>>,
    stagedChangesStore: Rc<RefCell<StagedChangesStore>>
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
        let diffView = Rc::new(RefCell::new(DiffView::new(
            &guiElementProvider, Rc::clone(&repository), sender.clone())));
        let refreshButton = Rc::new(RefCell::new(RefreshButton::new(&guiElementProvider, sender.clone())));
        let commitAmendCheckbox = Rc::new(RefCell::new(CommitAmendCheckbox::new(
            &guiElementProvider, &mut repository.borrow_mut(), sender.clone())));
        let commitMessageView = Rc::new(RefCell::new(CommitMessageView::new(
            &guiElementProvider, Rc::clone(&repository), sender.clone())));
        let commitButton = Rc::new(RefCell::new(CommitButton::new(
            &guiElementProvider, Rc::clone(&commitMessageView), Rc::clone(&repository), sender)));

        let mut settings = Settings::new();
        setupPanes(&guiElementProvider, &mut settings);
        let applicationWindow = ApplicationWindow::new(&guiElementProvider, settings);
        showFirstFileChange(&unstagedChangesView);

        let newSelf = Self{
            unstagedChangesView,
            stagedChangesView,
            diffView,
            refreshButton,
            commitMessageView,
            commitButton,
            commitAmendCheckbox,
            applicationWindow,
            unstagedChangesStore,
            stagedChangesStore
        };
        setupDispatching(&newSelf, repository, receiver);
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
fn setupDispatching(gui: &Gui, mut repository: Rc<RefCell<Repository>>, receiver: Receiver)
{
    let mut unstagedChangesView = Rc::clone(&gui.unstagedChangesView);
    let mut stagedChangesView = Rc::clone(&gui.stagedChangesView);
    let mut diffView = Rc::clone(&gui.diffView);
    let mut refreshButton = Rc::clone(&gui.refreshButton);
    let mut commitMessageView = Rc::clone(&gui.commitMessageView);
    let mut commitButton = Rc::clone(&gui.commitButton);
    let mut commitAmendCheckbox = Rc::clone(&gui.commitAmendCheckbox);
    let mut unstagedChangesStore = Rc::clone(&gui.unstagedChangesStore);
    let mut stagedChangesStore = Rc::clone(&gui.stagedChangesStore);

    use Source as S;
    use Event as E;
    attach(receiver, move |(source, event)| { match (source, &event) {
        (S::CommitAmendCheckbox,  E::CommitAmendDisabled)     => (&commitMessageView, &commitButton).handle(source, &event),
        (S::CommitAmendCheckbox,  E::CommitAmendEnabled)      => (&commitMessageView, &commitButton).handle(source, &event),
        (S::CommitAmendCheckbox,  E::Toggled)                 => commitAmendCheckbox.handle(source, &event),
        (S::CommitButton,         E::AmendCommitRequested(_)) => repository.handle(source, &event),
        (S::CommitButton,         E::Clicked)                 => commitButton.handle(source, &event),
        (S::CommitButton,         E::CommitRequested(_))      => repository.handle(source, &event),
        (S::CommitMessageView,    E::BufferChanged)           => commitMessageView.handle(source, &event),
        (S::CommitMessageView,    E::Emptied)                 => commitButton.handle(source, &event),
        (S::CommitMessageView,    E::Filled)                  => commitButton.handle(source, &event),
        (S::CommitMessageView,    E::ZoomRequested(_))        => commitMessageView.handle(source, &event),
        (S::DiffView,             E::ZoomRequested(_))        => diffView.handle(source, &event),
        (S::RefreshButton,        E::Clicked)                 => refreshButton.handle(source, &event),
        (S::RefreshButton,        E::RefreshRequested)        => repository.handle(source, &event),
        (S::Repository,           E::AddedToStaged(_))        => (&stagedChangesStore, &commitButton).handle(source, &event),
        (S::Repository,           E::AddedToUnstaged(_))      => unstagedChangesStore.handle(source, &event),
        (S::Repository,           E::AmendedCommit)           => (&stagedChangesStore, &commitAmendCheckbox).handle(source, &event),
        (S::Repository,           E::Committed)               => (&stagedChangesStore, &commitMessageView).handle(source, &event),
        (S::Repository,           E::RemovedFromStaged(_))    => (&stagedChangesStore, &commitButton).handle(source, &event),
        (S::Repository,           E::RemovedFromUnstaged(_))  => unstagedChangesStore.handle(source, &event),
        (S::Repository,           E::Refreshed)               => (&unstagedChangesStore, &stagedChangesStore).handle(source, &event),
        (S::Repository,           E::UpdatedInStaged(_))      => stagedChangesStore.handle(source, &event),
        (S::Repository,           E::UpdatedInUnstaged(_))    => unstagedChangesStore.handle(source, &event),
        (S::StagedChangesStore,   E::Refreshed)               => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,    E::FileChangeRefreshed(_))  => diffView.handle(source, &event),
        (S::StagedChangesView,    E::FileChangeSelected(_))   => (&diffView, &unstagedChangesView).handle(source, &event),
        (S::StagedChangesView,    E::FileChangeUnselected)    => diffView.handle(source, &event),
        (S::StagedChangesView,    E::RightClicked(_))         => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,    E::RowActivated(_))         => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,    E::SelectionChanged(_))     => stagedChangesView.handle(source, &event),
        (S::StagedChangesView,    E::UnstageRequested(_))     => repository.handle(source, &event),
        (S::UnstagedChangesStore, E::Refreshed)               => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,  E::FileChangeRefreshed(_))  => diffView.handle(source, &event),
        (S::UnstagedChangesView,  E::FileChangeSelected(_))   => (&diffView, &stagedChangesView).handle(source, &event),
        (S::UnstagedChangesView,  E::FileChangeUnselected)    => diffView.handle(source, &event),
        (S::UnstagedChangesView,  E::RightClicked(_))         => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,  E::RowActivated(_))         => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,  E::SelectionChanged(_))     => unstagedChangesView.handle(source, &event),
        (S::UnstagedChangesView,  E::StageRequested(_))       => repository.handle(source, &event),
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

fn showFirstFileChange(unstagedChangesView: &Rc<RefCell<UnstagedChangesView>>)
{
    let view = unstagedChangesView.borrow();
    view.focus();
    view.trySelectFirst();
}

impl<T, U> IEventHandler for (T, U)
    where T: IEventHandler, U: IEventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
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