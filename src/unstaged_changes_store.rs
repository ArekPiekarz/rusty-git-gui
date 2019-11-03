use crate::file_change::{FileChange, UpdatedFileChange};
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;

use std::rc::Rc;


pub struct UnstagedChangesStore
{
    store: FileChangesStore
}

impl UnstagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{store: FileChangesStore::new(
            guiElementProvider,
            "Unstaged changes store",
            repository.getUnstagedChanges())});
        newSelf.connectSelfToRepository(repository);
        newSelf
    }


    // private

    fn connectSelfToRepository(self: &Rc<Self>, repository: &mut Repository)
    {
        self.connectSelfToRepositoryOnAddedToUnstaged(repository);
        self.connectSelfToRepositoryOnUpdatedInUnstaged(repository);
        self.connectSelfToRepositoryOnRemovedFromUnstaged(repository);
    }

    fn connectSelfToRepositoryOnAddedToUnstaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnAddedToUnstaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onAddedToUnstaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnUpdatedInUnstaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnUpdatedInUnstaged(Box::new(move |updatedFileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onUpdatedInUnstaged(&updatedFileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRemovedFromUnstaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnRemovedFromUnstaged(Box::new(move |filePath| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onRemovedFromUnstaged(&filePath);
            }
            glib::Continue(true)
        }));
    }

    fn onAddedToUnstaged(&self, fileChange: &FileChange)
    {
        self.store.append(fileChange);
    }

    fn onUpdatedInUnstaged(&self, updatedFileChange: &UpdatedFileChange)
    {
        self.store.update(updatedFileChange);
    }

    fn onRemovedFromUnstaged(&self, fileChange: &FileChange)
    {
        self.store.removeWithPath(&fileChange.path);
    }
}