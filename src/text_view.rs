use crate::event_constants::{CONSUME_EVENT, FORWARD_EVENT};
use crate::gui_element_provider::GuiElementProvider;
use crate::line_count::LineCount;
use crate::line_number::LineNumber;
use crate::main_context::{attach, makeChannel};

use glib::object::ObjectType as _;
use glib::Sender;
use gtk::CssProviderExt as _;
use gtk::StyleContextExt as _;
use gtk::TextBufferExt as _;
use gtk::TextTagTableExt as _;
use gtk::TextViewExt as _;
use gtk::WidgetExt as _;
use std::cell::RefCell;
use std::cmp::{min, max};
use std::rc::{Rc, Weak};

pub const EXCLUDE_HIDDEN_CHARACTERS : bool = false;


pub struct TextView
{
    buffer: gtk::TextBuffer,
    onFilledSenders: Vec<Sender<()>>,
    onEmptiedSenders: Vec<Sender<()>>,
    shouldNotifyOnFilled: bool,
    style: Style
}

#[derive(Clone, Copy, Eq, PartialEq)]
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
            shouldNotifyOnFilled: true,
            style: Style::new(&widget.get_style_context()),
        }));
        if notifications == Notifications::Enabled {
            Self::connectSelfToBuffer(&newSelf);
        }
        Self::connectSelfToWidget(&newSelf, &widget);

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

    fn connectSelfToWidget(rcSelf: &Rc<RefCell<Self>>, widget: &gtk::TextView)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        widget.connect_scroll_event(move |_widget, event| {
            Self::onScrolled(&weakSelf, event)
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

    fn onScrolled(weakSelf: &Weak<RefCell<Self>>, event: &gdk::EventScroll) -> gtk::Inhibit
    {
        if !event.get_state().contains(gdk::ModifierType::CONTROL_MASK) {
            return FORWARD_EVENT;
        }

        if let Some(rcSelf) = weakSelf.upgrade() {
            rcSelf.borrow_mut().onZoomRequested(event);
        }
        CONSUME_EVENT
    }

    fn onZoomRequested(&mut self, event: &gdk::EventScroll)
    {
        let newFontSize = self.calculateNewFontSize(event);
        if self.style.font.size == newFontSize {
            return;
        }

        self.loadCss(newFontSize, event);
    }

    fn loadCss(&mut self, newFontSize: FontSize, event: &gdk::EventScroll)
    {
        match self.style.cssProvider.load_from_data(self.formatCss(newFontSize).as_bytes()) {
            Ok(_) => self.style.font.size = newFontSize,
            Err(error) => {
                validateCssError(&error);
                self.style.font.maxSize = Some(self.style.font.size);
                self.reloadCorrectCss(event);
            }
        }

    }

    fn reloadCorrectCss(&mut self, event: &gdk::EventScroll)
    {
        let newFontSize = self.calculateNewFontSize(event);
        match self.style.cssProvider.load_from_data(self.formatCss(newFontSize).as_bytes()) {
            Ok(_) => self.style.font.size = newFontSize,
            Err(e) => panic!("Unexpected error when reloading a corrected CSS: {}", e)
        }
    }

    fn calculateNewFontSize(&self, event: &gdk::EventScroll) -> FontSize
    {
        match getY(event.get_delta()) {
            y if y < 0.0 => self.calculateHigherFontSize(),
            y if y > 0.0 => self.calculateLowerFontSize(),
            _ => self.style.font.size
        }
    }

    fn calculateHigherFontSize(&self) -> FontSize
    {
        match self.style.font.maxSize {
            Some(maxSize) => min(self.style.font.size + 1, maxSize),
            None => self.style.font.size + 1
        }
    }

    fn calculateLowerFontSize(&self) -> FontSize
    {
        max(self.style.font.size - 1, 1)
    }

    fn formatCss(&self, fontSize: FontSize) -> String
    {
        format!("textview {{font: {}pt {:?}}}", fontSize, self.style.font.family)
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

struct Style
{
    cssProvider: gtk::CssProvider,
    font: Font
}

impl Style
{
    fn new(styleContext: &gtk::StyleContext) -> Self
    {
        let cssProvider = gtk::CssProvider::new();
        styleContext.add_provider(&cssProvider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        Self{
            cssProvider,
            font: Font::new(styleContext)
        }
    }
}

struct Font
{
    size: FontSize,
    maxSize: Option<FontSize>,
    family: FontFamily
}

type FontSize = i32;
type FontFamily = String;

impl Font
{
    fn new(styleContext: &gtk::StyleContext) -> Self
    {
        let fontDescription = unsafe {gtk_sys::gtk_style_context_get_font(
            styleContext.as_ptr(), gtk_sys::GTK_STATE_FLAG_NORMAL)};

        Self{
            size: getFontSize(fontDescription),
            maxSize: None,
            family: getFontFamily(fontDescription)
        }
    }
}

fn getFontSize(fontDescription: *const pango_sys::PangoFontDescription) -> FontSize
{
    (unsafe {pango_sys::pango_font_description_get_size(fontDescription)}) / pango_sys::PANGO_SCALE
}

fn getFontFamily(fontDescription: *const pango_sys::PangoFontDescription) -> FontFamily
{
    let familyPtr = unsafe {pango_sys::pango_font_description_get_family(fontDescription)};
    let familyCStr = unsafe {std::ffi::CStr::from_ptr(familyPtr)};
    familyCStr.to_str().unwrap().to_owned()
}

const fn getY(coordinates: (f64, f64)) -> f64
{
    coordinates.1
}

fn validateCssError(error: &glib::Error)
{
    match error.kind::<gtk::CssProviderError>() {
        Some(cssProviderError) => {
            if let gtk::CssProviderError::Syntax = cssProviderError {
                if error.to_string() != "<data>:1:19not a number" {
                    panic!("Unexpected CSS provider error message: {}", error)
                }
            } else {
                panic!("Unexpected CSS provider error kind: {}", error)
            }
        },
        None => panic!("Unexpected CSS error: {}", error) }
}