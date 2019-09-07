use crate::file_change::FileChange;
use crate::file_changes_storable::FileChangesStorable;
use crate::file_changes_store::FileChangesStore;
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;
use crate::staged_changes::StagedChanges;

use std::rc::Rc;


pub struct StagedChangesStore
{
    store: FileChangesStore
}

impl StagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, changes: &StagedChanges, repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{
            store: FileChangesStore::new(guiElementProvider, "Staged changes store", changes)});
        newSelf.connectSelfToRepository(repository);
        newSelf
    }


    // private

    fn connectSelfToRepository(self: &Rc<Self>, repository: &Repository)
    {
        let selfAsObserver : Rc<dyn RepositoryObserver> = self.clone();
        repository.connectOnStaged(Rc::downgrade(&selfAsObserver));
        repository.connectOnCommitted(Rc::downgrade(&selfAsObserver));
    }
}

impl FileChangesStorable for StagedChangesStore
{
    fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
    }
}

impl RepositoryObserver for StagedChangesStore
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

fn convertToStaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("WT", "INDEX")
}