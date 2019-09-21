const DEFAULT_CONTEXT : Option<&glib::MainContext> = None;


pub fn makeChannel<MessageType>() -> (glib::Sender<MessageType>, glib::Receiver<MessageType>)
{
    glib::MainContext::channel(glib::PRIORITY_DEFAULT)
}

pub fn attach<MessageType, HandlerType>(receiver: glib::Receiver<MessageType>, handler: HandlerType)
    where HandlerType: FnMut(MessageType) -> glib::Continue + 'static
{
    receiver.attach(DEFAULT_CONTEXT, handler);
}