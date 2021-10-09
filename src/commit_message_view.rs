use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::text_view::{Notifications, TextView};

use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitMessageView
{
    widget: TextView,
    repository: Rc<RefCell<Repository>>,
    stashedMessage: String
}

impl IEventHandler for CommitMessageView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        use crate::event::Event as E;
        match event {
            E::BufferChanged       => self.onBufferChanged(source, event),
            E::CommitAmendDisabled => self.onCommitAmendDisabled(),
            E::CommitAmendEnabled  => self.onCommitAmendEnabled(),
            E::Committed           => self.onCommitted(),
            E::ZoomRequested(_)    => self.onZoomRequested(source, event),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitMessageView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        repository: Rc<RefCell<Repository>>,
        sender: Sender)
        -> Self
    {
        Self{
            widget: TextView::new(
                guiElementProvider,
                "Commit message view",
                sender,
                Source::CommitMessageView,
                Notifications::Enabled),
            repository,
            stashedMessage: "".into()
        }
    }

    pub fn getText(&self) -> String
    {
        self.widget.getText()
    }

    pub fn setText(&self, text: &str)
    {
        self.widget.setText(text);
    }


    // private

    fn onBufferChanged(&mut self, source: Source, event: &Event)
    {
        self.widget.handle(source, event);
    }

    fn onCommitted(&self)
    {
        self.widget.clear();
    }

    fn onCommitAmendEnabled(&mut self)
    {
        self.stashedMessage = self.getText();
        self.setText(&self.repository.borrow().getLastCommitMessage().unwrap());
    }

    fn onCommitAmendDisabled(&mut self)
    {
        self.setText(&self.stashedMessage);
    }

    fn onZoomRequested(&mut self, source: Source, event: &Event)
    {
        self.widget.handle(source, event);
    }
}
