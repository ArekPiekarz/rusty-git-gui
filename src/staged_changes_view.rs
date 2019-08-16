use crate::file_change_column::FileChangeColumn;
use crate::gui_element_provider::GuiElementProvider;
use crate::file_change::{FileChange, StagedFileChanges};
use crate::file_change_store_observer::FileChangeStoreObserver;
use crate::file_change_view_observer::FileChangeViewObserver;
use crate::repository::Repository;
use crate::staged_changes_store::StagedFileChangesStore;
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};
use crate::tree_view_column_setup::setupColumn;

use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::TreeViewExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;


pub struct StagedChangesView
{
    store: Rc<StagedFileChangesStore>,
    widget: gtk::TreeView,
    repository: Rc<Repository>,
    onSelectedObservers: RefCell<Vec<Weak<dyn FileChangeViewObserver>>>,
    onDeselectedObservers: RefCell<Vec<Weak<dyn FileChangeViewObserver>>>,
    onFilledObservers: RefCell<Vec<Weak<dyn FileChangeViewObserver>>>,
    onEmptiedObservers: RefCell<Vec<Weak<dyn FileChangeViewObserver>>>,
}

impl StagedChangesView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        stagedFileChanges: &StagedFileChanges,
        repository: Rc<Repository>)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            store: makeStore(guiElementProvider, stagedFileChanges, &repository),
            widget: makeView(guiElementProvider),
            repository,
            onSelectedObservers: RefCell::new(vec![]),
            onDeselectedObservers: RefCell::new(vec![]),
            onFilledObservers: RefCell::new(vec![]),
            onEmptiedObservers: RefCell::new(vec![])
        });
        Self::connectSelfToStore(&newSelf);
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
                path: Self::getCell(model, iter, FileChangeColumn::Path),
                status: Self::getCell(model, iter, FileChangeColumn::Status)});
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

    pub fn connectOnSelected(&self, observer: Weak<dyn FileChangeViewObserver>)
    {
        self.onSelectedObservers.borrow_mut().push(observer);
    }

    pub fn connectOnDeselected(&self, observer: Weak<dyn FileChangeViewObserver>)
    {
        self.onDeselectedObservers.borrow_mut().push(observer);
    }

    pub fn connectOnFilled(&self, observer: Weak<dyn FileChangeViewObserver>)
    {
        self.onFilledObservers.borrow_mut().push(observer);
    }

    pub fn connectOnEmptied(&self, observer: Weak<dyn FileChangeViewObserver>)
    {
        self.onEmptiedObservers.borrow_mut().push(observer);
    }

    pub fn hasContent(&self) -> bool
    {
        self.store.isFilled()
    }


    // private

    fn getModel(&self) -> gtk::TreeModel
    {
        self.widget.get_model().unwrap()
    }

    fn getCell(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: FileChangeColumn) -> String
    {
        model.get_value(iter, column as i32).get::<String>().unwrap()
    }

    fn getFilePathColumn(&self) -> gtk::TreeViewColumn
    {
        self.widget.get_column(FileChangeColumn::Path as i32).unwrap()
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
                    rcSelf.unstageFileChange(row);
                }
            }
        );
    }

    fn unstageFileChange(&self, row: &gtk::TreePath)
    {
        let model = self.widget.get_model().unwrap();
        let iterator = model.get_iter(row).unwrap();
        let fileChange = FileChange{
            path: model.get_value(&iterator, FileChangeColumn::Path as i32).get::<String>().unwrap(),
            status: model.get_value(&iterator, FileChangeColumn::Status as i32).get::<String>().unwrap() };
        self.repository.unstageFileChange(&fileChange);
        self.store.remove(&iterator);
    }

    fn connectSelfToStore(rcSelf: &Rc<Self>)
    {
        rcSelf.store.connectOnFilled(Rc::downgrade(&(rcSelf.clone() as Rc<dyn FileChangeStoreObserver>)));
        rcSelf.store.connectOnEmptied(Rc::downgrade(&(rcSelf.clone() as Rc<dyn FileChangeStoreObserver>)));
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
            if Self::getCell(model, iter, FileChangeColumn::Path) != filePath {
                return CONTINUE_ITERATING_MODEL; }
            rowFound = true;
            action(&self.widget, &row, &iter);
            STOP_ITERATING_MODEL
        });
        rowFound
    }
}

impl FileChangeStoreObserver for StagedChangesView
{
    fn onFilled(&self)
    {
        for observer in &*self.onFilledObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onFilled();
            }
        }
    }

    fn onEmptied(&self)
    {
        for observer in &*self.onEmptiedObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onEmptied();
            }
        }
    }
}

impl FileChangeViewObserver for StagedChangesView
{
    fn onSelected(&self, _: &FileChange)
    {
        self.widget.get_selection().unselect_all();
    }
}

fn makeStore(builder: &GuiElementProvider, stagedFileChanges: &StagedFileChanges, repository: &Repository)
     -> Rc<StagedFileChangesStore>
{
    StagedFileChangesStore::new(builder, stagedFileChanges, repository)
}

fn makeView(builder: &GuiElementProvider) -> gtk::TreeView
{
    let view = builder.get::<gtk::TreeView>("Staged changes view");
    FileChangeColumn::asArrayOfI32().iter().for_each(|i| setupColumn(*i, &view));
    view
}

fn findSelectedFileChange(row: &gtk::TreePath, model: &gtk::TreeModel) -> FileChange
{
    let iterator = model.get_iter(row).unwrap();
    let path = model.get_value(&iterator, FileChangeColumn::Path as i32).get().unwrap();
    let status = model.get_value(&iterator, FileChangeColumn::Status as i32).get().unwrap();
    FileChange{path, status}
}