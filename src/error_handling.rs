use anyhow::Error;
use gtk::glib::Cast as _;
use gtk::prelude::ContainerExt as _;
use gtk::prelude::DialogExt as _;
use gtk::prelude::LabelExt as _;
use gtk::prelude::MessageDialogExt as _;
use std::error::Error as StdError;

const NO_WINDOW_PARENT: Option<&gtk::Window> = None;


#[allow(clippy::print_stdout)]
#[allow(clippy::use_debug)]
pub fn printErr(error: &Error)
{
    println!("{}", formatErr(error));
    println!("\n{:?}", error.backtrace());
}

#[must_use]
pub fn formatErr(error: &Error) -> String
{
    let mut result = String::new();
    result += &format!("Error: {}", error);

    if let Some(cause) = error.source() {
        match cause.source() {
            Some(_) => formatMultipleCauses(cause, &mut result),
            None => formatSingleCause(cause, &mut result)
        }
    }
    result
}

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

fn formatSingleCause(cause: &dyn StdError, result: &mut String)
{
    result.push_str(&format!("\n    Cause: {}", cause));
}

fn formatMultipleCauses(cause: &(dyn StdError + 'static), result: &mut String)
{
    result.push_str("\n    Causes:");
    for (n, causeEntry) in cause.chain().enumerate() {
        result.push_str(&format!("\n    {}: {}", n + 1, causeEntry));
    }
}

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
