use crate::event::{Event, Sender, Source};
use crate::file_changes_view::{FileChangesView, OnRowActivatedAction};
use crate::gui_element_provider::GuiElementProvider;
use crate::unstaged_changes_store::UnstagedChangesStore;

use std::cell::RefCell;
use std::rc::Rc;


pub type UnstagedChangesView = FileChangesView<UnstagedChangesStore>;

pub fn makeUnstagedChangesView(
    guiElementProvider: &GuiElementProvider,
    sender: Sender,
    store: Rc<RefCell<UnstagedChangesStore>>)
    -> UnstagedChangesView
{
    let sender2 = sender.clone();
    let onRowActivatedAction : OnRowActivatedAction = Box::new(move |fileChange|
        sender.send((Source::UnstagedChangesView, Event::StageRequested(fileChange.clone()))).unwrap());

    FileChangesView::new(
        guiElementProvider,
        "Unstaged changes view",
        store,
        onRowActivatedAction,
        sender2,
        Source::UnstagedChangesView
    )
}