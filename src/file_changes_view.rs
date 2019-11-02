use crate::file_change::FileChange;
use crate::file_changes_column::FileChangesColumn;
use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};
use crate::tree_view_column_setup::setupColumn;

use glib::Sender;
use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::TreeViewExt as _;
use std::cell::RefCell;
use std::rc::Rc;

pub type OnRowActivatedAction = Box<dyn Fn(&FileChange)>;


pub struct FileChangesView<StoreType>
{
    widget: gtk::TreeView,
    _store: Rc<StoreType>,
    onRowActivatedAction: OnRowActivatedAction,
    onSelectedSenders: Vec<Sender<FileChange>>,
    onUnselectedSenders: Vec<Sender<()>>
}

impl<StoreType> FileChangesView<StoreType>
    where StoreType: 'static
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        widgetName: &str,
        store: Rc<StoreType>,
        onRowActivatedAction: OnRowActivatedAction)
        -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self {
            _store: store,
            widget: makeView(guiElementProvider, widgetName),
            onRowActivatedAction,
            onSelectedSenders: vec![],
            onUnselectedSenders: vec![]
        }));
        Self::connectSelfToWidget(&newSelf);
        Self::connectSelfToWidgetSelection(&newSelf);
        newSelf
    }

    pub fn isEmpty(&self) -> bool
    {
        self.getModel().get_iter_first().is_none()
    }

    pub fn getData(&self) -> Vec<FileChange>
    {
        let mut content = vec![];
        self.getModel().foreach(|model, _row, iter| {
            content.push(FileChange{
                path: Self::getCell(model, iter, FileChangesColumn::Path),
                status: Self::getCell(model, iter, FileChangesColumn::Status)});
            CONTINUE_ITERATING_MODEL });
        content
    }

    pub fn select(&self, filePath: &str) -> bool
    {
        self.invokeForRowWith(
            filePath,
            |view: &gtk::TreeView, _row, iterator| { view.get_selection().select_iter(iterator); })
    }

    pub fn activate(&self, filePath: &str) -> bool
    {
        self.select(filePath);
        self.invokeForRowWith(
            filePath,
            |view: &gtk::TreeView, row, _iterator| { view.row_activated(row, &self.getFilePathColumn()); })
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
        self.widget.get_selection().unselect_all();
    }


    // private

    fn getModel(&self) -> gtk::TreeModel
    {
        self.widget.get_model().unwrap()
    }

    fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: FileChangesColumn) -> String
    {
        model.get_value(iter, column as i32).get::<String>().unwrap()
    }

    fn getFilePathColumn(&self) -> gtk::TreeViewColumn
    {
        self.widget.get_column(FileChangesColumn::Path as i32).unwrap()
    }

    fn connectSelfToWidgetSelection(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().widget.get_selection().connect_changed(
            move |selection| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().notifyBasedOnSelectionChanged(selection);
                }
            }
        );
    }

    #[allow(clippy::panic)]
    fn notifyBasedOnSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        let (rows, model) = selection.get_selected_rows();
        debug_assert!(rows.len() <= 1);
        match rows.get(0) {
            Some(row) => self.notifyOnSelected(&findSelectedFileChange(row, &model)),
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

    fn connectSelfToWidget(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().widget.connect_row_activated(
            move |_view, row, _column| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().onRowActivated(row);
                }
            }
        );
    }

    fn onRowActivated(&self, row: &gtk::TreePath)
    {
        let model = self.widget.get_model().unwrap();
        let iterator = model.get_iter(row).unwrap();
        let fileChange = FileChange{
            path: model.get_value(&iterator, FileChangesColumn::Path as i32).get::<String>().unwrap(),
            status: model.get_value(&iterator, FileChangesColumn::Status as i32).get::<String>().unwrap() };

        (self.onRowActivatedAction)(&fileChange);
    }

    fn invokeForRowWith(
        &self,
        filePath: &str,
        action: impl Fn(&gtk::TreeView, &gtk::TreePath, &gtk::TreeIter))
        -> bool
    {
        let model = self.getModel();
        let mut rowFound = false;
        model.foreach(|model, row, iter| {
            if Self::getCell(model, iter, FileChangesColumn::Path) != filePath {
                return CONTINUE_ITERATING_MODEL; }
            rowFound = true;
            action(&self.widget, row, iter);
            STOP_ITERATING_MODEL
        });
        rowFound
    }
}

fn makeView(guiElementProvider: &GuiElementProvider, widgetName: &str) -> gtk::TreeView
{
    let view = guiElementProvider.get::<gtk::TreeView>(widgetName);
    FileChangesColumn::asArrayOfI32().iter().for_each(|i| setupColumn(*i, &view));
    view
}

fn findSelectedFileChange(row: &gtk::TreePath, model: &gtk::TreeModel) -> FileChange
{
    let iterator = model.get_iter(row).unwrap();
    let path = model.get_value(&iterator, FileChangesColumn::Path as i32).get().unwrap();
    let status = model.get_value(&iterator, FileChangesColumn::Status as i32).get().unwrap();
    FileChange{path, status}
}