use crate::gui_element_provider::GuiElementProvider;
use crate::line_count::LineCount;
use crate::line_number::LineNumber;
use crate::main_context::{attach, makeChannel};

use glib::Sender;
use gtk::TextBufferExt as _;
use gtk::TextTagTableExt as _;
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
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str, notifications: Notifications) -> Rc<RefCell<Self>>
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

    pub fn insertTextAt(&self, text: &str, line: LineNumber)
    {
        self.buffer.insert(&mut self.buffer.get_iter_at_line(line.into()), text);
    }

    pub fn removeTextAt(&self, startLine: LineNumber, lineCount: LineCount)
    {
        self.buffer.delete(
            &mut self.buffer.get_iter_at_line(startLine.into()),
            &mut self.buffer.get_iter_at_line((startLine + lineCount).into()));
    }

    pub fn isFilled(&self) -> bool
    {
        !self.getText().is_empty()
    }

    pub fn isEmpty(&self) -> bool
    {
        !self.isFilled()
    }

    pub fn clear(&self)
    {
        self.setText("");
    }

    pub fn registerTags(&self, tags: &[&gtk::TextTag])
    {
        let tagTable = self.buffer.get_tag_table().unwrap();
        for tag in tags {
            assert!(tagTable.add(*tag));
        }
    }

    pub fn applyTag(&self, tag: &gtk::TextTag, startLine: LineNumber, endLine: LineNumber)
    {
        self.buffer.apply_tag(
            tag,
            &self.buffer.get_iter_at_line(startLine.into()),
            &self.buffer.get_iter_at_line(endLine.into()));
    }

    pub fn applyTagUntilEnd(&self, tag: &gtk::TextTag, startLine: LineNumber)
    {
        self.buffer.apply_tag(tag, &self.buffer.get_iter_at_line(startLine.into()), &self.buffer.get_end_iter());
    }

    pub fn removeTags(&self)
    {
        self.buffer.remove_all_tags(&self.buffer.get_start_iter(), &self.buffer.get_end_iter());
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
        } else {
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