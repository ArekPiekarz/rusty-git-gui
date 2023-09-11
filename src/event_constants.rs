use gtk::glib;

pub(crate) const CONSUME_EVENT: glib::Propagation = glib::Propagation::Stop;
pub(crate) const FORWARD_EVENT: glib::Propagation = glib::Propagation::Proceed;
