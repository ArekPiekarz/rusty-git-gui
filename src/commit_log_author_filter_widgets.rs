use crate::commit_log_text_filter_widgets::{CommitLogTextFilterWidgets, WidgetData};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;


pub(crate) struct CommitLogAuthorFilterWidgets
{
    inner: CommitLogTextFilterWidgets
}

impl IEventHandler for CommitLogAuthorFilterWidgets
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::InvalidAuthorTextInputted(e) => self.onInvalidTextInputted(e),
            Event::ValidAuthorTextInputted      => self.onValidTextInputted(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogAuthorFilterWidgets
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        Self{inner: CommitLogTextFilterWidgets::new(
            &WidgetData{name: "Commit log author filter entry", source: Source::CommitLogAuthorFilterEntry},
            &WidgetData{name: "Commit log author filter case button", source: Source::CommitLogAuthorFilterCaseButton},
            &WidgetData{name: "Commit log author filter regex button", source: Source::CommitLogAuthorFilterRegexButton},
            guiElementProvider,
            sender)
        }
    }

    fn onInvalidTextInputted(&mut self, error: &regex::Error)
    {
        self.inner.handle(Source::CommitLogAuthorFilterEntry, &Event::InvalidTextInputted(error.clone()));
    }

    fn onValidTextInputted(&mut self)
    {
        self.inner.handle(Source::CommitLogAuthorFilterEntry, &Event::ValidTextInputted);
    }
}
