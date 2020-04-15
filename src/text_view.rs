use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::event_constants::{CONSUME_EVENT, FORWARD_EVENT};
use crate::gui_element_provider::GuiElementProvider;
use crate::line_count::LineCount;
use crate::line_number::LineNumber;

use gtk::CssProviderExt as _;
use gtk::StyleContextExt as _;
use gtk::TextBufferExt as _;
use gtk::TextTagTableExt as _;
use gtk::TextViewExt as _;
use gtk::WidgetExt as _;
use std::cmp::{min, max};

pub const EXCLUDE_HIDDEN_CHARACTERS : bool = false;
const NO_SEARCH_LIMIT: Option<&gtk::TextIter> = None;
const SEARCH_VISIBLE_TEXT: gtk::TextSearchFlags = gtk::TextSearchFlags::from_bits_truncate(
    gtk::TextSearchFlags::VISIBLE_ONLY.bits() | gtk::TextSearchFlags::TEXT_ONLY.bits());


pub struct TextView
{
    buffer: gtk::TextBuffer,
    sender: Sender,
    source: Source,
    shouldNotifyOnFilled: bool,
    style: Style
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Notifications
{
    Enabled,
    Disabled
}

impl IEventHandler for TextView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::BufferChanged              => self.onBufferChanged(),
            Event::ZoomRequested(scrollEvent) => self.onZoomRequested(scrollEvent),
            _ => handleUnknown(source, event)
        }
    }
}

impl TextView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        name: &str,
        sender: Sender,
        source: Source,
        notifications: Notifications) -> Self
    {
        let widget = guiElementProvider.get::<gtk::TextView>(name);
        let newSelf = Self{
            buffer: widget.get_buffer().unwrap(),
            sender,
            source,
            shouldNotifyOnFilled: true,
            style: Style::new(&widget),
        };

        if notifications == Notifications::Enabled {
            newSelf.connectBuffer();
        }
        newSelf.connectWidget(&widget);
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

    pub fn applyTagUntilMatchEnd(&self, tag: &gtk::TextTag, startLine: LineNumber, pattern: &str)
    {
        let startIter = self.buffer.get_iter_at_line(startLine.into());
        let endIter = startIter.forward_search(pattern, SEARCH_VISIBLE_TEXT, NO_SEARCH_LIMIT).unwrap().1;
        self.buffer.apply_tag(tag, &startIter, &endIter);
    }

    pub fn removeTags(&self)
    {
        self.buffer.remove_all_tags(&self.buffer.get_start_iter(), &self.buffer.get_end_iter());
    }


    // private

    fn connectBuffer(&self)
    {
        let sender = self.sender.clone();
        let source = self.source;
        self.buffer.connect_changed(move |_buffer| {
            sender.send((source, Event::BufferChanged)).unwrap();
        });
    }

    fn connectWidget(&self, widget: &gtk::TextView)
    {
        let sender = self.sender.clone();
        let source = self.source;
        widget.connect_scroll_event(move |_widget, event| {
            onScrolled(event, &sender, source)
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
        self.sender.send((self.source, Event::Filled)).unwrap();
    }

    fn notifyOnEmptied(&self)
    {
        self.sender.send((self.source, Event::Emptied)).unwrap();
    }
}

fn onScrolled(event: &gdk::EventScroll, sender: &Sender, source: Source) -> gtk::Inhibit
{
    if !event.get_state().contains(gdk::ModifierType::CONTROL_MASK) {
        return FORWARD_EVENT;
    }

    sender.send((source, Event::ZoomRequested(event.clone()))).unwrap();
    CONSUME_EVENT
}

struct Style
{
    cssProvider: gtk::CssProvider,
    font: Font
}

impl Style
{
    fn new<T>(widget: &T) -> Self
        where T: glib::IsA<gtk::Widget>
    {
        let cssProvider = gtk::CssProvider::new();
        widget.get_style_context().add_provider(&cssProvider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        Self{
            cssProvider,
            font: Font::new(widget)
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
    fn new<T>(widget: &T) -> Self
        where T: glib::IsA<gtk::Widget>
    {
        let fontDescription = getFontDescription(widget);
        Self{
            size: getFontSize(&fontDescription),
            maxSize: None,
            family: getFontFamily(&fontDescription)
        }
    }
}

fn getFontDescription<T>(widget: &T) -> pango::FontDescription
    where T: glib::IsA<gtk::Widget>
{
    widget.get_pango_context().unwrap().get_font_description().unwrap()
}

fn getFontSize(fontDescription: &pango::FontDescription) -> FontSize
{
    fontDescription.get_size() / pango::SCALE
}

fn getFontFamily(fontDescription: &pango::FontDescription) -> FontFamily
{
    fontDescription.get_family().unwrap().into()
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