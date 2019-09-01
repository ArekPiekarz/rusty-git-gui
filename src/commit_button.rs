use crate::commit_message_view::CommitMessageView;
use crate::commit_message_view_observer::CommitMessageViewObserver;
use crate::file_change::FileChange;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;

use gtk::ButtonExt as _;
use gtk::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitButton
{
    widget: gtk::Button,
    repository: Rc<Repository>,
    commitMessageView: Rc<CommitMessageView>,
    areChangesStaged: RefCell<bool>,
    isCommitMessageWritten: RefCell<bool>
}

impl CommitButton
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        commitMessageView: Rc<CommitMessageView>,
        repository: Rc<Repository>)
        -> Rc<Self>
    {
        let isCommitMessageWritten = RefCell::new(commitMessageView.hasText());
        let newSelf = Rc::new(Self{
            widget: guiElementProvider.get::<gtk::Button>("Commit button"),
            repository: Rc::clone(&repository),
            commitMessageView,
            areChangesStaged: RefCell::new(repository.hasStagedChanges()),
            isCommitMessageWritten
        });
        newSelf.connectSelfToRepository(&repository);
        newSelf.connectSelfToCommitMessageView();
        newSelf.connectSelfToWidget();
        newSelf.update();
        newSelf
    }

    pub fn isEnabled(&self) -> bool
    {
        self.widget.is_sensitive()
    }

    pub fn isDisabled(&self) -> bool
    {
        !self.isEnabled()
    }

    pub fn getTooltip(&self) -> String
    {
        match self.widget.get_tooltip_text() {
            Some(text) => text.into(),
            None => "".into()
        }
    }

    pub fn click(&self)
    {
        self.widget.clicked();
    }


    // private

    fn connectSelfToRepository(self: &Rc<Self>, repository: &Repository)
    {
        repository.connectOnStaged(Rc::downgrade(&(self.clone() as Rc<dyn RepositoryObserver>)));
        repository.connectOnUnstaged(Rc::downgrade(&(self.clone() as Rc<dyn RepositoryObserver>)));
    }

    fn connectSelfToCommitMessageView(self: &Rc<Self>)
    {
        self.commitMessageView.connectOnFilled(Rc::downgrade(&(self.clone() as Rc<dyn CommitMessageViewObserver>)));
        self.commitMessageView.connectOnEmptied(Rc::downgrade(&(self.clone() as Rc<dyn CommitMessageViewObserver>)));
    }

    fn connectSelfToWidget(self: &Rc<Self>)
    {
        let weakSelf = Rc::downgrade(&self);
        self.widget.connect_clicked(move |_button| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.commit();
            }
        });
    }

    fn update(&self)
    {
        if self.noChangesAreStaged() {
            self.disable();
            self.setTooltip("No changes are staged for commit.");
            return;
        }

        if self.commitMessageIsEmpty() {
            self.disable();
            self.setTooltip("The commit message is empty.");
            return;
        }

        self.enable();
        self.clearTooltip();
    }

    fn noChangesAreStaged(&self) -> bool
    {
        !*self.areChangesStaged.borrow()
    }

    fn commitMessageIsEmpty(&self) -> bool
    {
        !*self.isCommitMessageWritten.borrow()
    }

    fn enable(&self)
    {
        self.widget.set_sensitive(true);
    }

    fn disable(&self)
    {
        self.widget.set_sensitive(false);
    }

    fn setTooltip(&self, text: &str)
    {
        self.widget.set_tooltip_text(Some(text));
    }

    fn clearTooltip(&self)
    {
        self.widget.set_tooltip_text(None);
    }

    fn commit(&self)
    {
        self.repository.commit(&self.commitMessageView.getText());
        *self.areChangesStaged.borrow_mut() = false;
        self.update();
    }
}

impl RepositoryObserver for CommitButton
{
    fn onStaged(&self, _: &FileChange)
    {
        if *self.areChangesStaged.borrow() {
            return;
        }
        *self.areChangesStaged.borrow_mut() = true;
        self.update();
    }

    fn onUnstaged(&self, _: &FileChange)
    {
        if self.repository.hasStagedChanges() {
            return;
        }
        *self.areChangesStaged.borrow_mut() = false;
        self.update();
    }
}

impl CommitMessageViewObserver for CommitButton
{
    fn onFilled(&self)
    {
        *self.isCommitMessageWritten.borrow_mut() = true;
        self.update();
    }

    fn onEmptied(&self)
    {
        *self.isCommitMessageWritten.borrow_mut() = false;
        self.update();
    }
}