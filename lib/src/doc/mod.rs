pub mod command;
mod command_scope;
mod token;

use crate::CMakeListsTokens;

use self::command::CommandParseError;

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
            .map(move |(identifier, text_nodes)| match identifier {
                b"add_compile_options" => text_nodes
                    .try_into()
                    .map(Box::new)
                    .map(Command::AddCompileOptions),
                b"add_custom_command" => text_nodes
                    .try_into()
                    .map(Box::new)
                    .map(Command::AddCustomCommand),
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
