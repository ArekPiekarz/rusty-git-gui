use crate::file_change::FileChange;
use crate::file_changes_storable::FileChangesStorable;
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;
use crate::unstaged_changes::UnstagedChanges;

use std::rc::Rc;


pub struct UnstagedChangesStore
{
    store: FileChangesStore
}

impl UnstagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, changes: &UnstagedChanges, repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{store: FileChangesStore::new(guiElementProvider, "Unstaged changes store", changes)});
        newSelf.connectSelfToRepository(repository);
        newSelf
    }


    // private

    fn connectSelfToRepository(self: &Rc<Self>, repository: &Repository)
    {
        repository.connectOnUnstaged(Rc::downgrade(&(self.clone() as Rc<dyn RepositoryObserver>)));
    }
}

impl FileChangesStorable for UnstagedChangesStore
{
    fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
    }
}

impl RepositoryObserver for UnstagedChangesStore
{
    fn onUnstaged(&self, fileChange: &FileChange)
    {
        if self.store.containsFilePath(&fileChange.path) {
            return; }

        let newStatus = convertToUnstaged(&fileChange.status);
        self.store.append(&FileChange{status: newStatus, path: fileChange.path.clone()});
    }
}

fn convertToUnstaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("INDEX", "WT")
}