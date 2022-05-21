use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::traits::StackExt;


pub(crate) fn setupMainStack(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let mainStack = guiElementProvider.get::<gtk::Stack>("Main stack");
    mainStack.connect_visible_child_name_notify(move |stack| {
        sender.send((Source::MainStack, Event::StackChildChanged(stack.visible_child_name().unwrap().into()))).unwrap();
    });
}
