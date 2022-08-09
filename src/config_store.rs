use crate::config::{CommitLogFilters, Config};
use crate::config_path::ConfigPath;
use crate::event::{Event, handleUnknown, IEventHandler, Source};
use crate::pane::PanePosition;

use std::path::PathBuf;


pub(crate) struct ConfigStore
{
    config: Config,
    dirPath: PathBuf,
    filePath: PathBuf
}

impl IEventHandler for ConfigStore
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        use Source as S;
        use Event as E;
        match (source, event) {
            (S::DiffAndCommitPane, E::PositionChanged(position))        => self.onDiffAndCommitPanePositionChanged(*position),
            (S::FileChangesPane,   E::PositionChanged(position))        => self.onFileChangesnPanePositionChanged(*position),
            (S::MainPane,          E::PositionChanged(position))        => self.onMainPanePositionChanged(*position),
            (_,                    E::ActivePageChanged(name))          => self.onMainStackActivePageChanged(name),
            (_,                    E::FiltersUpdated(filters))          => self.onFiltersUpdated(filters),
            (_,                    E::MaximizationChanged(isMaximized)) => self.onMaximizationChanged(*isMaximized),
            (_,                    E::QuitRequested)                    => self.onQuitRequested(),
            _ => handleUnknown(source, event)
        }
    }
}

impl ConfigStore
{
    pub fn new(configPath: &ConfigPath) -> Self
    {
        let dirPath = configPath.getDirPath();
        let filePath = configPath.getFilePath();
        let config = toml::from_str(&std::fs::read_to_string(filePath).unwrap_or_default()).unwrap();
        Self{config, dirPath: dirPath.into(), filePath: filePath.into()}
    }

    pub fn getConfig(&self) -> &Config
    {
        &self.config
    }


    // private

    fn onDiffAndCommitPanePositionChanged(&mut self, position: PanePosition)
    {
        if self.config.diffAndCommitPane.position == position {
            return;
        }
        self.config.diffAndCommitPane.position = position;
    }

    fn onFileChangesnPanePositionChanged(&mut self, position: PanePosition)
    {
        if self.config.fileChangesPane.position == position {
            return;
        }
        self.config.fileChangesPane.position = position;
    }

    fn onMainPanePositionChanged(&mut self, position: PanePosition)
    {
        if self.config.mainPane.position == position {
            return;
        }
        self.config.mainPane.position = position;
    }

    fn onFiltersUpdated(&mut self, filters: &CommitLogFilters)
    {
        self.config.commitLogFilters = filters.clone();
    }

    fn onMaximizationChanged(&mut self, isMaximized: bool)
    {
        if self.config.applicationWindow.isMaximized == isMaximized {
            return;
        }

        self.config.applicationWindow.isMaximized = isMaximized;
    }

    fn onQuitRequested(&self)
    {
        self.saveToFile();
    }

    fn onMainStackActivePageChanged(&mut self, name: &str)
    {
        self.config.mainStack.activePage = name.into();
    }

    fn saveToFile(&self)
    {
        std::fs::create_dir_all(&self.dirPath).unwrap();
        std::fs::write(&self.filePath, toml::to_string(&self.config).unwrap()).unwrap();
    }
}
