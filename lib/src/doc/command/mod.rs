mod add_compile_options;
mod add_custom_command;

pub use add_compile_options::AddCompileOptions;

use crate::command::add_custom_command::AddCustomCommand;

/// CMake command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/manual/cmake-commands.7.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<'t> {
    /// Adds options to the compilation of source files.
    AddCompileOptions(AddCompileOptions<'t>),
    /// Add a custom build rule to the generated build system.
    AddCustomCommand(AddCustomCommand<'t>),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandParseError {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("unknown option: {0}")]
    UnknownOption(String),
    #[error("expected: {expected:?}, found: {found:?}")]
    UnexpectedToken { expected: String, found: String },
}
