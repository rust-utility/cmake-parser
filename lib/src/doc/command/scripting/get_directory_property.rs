use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a property of DIRECTORY scope.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_directory_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct GetDirectoryProperty<'t> {
    pub name: Token<'t>,
    #[cmake(transparent)]
    pub directory: Option<Token<'t>>,
    pub scope: Scope<'t>,
}

impl<'t> ToCommandScope for GetDirectoryProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum Scope<'t> {
    #[cmake(transparent)]
    Definition(Token<'t>),
    Property(Token<'t>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_directory_property() {
        let src =
            include_bytes!("../../../../../fixture/commands/scripting/get_directory_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::GetDirectoryProperty(Box::new(
                    GetDirectoryProperty {
                        name: token(b"name1"),
                        directory: None,
                        scope: Scope::Property(token(b"property1")),
                    }
                ))),
                Ok(Command::GetDirectoryProperty(Box::new(
                    GetDirectoryProperty {
                        name: token(b"name1"),
                        directory: Some(token(b"directory1")),
                        scope: Scope::Property(token(b"property1")),
                    }
                ))),
                Ok(Command::GetDirectoryProperty(Box::new(
                    GetDirectoryProperty {
                        name: token(b"name1"),
                        directory: None,
                        scope: Scope::Definition(token(b"var1")),
                    }
                ))),
                Ok(Command::GetDirectoryProperty(Box::new(
                    GetDirectoryProperty {
                        name: token(b"name1"),
                        directory: Some(token(b"directory1")),
                        scope: Scope::Definition(token(b"var1")),
                    }
                ))),
            ]
        )
    }
}
