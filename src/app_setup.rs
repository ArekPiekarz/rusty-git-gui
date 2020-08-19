use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
#[cfg(test)] use mocktopus::macros::mockable;
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

#[cfg_attr(test, mockable)]
fn getAppArguments() -> Vec<String>
{
    std::env::args().collect()
}

#[cfg_attr(test, mockable)]
fn getCurrentDir() -> Result<PathBuf>
{
    std::env::current_dir().context("Failed to get current directory.")
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::error_handling::formatErr;
    use mocktopus::mocking::*;
    use mocktopus::mocking::MockResult::Return;

    #[test]
    fn findRepositoryDir_shouldReturnCurrentDir_whenNoArgumentToAppIsProvided()
    {
        setupPanicHandler();
        getAppArguments.mock_safe(|| Return(vec!["/path/to/app".into()]));
        getCurrentDir.mock_safe(|| Return(Ok(PathBuf::from("/current/dir"))));
        assert_eq!(PathBuf::from("/current/dir"), findRepositoryDir().unwrap());
    }

    #[test]
    fn findRepositoryDir_shouldReturnPathFromArgumentToApp_whenOneIsProvided()
    {
        setupPanicHandler();
        getAppArguments.mock_safe(|| Return(vec!["/path/to/app".into(), "/path/to/repository".into()]));
        assert_eq!(PathBuf::from("/path/to/repository"), findRepositoryDir().unwrap());
    }

    #[test]
    fn findRepositoryDir_shouldReturnTooManyArgumentsError_whenMoreThanOneArgumentIsProvided()
    {
        setupPanicHandler();
        getAppArguments.mock_safe(
            || Return(vec!["/path/to/app".into(), "/path/to/repository".into(), "unknown_argument".into()]));
        assert_eq!(
            "Error: Failed to find a repository directory.\n    \
                 Cause: Too many arguments to the application, expected 0 or 1 (repository directory), instead got 2:\n    \
                 [\"/path/to/app\", \"/path/to/repository\", \"unknown_argument\"]",
            formatErr(&findRepositoryDir().unwrap_err()));
    }

    #[test]
    fn findRepositoryDir_shouldReturnCurrentDirError_whenGettingCurrentDirFails()
    {
        setupPanicHandler();
        getAppArguments.mock_safe(|| Return(vec!["/path/to/app".into()]));
        getCurrentDir.mock_safe(|| Return(Err::<PathBuf,std::io::Error>(std::io::ErrorKind::PermissionDenied.into())
            .context("Failed to get current directory.")));
        assert_eq!("Error: Failed to find a repository directory.\n    \
                        Causes:\n    \
                        1: Failed to get current directory.\n    \
                        2: permission denied",
                   formatErr(&findRepositoryDir().unwrap_err()));
    }
}
