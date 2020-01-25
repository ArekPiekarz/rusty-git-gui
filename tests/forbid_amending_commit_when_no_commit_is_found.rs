#![allow(non_snake_case)]

mod common;

use common::gui_assertions::{
    assertCommitAmendCheckboxIsDisabled,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitAmendCheckboxTooltipIs};
use common::setup::{makeGui, setupTest};


#[test]
fn forbidAmendingCommitWhenNoCommitIsFound()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let gui = makeGui(&repositoryDir);

    assertCommitAmendCheckboxIsDisabled(&gui);
    assertCommitAmendCheckboxIsUnselected(&gui);
    assertCommitAmendCheckboxTooltipIs("No commit found to amend.", &gui);
}