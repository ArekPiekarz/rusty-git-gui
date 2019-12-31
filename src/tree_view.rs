use crate::error_handling::exit;
use crate::event_constants::KEEP_FORWARDING_EVENT;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::number_casts::ToI32 as _;
use crate::tree_model_utils::toRow;
use crate::tree_selection::TreeSelection;

use glib::Sender;
use gtk::CellLayoutExt as _;
use gtk::TreeModelExt as _;
use gtk::TreeViewExt as _;
use gtk::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;

const EXPAND_IN_LAYOUT : bool = true;
const NO_COLUMN_FOCUS: Option<&gtk::TreeViewColumn> = None;
const NO_EDITING: bool = false;
const MOUSE_RIGHT_BUTTON: u32 = 3;


pub struct TreeView
{
    widget: gtk::TreeView,
    selection: Rc<RefCell<TreeSelection>>,
    onRowActivatedSenders: Vec<Sender<(gtk::TreeView, gtk::TreePath, gtk::TreeViewColumn)>>,
    onRightClickedSenders: Vec<Sender<gdk::EventButton>>
}

type OnRightClickedHandler = Box<dyn Fn(gdk::EventButton) -> glib::Continue>;

impl TreeView
{
    pub fn new(guiElementProvider: &GuiElementProvider, widgetName: &str, columns: &[i32]) -> Rc<RefCell<Self>>
    {
        let widget = guiElementProvider.get::<gtk::TreeView>(widgetName);
        let selection = TreeSelection::new(widget.get_selection());
        let newSelf = Rc::new(RefCell::new(Self{
            widget,
            selection,
            onRowActivatedSenders: vec![],
            onRightClickedSenders: vec![]
        }));
        newSelf.borrow().setupColumns(columns);
        Self::connectSelfToRowActivated(&newSelf);
        Self::connectSelfToButtonPressEvent(&newSelf);
        newSelf
    }

    pub fn getColumn(&self, index: i32) -> gtk::TreeViewColumn
    {
        self.widget.get_column(index).unwrap()
    }

    pub fn getModel(&self) -> gtk::TreeModel
    {
        self.widget.get_model().unwrap()
    }

    pub const fn getSelection(&self) -> &Rc<RefCell<TreeSelection>>
    {
        &self.selection
    }

    pub fn getRowAtPosition(&self, x: f64, y: f64) -> Option<usize>
    {
        match self.widget.get_path_at_pos(x.toI32(), y.toI32()) {
            Some(result) => Some(toRow(&result.0.unwrap())),
            None => None
        }
    }

    pub fn connectOnRowActivated(
        &mut self,
        handler: Box<dyn Fn((gtk::TreeView, gtk::TreePath, gtk::TreeViewColumn)) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.onRowActivatedSenders.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnRightClicked(&mut self, handler: OnRightClickedHandler)
    {
        let (sender, receiver) = makeChannel();
        self.onRightClickedSenders.push(sender);
        attach(receiver, handler);
    }

    pub fn rowActivated(&self, path: &gtk::TreePath, column: &gtk::TreeViewColumn)
    {
        self.widget.row_activated(path, column);
    }

    pub fn focusFirstRow(&self)
    {
        let model = self.getModel();
        let iter = model.get_iter_first().unwrap();
        let rowPath = model.get_path(&iter).unwrap();
        self.widget.set_cursor(&rowPath, NO_COLUMN_FOCUS, NO_EDITING);
        self.widget.grab_focus();
    }


    // private

    fn setupColumns(&self, columns: &[i32])
    {
        columns.iter().for_each(|column| self.setupColumn(*column));
    }

    fn setupColumn(&self, columnIndex: i32)
    {
        let renderer = gtk::CellRendererText::new();
        let column = self.widget.get_column(columnIndex)
            .unwrap_or_else(|| exit(&format!("Failed to get column with index {}", columnIndex)));
        column.pack_start(&renderer, EXPAND_IN_LAYOUT);
        column.add_attribute(&renderer, "text", columnIndex);
    }

    fn connectSelfToRowActivated(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().widget.connect_row_activated(
            move |view, row, column| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().notifyOnRowActivated(view, row, column);
                }
            }
        );
    }

    fn connectSelfToButtonPressEvent(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().widget.connect_button_press_event(
            move |_view, button| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().onButtonPressEvent(button);
                }
                KEEP_FORWARDING_EVENT
            }
        );
    }

    fn notifyOnRowActivated(&self, view: &gtk::TreeView, row: &gtk::TreePath, column: &gtk::TreeViewColumn)
    {
        for sender in &self.onRowActivatedSenders {
            sender.send((view.clone(), row.clone(), column.clone())).unwrap();
        }
    }

    fn onButtonPressEvent(&self, event: &gdk::EventButton)
    {
        if event.get_button() == MOUSE_RIGHT_BUTTON {
            self.notifyOnRightClicked(event);
        }
    }

    fn notifyOnRightClicked(&self, event: &gdk::EventButton)
    {
        for sender in &self.onRightClickedSenders {
            sender.send(event.clone()).unwrap();
        }
    }
}
