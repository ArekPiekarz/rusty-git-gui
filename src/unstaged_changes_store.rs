use crate::file_change::FileChange;
use crate::file_changes_storable::FileChangesStorable;
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
        self.connectSelfToRepositoryOnRemovedFromUnstaged(repository);
    }

    fn connectSelfToRepositoryOnAddedToUnstaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(&self);
        repository.connectOnAddedToUnstaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onAddedToUnstaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnRemovedFromUnstaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(&self);
        repository.connectOnRemovedFromUnstaged(Box::new(move |filePath| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onRemovedFromUnstaged(&filePath);
            }
            glib::Continue(true)
        }));
    }

    fn onAddedToUnstaged(&self, fileChange: &FileChange)
    {
        if self.store.containsFilePath(&fileChange.path) {
            return; }

        let newStatus = convertToUnstaged(&fileChange.status);
        self.store.append(&FileChange{status: newStatus, path: fileChange.path.clone()});
    }

    fn onRemovedFromUnstaged(&self, fileChange: &FileChange)
    {
        self.store.removeWithPath(&fileChange.path);
    }
}

impl FileChangesStorable for UnstagedChangesStore
{
    fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.removeWithIterator(iterator);
    }
}

fn convertToUnstaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("INDEX", "WT")
}