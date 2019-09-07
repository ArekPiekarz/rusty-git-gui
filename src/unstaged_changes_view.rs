use crate::file_changes_view::{FileChangesView, OnRowActivatedAction};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::unstaged_changes::UnstagedChanges;
use crate::unstaged_changes_store::UnstagedChangesStore;

use std::rc::Rc;


pub type UnstagedChangesView = FileChangesView<UnstagedChangesStore>;

pub fn makeUnstagedChangesView(
    guiElementProvider: &GuiElementProvider,
    unstagedChanges: &UnstagedChanges,
    repository: Rc<Repository>)
    -> Rc<UnstagedChangesView>
{
    let repository2 = Rc::clone(&repository);
    let onRowActivatedAction : OnRowActivatedAction =
        Box::new(move |fileChange| repository.stageFileChange(fileChange));

    FileChangesView::new(
        guiElementProvider,
        "Unstaged changes view",
        UnstagedChangesStore::new(guiElementProvider, unstagedChanges, &repository2),
        onRowActivatedAction)
}