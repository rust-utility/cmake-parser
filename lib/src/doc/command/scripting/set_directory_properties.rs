use cmake_parser_derive::CMake;

use crate::{
    command::common::Property,
    doc::command_scope::{CommandScope, ToCommandScope},
};

/// Set properties of the current directory and subdirectories.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/set_directory_properties.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SetDirectoryProperties<'t> {
    #[cmake(transparent)]
    pub properties: Vec<Property<'t>>,
}

impl<'t> ToCommandScope for SetDirectoryProperties<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_directory_properties() {
        let src =
            include_bytes!("../../../../../fixture/commands/scripting/set_directory_properties");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::SetDirectoryProperties(Box::new(
                    SetDirectoryProperties {
                        properties: vec![Property {
                            prop: token(b"prop1"),
                            value: token(b"value1")
                        }],
                    }
                ))),
                Ok(Command::SetDirectoryProperties(Box::new(
                    SetDirectoryProperties {
                        properties: vec![
                            Property {
                                prop: token(b"prop1"),
                                value: token(b"value1")
                            },
                            Property {
                                prop: token(b"prop2"),
                                value: token(b"value2")
                            }
                        ],
                    }
                ))),
            ]
        )
    }
}
