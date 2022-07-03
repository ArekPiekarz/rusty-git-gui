use crate::event::{Event, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::prelude::PanedExt as _;

pub(crate) type PanePosition = i32;


pub(crate) fn setupPane(
    guiElementProvider: &GuiElementProvider,
    name: &'static str,
    position: PanePosition,
    source: Source,
    sender: Sender)
{
    let pane = guiElementProvider.get::<gtk::Paned>(name);
    pane.set_position(position);
    pane.connect_position_notify(move |pane| sender.send((source, Event::PositionChanged(pane.position()))).unwrap());
}
