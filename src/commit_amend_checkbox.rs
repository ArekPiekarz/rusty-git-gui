use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;

use gtk::prelude::ToggleButtonExt as _;
use gtk::prelude::WidgetExt as _;


pub struct CommitAmendCheckbox
{
    widget: gtk::CheckButton,
    sender: Sender
}

impl IEventHandler for CommitAmendCheckbox
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::AmendedCommit => self.onAmendedCommit(),
            Event::Committed     => self.onCommitted(),
            Event::Toggled       => self.onToggled(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitAmendCheckbox
{
    #[must_use]
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository, sender: Sender) -> Self
    {
        let widget = guiElementProvider.get::<gtk::CheckButton>("Commit amend checkbox");
        let newSelf = Self{widget, sender};
        if repository.isEmpty() {
            newSelf.disable();
        } else {
            newSelf.enable();
        }

        newSelf.connectWidget();
        newSelf
    }


    // private

    #[must_use]
    pub fn isSelected(&self) -> bool
    {
        self.widget.is_active()
    }

    pub fn unselect(&self)
    {
        self.widget.set_active(false);
    }

    #[must_use]
    fn isDisabled(&self) -> bool
    {
        !self.widget.is_sensitive()
    }

    fn enable(&self)
    {
        self.widget.set_sensitive(true);
        self.widget.set_tooltip_text(None);
    }

    fn disable(&self)
    {
        self.widget.set_sensitive(false);
        self.widget.set_tooltip_text(Some("No commit found to amend."));
    }

    fn connectWidget(&self)
    {
        let eventSender = self.sender.clone();
        self.widget.connect_toggled(move |_checkbox|
            eventSender.send((Source::CommitAmendCheckbox, Event::Toggled)).unwrap());
    }

    fn onAmendedCommit(&self)
    {
        self.unselect();
    }

    fn onCommitted(&self)
    {
        if self.isDisabled() {
            self.enable();
        }
    }

    fn onToggled(&self)
    {
        if self.isSelected() {
            self.notifyOnSelected();
        } else {
            self.notifyOnUnselected();
        }
    }

    fn notifyOnSelected(&self)
    {
        self.sender.send((Source::CommitAmendCheckbox, Event::CommitAmendEnabled)).unwrap();
    }

    fn notifyOnUnselected(&self)
    {
        self.sender.send((Source::CommitAmendCheckbox, Event::CommitAmendDisabled)).unwrap();
    }
}
