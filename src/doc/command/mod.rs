mod add_compile_options;

pub use add_compile_options::AddCompileOptions;

#[derive(Debug, thiserror::Error)]
pub enum CommandParseError {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("expected: {expected:?}, found: {found:?}")]
    UnexpectedToken { expected: String, found: String },
}
