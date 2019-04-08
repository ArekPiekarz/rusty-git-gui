#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]

#[macro_use] extern crate failure;

mod converters;
mod diff_line_printer;
mod diff_maker;
mod error_handling;
mod gui;
mod gui_utils;
mod repository;
mod std_utils;

use crate::gui::*;
use crate::repository::*;
use gio::ApplicationExt as _;
use gio::ApplicationExtManual as _;
use std::rc::Rc;

const NO_ARGUMENTS : [String; 0] = [];

fn main()
{
    let gtkApp = makeGtkApp();
    gtkApp.connect_startup(|_gtkApp| {});
    gtkApp.connect_activate(|gtkApp| buildGui(gtkApp, Rc::new(Repository::new())));
    gtkApp.run(&NO_ARGUMENTS);
}

fn makeGtkApp() -> gtk::Application
{
    gtk::Application::new("org.rusty-git-gui", gio::ApplicationFlags::default())
        .unwrap_or_else(|e| panic!("Failed to create GTK application: {}", e))
}