use crate::error_handling::exit;
use crate::file_change::FileChange;
use crate::file_changes_column::FileChangesColumn;
use crate::gui_element_provider::GuiElementProvider;
use crate::tree_model_constants::{CONTINUE_ITERATING_MODEL, STOP_ITERATING_MODEL};

use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use gtk::TreeModelExt as _;


pub struct FileChangesStore
{
    store: gtk::ListStore
}

impl FileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str, changes: &[FileChange]) -> Self
    {
        let newSelf = Self{store: guiElementProvider.get::<gtk::ListStore>(name)};
        newSelf.fillFileChangesStore(&changes);
        newSelf
    }

    pub fn containsFilePath(&self, filePath: &str) -> bool
    {
        let mut filePathFound = false;
        self.store.foreach(|model, row, iter| {
            let actualFilePath = model.get_value(iter, FileChangesColumn::Path as i32).get::<String>()
                .unwrap_or_else(|| exit(&format!("Failed to convert value in model to String in row {}", row)));
            if actualFilePath != filePath {
                return CONTINUE_ITERATING_MODEL; }
            filePathFound = true;
            STOP_ITERATING_MODEL });
        filePathFound
    }

    pub fn append(&self, fileChange: &FileChange)
    {
        self.store.set(
            &self.store.append(),
            &FileChangesColumn::asArrayOfU32(),
            &[&fileChange.status as &dyn gtk::ToValue, &fileChange.path as &dyn gtk::ToValue]);
    }

    pub fn remove(&self, iterator: &gtk::TreeIter)
    {
        self.store.remove(iterator);
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
            .collect::<Vec<_>>();

        for fileChange in fileChangesForStore {
            self.store.set(&self.store.append(), &FileChangesColumn::asArrayOfU32(), &fileChange); };
    }
}