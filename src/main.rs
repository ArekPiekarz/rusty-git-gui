#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![feature(try_blocks)]

use rusty_git_gui::app_setup::{findRepositoryDir, setupGtk, setupPanicHandler};
use rusty_git_gui::error_handling::printErr;
use rusty_git_gui::event::Sender;
use rusty_git_gui::gui::Gui;
use rusty_git_gui::main_context::makeChannel;
use rusty_git_gui::repository::Repository;

use anyhow::{Context, Result};
use std::cell::RefCell;
use std::rc::Rc;


fn main()
{
    let result : Result<()> = try {
        setupPanicHandler();
        setupGtk();
        let (sender, receiver) = makeChannel();
        let repository = makeRepository(sender.clone())?;
        let gui = Gui::new(repository, sender, receiver);
        gui.show();
        gtk::main(); };
    result.unwrap_or_else(|e| printErr(&e));
}

fn makeRepository(sender: Sender) -> Result<Rc<RefCell<Repository>>>
{
    Ok(Rc::new(RefCell::new(Repository::new(
        &findRepositoryDir().context("Failed to start the application.")?,
        sender))))
}