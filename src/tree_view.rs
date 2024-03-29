use crate::error_handling::exit;
use crate::event::{Event, Sender, Source};
use crate::event_constants::FORWARD_EVENT;
use crate::gui_element_provider::GuiElementProvider;
use crate::number_casts::ToI32 as _;
use crate::selections_comparer::SelectionsComparer;
use crate::tree_model_utils::toRow;
use crate::tree_selection::TreeSelection;

use gtk::prelude::TreeModelExt as _;
use gtk::prelude::TreeViewColumnExt as _;
use gtk::prelude::TreeViewExt as _;
use gtk::prelude::WidgetExt as _;

const EXPAND_IN_LAYOUT : bool = true;
const NO_COLUMN_FOCUS: Option<&gtk::TreeViewColumn> = None;
const NO_EDITING: bool = false;
const MOUSE_RIGHT_BUTTON: u32 = 3;


pub(crate) struct TreeView
{
    widget: gtk::TreeView,
    selection: TreeSelection,
}

impl TreeView
{
    pub(crate) fn new(
        guiElementProvider: &GuiElementProvider,
        widgetName: &str,
        selectionsComparer: Option<Box<dyn SelectionsComparer>>,
        sender: Sender,
        source: Source,
        columns: &[i32])
        -> Self
    {
        let widget = guiElementProvider.get::<gtk::TreeView>(widgetName);
        let selection = TreeSelection::new(widget.selection(), selectionsComparer, sender.clone(), source);
        let newSelf = Self{widget, selection};
        newSelf.setupColumns(columns);
        newSelf.connectWidget(sender, source);
        newSelf
    }

    pub fn getModel(&self) -> gtk::TreeModel
    {
        self.widget.model().unwrap()
    }

    pub(crate) const fn getSelection(&self) -> &TreeSelection
    {
        &self.selection
    }

    pub(crate) fn getSelectionMut(&mut self) -> &mut TreeSelection
    {
        &mut self.selection
    }

    pub fn getRowAtPosition(&self, x: f64, y: f64) -> Option<usize>
    {
        match self.widget.path_at_pos(x.toI32(), y.toI32()) {
            Some(result) => Some(toRow(&result.0.unwrap())),
            None => None
        }
    }

    pub fn focusFirstRow(&self)
    {
        let model = self.getModel();
        let iter = model.iter_first().unwrap();
        let rowPath = model.path(&iter).unwrap();
        self.widget.set_cursor(&rowPath, NO_COLUMN_FOCUS, NO_EDITING);
        self.focus();
    }

    pub fn focus(&self)
    {
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
        let column = self.widget.column(columnIndex)
            .unwrap_or_else(|| exit(&format!("Failed to get column with index {}", columnIndex)));
        column.pack_start(&renderer, EXPAND_IN_LAYOUT);
        column.add_attribute(&renderer, "text", columnIndex);
    }

    fn connectWidget(&self, sender: Sender, source: Source)
    {
        self.connectRowActivated(sender.clone(), source);
        self.connectButtonPressEvent(sender, source);
    }

    fn connectRowActivated(&self, sender: Sender, source: Source)
    {
        self.widget.connect_row_activated(move |_view, row, _column| {
            sender.send((source, Event::RowActivated(row.clone()))).unwrap();
        });
    }

    fn connectButtonPressEvent(&self, sender: Sender, source: Source)
    {
        self.widget.connect_button_press_event(move |_view, event| {
            if event.button() == MOUSE_RIGHT_BUTTON {
                sender.send((source, Event::RightClicked(event.clone()))).unwrap();
            }
            FORWARD_EVENT
        });
    }
}
