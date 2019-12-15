use crate::file_change::FileChange;
use crate::file_changes_column::FileChangesColumn;
use crate::file_changes_getter::FileChangesGetter;
use crate::file_changes_view_entry::FileChangesViewEntry;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};
use crate::tree_view::TreeView;

use glib::Sender;
use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use std::cell::RefCell;
use std::rc::Rc;

pub type OnRowActivatedAction = Box<dyn Fn(&FileChange)>;


pub struct FileChangesView<StoreType>
{
    view: Rc<RefCell<TreeView>>,
    store: Rc<RefCell<StoreType>>,
    onRowActivatedAction: OnRowActivatedAction,
    onSelectedSenders: Vec<Sender<FileChange>>,
    onUnselectedSenders: Vec<Sender<()>>
}

impl<StoreType> FileChangesView<StoreType>
    where StoreType: FileChangesGetter + 'static
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        widgetName: &str,
        store: Rc<RefCell<StoreType>>,
        onRowActivatedAction: OnRowActivatedAction)
        -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self {
            store,
            view: TreeView::new(guiElementProvider, widgetName, &FileChangesColumn::asArrayOfI32()),
            onRowActivatedAction,
            onSelectedSenders: vec![],
            onUnselectedSenders: vec![]
        }));
        Self::connectSelfToViewSelection(&newSelf);
        Self::connectSelfToView(&newSelf);
        newSelf
    }

    pub fn isEmpty(&self) -> bool
    {
        self.getModel().get_iter_first().is_none()
    }

    pub fn isFilled(&self) -> bool
    {
        !self.isEmpty()
    }

    pub fn getData(&self) -> Vec<FileChangesViewEntry>
    {
        let mut content = vec![];
        self.getModel().foreach(|model, _row, iter| {
            content.push(FileChangesViewEntry {
                status: Self::getCell(model, iter, FileChangesColumn::Status),
                path: Self::getCell(model, iter, FileChangesColumn::Path)});
            CONTINUE_ITERATING_MODEL });
        content
    }

    pub fn select(&self, filePath: &str) -> bool
    {
        self.invokeForRowWith(
            filePath,
            |view, _row, iterator| { view.getSelection().borrow().selectByIterator(iterator); })
    }

    pub fn activate(&self, filePath: &str) -> bool
    {
        self.select(filePath);
        self.invokeForRowWith(
            filePath,
            |view, row, _iterator| { view.rowActivated(row, &self.getFilePathColumn()); })
    }

    pub fn connectOnSelected(&mut self, handler: Box<dyn Fn(FileChange) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.onSelectedSenders.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnUnselected(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.onUnselectedSenders.push(sender);
        attach(receiver, handler);
    }

    pub fn unselectAll(&self)
    {
        self.view.borrow().getSelection().borrow().unselectAll();
    }

    pub fn trySelectFirst(&self)
    {
        if let Some(iter) = self.getModel().get_iter_first() {
            let view = self.view.borrow();
            view.getSelection().borrow().selectByIterator(&iter);
            view.focusFirstRow();
        }
    }


    // private

    fn getModel(&self) -> gtk::TreeModel
    {
        self.view.borrow().getModel()
    }

    fn getStatusCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
    {
        Self::getCell(model, iter, FileChangesColumn::Status)
    }

    fn getPathCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
    {
        Self::getCell(model, iter, FileChangesColumn::Path)
    }

    fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: FileChangesColumn) -> String
    {
        model.get_value(iter, column as i32).get::<String>().unwrap().unwrap()
    }

    fn getFilePathColumn(&self) -> gtk::TreeViewColumn
    {
        self.view.borrow().getColumn(FileChangesColumn::Path as i32)
    }

    fn connectSelfToViewSelection(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().view.borrow().getSelection().borrow_mut().connectOnChanged(Box::new(
            move |selection| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().notifyBasedOnSelectionChanged(&selection);
                }
                glib::Continue(true)
        }));
    }

    #[allow(clippy::panic)]
    fn notifyBasedOnSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        let (rows, _model) = selection.get_selected_rows();
        debug_assert!(rows.len() <= 1);
        match rows.get(0) {
            Some(row) => self.notifyOnSelected(self.store.borrow().getFileChange(row)),
            None => self.notifyOnUnselected()
        }
    }

    fn notifyOnSelected(&self, fileChange: &FileChange)
    {
        for sender in &self.onSelectedSenders {
            sender.send(fileChange.clone()).unwrap();
        }
    }

    fn notifyOnUnselected(&self)
    {
        for sender in &self.onUnselectedSenders {
            sender.send(()).unwrap();
        }
    }

    fn connectSelfToView(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().view.borrow_mut().connectOnRowActivated(Box::new(
            move |(_view, row, _column)| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().onRowActivated(&row);
                }
                glib::Continue(true)
        }));
    }

    fn onRowActivated(&self, row: &gtk::TreePath)
    {
        let model = self.getModel();
        let iterator = model.get_iter(row).unwrap();
        let fileChange = FileChange{
            status: Self::getStatusCell(&model, &iterator),
            path: Self::getPathCell(&model, &iterator),
            oldPath: None};

        (self.onRowActivatedAction)(&fileChange);
    }

    fn invokeForRowWith(
        &self,
        filePath: &str,
        action: impl Fn(&TreeView, &gtk::TreePath, &gtk::TreeIter))
        -> bool
    {
        let model = self.getModel();
        let mut rowFound = false;
        model.foreach(|model, row, iter| {
            if Self::getCell(model, iter, FileChangesColumn::Path) != filePath {
                return CONTINUE_ITERATING_MODEL; }
            rowFound = true;
            action(&self.view.borrow(), row, iter);
            STOP_ITERATING_MODEL
        });
        rowFound
    }
}