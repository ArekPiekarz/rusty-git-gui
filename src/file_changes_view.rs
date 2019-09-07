use crate::file_change::FileChange;
use crate::file_changes_column::FileChangesColumn;
use crate::file_changes_view_observer::FileChangesViewObserver;
use crate::file_changes_storable::FileChangesStorable;
use crate::gui_element_provider::GuiElementProvider;
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};
use crate::tree_view_column_setup::setupColumn;

use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::TreeViewExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type OnRowActivatedAction = Box<dyn Fn(&FileChange)>;

pub struct FileChangesView<StoreType>
    where StoreType: FileChangesStorable
{
    widget: gtk::TreeView,
    store: Rc<StoreType>,
    onRowActivatedAction: OnRowActivatedAction,
    onSelectedObservers: RefCell<Vec<Weak<dyn FileChangesViewObserver>>>,
    onDeselectedObservers: RefCell<Vec<Weak<dyn FileChangesViewObserver>>>,
}

impl<StoreType> FileChangesView<StoreType>
    where StoreType: FileChangesStorable + 'static
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        widgetName: &str,
        store: Rc<StoreType>,
        onRowActivatedAction: OnRowActivatedAction)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            store,
            widget: makeView(guiElementProvider, widgetName),
            onRowActivatedAction,
            onSelectedObservers: RefCell::new(vec![]),
            onDeselectedObservers: RefCell::new(vec![]) });
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
            &|view: &gtk::TreeView, _row, iterator| { view.get_selection().select_iter(iterator); })
    }

    pub fn activate(&self, filePath: &str) -> bool
    {
        self.invokeForRowWith(
            filePath,
            &|view: &gtk::TreeView, row, _iterator| { view.row_activated(row, &self.getFilePathColumn()); })
    }

    pub fn connectOnSelected(&self, observer: Weak<dyn FileChangesViewObserver>)
    {
        self.onSelectedObservers.borrow_mut().push(observer);
    }

    pub fn connectOnDeselected(&self, observer: Weak<dyn FileChangesViewObserver>)
    {
        self.onDeselectedObservers.borrow_mut().push(observer);
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

    fn connectSelfToWidgetSelection(rcSelf: &Rc<Self>)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        rcSelf.widget.get_selection().connect_changed(
            move |selection| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.notifyBasedOnSelectionChanged(selection);
                }
            }
        );
    }

    fn notifyBasedOnSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        let (rows, model) = selection.get_selected_rows();
        if rows.is_empty() {
            return self.notifyOnDeselected();
        }
        else if rows.len() > 1 {
            return;
        }

        self.notifyOnSelected(&findSelectedFileChange(&rows[0], &model));
    }

    fn notifyOnSelected(&self, fileChange: &FileChange)
    {
        for observer in &*self.onSelectedObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onSelected(fileChange);
            }
        }
    }

    fn notifyOnDeselected(&self)
    {
        for observer in &*self.onDeselectedObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onDeselected();
            }
        }
    }

    fn connectSelfToWidget(rcSelf: &Rc<Self>)
    {
        let weakSelf = Rc::downgrade(&rcSelf);
        rcSelf.widget.connect_row_activated(
            move |_view, row, _column| {
                if let Some(rcSelf) = weakSelf.upgrade() {
                    rcSelf.onRowActivated(row);
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
        self.store.remove(&iterator);
    }

    fn invokeForRowWith(
        &self,
        filePath: &str,
        action: &impl Fn(&gtk::TreeView, &gtk::TreePath, &gtk::TreeIter))
        -> bool
    {
        let model = self.getModel();
        let mut rowFound = false;
        model.foreach(|model, row, iter| {
            if Self::getCell(model, iter, FileChangesColumn::Path) != filePath {
                return CONTINUE_ITERATING_MODEL; }
            rowFound = true;
            action(&self.widget, &row, &iter);
            STOP_ITERATING_MODEL
        });
        rowFound
    }
}

impl<StoreType> FileChangesViewObserver for FileChangesView<StoreType>
    where StoreType: FileChangesStorable
{
    fn onSelected(&self, _: &FileChange)
    {
        self.widget.get_selection().unselect_all();
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