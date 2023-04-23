use std::borrow::Cow;
use std::fmt::{self, Display};

use crate::CMakeListsTokens;

pub mod command;

pub trait TextNode<'tn>: Display {
    fn text_node<T>(bytes: T) -> Self
    where
        T: Into<Cow<'tn, [u8]>>;
}

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

impl<'tn, TN: TextNode<'tn>> From<&'tn CMakeListsTokens<'tn>> for Doc<TN> {
    fn from(value: &'tn CMakeListsTokens<'tn>) -> Self {
        let commands = value
            .command_invocations()
            .filter_map(|ci| match ci.identifier {
                b"add_compile_options" => Some(Command::AddCompileOptions(ci.into())),
                _ => None,
            })
            .collect();
        Self { commands }
    }
}

pub type Utf8Doc<'doc> = Doc<Utf8TextNode<'doc>>;

/// CMake command.
///
/// Reference: <https://cmake.org/cmake/help/v3.0/manual/cmake-commands.7.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<TN> {
    AddCompileOptions(command::AddCompileOptions<TN>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8TextNode<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> TextNode<'a> for Utf8TextNode<'a> {
    fn text_node<T>(bytes: T) -> Self
    where
        T: Into<Cow<'a, [u8]>>,
    {
        Utf8TextNode {
            bytes: bytes.into(),
        }
    }
}

impl<'a> Display for Utf8TextNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}
