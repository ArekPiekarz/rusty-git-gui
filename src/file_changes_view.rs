use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::file_change::FileChange;
use crate::file_changes_column::FileChangesColumn;
use crate::gui_element_provider::GuiElementProvider;
use crate::ifile_changes_store::IFileChangesStore;
use crate::tree_model_utils::toRow;
use crate::tree_view::TreeView;

use gtk::gdk;
use gtk::prelude::GtkMenuExt as _;
use gtk::prelude::GtkMenuItemExt as _;
use gtk::prelude::TreeModelExt as _;
use gtk::prelude::TreeSelectionExt as _;
use gtk::prelude::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;

pub type OnRowActivatedAction = Box<dyn Fn(&FileChange)>;

const LEFT_MENU_ITEM_ATTACH: u32 = 0;
const RIGHT_MENU_ITEM_ATTACH: u32 = 1;
const TOP_MENU_ITEM_ATTACH: u32 = 0;
const BOTTOM_MENU_ITEM_ATTACH: u32 = 1;


pub struct FileChangesView<StoreType>
{
    view: TreeView,
    store: Rc<RefCell<StoreType>>,
    onRowActivatedAction: OnRowActivatedAction,
    source: Source,
    sender: Sender
}

impl<StoreType> IEventHandler for FileChangesView<StoreType>
    where StoreType: IFileChangesStore + 'static
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        use crate::event::Event as E;
        match event {
            E::FileChangeSelected(_)       => self.unselectAll(),
            E::Refreshed                   => self.onRefreshed(),
            E::RightClicked(buttonEvent)   => self.onRightClicked(buttonEvent),
            E::RowActivated(rowPath)       => self.onRowActivated(rowPath),
            E::SelectionChanged(selection) => self.notifyBasedOnSelectionChanged(selection),
            _ => handleUnknown(source, event)
        }
    }
}

impl<StoreType> FileChangesView<StoreType>
    where StoreType: IFileChangesStore + 'static
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        widgetName: &str,
        store: Rc<RefCell<StoreType>>,
        onRowActivatedAction: OnRowActivatedAction,
        sender: Sender,
        source: Source)
        -> Self
    {
        let view = TreeView::new(
            guiElementProvider,
            widgetName,
            sender.clone(),
            source,
            &FileChangesColumn::asArrayOfI32());

        Self{
            view,
            store,
            onRowActivatedAction,
            source,
            sender
        }
    }

    pub fn unselectAll(&self)
    {
        self.view.getSelection().unselectAll();
    }

    pub fn trySelectFirst(&self) -> bool
    {
        if let Some(iter) = self.getModel().iter_first() {
            self.view.getSelection().selectByIterator(&iter);
            self.view.focusFirstRow();
            return true;
        }
        false
    }

    pub fn focus(&self)
    {
        self.view.focus();
    }


    // private

    fn getModel(&self) -> gtk::TreeModel
    {
        self.view.getModel()
    }

    fn notifyBasedOnSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        let (rows, _model) = selection.selected_rows();
        match rows.get(0) {
            Some(rowPath) => self.notifyOnSelected(self.store.borrow().getFileChange(toRow(rowPath)).clone()),
            None => self.notifyOnUnselected()
        }
    }

    fn notifyOnSelected(&self, fileChange: FileChange)
    {
        self.sender.send((self.source, Event::FileChangeSelected(fileChange))).unwrap();
    }

    fn notifyOnUnselected(&self)
    {
        self.sender.send((self.source, Event::FileChangeUnselected)).unwrap();
    }

    fn notifyOnRefreshed(&self, fileChangeOpt: Option<FileChange>)
    {
        self.sender.send((self.source, Event::FileChangeRefreshed(fileChangeOpt))).unwrap();
    }

    fn onRowActivated(&self, rowPath: &gtk::TreePath)
    {
        let store = self.store.borrow();
        let fileChange = store.getFileChange(toRow(rowPath));
        (self.onRowActivatedAction)(fileChange);
    }

    fn onRightClicked(&self, event: &gdk::EventButton)
    {
        let (x, y) = event.position();
        if let Some(row) = self.view.getRowAtPosition(x, y) {
            self.onRightClickedRow(row, event);
        }
    }

    fn onRightClickedRow(&self, row: usize, event: &gdk::EventButton)
    {
        let filePath = self.store.borrow().getFilePath(row).to_owned();
        let menu = gtk::Menu::new();
        let menuItem = gtk::MenuItem::with_label("Copy path");
        menuItem.connect_activate(move |_item| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(&filePath);
        });
        menu.attach(
            &menuItem,
            LEFT_MENU_ITEM_ATTACH,
            RIGHT_MENU_ITEM_ATTACH,
            TOP_MENU_ITEM_ATTACH,
            BOTTOM_MENU_ITEM_ATTACH);
        menu.show_all();
        menu.popup_at_pointer(Some(event));
    }

    fn onRefreshed(&self)
    {
        let fileChangeOpt = self.view.getSelection().getSelectedRow()
            .map(|row| self.store.borrow().getFileChange(row).clone());
        self.notifyOnRefreshed(fileChangeOpt);
    }
}
