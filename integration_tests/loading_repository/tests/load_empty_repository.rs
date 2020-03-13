#![allow(non_snake_case)]

use common::gui_assertions::assertGuiIsEmpty;
use common::repository_assertions::assertRepositoryIsEmpty;
use common::setup::{makeGui, setupTest};


#[test]
fn loadEmptyRepository()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let gui = makeGui(&repositoryDir);

    assertRepositoryIsEmpty(&repositoryDir);
    assertGuiIsEmpty(&gui);
}