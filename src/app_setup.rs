use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
#[cfg(feature = "use_color_backtrace")] use color_backtrace::BacktracePrinter;
#[cfg(feature = "use_color_backtrace")] use termcolor::{ColorChoice, StandardStream};


#[cfg(feature = "use_color_backtrace")]
pub fn setupPanicHandler()
{
    BacktracePrinter::default().install(Box::new(StandardStream::stderr(ColorChoice::Always)));
}

#[cfg(not(feature = "use_color_backtrace"))]
pub const fn setupPanicHandler()
{
}

pub fn setupGtk()
{
    gtk::init()
        .unwrap_or_else(|e| panic!("Failed to initialize GTK. Cause: {}", e));
}

pub fn findRepositoryDir() -> Result<PathBuf>
{
    (|| -> Result<PathBuf> {
        let args = getAppArguments();
        match args.len() {
            0 | 1 => Ok(getCurrentDir()?),
            2 => Ok(PathBuf::from(&args[1])),
            size => Err(anyhow!("Too many arguments to the application, expected 0 or 1 (repository directory), \
                                 instead got {}:\n    {:?}", size-1, args))
        }
    })().context("Failed to find a repository directory.")
}


// private

fn getAppArguments() -> Vec<String>
{
    std::env::args().collect()
}

fn getCurrentDir() -> Result<PathBuf>
{
    std::env::current_dir().context("Failed to get current directory.")
}
