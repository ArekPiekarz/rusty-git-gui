use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_store::FileChangesStore;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::{IFileChangesStore, OnRefreshedHandler};
use crate::repository::Repository;

use std::cell::RefCell;
use std::rc::Rc;


pub struct StagedChangesStore
{
    store: FileChangesStore
}

impl StagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository) -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{store: FileChangesStore::new(
            guiElementProvider,
            "Staged changes store",
            repository.getStagedChanges())}));
        Self::connectSelfToRepository(&newSelf, repository);
        newSelf
    }


    // private

    fn connectSelfToRepository(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        Self::connectSelfToRepositoryOnAddedToStaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnUpdatedInStaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnRemovedFromStaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnCommitted(rcSelf, repository);
    }

    fn connectSelfToRepositoryOnAddedToStaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnAddedToStaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onAddedToStaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnUpdatedInStaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnUpdatedInStaged(Box::new(move |updatedFileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUpdatedInStaged(&updatedFileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRemovedFromStaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnRemovedFromStaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onRemovedFromStaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnCommitted(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnCommitted(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onCommitted();
            }
            glib::Continue(true)
        }));
    }

    fn onAddedToStaged(&mut self, fileChange: &FileChange)
    {
        self.store.append(fileChange);
    }

    fn onUpdatedInStaged(&mut self, updatedFileChange: &FileChangeUpdate)
    {
        self.store.update(updatedFileChange);
    }

    fn onRemovedFromStaged(&mut self, fileChange: &FileChange)
    {
        self.store.remove(&fileChange.path);
    }

    fn onCommitted(&mut self)
    {
        self.store.clear();
    }
}

impl IFileChangesStore for StagedChangesStore
{
    fn getFileChange(&self, row: usize) -> &FileChange
    {
        self.store.getFileChange(row)
    }

    fn getFilePath(&self, row: usize) -> &FilePathStr
    {
        self.store.getFilePath(row)
    }

    fn findFilePath(&self, path: &FilePathStr) -> Option<usize>
    {
        self.store.findFilePath(path)
    }

    fn connectOnRefreshed(&mut self, observer: OnRefreshedHandler)
    {
        self.store.connectOnRefreshed(observer);
    }
}