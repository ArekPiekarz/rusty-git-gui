use crate::file_change::{FileChange, UpdatedFileChange};
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;

use std::rc::Rc;


pub struct StagedChangesStore
{
    store: FileChangesStore
}

impl StagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{store: FileChangesStore::new(
            guiElementProvider,
            "Staged changes store",
            repository.getStagedChanges())});
        newSelf.connectSelfToRepository(repository);
        newSelf
    }


    // private

    fn connectSelfToRepository(self: &Rc<Self>, repository: &mut Repository)
    {
        self.connectSelfToRepositoryOnAddedToStaged(repository);
        self.connectSelfToRepositoryOnUpdatedInStaged(repository);
        self.connectSelfToRepositoryOnRemovedFromStaged(repository);
        self.connectSelfToRepositoryOnCommitted(repository);
    }

    fn connectSelfToRepositoryOnAddedToStaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnAddedToStaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onAddedToStaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnUpdatedInStaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnUpdatedInStaged(Box::new(move |updatedFileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onUpdatedInStaged(&updatedFileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRemovedFromStaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnRemovedFromStaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onRemovedFromStaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnCommitted(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(self);
        repository.connectOnCommitted(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onCommitted();
            }
            glib::Continue(true)
        }));
    }

    fn onAddedToStaged(&self, fileChange: &FileChange)
    {
        self.store.append(fileChange);
    }

    fn onUpdatedInStaged(&self, updatedFileChange: &UpdatedFileChange)
    {
        self.store.update(updatedFileChange);
    }

    fn onRemovedFromStaged(&self, fileChange: &FileChange)
    {
        self.store.removeWithPath(&fileChange.path);
    }

    fn onCommitted(&self)
    {
        self.store.clear();
    }
}