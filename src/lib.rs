#![allow(non_snake_case)]
#![feature(arbitrary_self_types)]
#[macro_use] extern crate failure;


pub mod app_setup;
pub mod error_handling;
pub mod file_change;
pub mod gui;
pub mod repository;

mod application_window;
mod color;
mod commit_button;
mod commit_message_view;
mod commit_message_view_observer;
mod diff_line_printer;
mod diff_maker;
mod diff_view;
mod file_change_column;
mod file_change_store_observer;
mod file_change_view_observer;
mod file_changes_store;
mod gui_element_provider;
mod repository_observer;
mod staged_changes_store;
mod staged_changes_view;
mod text_view;
mod text_view_observer;
mod tree_model_constants;
mod tree_view_column_setup;
mod unstaged_changes_store;
mod unstaged_changes_view;