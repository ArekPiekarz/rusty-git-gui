use crate::commit_log_text_filter_widgets::{CommitLogTextFilterWidgets, WidgetData};
use crate::event::{Event, handleUnknown, IEventHandler, Sender, Source};
use crate::gui_element_provider::GuiElementProvider;


pub(crate) struct CommitLogSummaryFilterWidgets
{
    inner: CommitLogTextFilterWidgets
}

impl IEventHandler for CommitLogSummaryFilterWidgets
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::InvalidSummaryTextInputted(e) => self.onInvalidTextInputted(e),
            Event::ValidSummaryTextInputted      => self.onValidTextInputted(),
            _ => handleUnknown(source, event)
        }
    }
}

impl CommitLogSummaryFilterWidgets
{
    pub fn new(guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        Self{inner: CommitLogTextFilterWidgets::new(
            &WidgetData{name: "Commit log summary filter entry", source: Source::CommitLogSummaryFilterEntry},
            &WidgetData{name: "Commit log summary filter case button", source: Source::CommitLogSummaryFilterCaseButton},
            &WidgetData{name: "Commit log summary filter regex button", source: Source::CommitLogSummaryFilterRegexButton},
            guiElementProvider,
            sender)
        }
    }

    fn onInvalidTextInputted(&mut self, error: &regex::Error)
    {
        self.inner.handle(Source::CommitLogSummaryFilterEntry, &Event::InvalidTextInputted(error.clone()));
    }

    fn onValidTextInputted(&mut self)
    {
        self.inner.handle(Source::CommitLogSummaryFilterEntry, &Event::ValidTextInputted);
    }
}
