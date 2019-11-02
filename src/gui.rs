use crate::application_window::ApplicationWindow;
use crate::commit_button::CommitButton;
use crate::commit_message_view::CommitMessageView;
use crate::diff_view::DiffView;
use crate::file_change::FileChange;
use crate::file_changes_view::FileChangesView;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
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
    applicationWindow: ApplicationWindow
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

        Self{
            unstagedChangesView: Rc::clone(&unstagedChangesView),
            stagedChangesView: Rc::clone(&stagedChangesView),
            diffView,
            commitMessageView,
            commitButton,
            applicationWindow: ApplicationWindow::new(&guiElementProvider)
        }
    }

    pub fn show(&self)
    {
        self.applicationWindow.show();
    }
}

fn makeOnOtherViewSelectedReaction<StoreType>(stagedChangesView: &Rc<RefCell<FileChangesView<StoreType>>>)
    -> Box<dyn Fn(FileChange) -> glib::Continue>
    where StoreType: 'static
{
    let weakView = Rc::downgrade(stagedChangesView);
    Box::new(move |_fileChange| {
        if let Some(rcView) = weakView.upgrade() {
            rcView.borrow().unselectAll();
        }
        glib::Continue(true)
    })
}