use crate::event::{Event, Sender, Source};
use crate::file_change::{FileChange, FileChangeUpdate};
use crate::file_changes_column::FileChangesColumn;
use crate::file_path::FilePathStr;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::IFileChangesStore;
use crate::number_casts::ToI32 as _;

use gtk::glib;
use gtk::prelude::GtkListStoreExt as _;
use gtk::prelude::GtkListStoreExtManual as _;
use gtk::prelude::TreeModelExt as _;
use std::cmp::{Ord, Ordering::Less, Ordering::Equal, Ordering::Greater};

const NO_PARENT: Option<&gtk::TreeIter> = None;


pub(crate) struct FileChangesStore
{
    store: gtk::ListStore,
    fileChanges: Vec<FileChange>,
    source: Source,
    sender: Sender
}

impl FileChangesStore
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        name: &str,
        source: Source,
        sender: Sender,
        fileChanges: &[FileChange]) -> Self
    {
        let newSelf = Self {
            store: guiElementProvider.get::<gtk::ListStore>(name),
            fileChanges: fileChanges.into(),
            source,
            sender
        };
        newSelf.fillFileChangesStore();
        newSelf
    }

    pub fn append(&mut self, fileChange: &FileChange)
    {
        self.fileChanges.push(fileChange.clone());
        self.store.set(
            &self.store.append(),
            &[(FileChangesColumn::Status.into(), &formatStatus(&fileChange.status)),
              (FileChangesColumn::Path.into(),   &fileChange.path)]);
    }

    pub fn update(&mut self, fileChangeUpdate: &FileChangeUpdate)
    {
        let indexAndFileChange = self.fileChanges.iter_mut().enumerate().find(
            |(_index, fileChange)| fileChange.path == fileChangeUpdate.old.path);
        let (index, currentFileChange) = indexAndFileChange.unwrap();
        currentFileChange.status = fileChangeUpdate.new.status.clone();

        self.store.set_value(
            &self.store.iter_nth_child(NO_PARENT, index.toI32()).unwrap(),
            FileChangesColumn::Status.into(),
            &glib::Value::from(&formatStatus(&fileChangeUpdate.new.status)));
    }

    pub fn remove(&mut self, filePath: &FilePathStr)
    {
        let index = self.fileChanges.iter().position(|fileChange| fileChange.path == filePath).unwrap();
        self.fileChanges.remove(index);
        self.store.remove(&self.store.iter_nth_child(NO_PARENT, index.toI32()).unwrap());
    }

    pub fn refresh(&mut self, newFileChanges: &[FileChange])
    {
        let mut oldFileChangeIndex = 0;
        for newFileChange in newFileChanges {
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
        self.fileChanges.iter().for_each(|fileChange| {
            self.store.set(
                &self.store.append(),
                &[(FileChangesColumn::Status.into(), &formatStatus(&fileChange.status)),
                  (FileChangesColumn::Path.into(),   &fileChange.path)]);
        });
    }

    fn notifyOnRefreshed(&self)
    {
        self.sender.send((self.source, Event::Refreshed)).unwrap();
    }

    fn refreshOldFileChanges(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
    {
        loop {
            if self.fileChanges.get(*oldFileChangeIndex).is_some() {
                if self.refreshEntryWhenOldFileChangeIsPresent(oldFileChangeIndex, newFileChange) == LoopControl::Break {
                    break;
                }
            } else {
                self.refreshEntryWhenOldFileChangeIsAbsent(oldFileChangeIndex, newFileChange);
                break;
            }
        }
    }

    fn refreshEntryWhenOldFileChangeIsPresent(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
        -> LoopControl
    {
        let oldFileChange = &mut self.fileChanges[*oldFileChangeIndex];
        match newFileChange.path.cmp(&oldFileChange.path) {
            Less => {
                self.refreshEntryWhenNewPathIsLessThanOld(oldFileChangeIndex, newFileChange);
                LoopControl::Break
            }
            Equal => {
                Self::refreshEntryWhenNewPathIsEqualToOld(&self.store, oldFileChangeIndex, oldFileChange, newFileChange);
                LoopControl::Break
            },
            Greater => {
                self.refreshEntryWhenNewPathIsGreaterThanOld(*oldFileChangeIndex);
                LoopControl::DoNotBreak
            }
        }
    }

    fn refreshEntryWhenNewPathIsLessThanOld(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
    {
        self.fileChanges.insert(*oldFileChangeIndex, newFileChange.clone());
        self.store.set(
            &self.store.insert((*oldFileChangeIndex).toI32()),
            &[(FileChangesColumn::Status.into(), &formatStatus(&newFileChange.status)),
              (FileChangesColumn::Path.into(),   &newFileChange.path)]);
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
                &store.iter_nth_child(NO_PARENT, (*oldFileChangeIndex).toI32()).unwrap(),
                FileChangesColumn::Status.into(),
                &glib::Value::from(&formatStatus(&newFileChange.status))
            );
        }

        if newFileChange.oldPath != oldFileChange.oldPath {
            oldFileChange.oldPath = newFileChange.oldPath.clone();
            store.set_value(
                &store.iter_nth_child(NO_PARENT, (*oldFileChangeIndex).toI32()).unwrap(),
                FileChangesColumn::Path.into(),
                &glib::Value::from(&newFileChange.path)
            );
        }

        *oldFileChangeIndex += 1;
    }

    fn refreshEntryWhenNewPathIsGreaterThanOld(&mut self, oldFileChangeIndex: usize)
    {
        self.fileChanges.remove(oldFileChangeIndex);
        self.store.remove(&self.store.iter_nth_child(NO_PARENT, oldFileChangeIndex.toI32()).unwrap());
    }

    fn refreshEntryWhenOldFileChangeIsAbsent(&mut self, oldFileChangeIndex: &mut usize, newFileChange: &FileChange)
    {
        self.fileChanges.push(newFileChange.clone());
        self.store.set(
            &self.store.append(),
            &[(FileChangesColumn::Status.into(), &formatStatus(&newFileChange.status)),
              (FileChangesColumn::Path.into(),   &newFileChange.path)]);
        *oldFileChangeIndex += 1;
    }

    fn removeLeftoverFileChanges(&mut self, newFileChangesSize: usize)
    {
        if newFileChangesSize < self.fileChanges.len() {
            for row in (newFileChangesSize..self.fileChanges.len()).rev() {
                self.fileChanges.pop();
                self.store.remove(&self.store.iter_nth_child(NO_PARENT, row.toI32()).unwrap());
            }
        }
    }
}

#[derive(Eq, PartialEq)]
enum LoopControl
{
    Break,
    DoNotBreak
}

impl IFileChangesStore for FileChangesStore
{
    fn getFileChange(&self, row: usize) -> &FileChange
    {
        &self.fileChanges[row]
    }

    fn getFilePath(&self, row: usize) -> &FilePathStr
    {
        &self.getFileChange(row).path
    }

    fn findFilePath(&self, path: &FilePathStr) -> Option<usize>
    {
        self.fileChanges.iter().enumerate().find_map(
            |(index, fileChange)| (fileChange.path == path).then_some(index))
    }
}

fn formatStatus(status: &str) -> &str
{
    match status {
        "WT_NEW" | "INDEX_NEW" => "New",
        "WT_MODIFIED" | "INDEX_MODIFIED" => "Modified",
        "WT_DELETED" | "INDEX_DELETED" => "Deleted",
        "WT_RENAMED" | "INDEX_RENAMED" => "Renamed",
        "Added" => "New",
        "Modified" => "Modified",
        _ => panic!("Cannot format unknown status: {}", status)
    }
}
