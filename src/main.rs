#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]

use rusty_git_gui::app_setup::{findRepositoryDir, makeGtkApp, NO_APP_ARGUMENTS, Result, setupPanicHandler};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;

use gio::ApplicationExt as _;
use gio::ApplicationExtManual as _;
use std::rc::Rc;


fn main() -> Result<()>
{
    setupPanicHandler();
    let gtkApp = makeGtkApp();
    let repositoryDir = findRepositoryDir()?;
    gtkApp.connect_activate(move |gtkApp| buildGui(gtkApp, Rc::new(Repository::new(&repositoryDir))));
    gtkApp.run(&NO_APP_ARGUMENTS);
    Ok(())
}