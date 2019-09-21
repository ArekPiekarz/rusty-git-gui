#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![feature(try_blocks)]
#[macro_use] extern crate failure;

use rusty_git_gui::app_setup::{findRepositoryDir, setupGtk, setupPanicHandler};
use rusty_git_gui::error_handling::printFail;
use rusty_git_gui::gui::Gui;
use rusty_git_gui::repository::Repository;

use failchain::{ResultExt as _};
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "Failed to start the application.")]
    AppStartFailed
}

impl failchain::ChainErrorKind for ErrorKind
{
    type Error = Error;
}

pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;
use ErrorKind::*;


fn main()
{
    let result : Result<()> = try {
        setupPanicHandler();
        setupGtk();
        let repository = makeRepository()?;
        let gui = Gui::new(repository);
        gui.show();
        gtk::main(); };
    result.unwrap_or_else(|e| printFail(&e));
}

fn makeRepository() -> Result<Rc<RefCell<Repository>>>
{
    Ok(Rc::new(RefCell::new(Repository::new(&findRepositoryDir().chain_err(|| AppStartFailed)?))))
}