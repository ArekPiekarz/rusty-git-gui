use crate::commit_amend_checkbox::CommitAmendCheckbox;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::text_view::{Notifications, TextView};

use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitMessageView
{
    widget: Rc<RefCell<TextView>>,
    repository: Rc<RefCell<Repository>>,
    stashedMessage: String
}

impl CommitMessageView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        repository: &Rc<RefCell<Repository>>,
        commitAmendCheckbox: &mut CommitAmendCheckbox)
        -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{
            widget: TextView::new(guiElementProvider, "Commit message view", Notifications::Enabled),
            repository: Rc::clone(repository),
            stashedMessage: "".into()
        }));
        Self::connectSelfToRepository(&newSelf, &mut repository.borrow_mut());
        Self::connectSelfToCommitAmendCheckbox(&newSelf, commitAmendCheckbox);
        newSelf
    }

    pub fn hasText(&self) -> bool
    {
        !self.getText().is_empty()
    }

    pub fn isEmpty(&self) -> bool
    {
        !self.hasText()
    }

    pub fn getText(&self) -> String
    {
        self.widget.borrow().getText()
    }

    pub fn setText(&self, text: &str)
    {
        self.widget.borrow().setText(text);
    }

    pub fn connectOnFilled(&self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        self.widget.borrow_mut().connectOnFilled(handler);
    }

    pub fn connectOnEmptied(&self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        self.widget.borrow_mut().connectOnEmptied(handler);
    }


    // private

    fn connectSelfToRepository(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnCommitted(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow().onCommitted();
            }
            glib::Continue(true)
        }))
    }

    fn connectSelfToCommitAmendCheckbox(rcSelf: &Rc<RefCell<Self>>, commitAmendCheckbox: &mut CommitAmendCheckbox)
    {
        Self::connectSelfToCommitAmendEnabled(rcSelf, commitAmendCheckbox);
        Self::connectSelfToCommitAmendDisabled(rcSelf, commitAmendCheckbox);
    }

    fn connectSelfToCommitAmendEnabled(rcSelf: &Rc<RefCell<Self>>, commitAmendCheckbox: &mut CommitAmendCheckbox)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        commitAmendCheckbox.connectOnSelected(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onCommitAmendEnabled();
            }
            glib::Continue(true)
        }))
    }

    fn connectSelfToCommitAmendDisabled(rcSelf: &Rc<RefCell<Self>>, commitAmendCheckbox: &mut CommitAmendCheckbox)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        commitAmendCheckbox.connectOnUnselected(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onCommitAmendDisabled();
            }
            glib::Continue(true)
        }))
    }

    fn onCommitted(&self)
    {
        self.widget.borrow().clear();
    }

    fn onCommitAmendEnabled(&mut self)
    {
        self.stashedMessage = self.getText();
        self.setText(&self.repository.borrow().getLastCommitMessage().unwrap().unwrap());
    }

    fn onCommitAmendDisabled(&mut self)
    {
        self.setText(&self.stashedMessage);
    }
}