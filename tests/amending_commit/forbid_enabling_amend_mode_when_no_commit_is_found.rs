use crate::common::gui_assertions::{
    assertCommitAmendCheckboxIsDisabled,
    assertCommitAmendCheckboxIsUnselected,
    assertCommitAmendCheckboxTooltipIs};
use crate::common::setup::{makeGui, setupTest};

use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn forbidEnablingAmendModeWhenNoCommitIsFound()
{
    let repositoryDir = setupTest();
    let repositoryDir = repositoryDir.path().to_owned();

    let gui = makeGui(&repositoryDir);

    assertCommitAmendCheckboxIsDisabled(&gui);
    assertCommitAmendCheckboxIsUnselected(&gui);
    assertCommitAmendCheckboxTooltipIs("No commit found to amend.", &gui);
}
}
