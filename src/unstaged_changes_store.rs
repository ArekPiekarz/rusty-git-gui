use crate::gui_element_provider::GuiElementProvider;
use crate::file_change::{FileChange, UnstagedFileChanges};
use crate::file_changes_store::FileChangesStore;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;

use std::rc::Rc;


pub struct UnstagedFileChangesStore
{
    store: FileChangesStore
}

impl UnstagedFileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, changes: &UnstagedFileChanges, repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{store: FileChangesStore::new(guiElementProvider, "Unstaged changes store", changes)});
        newSelf.connectSelfToRepository(repository);
        newSelf
    }

    pub fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
    }


    // private

    fn connectSelfToRepository(self: &Rc<Self>, repository: &Repository)
    {
        repository.connectOnUnstaged(Rc::downgrade(&(self.clone() as Rc<dyn RepositoryObserver>)));
    }
}

impl RepositoryObserver for UnstagedFileChangesStore
{
    fn onUnstaged(&self, fileChange: &FileChange)
    {
        if self.store.containsFilePath(&fileChange.path) {
            return; }

        let newStatus = convertToUnstaged(&fileChange.status);
        self.store.append(&FileChange{status: newStatus, path: fileChange.path.clone()});
    }
}

pub fn convertToUnstaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("INDEX", "WT")
}