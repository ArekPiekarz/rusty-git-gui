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
        match event {
            Event::BufferChanged       => self.onBufferChanged(source, event),
            Event::CommitAmendDisabled => self.onCommitAmendDisabled(),
            Event::CommitAmendEnabled  => self.onCommitAmendEnabled(),
            Event::Committed           => self.onCommitted(),
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

    pub fn hasText(&self) -> bool
    {
        !self.getText().is_empty()
    }

    pub fn isEmpty(&self) -> bool
    {
        !self.hasText()
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
        self.setText(&self.repository.borrow().getLastCommitMessage().unwrap().unwrap());
    }

    fn onCommitAmendDisabled(&mut self)
    {
        self.setText(&self.stashedMessage);
    }
}