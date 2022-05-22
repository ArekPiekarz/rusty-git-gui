use crate::event::{Event, Sender, Source};
use crate::selections_comparer::SelectionsComparer;
use crate::tree_model_utils::toRow;

use glib::ObjectExt;
use gtk::glib;
use gtk::traits::TreeSelectionExt;


pub(crate) struct TreeSelection
{
    selection: gtk::TreeSelection,
    signalHandlerId: glib::SignalHandlerId,
    selectionsComparer: Option<Box<dyn SelectionsComparer>>,
    sender: Sender,
    eventSource: Source
}

impl TreeSelection
{
    pub fn new(
        selection: gtk::TreeSelection,
        selectionsComparer: Option<Box<dyn SelectionsComparer>>,
        sender: Sender,
        eventSource: Source)
        -> Self
    {
        let signalHandlerId = connectSelection(&selection, sender.clone(), eventSource);
        Self{selection, signalHandlerId, selectionsComparer, sender, eventSource}
    }

    pub fn getSelectedRow(&self) -> Option<usize>
    {
        let (rowPaths, _model) = self.selection.selected_rows();
        rowPaths.get(0).map(toRow)
    }

    pub fn selectByIterator(&self, iterator: &gtk::TreeIter)
    {
        self.selection.select_iter(iterator);
    }

    pub fn unselectAll(&self)
    {
        self.selection.unselect_all();
    }

    pub fn blockSignals(&mut self)
    {
        self.selection.block_signal(&self.signalHandlerId);
        if let Some(selectionsComparer) = &mut self.selectionsComparer {
            selectionsComparer.setFirst(&self.selection);
        }
    }

    pub fn unblockSignals(&mut self)
    {
        self.selection.unblock_signal(&self.signalHandlerId);
        if let Some(selectionsComparer) = &mut self.selectionsComparer {
            selectionsComparer.setSecond(&self.selection);
            if selectionsComparer.areDifferent() {
                self.sender.send((self.eventSource, Event::SelectionChanged(self.selection.clone()))).unwrap();
            }
        }
    }
}

fn connectSelection(selection: &gtk::TreeSelection, sender: Sender, eventSource: Source) -> glib::SignalHandlerId
{
    selection.connect_changed(move |selection|
        sender.send((eventSource, Event::SelectionChanged(selection.clone()))).unwrap())
}
