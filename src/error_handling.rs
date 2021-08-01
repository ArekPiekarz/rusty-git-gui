use gtk::glib::Cast as _;
use gtk::prelude::ContainerExt as _;
use gtk::prelude::DialogExt as _;
use gtk::prelude::LabelExt as _;
use gtk::prelude::MessageDialogExt as _;

const NO_WINDOW_PARENT: Option<&gtk::Window> = None;


pub fn exit(errorMessage: &str) -> !
{
    showErrorDialog(errorMessage);
    panic!("{}", errorMessage);
}

pub fn showErrorDialog(message: &str)
{
    let dialog = makeDialog(message);
    dialog.run();
}


// private

fn makeDialog(errorMessage: &str) -> gtk::MessageDialog
{
    let dialog = gtk::MessageDialog::new(
        NO_WINDOW_PARENT,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        errorMessage);

    let messageArea = dialog.message_area();
    let messageArea = messageArea.downcast_ref::<gtk::Box>()
        .unwrap_or_else(|| panic!("Failed to convert widget into box"));
    let children = messageArea.children();
    let child = children.get(0)
        .unwrap_or_else(|| panic!("Failed to get the 0th child of message area"));
    let label = child.downcast_ref::<gtk::Label>()
        .unwrap_or_else(|| panic!("Failed to convert child widget into label"));
    label.set_selectable(true);
    dialog
}
