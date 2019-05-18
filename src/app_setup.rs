use crate::error_handling::exit;
use gio::ApplicationExt as _;
use std::path::PathBuf;

pub const NO_APP_ARGUMENTS : [String; 0] = [];

pub fn makeGtkApp() -> gtk::Application
{
    let gtkApp = gtk::Application::new("org.rusty-git-gui", gio::ApplicationFlags::default())
        .unwrap_or_else(|e| panic!("Failed to create GTK application: {}", e));
    gtkApp.connect_startup(|_gtkApp| {});
    gtkApp
}

pub fn findRepositoryDir() -> PathBuf
{
    {
        let args = &std::env::args().collect::<Vec<String>>();
        match args.len() {
            1 => std::env::current_dir()
                .map_err(|e| format_err!("Failed to get current directory: {}", e)),
            2 => Ok(PathBuf::from(&args[1])),
            size => Err(format_err!(
                "Too many arguments to the application, expected 0 or 1 (repository directory), instead got {}:\n{:?}",
                size-1, args))
        }
    }.unwrap_or_else(|e| exit(&format!("Failed to find a repository directory. {}", e)))
}