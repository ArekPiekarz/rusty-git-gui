use crate::gui_element_provider::GuiElementProvider;
use crate::main_context::{attach, makeChannel};
use crate::repository::Repository;

use glib::Sender;
use gtk::ToggleButtonExt as _;
use gtk::WidgetExt as _;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CommitAmendCheckbox
{
    widget: gtk::CheckButton,
    senders: Senders
}

struct Senders
{
    onSelected: Vec<Sender<()>>,
    onUnselected: Vec<Sender<()>>
}

impl Senders
{
    fn new() -> Self
    {
        Self{ onSelected: vec![], onUnselected: vec![]}
    }
}

type OnSelectedHandler = Box<dyn Fn(()) -> glib::Continue>;
type OnUnselectedHandler = Box<dyn Fn(()) -> glib::Continue>;

impl CommitAmendCheckbox
{
    #[must_use]
    pub fn new(guiElementProvider: &GuiElementProvider, repository: &mut Repository) -> Rc<RefCell<Self>>
    {
        let widget = guiElementProvider.get::<gtk::CheckButton>("Commit amend checkbox");
        if repository.isEmpty() {
            widget.set_sensitive(false);
            widget.set_tooltip_text(Some("No commit found to amend."));
        } else {
            widget.set_sensitive(true);
        }

        let newSelf = Rc::new(RefCell::new(Self{widget, senders: Senders::new()}));
        Self::connectSelfToRepository(&newSelf, repository);
        Self::connectSelfToWidget(&newSelf);
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

    pub fn connectOnSelected(&mut self, handler: OnSelectedHandler)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onSelected.push(sender);
        attach(receiver, handler);
    }

    pub fn connectOnUnselected(&mut self, handler: OnUnselectedHandler)
    {
        let (sender, receiver) = makeChannel();
        self.senders.onUnselected.push(sender);
        attach(receiver, handler);
    }


    // private

    fn connectSelfToRepository(rcSelf: &Rc<RefCell<Self>>, repository: &mut Repository)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        repository.connectOnAmendedCommit(Box::new(move |_| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow().onAmendedCommit();
            }
            glib::Continue(true)
        }));
    }

    fn connectSelfToWidget(rcSelf: &Rc<RefCell<Self>>)
    {
        let weakSelf = Rc::downgrade(rcSelf);
        rcSelf.borrow().widget.connect_toggled(move |_checkbox| {
            if let Some(rcSelf) = weakSelf.upgrade() {
                rcSelf.borrow().onToggled();
            }
        });
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
        for sender in &self.senders.onSelected {
            sender.send(()).unwrap();
        }
    }

    fn notifyOnUnselected(&self)
    {
        for sender in &self.senders.onUnselected {
            sender.send(()).unwrap();
        }
    }
}