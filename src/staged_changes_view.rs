use crate::file_changes_view::{FileChangesView, OnRowActivatedAction};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes::StagedChanges;
use crate::staged_changes_store::StagedChangesStore;

use std::rc::Rc;


pub type StagedChangesView = FileChangesView<StagedChangesStore>;

pub fn makeStagedChangesView(
    guiElementProvider: &GuiElementProvider,
    stagedChanges: &StagedChanges,
    repository: Rc<Repository>)
    -> Rc<StagedChangesView>
{
    let repository2 = Rc::clone(&repository);
    let onRowActivatedAction : OnRowActivatedAction =
        Box::new(move |fileChange| repository.unstageFileChange(fileChange));

    FileChangesView::new(
        guiElementProvider,
        "Staged changes view",
        StagedChangesStore::new(guiElementProvider, stagedChanges, &repository2),
        onRowActivatedAction)
}