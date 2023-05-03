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
            .map(move |(identifier, tokens)| match identifier {
                b"add_compile_definitions" => to_command(tokens, Command::AddCompileDefinitions),
                b"add_compile_options" => to_command(tokens, Command::AddCompileOptions),
                b"add_custom_command" => to_command(tokens, Command::AddCustomCommand),
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
    C: TryFrom<Vec<Token<'t>>, Error = CommandParseError>,
    F: Fn(Box<C>) -> Command<'t>,
{
    tokens.try_into().map(Box::new).map(f)
}
