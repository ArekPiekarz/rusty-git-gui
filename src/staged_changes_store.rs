use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_store::FileChangesStore;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::IFileChangesStore;
use crate::repository::Repository;

use std::cell::RefCell;
use std::rc::Rc;


pub struct StagedChangesStore
{
    store: FileChangesStore,
    repository: Rc<RefCell<Repository>>
}

impl IEventHandler for StagedChangesStore
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        use crate::event::Event as E;
        match event {
            E::AddedToStaged(fileChange)         => self.onAddedToStaged(fileChange),
            E::AmendedCommit                     => self.onAmendedCommit(),
            E::Committed                         => self.onCommitted(),
            E::Refreshed                         => self.onRefreshed(),
            E::RemovedFromStaged(fileChange)     => self.onRemovedFromStaged(fileChange),
            E::UpdatedInStaged(fileChangeUpdate) => self.onUpdatedInStaged(fileChangeUpdate),
            _ => handleUnknown(source, event)
        }
    }
}

impl StagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender, repository: &Rc<RefCell<Repository>>) -> Self
    {
        Self{
            store: FileChangesStore::new(
                guiElementProvider,
                "Staged changes store",
                Source::StagedChangesStore,
                sender,
                repository.borrow().getStagedChanges()),
            repository: Rc::clone(repository)
        }
    }


    // private

    fn onAddedToStaged(&mut self, fileChange: &FileChange)
    {
        self.store.append(fileChange);
    }

    fn onUpdatedInStaged(&mut self, updatedFileChange: &FileChangeUpdate)
    {
        self.store.update(updatedFileChange);
    }

    fn onRemovedFromStaged(&mut self, fileChange: &FileChange)
    {
        self.store.remove(&fileChange.path);
    }

    fn onCommitted(&mut self)
    {
        self.store.clear();
    }

    fn onAmendedCommit(&mut self)
    {
        self.store.clear();
    }

    fn onRefreshed(&mut self)
    {
        self.store.refresh(self.repository.borrow().getStagedChanges());
    }
}

impl IFileChangesStore for StagedChangesStore
{
    fn getFileChange(&self, row: usize) -> &FileChange
    {
        self.store.getFileChange(row)
    }

    fn getFilePath(&self, row: usize) -> &FilePathStr
    {
        self.store.getFilePath(row)
    }

    fn findFilePath(&self, path: &FilePathStr) -> Option<usize>
    {
        self.store.findFilePath(path)
    }
}