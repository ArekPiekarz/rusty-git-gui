use crate::commit_message_view_observer::CommitMessageViewObserver;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;
use crate::text_view::TextView;
use crate::text_view_observer::TextViewObserver;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;


pub struct CommitMessageView
{
    widget: Rc<TextView>,
    onFilledObservers: RefCell<Vec<Weak<dyn CommitMessageViewObserver>>>,
    onEmptiedObservers: RefCell<Vec<Weak<dyn CommitMessageViewObserver>>>
}

impl CommitMessageView
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &Repository) -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            widget: TextView::new(guiElementProvider, "Commit message view"),
            onFilledObservers: RefCell::new(vec![]),
            onEmptiedObservers: RefCell::new(vec![])
        });
        newSelf.connectSelfToWidget();
        newSelf.connectSelfToRepository(repository);
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
        self.widget.getText()
    }

    pub fn setText(&self, text: &str)
    {
        self.widget.setText(text);
    }

    pub fn connectOnFilled(&self, observer: Weak<dyn CommitMessageViewObserver>)
    {
        self.onFilledObservers.borrow_mut().push(observer);
    }

    pub fn connectOnEmptied(&self, observer: Weak<dyn CommitMessageViewObserver>)
    {
        self.onEmptiedObservers.borrow_mut().push(observer);
    }


    // private

    fn connectSelfToWidget(self: &Rc<Self>)
    {
        self.widget.connectOnFilled(Rc::downgrade(&(self.clone() as Rc<dyn TextViewObserver>)));
        self.widget.connectOnEmptied(Rc::downgrade(&(self.clone() as Rc<dyn TextViewObserver>)));
    }

    fn connectSelfToRepository(self: &Rc<Self>, repository: &Repository)
    {
        repository.connectOnCommitted(Rc::downgrade(&(self.clone() as Rc<dyn RepositoryObserver>)));
    }
}

impl RepositoryObserver for CommitMessageView
{
    fn onCommitted(&self)
    {
        self.widget.clear();
    }
}

impl TextViewObserver for CommitMessageView
{
    fn onFilled(&self)
    {
        for observer in &*self.onFilledObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onFilled();
            }
        }
    }

    fn onEmptied(&self)
    {
        for observer in &*self.onEmptiedObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onEmptied();
            }
        }
    }
}