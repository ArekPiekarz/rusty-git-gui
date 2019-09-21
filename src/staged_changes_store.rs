use crate::file_change::FileChange;
use crate::file_changes_storable::FileChangesStorable;
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
        self.connectSelfToRepositoryOnStaged(repository);
        self.connectSelfToRepositoryOnCommitted(repository);
    }

    fn connectSelfToRepositoryOnStaged(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(&self);
        repository.connectOnStaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onStaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToRepositoryOnCommitted(self: &Rc<Self>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(&self);
        repository.connectOnCommitted(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onCommitted();
            }
            glib::Continue(true)
        }));
    }

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

impl FileChangesStorable for StagedChangesStore
{
    fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
    }
}

fn convertToStaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("WT", "INDEX")
}