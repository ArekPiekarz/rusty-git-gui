use crate::file_change::{FileChange, StagedFileChanges};
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;

use std::rc::Rc;


pub struct StagedFileChangesStore
{
    store: FileChangesStore
}

impl StagedFileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, changes: &StagedFileChanges, repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            store: FileChangesStore::new(guiElementProvider, "Staged changes store", changes)});
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
        let selfAsObserver : Rc<dyn RepositoryObserver> = self.clone();
        repository.connectOnStaged(Rc::downgrade(&selfAsObserver));
        repository.connectOnCommitted(Rc::downgrade(&selfAsObserver));
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
    }

    fn onCommitted(&self)
    {
        self.store.clear();
    }
}

pub fn convertToStaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("WT", "INDEX")
}