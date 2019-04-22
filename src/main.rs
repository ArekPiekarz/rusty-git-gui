#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]

use rusty_git_gui::app_setup::{findRepositoryDir, makeGtkApp};
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::ApplicationExt as _;
use gio::ApplicationExtManual as _;
use std::rc::Rc;

const NO_ARGUMENTS : [String; 0] = [];

fn main()
{
    color_backtrace::install();
    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(|gtkApp| buildGui(gtkApp, Rc::new(Repository::new(&findRepositoryDir()))));
    gtkApp.run(&NO_ARGUMENTS);
}