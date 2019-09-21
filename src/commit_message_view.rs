use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::text_view::TextView;

use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitMessageView
{
    widget: Rc<RefCell<TextView>>,
}

impl CommitMessageView
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository) -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{
            widget: TextView::new(guiElementProvider, "Commit message view"),
        }));
        Self::connectSelfToRepository(&newSelf, repository);
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

    fn onCommitted(&self)
    {
        self.widget.borrow().clear();
    }
}