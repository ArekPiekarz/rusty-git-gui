use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_column::FileChangesColumn;
use crate::file_changes_store_entry::FileChangesStoreEntry;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::{IFileChangesStore, OnRefreshedHandler};
use crate::main_context::{attach, makeChannel};

use glib::Sender;
use gtk::GtkListStoreExt as _;
use gtk::prelude::GtkListStoreExtManual as _;
use gtk::TreeModelExt as _;
use itertools::Itertools;
use std::cmp::{Ord, Ordering::Less, Ordering::Equal, Ordering::Greater};

const NO_PARENT: Option<&gtk::TreeIter> = None;


pub struct FileChangesStore
{
    store: gtk::ListStore,
    fileChanges: Vec<FileChange>,
    onRefreshedSenders: Vec<Sender<()>>
}

#[derive(Eq, PartialEq)]
enum LoopControl
{
    Break,
    DoNotBreak
}

impl FileChangesStore
{
    pub fn new(guiElementProvider: &GuiElementProvider, name: &str, fileChanges: &[FileChange]) -> Self
    {
        let newSelf = Self {
            store: guiElementProvider.get::<gtk::ListStore>(name),
            fileChanges: fileChanges.into(),
            onRefreshedSenders: vec![]
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
        let indexAndFileChange = self.fileChanges.iter_mut().enumerate().find(
            |(_index, fileChange)| fileChange.path == fileChangeUpdate.old.path);
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

    pub fn refresh(&mut self, newFileChanges: Vec<FileChange>)
    {
        let mut oldFileChangeIndex = 0;
        for newFileChange in &newFileChanges {
            self.refreshOldFileChanges(&mut oldFileChangeIndex, newFileChange);
        }
        self.removeLeftoverFileChanges(newFileChanges.len());
        self.notifyOnRefreshed();
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

    fn notifyOnRefreshed(&self)
    {
        for sender in &self.onRefreshedSenders {
            sender.send(()).unwrap();
        }
    }

    fn refreshOldFileChanges(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
    {
        loop {
            match self.fileChanges.get(*oldFileChangeIndex) {
                Some(_) => {
                    if self.refreshEntryWhenOldFileChangeIsPresent(oldFileChangeIndex, newFileChange)
                        == LoopControl::Break {
                            break;
                    }
                },
                None => {
                    self.refreshEntryWhenOldFileChangeIsAbsent(oldFileChangeIndex, newFileChange);
                    break;
                }
            }
        }
    }

    fn refreshEntryWhenOldFileChangeIsPresent(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
        -> LoopControl
    {
        let oldFileChange = self.fileChanges.get_mut(*oldFileChangeIndex).unwrap();
        match newFileChange.path.cmp(&oldFileChange.path) {
            Less => {
                self.refreshEntryWhenNewPathIsLessThanOld(oldFileChangeIndex, newFileChange);
                return LoopControl::Break;
            }
            Equal => {
                Self::refreshEntryWhenNewPathIsEqualToOld(&self.store, oldFileChangeIndex, oldFileChange, newFileChange);
                return LoopControl::Break;
            },
            Greater => {
                self.refreshEntryWhenNewPathIsGreaterThanOld(*oldFileChangeIndex);
                return LoopControl::DoNotBreak;
            }
        }
    }

    fn refreshEntryWhenNewPathIsLessThanOld(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
    {
        self.fileChanges.insert(*oldFileChangeIndex, newFileChange.clone());
        self.store.set(
            &self.store.insert(*oldFileChangeIndex as i32),
            &FileChangesColumn::asArrayOfU32(),
            &[&newFileChange.status as &dyn glib::ToValue, &newFileChange.path as &dyn glib::ToValue]);
        *oldFileChangeIndex += 1;
    }

    fn refreshEntryWhenNewPathIsEqualToOld(
        store: &gtk::ListStore,
        oldFileChangeIndex: &mut usize,
        oldFileChange: &mut FileChange,
        newFileChange: &FileChange)
    {
        if newFileChange.status != oldFileChange.status {
            oldFileChange.status = newFileChange.status.clone();
            store.set_value(
                &store.iter_nth_child(NO_PARENT, *oldFileChangeIndex as i32).unwrap(),
                FileChangesColumn::Status as u32,
                &glib::Value::from(&newFileChange.status)
            );
        }

        if newFileChange.oldPath != oldFileChange.oldPath {
            oldFileChange.oldPath = newFileChange.oldPath.clone();
            store.set_value(
                &store.iter_nth_child(NO_PARENT, *oldFileChangeIndex as i32).unwrap(),
                FileChangesColumn::Path as u32,
                &glib::Value::from(&Self::formatFilePath(&newFileChange))
            );
        }

        *oldFileChangeIndex += 1;
    }

    fn refreshEntryWhenNewPathIsGreaterThanOld(&mut self, oldFileChangeIndex: usize)
    {
        self.fileChanges.remove(oldFileChangeIndex);
        self.store.remove(&self.store.iter_nth_child(NO_PARENT, oldFileChangeIndex as i32).unwrap());
    }

    fn refreshEntryWhenOldFileChangeIsAbsent(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
    {
        self.fileChanges.push(newFileChange.clone());
        self.store.set(
            &self.store.append(),
            &FileChangesColumn::asArrayOfU32(),
            &[&newFileChange.status as &dyn glib::ToValue, &newFileChange.path as &dyn glib::ToValue]);
        *oldFileChangeIndex += 1;
    }

    fn removeLeftoverFileChanges(&mut self, newFileChangesSize: usize)
    {
        if newFileChangesSize < self.fileChanges.len() {
            for i in (newFileChangesSize..self.fileChanges.len()).rev() {
                self.fileChanges.pop();
                self.store.remove(&self.store.iter_nth_child(NO_PARENT, i as i32).unwrap());
            }
        }
    }
}

impl IFileChangesStore for FileChangesStore
{
    fn getFileChange(&self, row: usize) -> &FileChange
    {
        self.fileChanges.get(row).unwrap()
    }

    fn getFilePath(&self, row: usize) -> &FilePathStr
    {
        &self.getFileChange(row).path
    }

    fn findFilePath(&self, path: &FilePathStr) -> Option<usize>
    {
        self.fileChanges.iter().enumerate().find(|(_index, fileChange)| fileChange.path == path)
            .map(|(index, _fileChange)| index )
    }

    fn connectOnRefreshed(&mut self, handler: OnRefreshedHandler)
    {
        let (sender, receiver) = makeChannel();
        self.onRefreshedSenders.push(sender);
        attach(receiver, handler);
    }
}