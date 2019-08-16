use crate::color::Color;
use crate::gui_element_provider::GuiElementProvider;
use crate::text_view_observer::TextViewObserver;

use gtk::TextBufferExt as _;
use gtk::TextViewExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;


pub const EXCLUDE_HIDDEN_CHARACTERS : bool = false;


pub struct TextView
{
    buffer: gtk::TextBuffer,
    onFilledObservers: RefCell<Vec<Weak<dyn TextViewObserver>>>,
    onEmptiedObservers: RefCell<Vec<Weak<dyn TextViewObserver>>>,
    shouldNotifyOnFilled: RefCell<bool>
}

impl TextView
{
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str) -> Rc<Self>
    {
        let widget = guiElementProvider.get::<gtk::TextView>(name);
        let newSelf = Rc::new(Self{
            buffer: widget.get_buffer().unwrap(),
            onFilledObservers: RefCell::new(vec![]),
            onEmptiedObservers: RefCell::new(vec![]),
            shouldNotifyOnFilled: RefCell::new(true)
        });
        newSelf.connectSelfToBuffer();
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

    pub fn append(&self, text: &str)
    {
        self.buffer.insert(&mut self.buffer.get_end_iter(), text);
    }

    pub fn appendColored(&self, color: Color, text: &str)
    {
        self.buffer.insert_markup(
            &mut self.buffer.get_end_iter(),
            &format!("<span color='{}'>{}</span>", *color, glib::markup_escape_text(text)));
    }

    pub fn clear(&self)
    {
        self.buffer.delete(&mut self.buffer.get_start_iter(), &mut self.buffer.get_end_iter());
    }

    pub fn connectOnFilled(&self, observer: Weak<dyn TextViewObserver>)
    {
        self.onFilledObservers.borrow_mut().push(observer);
    }

    pub fn connectOnEmptied(&self, observer: Weak<dyn TextViewObserver>)
    {
        self.onEmptiedObservers.borrow_mut().push(observer);
    }


    // private

    fn connectSelfToBuffer(self: &Rc<Self>)
    {
        let weakSelf = Rc::downgrade(&self);
        self.buffer.connect_changed(move |_buffer| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onBufferChanged();
            }
        });
    }

    fn onBufferChanged(&self)
    {
        if self.isFilled() {
            if *self.shouldNotifyOnFilled.borrow() {
                self.notifyOnFilled();
            }
        }
        else {
            self.notifyOnEmptied();
        }
    }

    fn notifyOnFilled(&self)
    {
        *self.shouldNotifyOnFilled.borrow_mut() = false;
        for observer in &*self.onFilledObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onFilled();
            }
        }
    }

    fn notifyOnEmptied(&self)
    {
        *self.shouldNotifyOnFilled.borrow_mut() = true;
        for observer in &*self.onEmptiedObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onEmptied();
            }
        }
    }
}