#![allow(non_snake_case)]

mod common;

use common::gui_assertions::assertGuiIsEmpty;
use common::gui_interactions::clickRefreshButton;
use common::repository_assertions::assertRepositoryIsEmpty;
use common::setup::{makeGui, setupTest};


#[test]
fn refreshEmptyRepository()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();
    let gui = makeGui(&repositoryDir);
    assertRepositoryIsEmpty(&repositoryDir);
    assertGuiIsEmpty(&gui);

    clickRefreshButton(&gui);

    assertRepositoryIsEmpty(&repositoryDir);
    assertGuiIsEmpty(&gui);
}