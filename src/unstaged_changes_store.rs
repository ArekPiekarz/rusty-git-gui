use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_store::FileChangesStore;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::IFileChangesStore;
use crate::repository::Repository;

use std::cell::RefCell;
use std::rc::Rc;


pub struct UnstagedChangesStore
{
    store: FileChangesStore,
    repository: Rc<RefCell<Repository>>
}

impl IEventHandler for UnstagedChangesStore
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::AddedToUnstaged(fileChange)         => self.onAddedToUnstaged(fileChange),
            Event::RemovedFromUnstaged(fileChange)     => self.onRemovedFromUnstaged(fileChange),
            Event::Refreshed                           => self.onRefreshed(),
            Event::UpdatedInUnstaged(fileChangeUpdate) => self.onUpdatedInUnstaged(fileChangeUpdate),
            _ => handleUnknown(source, event)
        }
    }
}

impl UnstagedChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender, repository: &Rc<RefCell<Repository>>)
        -> Self
    {
        Self{
            store: FileChangesStore::new(
                guiElementProvider,
                "Unstaged changes store",
                Source::UnstagedChangesStore,
                sender,
                repository.borrow().getUnstagedChanges()),
            repository: Rc::clone(repository)
        }
    }


    // private

    fn onAddedToUnstaged(&mut self, fileChange: &FileChange)
    {
        self.store.append(fileChange);
    }

    fn onUpdatedInUnstaged(&mut self, updatedFileChange: &FileChangeUpdate)
    {
        self.store.update(updatedFileChange);
    }

    fn onRemovedFromUnstaged(&mut self, fileChange: &FileChange)
    {
        self.store.remove(&fileChange.path);
    }

    fn onRefreshed(&mut self)
    {
        self.store.refresh(self.repository.borrow().getUnstagedChanges());
    }
}

impl IFileChangesStore for UnstagedChangesStore
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