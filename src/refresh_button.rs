use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::repository::Repository;

use gtk::ButtonExt as _;
use std::cell::RefCell;
use std::rc::Rc;


pub struct RefreshButton
{
    widget: gtk::Button,
    repository: Rc<RefCell<Repository>>
}

impl RefreshButton
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: Rc<RefCell<Repository>>) -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            widget: guiElementProvider.get::<gtk::Button>("Refresh button"),
            repository
        });
        newSelf.connectSelfToWidget();
        newSelf
    }

    pub fn click(&self)
    {
        self.widget.clicked();
    }


    // private

    fn connectSelfToWidget(self: &Rc<Self>)
    {
        let (sender, receiver) = makeChannel();
        self.widget.connect_clicked(move |_button| {
            sender.send(()).unwrap();
        });

        let weakSelf = Rc::downgrade(self);
        attach(receiver, move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.refresh();
            }
            glib::Continue(true)
        });
    }

    fn refresh(&self)
    {
        self.repository.borrow_mut().refresh();
    }
}