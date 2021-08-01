#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]

use rusty_git_gui::app_setup::{findRepositoryDir, setupGtk, setupPanicHandler};
use rusty_git_gui::event::Sender;
use rusty_git_gui::gui::Gui;
use rusty_git_gui::main_context::makeChannel;
use rusty_git_gui::repository::Repository;

use anyhow::{Context, Result};
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "use_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;


fn main()
{
    setupPanicHandler();
    setupGtk();
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire()
        .unwrap_or_else(|error| panic!("Failed to acquire the main context from glib. Cause: {:?}", error));
    let (sender, receiver) = makeChannel();
    let repository = makeRepository(sender.clone()).unwrap();
    let gui = Gui::new(repository, sender, receiver);
    gui.show();
    gtk::main();
}

fn makeRepository(sender: Sender) -> Result<Rc<RefCell<Repository>>>
{
    Ok(Rc::new(RefCell::new(Repository::new(
        &findRepositoryDir().context("Failed to start the application.")?,
        sender))))
}
