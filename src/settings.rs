use std::cell::RefCell;

const CONFIG_FILE_NAME: &str = "config.ini";


pub struct Settings
{
    config: RefCell<ini::Ini>,
    savers: Vec<SettingsSaver>
}

type SettingsSaver = Box<dyn Fn(&Settings)>;

impl Settings
{
    pub fn new() -> Self
    {
        Self{
            config: RefCell::new(ini::Ini::load_from_file(CONFIG_FILE_NAME).unwrap_or_default()),
            savers: vec![]
        }
    }

    pub fn addSaver(&mut self, saver: SettingsSaver)
    {
        self.savers.push(saver);
    }

    pub fn getI32(&self, section: &str, key: &str, default: i32) -> i32
    {
        match self.config.borrow().get_from(Some(section), key) {
            Some(value) => value.parse::<i32>().unwrap_or(default),
            None => default
        }
    }

    pub fn setI32(&self, section: &str, key: &str, value: i32)
    {
        self.config.borrow_mut().set_to(Some(section), key.into(), value.to_string());
    }

    pub fn save(&self)
    {
        for saver in &self.savers {
            saver(&self);
        }
        self.config.borrow_mut().write_to_file(CONFIG_FILE_NAME).unwrap();
    }
}