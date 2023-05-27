use ::cmake_parser_derive::CMake;

use crate::{CommandScope, ToCommandScope, Token};

/// Create a test driver and source list for building test programs.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/create_test_sourcelist.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "tests")]
pub struct CreateTestSourceList<'t> {
    #[cmake(positional)]
    pub source_list_name: Token<'t>,
    #[cmake(positional)]
    pub driver_name: Token<'t>,
    #[cmake(rename = "")]
    pub tests: Vec<Token<'t>>,
    pub extra_include: Option<Token<'t>>,
    pub function: Option<Token<'t>>,
}

impl<'t> ToCommandScope for CreateTestSourceList<'t> {
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
    fn create_test_sourcelist() {
        let src = include_bytes!("../../../../fixture/commands/project/create_test_sourcelist");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);

        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CreateTestSourceList(Box::new(CreateTestSourceList {
                    source_list_name: b"srclist".into(),
                    driver_name: b"test_runner.cpp".into(),
                    tests: tokens([b"${cpptestsrc}"]).to_vec(),
                    extra_include: None,
                    function: None,
                })),
                Command::CreateTestSourceList(Box::new(CreateTestSourceList {
                    source_list_name: b"Tests".into(),
                    driver_name: b"CommonCxxTests.cxx".into(),
                    tests: tokens([
                        b"ObjectFactory.cxx",
                        b"otherArrays.cxx",
                        b"otherEmptyCell.cxx",
                        b"TestSmartPointer.cxx",
                        b"SystemInformation.cxx",
                    ])
                    .to_vec(),
                    extra_include: Some(b"hello.h".into()),
                    function: Some(b"hello_world".into()),
                })),
            ])
        )
    }
}
