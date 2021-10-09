use std::cell::RefCell;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ToString;


pub struct Settings
{
    config: RefCell<ini::Ini>,
    configFilePath: PathBuf,
    savers: Vec<SettingsSaver>
}

type SettingsSaver = Box<dyn Fn(&Settings)>;

impl Settings
{
    #[must_use]
    pub fn new() -> Self
    {
        let configFilePath = dirs::config_dir().unwrap().join("rusty-git-gui/config.ini");
        Self{
            config: RefCell::new(ini::Ini::load_from_file(&configFilePath).unwrap_or_default()),
            configFilePath,
            savers: vec![]
        }
    }

    pub fn get<T>(&self, section: &str, key: &str, default: T) -> T
        where T: FromStr
    {
        match self.config.borrow().get_from(Some(section), key) {
            Some(value) => value.parse::<T>().unwrap_or(default),
            None => default
        }
    }

    pub fn set<T>(&self, section: &str, key: &str, value: T)
        where T: Copy + ToString
    {
        self.config.borrow_mut().set_to(Some(section), key.into(), value.to_string());
    }

    pub fn addSaver(&mut self, saver: SettingsSaver)
    {
        self.savers.push(saver);
    }

    pub fn save(&self)
    {
        for saver in &self.savers {
            saver(self);
        }
        self.ensureConfigDirExists();
        self.config.borrow_mut().write_to_file(&self.configFilePath).unwrap();
    }


    // private

    fn ensureConfigDirExists(&self)
    {
        let configDir = self.configFilePath.parent().unwrap();
        if !configDir.exists() {
            std::fs::create_dir_all(configDir).unwrap();
        }
    }
}
