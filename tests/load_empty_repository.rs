#![allow(non_snake_case)]

use rusty_git_gui::app_setup::makeGtkApp;
use rusty_git_gui::gui_setup::buildGui;
use rusty_git_gui::repository::Repository;
use gio::{ApplicationExt as _, ApplicationExtManual as _};
use glib::object::Cast as _;
use gtk::{TextBufferExt as _, TextViewExt as _, TreeModelExt as _, TreeViewExt as _, WidgetExt as _};
use std::path::Path;
use std::process::{Command, Stdio};
use std::rc::Rc;

const EXCLUDE_HIDDEN_CHARACTERS : bool = false;
const NO_ARGUMENTS : [String; 0] = [];

#[test]
fn loadEmptyRepository()
{
    color_backtrace::install();
    let repositoryDir = makeTemporaryDirectory();
    initializeGitRepository(repositoryDir.path());

    let gtkApp = makeGtkApp();
    gtkApp.connect_activate(move |gtkApp| {
        buildGui(gtkApp, Rc::new(Repository::new(repositoryDir.path())));

        let window = getWindow();
        assertUnstagedFilesViewIsEmpty(&window);
        assertStagedFilesViewIsEmpty(&window);
        assertDiffViewIsEmpty(&window);
        assertCommitMessageViewIsEmpty(&window);
        assertCommitButtonIsDisabled(&window);

        window.destroy();
    });
    gtkApp.run(&NO_ARGUMENTS);
}

fn makeTemporaryDirectory() -> tempfile::TempDir
{
    tempfile::tempdir().unwrap_or_else(|e| panic!("Failed to create temporary directory: {}", e))
}

fn initializeGitRepository(repositoryDir: &Path)
{
    let status = Command::new("git").arg("init")
        .current_dir(&repositoryDir).stdout(Stdio::null()).status().unwrap();

    if !status.success() {
        panic!("Failed to initialize git repository with exit status code: {}", status);
    }
}

fn getWindow() -> gtk::Widget
{
    let mut topLevelWindows = gtk::Window::list_toplevels();
    assert_eq!(topLevelWindows.len(), 1);
    topLevelWindows.remove(0)
}

fn assertUnstagedFilesViewIsEmpty(window: &gtk::Widget)
{
    assertFilesViewIsEmpty(window, "Unstaged files view");
}

fn assertStagedFilesViewIsEmpty(window: &gtk::Widget)
{
    assertFilesViewIsEmpty(window, "Staged files view");
}

fn assertFilesViewIsEmpty(window: &gtk::Widget, name: &str)
{
    let widget = gtk_test::find_widget_by_name(window, name).unwrap();
    let treeView = widget.downcast::<gtk::TreeView>().unwrap();
    let model = treeView.get_model().unwrap();
    assert_eq!(model.get_iter_first(), None);
}

fn assertDiffViewIsEmpty(window: &gtk::Widget)
{
    assertTextViewIsEmpty(window, "Diff view");
}

fn assertCommitMessageViewIsEmpty(window: &gtk::Widget)
{
    assertTextViewIsEmpty(window, "Commit message view");
}

fn assertTextViewIsEmpty(window: &gtk::Widget, name: &str)
{
    let widget = gtk_test::find_widget_by_name(window, name).unwrap();
    let textView = widget.downcast::<gtk::TextView>().unwrap();
    let buffer = textView.get_buffer().unwrap();
    let textViewContent =
        buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), EXCLUDE_HIDDEN_CHARACTERS).unwrap();
    assert_eq!(textViewContent, "");
}

fn assertCommitButtonIsDisabled(window: &gtk::Widget)
{
    let widget = gtk_test::find_widget_by_name(window, "Commit button").unwrap();
    let button = widget.downcast::<gtk::Button>().unwrap();
    assert_eq!(button.is_sensitive(), false);

}