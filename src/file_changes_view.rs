use crate::file_change::FileChange;
use crate::file_changes_column::FileChangesColumn;
use crate::file_changes_view_entry::FileChangesViewEntry;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::IFileChangesStore;
use crate::main_context::{attach, makeChannel};
use crate::tree_model_utils::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL, toRow};
use crate::tree_view::TreeView;

use glib::Sender;
use gtk::GtkMenuExt as _;
use gtk::GtkMenuItemExt as _;
use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;

pub type OnRowActivatedAction = Box<dyn Fn(&FileChange)>;

const LEFT_MENU_ITEM_ATTACH: u32 = 0;
const RIGHT_MENU_ITEM_ATTACH: u32 = 1;
const TOP_MENU_ITEM_ATTACH: u32 = 0;
const BOTTOM_MENU_ITEM_ATTACH: u32 = 1;


pub struct FileChangesView<StoreType>
{
    view: Rc<RefCell<TreeView>>,
    store: Rc<RefCell<StoreType>>,
    onRowActivatedAction: OnRowActivatedAction,
    senders: Senders
}

struct Senders
{
    onSelected: Vec<Sender<FileChange>>,
    onUnselected: Vec<Sender<()>>,
    onRefreshed: Vec<Sender<Option<FileChange>>>
}

impl Senders
{
    fn new() -> Self
    {
        Self{
            onSelected: vec![],
            onUnselected: vec![],
            onRefreshed: vec![]
        }
    }
}

impl<StoreType> FileChangesView<StoreType>
    where StoreType: IFileChangesStore + 'static
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
            senders: Senders::new()
        }));
        Self::connectSelfToViewSelection(&newSelf);
        Self::connectSelfToView(&newSelf);
        Self::connectSelfToStore(&newSelf);

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
        self.senders.onSelected.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnUnselected(&mut self, handler: Box<dyn Fn(()) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onUnselected.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnRefreshed(&mut self, handler: Box<dyn Fn(Option<FileChange>) -> glib::Continue>)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onRefreshed.push(sender);
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
        model.get_value(iter, column.into()).get::<String>().unwrap().unwrap()
    }

    fn getFilePathColumn(&self) -> gtk::TreeViewColumn
    {
        self.view.borrow().getColumn(FileChangesColumn::Path.into())
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

    fn notifyBasedOnSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        let (rows, _model) = selection.get_selected_rows();
        match rows.get(0) {
            Some(rowPath) => self.notifyOnSelected(self.store.borrow().getFileChange(toRow(rowPath))),
            None => self.notifyOnUnselected()
        }
    }

    fn notifyOnSelected(&self, fileChange: &FileChange)
    {
        for sender in &self.senders.onSelected {
            sender.send(fileChange.clone()).unwrap();
        }
    }

    fn notifyOnUnselected(&self)
    {
        for sender in &self.senders.onUnselected {
            sender.send(()).unwrap();
        }
    }

    fn notifyOnRefreshed(&self, fileChangeOpt: &Option<FileChange>)
    {
        for sender in &self.senders.onRefreshed {
            sender.send(fileChangeOpt.clone()).unwrap();
        }
    }

    fn connectSelfToView(rcSelf: &Rc<RefCell<Self>>)
    {
        Self::connectSelfToRowActivated(rcSelf);
        Self::connectSelfToRightClicked(rcSelf);
    }

    fn connectSelfToRowActivated(rcSelf: &Rc<RefCell<Self>>)
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

    fn connectSelfToRightClicked(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().view.borrow_mut().connectOnRightClicked(Box::new(
            move |event| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().onRightClicked(&event);
                }
                glib::Continue(true)
        }));
    }

    fn connectSelfToStore(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().store.borrow_mut().connectOnRefreshed(Box::new(
            move |_| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.borrow().onRefreshed();
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

    fn onRightClicked(&self, event: &gdk::EventButton)
    {
        let (x, y) = event.get_position();
        if let Some(row) = self.view.borrow().getRowAtPosition(x, y) {
            self.onRightClickedRow(row, event);
        }
    }

    fn onRightClickedRow(&self, row: usize, event: &gdk::EventButton)
    {
        let filePath = self.store.borrow().getFilePath(row).to_owned();
        let menu = gtk::Menu::new();
        let menuItem = gtk::MenuItem::new_with_label("Copy path");
        menuItem.connect_activate(move |_item| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(&filePath);
        });
        menu.attach(
            &menuItem,
            LEFT_MENU_ITEM_ATTACH,
            RIGHT_MENU_ITEM_ATTACH,
            TOP_MENU_ITEM_ATTACH,
            BOTTOM_MENU_ITEM_ATTACH);
        menu.show_all();
        menu.popup_at_pointer(Some(event));
    }

    fn onRefreshed(&self)
    {
        let fileChangeOpt = match self.view.borrow().getSelection().borrow().getSelectedRow() {
            Some(row) => Some(self.store.borrow().getFileChange(row).clone()),
            None => None
        };
        self.notifyOnRefreshed(&fileChangeOpt);
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