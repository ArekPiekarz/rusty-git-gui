use crate::common::gui_assertions::assertGuiIsEmpty;
use crate::common::gui_interactions::clickRefreshButton;
use crate::common::repository_assertions::assertRepositoryIsEmpty;
use crate::common::setup::{makeGui, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn refreshEmptyRepository()
{
    let context = glib::MainContext::default();
    let _contextGuard = context.acquire().unwrap();
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let gui = makeGui(&repositoryDir);
    assertRepositoryIsEmpty(&repositoryDir);
    assertGuiIsEmpty(&gui);

    clickRefreshButton(&gui);

    assertRepositoryIsEmpty(&repositoryDir);
    assertGuiIsEmpty(&gui);
}
}
