use cmake_parser_derive::CMake;

use crate::{
    command::common::WindowsRegistryView,
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Query various host system information.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/cmake_host_system_information.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CMakeHostSystemInformation<'t> {
    pub result: Token<'t>,
    pub query: Query<'t>,
}

impl<'t> ToCommandScope for CMakeHostSystemInformation<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum Query<'t> {
    #[cmake(transparent)]
    WindowsRegistry(WindowsRegistryQuery<'t>),
    Regular(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct WindowsRegistryQuery<'t> {
    #[cmake(positional)]
    pub key: Token<'t>,
    pub selector: Option<WindowsRegistrySelector<'t>>,
    pub view: Option<WindowsRegistryView>,
    pub separator: Option<Token<'t>>,
    pub error_variable: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum WindowsRegistrySelector<'t> {
    ValueNames,
    Subkeys,
    #[cmake(transparent)]
    Value(Token<'t>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cmake_host_system_information() {
        let src = include_bytes!(
            "../../../../../fixture/commands/scripting/cmake_host_system_information"
        );
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CMakeHostSystemInformation(Box::new(CMakeHostSystemInformation {
                    result: token(b"Ncpu"),
                    query: Query::Regular(tokens_vec([b"NUMBER_OF_PHYSICAL_CORES"])),
                })),
                Command::CMakeHostSystemInformation(Box::new(CMakeHostSystemInformation {
                    result: token(b"result"),
                    query: Query::WindowsRegistry(WindowsRegistryQuery {
                        key: quoted_token(b"HKLM/SOFTWARE/Kitware"),
                        selector: None,
                        view: None,
                        separator: None,
                        error_variable: None,
                    }),
                })),
                Command::CMakeHostSystemInformation(Box::new(CMakeHostSystemInformation {
                    result: token(b"result"),
                    query: Query::WindowsRegistry(WindowsRegistryQuery {
                        key: quoted_token(b"HKLM/SOFTWARE/Kitware"),
                        selector: Some(WindowsRegistrySelector::Value(quoted_token(b"(default)"))),
                        view: None,
                        separator: None,
                        error_variable: None,
                    }),
                })),
                Command::CMakeHostSystemInformation(Box::new(CMakeHostSystemInformation {
                    result: token(b"result"),
                    query: Query::WindowsRegistry(WindowsRegistryQuery {
                        key: quoted_token(b"HKLM/SOFTWARE/Kitware"),
                        selector: Some(WindowsRegistrySelector::Subkeys),
                        view: Some(WindowsRegistryView::Bits32Fallback64),
                        separator: Some(token(b"separator1")),
                        error_variable: Some(token(b"error_variable1")),
                    }),
                })),
                Command::CMakeHostSystemInformation(Box::new(CMakeHostSystemInformation {
                    result: token(b"_vs_dir"),
                    query: Query::Regular(tokens_vec([b"VS_${_vs_ver}_DIR"])),
                })),
            ])
        )
    }
}
