use crate::main_context::{attach, makeChannel};

use glib::Sender;
use gtk::TreeSelectionExt as _;
use std::cell::RefCell;
use std::rc::Rc;


pub struct TreeSelection
{
    selection: gtk::TreeSelection,
    onChangedSenders: Vec<Sender<gtk::TreeSelection>>
}

impl TreeSelection
{
    pub fn new(selection: gtk::TreeSelection) -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{selection, onChangedSenders: vec![]}));
        Self::connectSelfToChanged(&newSelf);
        newSelf
    }

    pub fn getSelectedRow(&self) -> Option<usize>
    {
        let (rowPaths, _model) = self.selection.get_selected_rows();
        match rowPaths.get(0) {
            Some(rowPath) => Some(*rowPath.get_indices().get(0).unwrap() as usize),
            None => None
        }
    }

    pub fn connectOnChanged(&mut self, handler: Box<dyn Fn(gtk::TreeSelection) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.onChangedSenders.push(sender);
        attach(receiver, handler);
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

    fn connectSelfToChanged(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().selection.connect_changed(
            move |selection| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().notifyOnChanged(selection);
                }
            }
        );
    }

    fn notifyOnChanged(&self, selection: &gtk::TreeSelection)
    {
        for sender in &self.onChangedSenders {
            sender.send(selection.clone()).unwrap();
        }
    }
}