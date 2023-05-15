mod add_compile_definitions;
mod add_compile_options;
mod add_custom_command;
mod add_custom_target;
mod custom_command;

pub use add_compile_definitions::AddCompileDefinitions;
pub use add_compile_options::AddCompileOptions;
pub use add_custom_command::AddCustomCommand;
pub use add_custom_target::AddCustomTarget;
pub use custom_command::CustomCommand;

use crate::Token;

/// CMake command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/manual/cmake-commands.7.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<'t> {
    /// Add preprocessor definitions to the compilation of source files.
    AddCompileDefinitions(Box<AddCompileDefinitions<'t>>),
    /// Adds options to the compilation of source files.
    AddCompileOptions(Box<AddCompileOptions<'t>>),
    /// Add a custom build rule to the generated build system.
    AddCustomCommand(Box<AddCustomCommand<'t>>),
    /// Add a target with no output so it will always be built.
    AddCustomTarget(Box<AddCustomTarget<'t>>),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandParseError {
    #[error("required token is missing: {0}")]
    MissingToken(String),
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("unknown option: {0}")]
    UnknownOption(String),
    #[error("expected: {expected:?}, found: {found:?}")]
    UnexpectedToken { expected: String, found: String },
    #[error("token required")]
    TokenRequired,
    #[error("flag option must have no arguments")]
    NotFlag,
    #[error("all arguments must be parsed")]
    Incomplete,
}

pub trait CMakeCommand<'t>: 't + Sized {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError>;

    fn parse_complete(tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        let (result, tokens) = Self::parse(tokens)?;
        if !tokens.is_empty() {
            return Err(CommandParseError::Incomplete);
        }
        Ok(result)
    }

    fn update(
        command: &mut Option<Self>,
        expected: &'static [u8],
        option: &Token<'t>,
        tokens: &[Token<'t>],
    ) -> Result<bool, CommandParseError> {
        if !Self::matches_option(expected, option) {
            return Ok(false);
        }

        *command = Some(Self::parse_complete(tokens)?);

        Ok(true)
    }

    fn init(default_name: &'static [u8], keywords: &mut Vec<&'static [u8]>) -> Option<Self> {
        keywords.push(default_name);
        Self::default_value()
    }

    fn default_value() -> Option<Self> {
        None
    }

    fn matches_option(expected: &'static [u8], option: &Token<'t>) -> bool {
        expected == option.as_bytes()
    }
}

impl<'t> CMakeCommand<'t> for Token<'t> {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        tokens
            .split_first()
            .map(|(first, rest)| (first.clone(), rest))
            .ok_or(CommandParseError::TokenRequired)
    }
}

impl<'t> CMakeCommand<'t> for bool {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        if tokens.is_empty() {
            Ok((true, tokens))
        } else {
            Err(CommandParseError::NotFlag)
        }
    }

    fn default_value() -> Option<Self> {
        Some(false)
    }
}

impl<'t, T> CMakeCommand<'t> for Option<T>
where
    T: CMakeCommand<'t>,
{
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T::parse(tokens).map(|(result, rest)| (Some(result), rest))
    }

    fn init(default_name: &'static [u8], keywords: &mut Vec<&'static [u8]>) -> Option<Self> {
        Some(T::init(default_name, keywords))
    }
}

impl<'t, T> CMakeCommand<'t> for Vec<T>
where
    T: CMakeCommand<'t>,
{
    fn parse<'tv>(
        mut tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        let mut result = vec![];
        loop {
            let (val, new_tokens) = T::parse(tokens)?;
            result.push(val);
            if new_tokens.len() == tokens.len() {
                break;
            }
            tokens = new_tokens;
            if tokens.is_empty() {
                break;
            }
        }
        Ok((result, tokens))
    }

    fn update(
        command: &mut Option<Self>,
        expected: &'static [u8],
        option: &Token<'t>,
        tokens: &[Token<'t>],
    ) -> Result<bool, CommandParseError> {
        if !Self::matches_option(expected, option) {
            return Ok(false);
        }
        let result = Self::parse_complete(tokens)?;
        if let Some(command) = command.as_mut() {
            command.extend(result);
        } else {
            *command = Some(result);
        }
        Ok(true)
    }
}

impl<'t, T1, T2> CMakeCommand<'t> for (T1, T2)
where
    T1: CMakeCommand<'t>,
    T2: CMakeCommand<'t>,
{
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T1::parse(tokens)
            .and_then(|(t1, tokens)| T2::parse(tokens).map(|(t2, tokens)| ((t1, t2), tokens)))
    }
}
