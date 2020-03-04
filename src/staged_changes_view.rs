use crate::event::{Event, Source, Sender};
use crate::file_changes_view::{FileChangesView, OnRowActivatedAction};
use crate::gui_element_provider::GuiElementProvider;
use crate::staged_changes_store::StagedChangesStore;

use std::cell::RefCell;
use std::rc::Rc;


pub type StagedChangesView = FileChangesView<StagedChangesStore>;

pub fn makeStagedChangesView(
    guiElementProvider: &GuiElementProvider,
    sender: Sender,
    store: Rc<RefCell<StagedChangesStore>>)
    -> Rc<RefCell<StagedChangesView>>
{
    let sender2 = sender.clone();
    let onRowActivatedAction : OnRowActivatedAction = Box::new(move |fileChange|
        sender.send((Source::StagedChangesView, Event::UnstageRequested(fileChange.clone()))).unwrap());

    Rc::new(RefCell::new(FileChangesView::new(
        guiElementProvider,
        "Staged changes view",
        store,
        onRowActivatedAction,
        sender2,
        Source::StagedChangesView
    )))
}