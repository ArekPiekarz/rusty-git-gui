use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};

use glib::Sender;
use gtk::TextBufferExt as _;
use gtk::TextViewExt as _;
use std::cell::RefCell;
use std::rc::Rc;

pub const EXCLUDE_HIDDEN_CHARACTERS : bool = false;


pub struct TextView
{
    buffer: gtk::TextBuffer,
    onFilledSenders: Vec<Sender<()>>,
    onEmptiedSenders: Vec<Sender<()>>,
    shouldNotifyOnFilled: bool
}

#[derive(Eq, PartialEq)]
pub enum Notifications
{
    Enabled,
    Disabled
}

impl TextView
{
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str, notifications: Notifications)
        -> Rc<RefCell<Self>>
    {
        let widget = guiElementProvider.get::<gtk::TextView>(name);
        let newSelf = Rc::new(RefCell::new(Self{
            buffer: widget.get_buffer().unwrap(),
            onFilledSenders: vec![],
            onEmptiedSenders: vec![],
            shouldNotifyOnFilled: true
        }));
        if notifications == Notifications::Enabled {
            Self::connectSelfToBuffer(&newSelf);
        }
        newSelf
    }

    pub fn getText(&self) -> String
    {
        self.buffer.get_text(&self.buffer.get_start_iter(), &self.buffer.get_end_iter(), EXCLUDE_HIDDEN_CHARACTERS)
            .unwrap().into()
    }

    pub fn setText(&self, text: &str)
    {
        self.buffer.set_text(text);
    }

    pub fn isFilled(&self) -> bool
    {
        !self.getText().is_empty()
    }

    pub fn isEmpty(&self) -> bool
    {
        !self.isFilled()
    }

    pub fn setRichText(&self, text: &str)
    {
        self.clear();
        self.buffer.insert_markup(&mut self.buffer.get_start_iter(), text);
    }

    pub fn clear(&self)
    {
        self.setText("");
    }

    pub fn connectOnFilled(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.onFilledSenders.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnEmptied(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.onEmptiedSenders.push(sender);
        attach(receiver, handler);
    }


    // private

    fn connectSelfToBuffer(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().buffer.connect_changed(move |_buffer| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow().onBufferChanged();
            }
        });
    }

    fn onBufferChanged(&self)
    {
        if self.isFilled() {
            if self.shouldNotifyOnFilled {
                self.notifyOnFilled();
            }
        }
        else {
            self.notifyOnEmptied();
        }
    }

    fn notifyOnFilled(&self)
    {
        for sender in &self.onFilledSenders {
            sender.send(()).unwrap();
        }
    }

    fn notifyOnEmptied(&self)
    {
        for sender in &self.onEmptiedSenders {
            sender.send(()).unwrap();
        }
    }
}