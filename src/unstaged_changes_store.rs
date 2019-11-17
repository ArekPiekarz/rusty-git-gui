use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_getter::FileChangesGetter;
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;

use std::cell::RefCell;
use std::rc::Rc;


pub struct UnstagedChangesStore
{
    store: FileChangesStore
}

impl UnstagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository) -> Rc<RefCell<Self>>
    {
        let newSelf = Rc::new(RefCell::new(Self{store: FileChangesStore::new(
            guiElementProvider,
            "Unstaged changes store",
            repository.getUnstagedChanges())}));
        Self::connectSelfToRepository(&newSelf, repository);
        newSelf
    }


    // private

    fn connectSelfToRepository(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        Self::connectSelfToRepositoryOnAddedToUnstaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnUpdatedInUnstaged(rcSelf, repository);
        Self::connectSelfToRepositoryOnRemovedFromUnstaged(rcSelf, repository);
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
}

impl FileChangesGetter for UnstagedChangesStore
{
    fn getFileChange(&self, row: &gtk::TreePath) -> &FileChange
    {
        self.store.getFileChange(row)
    }
}