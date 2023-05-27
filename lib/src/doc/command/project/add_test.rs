use ::cmake_parser_derive::CMake;

use crate::{command::project::CustomCommand, CommandScope, ToCommandScope, Token};

/// Add a test to the project to be run by ctest.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_test.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddTest<'t> {
    name: Token<'t>,
    #[cmake(rename = "COMMAND")]
    commands: Vec<CustomCommand<'t>>,
    configurations: Option<Vec<Vec<Token<'t>>>>,
    working_directory: Option<Token<'t>>,
    command_expand_lists: bool,
}

impl<'t> ToCommandScope for AddTest<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens;
    use crate::*;

    #[test]
    fn add_test() {
        let src = include_bytes!("../../../../../fixture/commands/project/add_test");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);

        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::AddTest(Box::new(AddTest {
                    name: b"runtest".into(),
                    commands: vec![CustomCommand {
                        name: b"runtest".into(),
                        args: Some(tokens([b"--out", b"${CMAKE_CURRENT_BINARY_DIR}"]).to_vec()),
                    }],
                    configurations: None,
                    working_directory: Some(b"${CMAKE_CURRENT_SOURCE_DIR}".into()),
                    command_expand_lists: false,
                })),
                Command::AddTest(Box::new(AddTest {
                    name: b"TestInstantiator".into(),
                    commands: vec![CustomCommand {
                        name: b"TestInstantiator".into(),
                        args: None,
                    }],
                    configurations: None,
                    working_directory: None,
                    command_expand_lists: true,
                })),
            ])
        )
    }
}
