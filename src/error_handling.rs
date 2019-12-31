use glib::Cast as _;
use gtk::ContainerExt as _;
use gtk::DialogExt as _;
use gtk::LabelExt as _;
use gtk::MessageDialogExt as _;

const NO_WINDOW_PARENT: Option<&gtk::Window> = None;

#[allow(clippy::print_stdout)]
pub fn printFail(fail: &dyn failure::Fail)
{
    println!("{}", formatFail(fail));
    #[cfg(feature = "use_color_backtrace")]
    {
        if let Some(backtrace) = fail.find_root_cause().backtrace() {
            unsafe {
                color_backtrace::failure::print_backtrace(backtrace, &mut color_backtrace::Settings::new())
                    .unwrap_or_else(|e| println!("Failed to print backtrace: {}", e)); }}
    }
}

pub fn formatFail(fail: &dyn failure::Fail) -> String
{
    let mut result = format!("error: {}", fail);
    for cause in fail.iter_causes() {
        result.push_str(&format!("\n  cause: {}", cause)); }
    result
}

pub fn exit(errorMessage: &str) -> !
{
    let dialog = makeDialog(errorMessage);
    dialog.run();
    panic!("{}", errorMessage);
}

fn makeDialog(errorMessage: &str) -> gtk::MessageDialog
{
    let dialog = gtk::MessageDialog::new(
        NO_WINDOW_PARENT,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        errorMessage);

    let messageArea = dialog.get_message_area()
        .unwrap_or_else(|| panic!("Failed to get message area from dialog"));
    let messageArea = messageArea.downcast_ref::<gtk::Box>()
        .unwrap_or_else(|| panic!("Failed to convert widget into box"));
    let children = messageArea.get_children();
    let child = children.get(0)
        .unwrap_or_else(|| panic!("Failed to get the 0th child of message area"));
    let label = child.downcast_ref::<gtk::Label>()
        .unwrap_or_else(|| panic!("Failed to convert child widget into label"));
    label.set_selectable(true);
    dialog
}