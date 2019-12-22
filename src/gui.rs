use crate::application_window::ApplicationWindow;
use crate::commit_button::CommitButton;
use crate::commit_message_view::CommitMessageView;
use crate::diff_and_commit_paned::setupDiffAndCommitPaned;
use crate::diff_view::DiffView;
use crate::file_change::FileChange;
use crate::file_changes_paned::setupFileChangesPaned;
use crate::file_changes_view::FileChangesView;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::IFileChangesStore;
use crate::main_paned::setupMainPaned;
use crate::refresh_button::RefreshButton;
use crate::repository::Repository;
use crate::settings::Settings;
use crate::staged_changes_view::{makeStagedChangesView, StagedChangesView};
use crate::unstaged_changes_view::{makeUnstagedChangesView, UnstagedChangesView};

use std::cell::RefCell;
use std::rc::Rc;


pub struct Gui
{
    pub unstagedChangesView: Rc<RefCell<UnstagedChangesView>>,
    pub stagedChangesView: Rc<RefCell<StagedChangesView>>,
    pub diffView: Rc<RefCell<DiffView>>,
    pub commitMessageView: Rc<RefCell<CommitMessageView>>,
    pub commitButton: Rc<RefCell<CommitButton>>,
    pub refreshButton: Rc<RefreshButton>,
    applicationWindow: Rc<ApplicationWindow>
}

impl Gui
{
    pub fn new(repository: &Rc<RefCell<Repository>>) -> Self
    {
        let guiElementProvider = GuiElementProvider::new(include_str!("main_window.glade"));
        let unstagedChangesView = makeUnstagedChangesView(&guiElementProvider, Rc::clone(repository));
        let stagedChangesView = makeStagedChangesView(&guiElementProvider, Rc::clone(repository));

        unstagedChangesView.borrow_mut().connectOnSelected(makeOnOtherViewSelectedReaction(&stagedChangesView));
        stagedChangesView.borrow_mut().connectOnSelected(makeOnOtherViewSelectedReaction(&unstagedChangesView));

        let diffView = DiffView::new(
            &guiElementProvider,
            &mut unstagedChangesView.borrow_mut(),
            &mut stagedChangesView.borrow_mut(),
            Rc::clone(repository));

        let commitMessageView = CommitMessageView::new(&guiElementProvider, &mut repository.borrow_mut());
        let commitButton = CommitButton::new(
            &guiElementProvider, Rc::clone(&commitMessageView), Rc::clone(repository));

        let refreshButton = RefreshButton::new(&guiElementProvider, Rc::clone(repository));

        let mut settings = Settings::new();
        setupPanes(&guiElementProvider, &mut settings);
        showFirstFileChange(&unstagedChangesView);

        Self{
            unstagedChangesView: Rc::clone(&unstagedChangesView),
            stagedChangesView: Rc::clone(&stagedChangesView),
            diffView,
            commitMessageView,
            commitButton,
            refreshButton,
            applicationWindow: ApplicationWindow::new(&guiElementProvider, settings),
        }
    }

    pub fn show(&self)
    {
        self.applicationWindow.show();
    }
}

fn makeOnOtherViewSelectedReaction<StoreType>(fileChangesView: &Rc<RefCell<FileChangesView<StoreType>>>)
    -> Box<dyn Fn(FileChange) -> glib::Continue>
    where StoreType: IFileChangesStore + 'static
{
    let weakView = Rc::downgrade(fileChangesView);
    Box::new(move |_fileChange| {
        if let Some(rcView) = weakView.upgrade() {
            rcView.borrow().unselectAll();
        }
        glib::Continue(true)
    })
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
    if view.isFilled() {
        view.trySelectFirst();
    }
}