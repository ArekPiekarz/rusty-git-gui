use crate::application_window::ApplicationWindow;
use crate::commit_button::CommitButton;
use crate::commit_message_view::CommitMessageView;
use crate::diff_view::DiffView;
use crate::file_changes_view_observer::FileChangesViewObserver;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes_view::{makeStagedChangesView, StagedChangesView};
use crate::unstaged_changes_view::{makeUnstagedChangesView, UnstagedChangesView};

use std::rc::Rc;


pub struct Gui
{
    applicationWindow: ApplicationWindow,
    pub unstagedChangesView: Rc<UnstagedChangesView>,
    pub stagedChangesView: Rc<StagedChangesView>,
    pub diffView: DiffView,
    pub commitMessageView: Rc<CommitMessageView>,
    pub commitButton: Rc<CommitButton>
}

impl Gui
{
    pub fn new(repository: Rc<Repository>) -> Self
    {
        let guiElementProvider = GuiElementProvider::new(include_str!("main_window.glade"));
        let fileChanges = repository.getFileChanges();

        let unstagedChangesView = makeUnstagedChangesView(
            &guiElementProvider, &fileChanges.unstaged, Rc::clone(&repository));
        let stagedChangesView = makeStagedChangesView(
            &guiElementProvider, &fileChanges.staged, Rc::clone(&repository));

        unstagedChangesView.connectOnSelected(
            Rc::downgrade(&(stagedChangesView.clone() as Rc<dyn FileChangesViewObserver>)));
        stagedChangesView.connectOnSelected(
            Rc::downgrade(&(unstagedChangesView.clone() as Rc<dyn FileChangesViewObserver>)));

        let commitMessageView = CommitMessageView::new(&guiElementProvider, &repository);
        let commitButton = CommitButton::new(
            &guiElementProvider, Rc::clone(&commitMessageView), Rc::clone(&repository));

        Self{
            applicationWindow: ApplicationWindow::new(&guiElementProvider),
            unstagedChangesView: Rc::clone(&unstagedChangesView),
            stagedChangesView: Rc::clone(&stagedChangesView),
            diffView: DiffView::new(
                &guiElementProvider, &unstagedChangesView, &stagedChangesView, Rc::clone(&repository)),
            commitMessageView,
            commitButton
        }
    }

    pub fn show(&self)
    {
        self.applicationWindow.show();
    }
}