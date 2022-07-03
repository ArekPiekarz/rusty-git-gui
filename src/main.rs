#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]

use rusty_git_gui::app_setup::{findRepositoryDir, setupGtk, setupPanicHandler};
use rusty_git_gui::gui::Gui;

use anyhow::Result;


fn main() -> Result<()>
{
    setupPanicHandler();
    setupGtk();
    let gui = Gui::new(&findRepositoryDir()?);
    gui.show();
    gtk::main();
    Ok(())
}
