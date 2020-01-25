#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::let_underscore_must_use)] // false positives in Debug
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::option_unwrap_used)]
#![allow(clippy::panic)]
#![allow(clippy::result_unwrap_used)]
#![allow(clippy::string_add)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![feature(arbitrary_self_types)]
#[macro_use] extern crate failure;


pub mod app_setup;
pub mod error_handling;
pub mod file_changes_view_entry;
pub mod gui;
pub mod repository;

mod application_window;
mod color;
mod commit_amend_checkbox;
mod commit_button;
mod commit_message_view;
mod diff_and_commit_paned;
mod diff_colorizer;
mod diff_formatter;
mod diff_view;
mod event_constants;
mod file_change;
mod file_changes_column;
mod file_changes_paned;
mod file_changes_store;
mod file_changes_view;
mod file_path;
mod grouped_file_changes;
mod gui_element_provider;
mod ifile_changes_store;
mod line_count;
mod line_number;
mod main_context;
mod main_paned;
mod number_casts;
mod paned;
mod refresh_button;
mod settings;
mod staged_changes;
mod staged_changes_store;
mod staged_changes_view;
mod text_view;
mod tree_model_utils;
mod tree_selection;
mod tree_view;
mod unstaged_changes;
mod unstaged_changes_store;
mod unstaged_changes_view;