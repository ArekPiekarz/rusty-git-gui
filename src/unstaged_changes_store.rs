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
        let weakSelf = Rc::downgrade(&self);
        repository.connectOnUnstaged(Box::new(move |fileChange| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.onUnstaged(&fileChange);
            }
            glib::Continue(true)
        }));
    }

    fn onUnstaged(&self, fileChange: &FileChange)
    {
        if self.store.containsFilePath(&fileChange.path) {
            return; }

        let newStatus = convertToUnstaged(&fileChange.status);
        self.store.append(&FileChange{status: newStatus, path: fileChange.path.clone()});
    }
}

impl FileChangesStorable for UnstagedChangesStore
{
    fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
    }
}

fn convertToUnstaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("INDEX", "WT")
}