// The silence below is needed because each test file is compiled by "cargo test" as a separate crate,
// but no test file uses every entry from this common module, thus unnecessarily warning
// about unused functions and constants.
#![allow(dead_code)]

pub mod file_change_view_utils;
pub mod gui_assertions;
pub mod gui_interactions;
pub mod repository_assertions;
pub mod repository_status_utils;
pub mod setup;
