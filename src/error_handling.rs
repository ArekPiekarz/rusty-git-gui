use gtk::DialogExt as _;

const NO_WINDOW_PARENT: Option<&gtk::Window> = None;

pub fn exit(errorMessage: &str) -> !
{
    let dialog = gtk::MessageDialog::new(
        NO_WINDOW_PARENT,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        errorMessage);
    dialog.run();
    panic!("{}", errorMessage);
}