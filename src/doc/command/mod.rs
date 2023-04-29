mod add_compile_options;
mod add_custom_command;

pub use add_compile_options::AddCompileOptions;

/// CMake command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/manual/cmake-commands.7.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<TN> {
    /// Adds options to the compilation of source files.
    AddCompileOptions(AddCompileOptions<TN>),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandParseError {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("expected: {expected:?}, found: {found:?}")]
    UnexpectedToken { expected: String, found: String },
}
