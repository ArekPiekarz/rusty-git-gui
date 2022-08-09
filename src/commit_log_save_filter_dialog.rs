use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;

use gtk::EditableSignals;
use gtk::traits::{ButtonExt, DialogExt, EntryExt, GtkWindowExt, WidgetExt};


pub(crate) struct CommitLogSaveFilterDialog
{
    widgets: Option<Widgets>,
    sender: Sender
}

impl IEventHandler for CommitLogSaveFilterDialog
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::DialogResponded(response) => self.onDialogResponded(*response),
            Event::OpenDialogRequested       => self.onOpenDialogRequested(),
            Event::TextEntered(text)         => self.onTextEntered(text),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogSaveFilterDialog
{
    pub fn new(sender: Sender) -> Self
    {
        Self{widgets: None, sender}
    }

    fn onOpenDialogRequested(&mut self)
    {
        let guiElementProvider = GuiElementProvider::new(include_str!("commit_log_save_filter_dialog.glade"));
        let filterNameEntry = guiElementProvider.get::<gtk::Entry>("Filter name entry");
        let sender = self.sender.clone();
        filterNameEntry.connect_changed(move |widget|
            sender.send((Source::CommitLogSaveFilterDialogWidget, Event::TextEntered(widget.text().into()))).unwrap());

        let dialog = guiElementProvider.get::<gtk::Dialog>("dialog");
        let sender = self.sender.clone();
        dialog.connect_response(move |_dialog, response| {
            sender.send((Source::CommitLogSaveFilterDialogWidget, Event::DialogResponded(response))).unwrap();
        });

        let saveButton = guiElementProvider.get::<gtk::Button>("Save button");
        let sender = self.sender.clone();
        saveButton.set_sensitive(false);
        saveButton.connect_clicked(move |_button| {
            sender.send((Source::CommitLogSaveFilterDialogWidget, Event::DialogResponded(gtk::ResponseType::Apply)))
                .unwrap();
        });

        let cancelButton = guiElementProvider.get::<gtk::Button>("Cancel button");
        let sender = self.sender.clone();
        cancelButton.connect_clicked(move |_button| {
            sender.send((Source::CommitLogSaveFilterDialogWidget, Event::DialogResponded(gtk::ResponseType::Cancel)))
                .unwrap();
        });

        dialog.set_modal(true);
        dialog.show();

        self.widgets = Some(Widgets{dialog, filterNameEntry, saveButton});
    }

    fn onDialogResponded(&mut self, response: gtk::ResponseType)
    {
        match response {
            gtk::ResponseType::Apply       => self.onSaveDialog(),
            gtk::ResponseType::Cancel      => self.onCancelDialog(),
            gtk::ResponseType::DeleteEvent => self.onDialogDeleted(),
            _ => self.onUnknownDialogResponse(response)
        }
    }

    fn onTextEntered(&mut self, text: &str)
    {
        let saveButton = match &self.widgets {
            Some(widgets) => &widgets.saveButton,
            None => {
                eprintln!("Expected CommitLogSaveFilterDialog::widgets to be filled, but it was empty");
                return;
            }
        };
        saveButton.set_sensitive(!text.is_empty() && text != "No filter");
    }

    fn onSaveDialog(&mut self)
    {
        let widgets = match &self.widgets {
            Some(widgets) => widgets,
            None => {
                eprintln!("Expected CommitLogSaveFilterDialog::widgets to be filled, but it was empty");
                return;
            }
        };

        let filterName = widgets.filterNameEntry.text().to_string();
        self.sender.send((Source::CommitLogSaveFilterDialog, Event::FilterNameChosen(filterName))).unwrap();
        self.close();
    }

    fn onCancelDialog(&mut self)
    {
        self.close();
    }

    fn onDialogDeleted(&mut self)
    {
        self.widgets = None;
    }

    fn onUnknownDialogResponse(&mut self, response: gtk::ResponseType)
    {
        eprintln!("Received unknown dialog response: {:?}", response);
        self.close();
    }

    fn close(&mut self)
    {
        if let Some(widgets) = &self.widgets {
            widgets.dialog.close();
            self.widgets = None;
        }
    }
}

struct Widgets
{
    dialog: gtk::Dialog,
    filterNameEntry: gtk::Entry,
    saveButton: gtk::Button
}
