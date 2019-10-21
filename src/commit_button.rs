use crate::commit_message_view::CommitMessageView;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::repository::Repository;

use gtk::ButtonExt as _;
use gtk::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitButton
{
    widget: gtk::Button,
    repository: Rc<RefCell<Repository>>,
    commitMessageView: Rc<RefCell<CommitMessageView>>,
    areChangesStaged: bool,
    isCommitMessageWritten: bool
}

impl CommitButton
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        commitMessageView: Rc<RefCell<CommitMessageView>>,
        repository: Rc<RefCell<Repository>>)
        -> Rc<RefCell<Self>>
    {
        let isCommitMessageWritten = commitMessageView.borrow().hasText();
        let newSelf = Rc::new(RefCell::new(Self {
            widget: guiElementProvider.get::<gtk::Button>("Commit button"),
            repository: Rc::clone(&repository),
            commitMessageView,
            areChangesStaged: repository.borrow().hasStagedChanges(),
            isCommitMessageWritten
        }));
        Self::connectSelfToRepository(&newSelf, &mut repository.borrow_mut());
        Self::connectSelfToCommitMessageView(&newSelf);
        Self::connectSelfToWidget(&newSelf);
        newSelf.borrow().update();
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

    fn connectSelfToRepository(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        Self::connectSelfToRepositoryOnAddedToStaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnRemovedFromStaged(rcSelf, repository);
    }

    fn connectSelfToRepositoryOnAddedToStaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        repository.connectOnAddedToStaged(Box::new(move |_fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onAddedToStaged();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRemovedFromStaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        repository.connectOnRemovedFromStaged(Box::new(move |_fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onRemovedFromStaged();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToCommitMessageView(rcSelf: &Rc<RefCell<Self>>)
    {
        Self::connectSelfToCommitMessageViewOnFilled(rcSelf);
        Self::connectSelfToCommitMessageViewOnEmptied(rcSelf);
    }

    fn connectSelfToCommitMessageViewOnFilled(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        rcSelf.borrow().commitMessageView.borrow_mut().connectOnFilled(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onCommitMessageFilled();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToCommitMessageViewOnEmptied(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        rcSelf.borrow().commitMessageView.borrow_mut().connectOnEmptied(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onCommitMessageEmptied();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToWidget(rcSelf: &Rc<RefCell<Self>>)
    {
        let (sender, receiver) = makeChannel();
        rcSelf.borrow().widget.connect_clicked(move |_button| {
            sender.send(()).unwrap();
        });

        let weakSelf = Rc::downgrade(&rcSelf);
        attach(receiver, move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().commit();
            }
            glib::Continue(true)
        });
    }

    fn onAddedToStaged(&mut self)
    {
        if self.areChangesStaged {
            return;
        }
        self.areChangesStaged = true;
        self.update();
    }

    fn onRemovedFromStaged(&mut self)
    {
        if self.repository.borrow().hasStagedChanges() {
            return;
        }
        self.areChangesStaged = false;
        self.update();
    }

    fn onCommitMessageFilled(&mut self)
    {
        self.isCommitMessageWritten = true;
        self.update();
    }

    fn onCommitMessageEmptied(&mut self)
    {
        self.isCommitMessageWritten = false;
        self.update();
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
        !self.areChangesStaged
    }

    fn commitMessageIsEmpty(&self) -> bool
    {
        !self.isCommitMessageWritten
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

    fn commit(&mut self)
    {
        self.repository.borrow_mut().commit(&self.commitMessageView.borrow().getText());
        self.areChangesStaged = false;
        self.update();
    }
}