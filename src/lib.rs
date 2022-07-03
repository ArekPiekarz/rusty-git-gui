#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::match_bool)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::panic)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::str_to_string)]
#![allow(clippy::string_add)]
#![allow(clippy::unwrap_in_result)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::wildcard_enum_match_arm)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]


pub mod app_setup;
pub mod error_handling;
pub mod event;
pub mod file_changes_view_entry;
pub mod gui;
pub mod main_context;
pub mod tree_model_utils;

mod app_quitter;
mod application_window;
mod color;
mod commit_amend_checkbox;
mod commit_button;
mod commit_diff;
mod commit_diff_view;
mod commit_log;
mod commit_log_author_filter_entry;
mod commit_log_column;
mod commit_log_filters_view;
mod commit_log_model;
mod commit_log_model_filter;
mod commit_log_selections_comparer;
mod commit_log_view;
mod commit_message;
mod commit_message_reader;
mod commit_message_view;
mod config;
mod config_path;
mod config_store;
mod date_time;
mod diff_and_commit_pane;
mod diff_colorizer;
mod diff_formatter;
mod diff_view;
mod event_constants;
mod file_change;
mod file_changes_column;
mod file_changes_pane;
mod file_changes_store;
mod file_changes_view;
mod file_path;
mod grouped_file_changes;
mod gui_element_provider;
mod ifile_changes_store;
mod line_count;
mod line_number;
mod main_pane;
mod main_stack;
mod number_casts;
mod original_row;
mod pane;
mod refresh_button;
mod repository;
mod selections_comparer;
mod show_commit_log_filters_button;
mod staged_changes;
mod staged_changes_store;
mod staged_changes_view;
mod line_diff;
mod text_filter;
mod text_view;
mod tool_bar_stack;
mod tree_selection;
mod tree_view;
mod unstaged_changes;
mod unstaged_changes_store;
mod unstaged_changes_view;
