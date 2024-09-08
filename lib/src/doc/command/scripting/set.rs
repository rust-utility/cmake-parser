use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Set a normal, cache, or environment variable to a given value.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/set.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum Set<'t> {
    Cache(SetCache<'t>),
    Normal(SetNormal<'t>),
}

impl<'t> ToCommandScope for Set<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SetNormal<'t> {
    pub variable: Token<'t>,
    #[cmake(in_range, allow_empty)]
    pub value: Vec<Token<'t>>,
    pub parent_scope: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SetCache<'t> {
    pub variable: Token<'t>,
    #[cmake(in_range)]
    pub value: Vec<Token<'t>>,
    #[cmake(transparent)]
    pub cache: Cache,
    pub docstring: Token<'t>,
    pub force: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Cache {
    Bool,
    #[cmake(rename = "FILEPATH")]
    FilePath,
    Path,
    String,
    Internal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/set");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Set(Box::new(Set::Normal(SetNormal {
                    variable: token(b"var1"),
                    value: tokens_vec([b"value1", b"value2"]),
                    parent_scope: false,
                })))),
                Ok(Command::Set(Box::new(Set::Normal(SetNormal {
                    variable: token(b"var1"),
                    value: tokens_vec([b"value1", b"value2"]),
                    parent_scope: true,
                })))),
                Ok(Command::Set(Box::new(Set::Cache(SetCache {
                    variable: token(b"var1"),
                    value: tokens_vec([b"value1", b"value2"]),
                    cache: Cache::Bool,
                    docstring: quoted_token(b"docstring1"),
                    force: false,
                })))),
                Ok(Command::Set(Box::new(Set::Cache(SetCache {
                    variable: token(b"var1"),
                    value: tokens_vec([b"value1", b"value2"]),
                    cache: Cache::FilePath,
                    docstring: quoted_token(b"docstring1"),
                    force: true,
                })))),
            ]
        )
    }
}
