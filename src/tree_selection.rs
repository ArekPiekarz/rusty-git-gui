use crate::event::{Event, Sender, Source};
use crate::tree_model_utils::toRow;

use gtk::TreeSelectionExt as _;


pub struct TreeSelection
{
    selection: gtk::TreeSelection,
}

impl TreeSelection
{
    pub fn new(selection: gtk::TreeSelection, sender: Sender, selectionSource: Source) -> Self
    {
        let newSelf = Self{selection};
        newSelf.connectSelection(sender, selectionSource);
        newSelf
    }

    pub fn getSelectedRow(&self) -> Option<usize>
    {
        let (rowPaths, _model) = self.selection.get_selected_rows();
        match rowPaths.get(0) {
            Some(rowPath) => Some(toRow(rowPath)),
            None => None
        }
    }

    pub fn selectByIterator(&self, iterator: &gtk::TreeIter)
    {
        self.selection.select_iter(iterator);
    }

    pub fn unselectAll(&self)
    {
        self.selection.unselect_all();
    }


    // private

    fn connectSelection(&self, sender: Sender, eventSource: Source)
    {
        self.selection.connect_changed(move |selection|
            sender.send((eventSource, Event::SelectionChanged(selection.clone()))).unwrap());
    }
}