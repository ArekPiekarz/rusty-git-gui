use crate::file_changes_view::{FileChangesView, OnRowActivatedAction};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::unstaged_changes_store::UnstagedChangesStore;

use std::cell::RefCell;
use std::rc::Rc;

pub type UnstagedChangesView = FileChangesView<UnstagedChangesStore>;


pub fn makeUnstagedChangesView(
    guiElementProvider: &GuiElementProvider,
    repository: Rc<RefCell<Repository>>)
    -> Rc<RefCell<UnstagedChangesView>>
{
    let repository2 = Rc::clone(&repository);
    let onRowActivatedAction : OnRowActivatedAction =
        Box::new(move |fileChange| repository.borrow_mut().stageFileChange(fileChange));

    let mut repository2 = repository2.borrow_mut();
    FileChangesView::new(
        guiElementProvider,
        "Unstaged changes view",
        UnstagedChangesStore::new(guiElementProvider, &mut repository2),
        onRowActivatedAction)
}