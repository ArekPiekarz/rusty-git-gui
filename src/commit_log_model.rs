use crate::commit_log::CommitLog;
use crate::commit_log_column::CommitLogColumn;
use crate::gui_element_provider::GuiElementProvider;
use crate::original_row::OriginalRow;

use gtk::prelude::GtkListStoreExt as _;
use gtk::prelude::GtkListStoreExtManual as _;
use time::{format_description::FormatItem, macros::format_description};
use to_trait::To;

const DATE_TIME_FORMAT: &[FormatItem] =
    format_description!("[day padding:space] [month repr:short] [year] [hour padding:space]:[minute]:[second]");


pub(crate) struct CommitLogModel
{
    store: gtk::ListStore
}

impl CommitLogModel
{
    pub fn new(commitLog: &CommitLog, guiElementProvider: &GuiElementProvider) -> Self
    {
        let newSelf = Self{store: guiElementProvider.get::<gtk::ListStore>("Commit log store")};
        newSelf.storeCommits(commitLog);
        newSelf
    }


    // private

    fn storeCommits(&self, commitLog: &CommitLog)
    {
        self.store.clear();
        for (row, commit) in commitLog.getCommits().iter().enumerate() {
            self.store.set(
                &self.store.append(),
                &[(CommitLogColumn::Summary.into(),     &commit.summary),
                  (CommitLogColumn::Date.into(),        &formatDateTime(&commit.date)),
                  (CommitLogColumn::Author.into(),      &commit.author),
                  (CommitLogColumn::Email.into(),       &commit.email),
                  (CommitLogColumn::OriginalRow.into(), &(row.try_to::<OriginalRow>().unwrap()))]);
        }
    }
}

fn formatDateTime(date: &time::OffsetDateTime) -> String
{
    date.format(DATE_TIME_FORMAT).unwrap()
}
