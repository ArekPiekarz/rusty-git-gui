use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;

use gtk::ToggleButtonExt as _;
use gtk::WidgetExt as _;

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
        if repository.isEmpty() {
            widget.set_sensitive(false);
            widget.set_tooltip_text(Some("No commit found to amend."));
        } else {
            widget.set_sensitive(true);
        }

        let newSelf = Self{widget, sender};
        newSelf.connectWidget();
        newSelf
    }

    #[must_use]
    pub fn isEnabled(&self) -> bool
    {
        self.widget.is_sensitive()
    }

    #[must_use]
    pub fn isDisabled(&self) -> bool
    {
        !self.isEnabled()
    }

    #[must_use]
    pub fn isSelected(&self) -> bool
    {
        self.widget.get_active()
    }

    #[must_use]
    pub fn isUnselected(&self) -> bool
    {
        !self.isSelected()
    }

    #[must_use]
    pub fn getTooltip(&self) -> String
    {
        match self.widget.get_tooltip_text() {
            Some(text) => text.into(),
            None => "".into()
        }
    }

    pub fn select(&self)
    {
        self.widget.set_active(true);
    }

    pub fn unselect(&self)
    {
        self.widget.set_active(false);
    }


    // private

    fn connectWidget(&self)
    {
        let eventSender = self.sender.clone();
        self.widget.connect_toggled(move |_checkbox|
            eventSender.send((Source::CommitAmendCheckbox, Event::Toggled)).unwrap());
    }

    fn onToggled(&self)
    {
        if self.isSelected() {
            self.notifyOnSelected();
        } else {
            self.notifyOnUnselected();
        }
    }

    fn onAmendedCommit(&self)
    {
        self.unselect();
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