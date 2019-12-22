use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_store::FileChangesStore;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::{IFileChangesStore, OnRefreshedHandler};
use crate::repository::Repository;

use std::cell::RefCell;
use std::rc::Rc;


pub struct UnstagedChangesStore
{
    store: FileChangesStore,
    repository: Rc<RefCell<Repository>>
}

impl UnstagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: Rc<RefCell<Repository>>) -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{
            store: FileChangesStore::new(
                guiElementProvider,
                "Unstaged changes store",
                repository.borrow().getUnstagedChanges()),
            repository: Rc::clone(&repository)
        }));
        Self::connectSelfToRepository(&newSelf, &mut repository.borrow_mut());
        newSelf
    }


    // private

    fn connectSelfToRepository(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        Self::connectSelfToRepositoryOnAddedToUnstaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnUpdatedInUnstaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnRemovedFromUnstaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnRefreshed(rcSelf, repository);
    }

    fn connectSelfToRepositoryOnAddedToUnstaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnAddedToUnstaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onAddedToUnstaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnUpdatedInUnstaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnUpdatedInUnstaged(Box::new(move |updatedFileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onUpdatedInUnstaged(&updatedFileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRemovedFromUnstaged(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnRemovedFromUnstaged(Box::new(move |filePath| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onRemovedFromUnstaged(&filePath);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRefreshed(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnRefreshed(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow_mut().onRefreshed();
            }
            glib::Continue(true)
        }));
    }

    fn onAddedToUnstaged(&mut self, fileChange: &FileChange)
    {
        self.store.append(fileChange);
    }

    fn onUpdatedInUnstaged(&mut self, updatedFileChange: &FileChangeUpdate)
    {
        self.store.update(updatedFileChange);
    }

    fn onRemovedFromUnstaged(&mut self, fileChange: &FileChange)
    {
        self.store.remove(&fileChange.path);
    }

    fn onRefreshed(&mut self)
    {
        self.store.refresh(self.repository.borrow().getUnstagedChanges().to_vec());
    }
}

impl IFileChangesStore for UnstagedChangesStore
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