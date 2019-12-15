use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_column::FileChangesColumn;
use crate::file_changes_getter::FileChangesGetter;
use crate::file_changes_store_entry::FileChangesStoreEntry;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;

use gtk::GtkListStoreExt as _;
use gtk::prelude::GtkListStoreExtManual as _;
use gtk::TreeModelExt as _;
use itertools::Itertools;

const NO_PARENT: Option<&gtk::TreeIter> = None;


pub struct FileChangesStore
{
    store: gtk::ListStore,
    fileChanges: Vec<FileChange>
}

impl FileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str, fileChanges: &[FileChange]) -> Self
    {
        let newSelf = Self {
            store: guiElementProvider.get::<gtk::ListStore>(name),
            fileChanges: fileChanges.into()
        };
        newSelf.fillFileChangesStore();
        newSelf
    }

    pub fn append(&mut self, fileChange: &FileChange)
    {
        self.fileChanges.push(fileChange.clone());
        self.store.set(
            &self.store.append(),
            &FileChangesColumn::asArrayOfU32(),
            &[&fileChange.status as &dyn glib::ToValue, &fileChange.path as &dyn glib::ToValue]);
    }

    pub fn update(&mut self, fileChangeUpdate: &FileChangeUpdate)
    {
        let indexAndFileChange =
            self.fileChanges.iter_mut().enumerate().find(|(_index, fileChange)| fileChange.path == fileChangeUpdate.old.path);
        let (index, currentFileChange) = indexAndFileChange.unwrap();
        currentFileChange.status = fileChangeUpdate.new.status.clone();

        self.store.set_value(
            &self.store.iter_nth_child(NO_PARENT, index as i32).unwrap(),
            FileChangesColumn::Status as u32,
            &glib::Value::from(&fileChangeUpdate.new.status));
    }

    pub fn remove(&mut self, filePath: &FilePathStr)
    {
        let index = self.fileChanges.iter().position(|fileChange| fileChange.path == filePath).unwrap();
        self.fileChanges.remove(index);
        self.store.remove(&self.store.iter_nth_child(NO_PARENT, index as i32).unwrap());
    }

    pub fn replace(&mut self, fileChanges: Vec<FileChange>)
    {
        self.store.clear();
        self.fileChanges = fileChanges;
        self.fillFileChangesStore();
    }

    pub fn clear(&mut self)
    {
        self.fileChanges.clear();
        self.store.clear();
    }


    // private

    fn fillFileChangesStore(&self)
    {
        let fileChangesStoreEntries = self.fileChanges.iter().map(|fileChange| FileChangesStoreEntry {
            status: fileChange.status.clone(),
            path: Self::formatFilePath(fileChange)})
            .collect_vec();

        let internalStoreEntries = fileChangesStoreEntries.iter().map(
            |fileChange| [&fileChange.status as &dyn glib::ToValue, &fileChange.path as &dyn glib::ToValue])
            .collect_vec();

        for fileChange in internalStoreEntries {
            self.store.set(&self.store.append(), &FileChangesColumn::asArrayOfU32(), &fileChange);
        };
    }

    fn formatFilePath(fileChange: &FileChange) -> String
    {
        match fileChange.status.as_str() {
            "WT_RENAMED" => format!("{} -> {}", fileChange.oldPath.as_ref().unwrap(), fileChange.path),
            _ => fileChange.path.clone()
        }
    }
}

impl FileChangesGetter for FileChangesStore
{
    fn getFileChange(&self, row: &gtk::TreePath) -> &FileChange
    {
        self.fileChanges.get(*row.get_indices().get(0).unwrap() as usize).unwrap()
    }
}