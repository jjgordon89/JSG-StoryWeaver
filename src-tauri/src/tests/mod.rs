//! Test modules for StoryWeaver backend
//! 
//! This module contains comprehensive test suites for validating the functionality,
//! security, and performance of all Tauri command handlers and core systems.

#[cfg(test)]
pub mod integration_commands_tests;

#[cfg(test)]
pub mod command_validation_tests;

#[cfg(test)]
pub mod comprehensive_validation_tests;

#[cfg(test)]
pub mod error_handling_tests;

#[cfg(test)]
pub mod ai_card_filtering_tests;

#[cfg(test)]
pub mod critical_workflows_tests;
