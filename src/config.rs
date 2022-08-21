use crate::pane::PanePosition;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config
{
    #[serde(default)]
    pub applicationWindow: ApplicationWindow,
    #[serde(default)]
    pub repository: Repository,
    #[serde(default)]
    pub mainStack: MainStack,
    #[serde(default)]
    pub mainPane: MainPane,
    #[serde(default)]
    pub fileChangesPane: FileChangesPane,
    #[serde(default)]
    pub diffAndCommitPane: DiffAndCommitPane,
    #[serde(default)]
    pub commitLogFilters: CommitLogFilters
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ApplicationWindow
{
    pub isMaximized: bool
}

impl Default for ApplicationWindow
{
    fn default() -> Self
    {
        Self{isMaximized: true}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Repository
{
    pub diffContextSize: u32
}

impl Default for Repository
{
    fn default() -> Self
    {
        Self{diffContextSize: 3}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MainStack
{
    pub activePage: String
}

impl Default for MainStack
{
    fn default() -> Self
    {
        Self{activePage: "Current changes".into()}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MainPane
{
    pub position: PanePosition
}

impl Default for MainPane
{
    fn default() -> Self
    {
        Self{position: 200}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FileChangesPane
{
    pub position: PanePosition
}

impl Default for FileChangesPane
{
    fn default() -> Self
    {
        Self{position: 200}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DiffAndCommitPane
{
    pub position: PanePosition
}

impl Default for DiffAndCommitPane
{
    fn default() -> Self
    {
        Self{position: 450}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CommitLogFilters
{
    pub active: usize,
    pub filters: Vec<CommitLogFilter>
}

impl Default for CommitLogFilters
{
    fn default() -> Self
    {
        Self{
            active: 0,
            filters: vec![CommitLogFilter{
                name: "No filter".into(),
                summaryFilter: SummaryFilter::default(),
                authorFilter: AuthorFilter::default()
            }]
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CommitLogFilter
{
    pub name: String,
    #[serde(default)]
    pub summaryFilter: SummaryFilter,
    pub authorFilter: AuthorFilter,
}

pub(crate) type SummaryFilter = TextFilter;
pub(crate) type AuthorFilter = TextFilter;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct TextFilter
{
    pub pattern: String,
    pub caseSensitive: bool,
    pub usesRegex: bool
}
