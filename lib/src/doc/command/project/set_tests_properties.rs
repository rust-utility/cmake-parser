use cmake_parser_derive::CMake;

use crate::{
    command::common::Property,
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Set a property of the tests.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/set_tests_properties.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "tests")]
pub struct SetTestsProperties<'t> {
    #[cmake(rename = "")]
    pub tests: Vec<Token<'t>>,
    pub properties: Vec<Property<'t>>,
}

impl<'t> ToCommandScope for SetTestsProperties<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_tests_properties() {
        let src = include_bytes!("../../../../../fixture/commands/project/set_tests_properties");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::SetTestsProperties(Box::new(
                SetTestsProperties {
                    tests: tokens_vec([b"test1", b"test2"]),
                    properties: vec![
                        Property {
                            prop: token(b"prop1"),
                            value: token(b"value1"),
                        },
                        Property {
                            prop: token(b"prop2"),
                            value: token(b"value2"),
                        }
                    ]
                }
            )),])
        )
    }
}
