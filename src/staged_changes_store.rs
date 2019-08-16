use crate::error_handling::exit;
use crate::file_change::{FileChange, StagedFileChanges};
use crate::file_change_column::FileChangeColumn;
use crate::file_change_store_observer::FileChangeStoreObserver;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};

use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use gtk::TreeModelExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

const NO_PARENT_ITERATOR : Option<&gtk::TreeIter> = None;


pub struct StagedFileChangesStore
{
    store: gtk::ListStore,
    onFilledObservers: RefCell<Vec<Weak<dyn FileChangeStoreObserver>>>,
    onEmptiedObservers: RefCell<Vec<Weak<dyn FileChangeStoreObserver>>>
}

impl StagedFileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, changes: &StagedFileChanges, repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            store: guiElementProvider.get::<gtk::ListStore>("Staged changes store"),
            onFilledObservers: RefCell::new(vec![]),
            onEmptiedObservers: RefCell::new(vec![]) });
        newSelf.fillFileChangesStore(&changes);
        newSelf.connectSelfToRepository(repository);
        newSelf
    }

    pub fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
        if self.isEmpty() {
            self.notifyOnEmptied();
        }
    }

    pub fn isFilled(&self) -> bool
    {
        self.store.get_iter_first().is_some()
    }

    pub fn isEmpty(&self) -> bool
    {
        self.store.get_iter_first().is_none()
    }

    pub fn connectOnFilled(&self, observer: Weak<dyn FileChangeStoreObserver>)
    {
        self.onFilledObservers.borrow_mut().push(observer);
    }

    pub fn connectOnEmptied(&self, observer: Weak<dyn FileChangeStoreObserver>)
    {
        self.onEmptiedObservers.borrow_mut().push(observer);
    }

    // private

    fn fillFileChangesStore(&self, fileChanges: &[FileChange])
    {
        let changeInfosForStore = fileChanges.iter().map(
            |changeInfo|
                [&changeInfo.status as &dyn gtk::ToValue,
                 &changeInfo.path as &dyn gtk::ToValue])
                .collect::<Vec<_>>();

        for changeInfo in changeInfosForStore {
            self.store.set(&self.store.append(), &FileChangeColumn::asArrayOfU32(), &changeInfo); };
    }

    fn connectSelfToRepository(self: &Rc<Self>, repository: &Repository)
    {
        let selfAsObserver : Rc<dyn RepositoryObserver> = self.clone();
        repository.connectOnStaged(Rc::downgrade(&selfAsObserver));
        repository.connectOnCommitted(Rc::downgrade(&selfAsObserver));
    }

    fn notifyOnFilled(&self)
    {
        for observer in &*self.onFilledObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onFilled();
            }
        }
    }

    fn notifyOnEmptied(&self)
    {
        for observer in &*self.onEmptiedObservers.borrow() {
            if let Some(observer) = observer.upgrade() {
                observer.onEmptied();
            }
        }
    }

    fn rowCount(&self) -> i32
    {
        self.store.iter_n_children(NO_PARENT_ITERATOR)
    }
}

impl RepositoryObserver for StagedFileChangesStore
{
    fn onStaged(&self, fileChange: &FileChange)
    {
        if containsFilePath(&self.store, &fileChange.path) {
            return; }

        let newStatus = convertToStaged(&fileChange.status);
        self.store.set(
            &self.store.append(),
            &FileChangeColumn::asArrayOfU32(),
            &[&newStatus as &dyn gtk::ToValue, &fileChange.path as &dyn gtk::ToValue]);

        if self.rowCount() == 1 {
            self.notifyOnFilled(); }
    }

    fn onCommitted(&self)
    {
        self.store.clear();
        self.notifyOnEmptied();
    }
}

fn containsFilePath(model: &gtk::ListStore, filePath: &str) -> bool
{
    let mut filePathFound = false;
    model.foreach(|model, row, iter| {
        let actualFilePath = model.get_value(iter, FileChangeColumn::Path as i32).get::<String>()
            .unwrap_or_else(|| exit(&format!("Failed to convert value in model to String in row {}", row)));
        if actualFilePath != filePath {
            return CONTINUE_ITERATING_MODEL; }
        filePathFound = true;
        STOP_ITERATING_MODEL });
    filePathFound
}

pub fn convertToStaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("WT", "INDEX")
}