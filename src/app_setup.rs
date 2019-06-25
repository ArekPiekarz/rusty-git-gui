use failchain::ResultExt as _;
use gio::ApplicationExt as _;
use std::path::PathBuf;
#[cfg(test)] use mocktopus::macros::mockable;


pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to get current directory.")]
    GetCurrentDirFailed,
    #[fail(display = "Failed to find a repository directory.")]
    FindRepositoryDirFailed,
    #[fail(display = "Too many arguments to the application, expected 0 or 1 (repository directory), \
                      instead got {}:\n{:?}", 0, 1)]
    TooManyArgumentsToApp(usize, Vec<String>)
}

impl failchain::ChainErrorKind for ErrorKind
{
    type Error = Error;
}

use ErrorKind::*;


pub const NO_APP_ARGUMENTS : [String; 0] = [];
const NO_APPLICATION_ID : Option<&str> = None;


pub fn setupPanicHandler()
{
    // color-backtrace since version 0.2.0 doesn't print colors by default in IntelliJ IDEA with Rust plugin,
    // therefore we force it to do it even when stderr is not a TTY.
    // See https://github.com/athre0z/color-backtrace/issues/14 for details.
    match term::stderr() {
        Some(stderr) => {
            let output = color_backtrace::ColorizedStderrOutput::new(stderr);
            let settings = color_backtrace::Settings::new().output_stream(Box::new(output));
            color_backtrace::install_with_settings(settings); },
        None => color_backtrace::install() }
}

pub fn makeGtkApp() -> gtk::Application
{
    let gtkApp = gtk::Application::new(NO_APPLICATION_ID, gio::ApplicationFlags::default())
        .unwrap_or_else(|e| panic!("Failed to create GTK application: {}", e));
    gtkApp.connect_startup(|_gtkApp| {});
    gtkApp
}

pub fn findRepositoryDir() -> Result<PathBuf>
{
    (|| -> Result<PathBuf> {
        let args = getAppArguments();
        match args.len() {
            1 => Ok(getCurrentDir()?),
            2 => Ok(PathBuf::from(&args[1])),
            size => Err(TooManyArgumentsToApp(size-1, args.clone()).into())
        }
    })().chain_err(|| FindRepositoryDirFailed)
}


#[cfg_attr(test, mockable)]
fn getAppArguments() -> Vec<String>
{
    std::env::args().collect::<Vec<String>>()
}

#[cfg_attr(test, mockable)]
fn getCurrentDir() -> Result<PathBuf>
{
    std::env::current_dir().chain_err(|| GetCurrentDirFailed)
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::error_handling::formatFail;
    use mocktopus::mocking::*;
    use mocktopus::mocking::MockResult::Return;

    #[test]
    fn findRepositoryDir_shouldReturnCurrentDir_whenNoArgumentToAppIsProvided()
    {
        getAppArguments.mock_safe(|| Return(vec!["/path/to/app".into()]));
        getCurrentDir.mock_safe(|| Return(Ok(PathBuf::from("/current/dir"))));
        assert_eq!(PathBuf::from("/current/dir"), findRepositoryDir().unwrap());
    }

    #[test]
    fn findRepositoryDir_shouldReturnPathFromArgumentToApp_whenOneIsProvided()
    {
        getAppArguments.mock_safe(|| Return(vec!["/path/to/app".into(), "/path/to/repository".into()]));
        assert_eq!(PathBuf::from("/path/to/repository"), findRepositoryDir().unwrap());
    }

    #[test]
    fn findRepositoryDir_shouldReturnTooManyArgumentsError_whenMoreThanOneArgumentIsProvided()
    {
        getAppArguments.mock_safe(
            || Return(vec!["/path/to/app".into(), "/path/to/repository".into(), "unknown_argument".into()]));
        assert_eq!(
            "error: Failed to find a repository directory.\n  \
               cause: Too many arguments to the application, expected 0 or 1 (repository directory), instead got 2:\n\
             [\"/path/to/app\", \"/path/to/repository\", \"unknown_argument\"]",
            formatFail(&findRepositoryDir().unwrap_err()));
    }

    #[test]
    fn findRepositoryDir_shouldReturnCurrentDirError_whenGettingCurrentDirFails()
    {
        getAppArguments.mock_safe(|| Return(vec!["/path/to/app".into()]));
        getCurrentDir.mock_safe(|| Return(Err::<PathBuf,std::io::Error>(std::io::ErrorKind::PermissionDenied.into())
            .chain_err(|| GetCurrentDirFailed)));
        assert_eq!("error: Failed to find a repository directory.\n  \
                      cause: Failed to get current directory.\n  \
                      cause: permission denied",
                   formatFail(&findRepositoryDir().unwrap_err()));
    }
}
