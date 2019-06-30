#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]

use rusty_git_gui::app_setup::{findRepositoryDir, Result, setupGtk, setupPanicHandler};
use rusty_git_gui::gui_setup::makeGui;
use rusty_git_gui::repository::Repository;

use std::rc::Rc;


fn main() -> Result<()>
{
    setupPanicHandler();
    setupGtk();
    let repositoryDir = findRepositoryDir()?;
    let gui = makeGui(Rc::new(Repository::new(&repositoryDir)));
    gui.show();
    gtk::main();
    Ok(())
}
