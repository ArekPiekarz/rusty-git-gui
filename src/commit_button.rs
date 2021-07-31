use crate::commit_message_reader::CommitMessageReader;
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;

use gtk::prelude::ButtonExt as _;
use gtk::prelude::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;


pub struct CommitButton
{
    widget: gtk::Button,
    repository: Rc<RefCell<Repository>>,
    commitMessageReader: CommitMessageReader,
    sender: Sender,
    areChangesStaged: bool,
    isCommitMessageWritten: bool,
    isCommitAmendEnabled: bool
}

impl IEventHandler for CommitButton
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::AddedToStaged(_)     => self.onAddedToStaged(),
            Event::Clicked              => self.onClicked(),
            Event::CommitAmendDisabled  => self.onCommitAmendDisabled(),
            Event::CommitAmendEnabled   => self.onCommitAmendEnabled(),
            Event::Emptied              => self.onCommitMessageEmptied(),
            Event::Filled               => self.onCommitMessageFilled(),
            Event::RemovedFromStaged(_) => self.onRemovedFromStaged(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitButton
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        commitMessageReader: CommitMessageReader,
        repository: Rc<RefCell<Repository>>,
        sender: Sender)
        -> Self
    {
        let isCommitMessageWritten = commitMessageReader.hasText();
        let areChangesStaged = repository.borrow().hasStagedChanges();
        let newSelf = Self {
            widget: guiElementProvider.get::<gtk::Button>("Commit button"),
            repository,
            commitMessageReader,
            sender,
            areChangesStaged,
            isCommitMessageWritten,
            isCommitAmendEnabled: false
        };
        newSelf.connectWidget();
        newSelf.update();
        newSelf
    }


    // private

    fn connectWidget(&self)
    {
        let sender = self.sender.clone();
        self.widget.connect_clicked(move |_button| {
            sender.send((Source::CommitButton, Event::Clicked)).unwrap();
        });
    }

    fn onAddedToStaged(&mut self)
    {
        if self.areChangesStaged {
            return;
        }
        self.areChangesStaged = true;
        self.update();
    }

    fn onRemovedFromStaged(&mut self)
    {
        if self.repository.borrow().hasStagedChanges() {
            return;
        }
        self.areChangesStaged = false;
        self.update();
    }

    fn onCommitMessageFilled(&mut self)
    {
        self.isCommitMessageWritten = true;
        self.update();
    }

    fn onCommitMessageEmptied(&mut self)
    {
        self.isCommitMessageWritten = false;
        self.update();
    }

    fn onCommitAmendEnabled(&mut self)
    {
        self.isCommitAmendEnabled = true;
        self.update();
    }

    fn onCommitAmendDisabled(&mut self)
    {
        self.isCommitAmendEnabled = false;
        self.update();
    }

    fn onClicked(&mut self)
    {
        self.commit()
    }

    fn update(&self)
    {
        if self.noChangesAreStaged() && self.commitAmendIsDisabled() {
            self.disable();
            self.setTooltip("No changes are staged for commit.");
            return;
        }

        if self.commitMessageIsEmpty() {
            self.disable();
            self.setTooltip("The commit message is empty.");
            return;
        }

        self.enable();
        self.clearTooltip();
    }

    const fn noChangesAreStaged(&self) -> bool
    {
        !self.areChangesStaged
    }

    const fn commitMessageIsEmpty(&self) -> bool
    {
        !self.isCommitMessageWritten
    }

    const fn commitAmendIsEnabled(&self) -> bool
    {
        self.isCommitAmendEnabled
    }

    const fn commitAmendIsDisabled(&self) -> bool
    {
        !self.commitAmendIsEnabled()
    }

    fn enable(&self)
    {
        self.widget.set_sensitive(true);
    }

    fn disable(&self)
    {
        self.widget.set_sensitive(false);
    }

    fn setTooltip(&self, text: &str)
    {
        self.widget.set_tooltip_text(Some(text));
    }

    fn clearTooltip(&self)
    {
        self.widget.set_tooltip_text(None);
    }

    fn commit(&mut self)
    {
        let message = self.commitMessageReader.getText();
        if self.commitAmendIsEnabled() {
            self.sender.send((Source::CommitButton, Event::AmendCommitRequested(message))).unwrap();
        } else {
            self.sender.send((Source::CommitButton, Event::CommitRequested(message))).unwrap();
        }
        self.areChangesStaged = false;
        self.update();
    }
}
