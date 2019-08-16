use crate::file_change::{FileChange, StagedFileChanges};
use crate::file_change_store_observer::FileChangeStoreObserver;
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;


pub struct StagedFileChangesStore
{
    store: FileChangesStore,
    onFilledObservers: RefCell<Vec<Weak<dyn FileChangeStoreObserver>>>,
    onEmptiedObservers: RefCell<Vec<Weak<dyn FileChangeStoreObserver>>>
}

impl StagedFileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, changes: &StagedFileChanges, repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            store: FileChangesStore::new(guiElementProvider, "Staged changes store", changes),
            onFilledObservers: RefCell::new(vec![]),
            onEmptiedObservers: RefCell::new(vec![]) });
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
        self.store.isFilled()
    }

    pub fn isEmpty(&self) -> bool
    {
        self.store.isEmpty()
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
        self.store.rowCount()
    }
}

impl RepositoryObserver for StagedFileChangesStore
{
    fn onStaged(&self, fileChange: &FileChange)
    {
        if self.store.containsFilePath(&fileChange.path) {
            return; }

        let newStatus = convertToStaged(&fileChange.status);
        self.store.append(&FileChange{status: newStatus, path: fileChange.path.clone()});

        if self.rowCount() == 1 {
            self.notifyOnFilled(); }
    }

    fn onCommitted(&self)
    {
        self.store.clear();
        self.notifyOnEmptied();
    }
}

pub fn convertToStaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("WT", "INDEX")
}