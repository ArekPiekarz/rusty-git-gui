use crate::commit_log::CommitLog;
use crate::commit_log_column::CommitLogColumn;
use crate::gui_element_provider::GuiElementProvider;

use gtk::prelude::GtkListStoreExt as _;
use gtk::prelude::GtkListStoreExtManual as _;


pub struct CommitLogModel
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
        // for date formatting below, see https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
        for commit in commitLog.getCommits() {
            self.store.set(
                &self.store.append(),
                &[(CommitLogColumn::Summary.into(), &commit.summary),
                  (CommitLogColumn::Date.into(),    &commit.date.format("%_d %b %Y %_H:%M:%S").to_string()),
                  (CommitLogColumn::Author.into(),  &commit.author),
                  (CommitLogColumn::Email.into(),   &commit.email)]);
        }
    }
}
