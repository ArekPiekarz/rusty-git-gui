use crate::error_handling::exit;
use crate::file_change::{FileChange, UpdatedFileChange};
use crate::file_changes_column::FileChangesColumn;
use crate::file_path::{FilePathStr, FilePathString};
use crate::gui_element_provider::GuiElementProvider;
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};

use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use gtk::TreeModelExt as _;
use itertools::Itertools;


pub struct FileChangesStore
{
    store: gtk::ListStore
}

impl FileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str, changes: &[FileChange]) -> Self
    {
        let newSelf = Self{store: guiElementProvider.get::<gtk::ListStore>(name)};
        newSelf.fillFileChangesStore(changes);
        newSelf
    }

    pub fn append(&self, fileChange: &FileChange)
    {
        self.store.set(
            &self.store.append(),
            &FileChangesColumn::asArrayOfU32(),
            &[&fileChange.status as &dyn gtk::ToValue, &fileChange.path as &dyn gtk::ToValue]);
    }

    pub fn update(&self, updatedFileChange: &UpdatedFileChange)
    {
        let mut fileFound = false;
        self.store.foreach(|model, row, iter| {
            if updatedFileChange.old.path != getPath(model, row, iter) {
                return CONTINUE_ITERATING_MODEL;
            }
            self.store.set_value(
                iter, FileChangesColumn::Status as u32, &gtk::Value::from(&updatedFileChange.new.status));
            fileFound = true;
            STOP_ITERATING_MODEL
        });

        if !fileFound {
            exit(&format!("Failed to find file path for updating in store: {}", updatedFileChange.old.path));
        }
    }

    pub fn removeWithPath(&self, filePath: &FilePathStr)
    {
        let mut fileFound = false;
        self.store.foreach(|model, row, iter| {
            let currentFilePath = getPath(model, row, iter);
            if currentFilePath != filePath {
                return CONTINUE_ITERATING_MODEL;
            }
            self.store.remove(iter);
            fileFound = true;
            STOP_ITERATING_MODEL
        });

        if !fileFound {
            exit(&format!("Failed to find file path for removal from store: {}", filePath));
        }
    }

    pub fn clear(&self)
    {
        self.store.clear()
    }


    // private

    fn fillFileChangesStore(&self, fileChanges: &[FileChange])
    {
        let fileChangesForStore = fileChanges.iter().map(
            |fileChange| [&fileChange.status as &dyn gtk::ToValue, &fileChange.path as &dyn gtk::ToValue])
            .collect_vec();

        for fileChange in fileChangesForStore {
            self.store.set(&self.store.append(), &FileChangesColumn::asArrayOfU32(), &fileChange); };
    }
}

fn getPath(model: &gtk::TreeModel, row: &gtk::TreePath, iter: &gtk::TreeIter) -> FilePathString
{
    model.get_value(iter, FileChangesColumn::Path as i32).get::<String>()
        .unwrap_or_else(|| exit(&format!("Failed to convert value in model to String in row {}", row)))
}