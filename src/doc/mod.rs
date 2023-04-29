pub mod command;
mod command_scope;
mod text_node;

use crate::CMakeListsTokens;

use self::command::CommandParseError;

pub use command::Command;
pub use text_node::{TextNode, Utf8TextNode};

pub struct Doc<TN> {
    commands: Vec<Command<TN>>,
}

impl<'tn, TN: TextNode<'tn>> Doc<TN> {
    pub fn commands(&self) -> &[Command<TN>] {
        &self.commands
    }
}

impl<'tn, TN: TextNode<'tn>> IntoIterator for Doc<TN> {
    type Item = Command<TN>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

impl<'tn, TN: TextNode<'tn>> TryFrom<&'tn CMakeListsTokens<'tn>> for Doc<TN> {
    type Error = CommandParseError;

    fn try_from(value: &'tn CMakeListsTokens<'tn>) -> Result<Self, Self::Error> {
        let commands = value
            .command_invocations()
            .map(|ci| (ci.identifier, ci.to_text_nodes::<TN>()))
            .map(|(identifier, text_nodes)| match identifier {
                b"add_compile_options" => text_nodes.try_into().map(Command::AddCompileOptions),
                b"add_custom_command" => text_nodes.try_into().map(Command::AddCustomCommand),
                unknown => Err(CommandParseError::UnknownCommand(
                    String::from_utf8_lossy(unknown).to_string(),
                )),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { commands })
    }
}

pub type Utf8Doc<'doc> = Doc<Utf8TextNode<'doc>>;
