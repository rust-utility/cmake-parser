mod cmake_parse;
mod cmake_positional;
pub mod command;
mod command_scope;
mod token;

use crate::CMakeListsTokens;

pub use cmake_parse::CMakeParse;
pub use cmake_positional::{CMakePositional, Keyword};
use command::CommandParseError;

pub use command::Command;
pub use command_scope::{CommandScope, ToCommandScope};
pub use token::{declarations_by_keywords, TextNodeDeclaration, Token, TokenDeclarations};

pub struct Doc<'t> {
    tokens: CMakeListsTokens<'t>,
}

impl<'t> Doc<'t> {
    pub fn to_commands_iter<'a: 't>(
        &'a self,
    ) -> impl Iterator<Item = Result<Command<'t>, CommandParseError>> {
        self.tokens
            .command_invocations()
            .map(|ci| (ci.identifier, ci.to_text_nodes()))
            .map(move |(identifier, tokens)| match identifier {
                b"add_compile_definitions" => to_command(tokens, Command::AddCompileDefinitions),
                b"add_compile_options" => to_command(tokens, Command::AddCompileOptions),
                b"add_custom_command" => to_command(tokens, Command::AddCustomCommand),
                b"add_custom_target" => to_command(tokens, Command::AddCustomTarget),
                b"add_definitions" => to_command(tokens, Command::AddDefinitions),
                b"add_dependencies" => to_command(tokens, Command::AddDependencies),
                b"add_executable" => to_command(tokens, Command::AddExecutable),
                b"add_library" => to_command(tokens, Command::AddLibrary),
                b"add_link_options" => to_command(tokens, Command::AddLinkOptions),
                b"add_subdirectory" => to_command(tokens, Command::AddSubdirectory),
                b"add_test" => to_command(tokens, Command::AddTest),
                unknown => Err(CommandParseError::UnknownCommand(
                    String::from_utf8_lossy(unknown).to_string(),
                )),
            })
    }

    pub fn commands<'a: 't>(&'a self) -> Result<Vec<Command<'t>>, CommandParseError> {
        self.to_commands_iter().collect()
    }
}

impl<'t> From<CMakeListsTokens<'t>> for Doc<'t> {
    fn from(tokens: CMakeListsTokens<'t>) -> Self {
        Self { tokens }
    }
}
fn to_command<'t, C, F>(tokens: Vec<Token<'t>>, f: F) -> Result<Command<'t>, CommandParseError>
where
    C: CMakeParse<'t>,
    F: Fn(Box<C>) -> Command<'t>,
{
    CMakeParse::complete(&tokens).map(f)
}
