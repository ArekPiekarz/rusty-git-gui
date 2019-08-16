use crate::error_handling::exit;

use gtk::CellLayoutExt;
use gtk::TreeViewExt;

const EXPAND_IN_LAYOUT : bool = true;


pub fn setupColumn(columnIndex: i32, view: &gtk::TreeView)
{
    let renderer = gtk::CellRendererText::new();
    let column = view.get_column(columnIndex)
        .unwrap_or_else(|| exit(&format!("Failed to get column with index {}", columnIndex)));
    column.pack_start(&renderer, EXPAND_IN_LAYOUT);
    column.add_attribute(&renderer, "text", columnIndex);
}