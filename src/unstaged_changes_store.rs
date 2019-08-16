use crate::error_handling::exit;
use crate::gui_element_provider::GuiElementProvider;
use crate::file_change::{FileChange, UnstagedFileChanges};
use crate::file_change_column::FileChangeColumn;
use crate::repository::Repository;
use crate::repository_observer::RepositoryObserver;
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};

use gtk::TreeModelExt as _;
use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use std::rc::Rc;


pub struct UnstagedFileChangesStore
{
    store: gtk::ListStore
}

impl UnstagedFileChangesStore
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        changes: &UnstagedFileChanges,
        repository: &Repository)
        -> Rc<Self>
    {
        let newSelf = Rc::new(Self{store: guiElementProvider.get::<gtk::ListStore>("Unstaged changes store")});
        newSelf.fillFileChangesStore(&changes);
        newSelf.connectOnUnstaged(repository);
        newSelf
    }

    pub fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
    }


    // private

    fn fillFileChangesStore(&self, fileChanges: &[FileChange])
    {
        let changeInfosForStore = fileChanges.iter().map(
            |changeInfo|
                [&changeInfo.status as &dyn gtk::ToValue,
                    &changeInfo.path as &dyn gtk::ToValue])
            .collect::<Vec<_>>();

        for changeInfo in changeInfosForStore {
            self.store.set(&self.store.append(), &FileChangeColumn::asArrayOfU32(), &changeInfo); };
    }

    fn connectOnUnstaged(self: &Rc<Self>, repository: &Repository)
    {
        let selfAsObserver : Rc<dyn RepositoryObserver> = self.clone();
        repository.connectOnUnstaged(Rc::downgrade(&selfAsObserver));
    }
}

impl RepositoryObserver for UnstagedFileChangesStore
{
    fn onUnstaged(&self, fileChange: &FileChange)
    {
        if containsFilePath(&self.store, &fileChange.path) {
            return; }

        let newStatus = convertToUnstaged(&fileChange.status);
        self.store.set(
            &self.store.append(),
            &FileChangeColumn::asArrayOfU32(),
            &[&newStatus as &dyn gtk::ToValue, &fileChange.path as &dyn gtk::ToValue]);
    }
}

fn containsFilePath(model: &gtk::ListStore, filePath: &str) -> bool
{
    let mut filePathFound = false;
    model.foreach(|model, row, iter| {
        let actualFilePath = model.get_value(iter, FileChangeColumn::Path as i32).get::<String>()
            .unwrap_or_else(|| exit(&format!("Failed to convert value in model to String in row {}", row)));
        if actualFilePath != filePath {
            return CONTINUE_ITERATING_MODEL; }
        filePathFound = true;
        STOP_ITERATING_MODEL });
    filePathFound
}

pub fn convertToUnstaged(fileChangeStatus: &str) -> String
{
    fileChangeStatus.replace("INDEX", "WT")
}