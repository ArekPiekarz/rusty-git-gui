use crate::file_changes_view::{FileChangesView, OnRowActivatedAction};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::staged_changes_store::StagedChangesStore;

use std::cell::RefCell;
use std::rc::Rc;


pub type StagedChangesView = FileChangesView<StagedChangesStore>;

pub fn makeStagedChangesView(
    guiElementProvider: &GuiElementProvider,
    repository: Rc<RefCell<Repository>>)
    -> Rc<RefCell<StagedChangesView>>
{
    let repository2 = Rc::clone(&repository);
    let onRowActivatedAction : OnRowActivatedAction =
        Box::new(move |fileChange| repository.borrow_mut().unstageFileChange(fileChange));

    let mut repository2 = repository2.borrow_mut();
    FileChangesView::new(
        guiElementProvider,
        "Staged changes view",
        StagedChangesStore::new(guiElementProvider, &mut repository2),
        onRowActivatedAction)
}