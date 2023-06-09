use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Perform the CTest Build Step as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_build.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct CTestBuild<'t> {
    #[cmake(rename = "BUILD")]
    pub build_dir: Option<Token<'t>>,
    pub append: bool,
    pub configuration: Option<Token<'t>>,
    pub parallel_level: Option<Token<'t>>,
    pub flags: Option<Token<'t>>,
    pub project_name: Option<Token<'t>>,
    pub target: Option<Token<'t>>,
    pub number_errors: Option<Token<'t>>,
    pub number_warnings: Option<Token<'t>>,
    pub return_value: Option<Token<'t>>,
    pub capture_cmake_error: Option<Token<'t>>,
}

impl<'t> ToCommandScope for CTestBuild<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_build() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_build");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestBuild(Box::new(CTestBuild {
                    build_dir: None,
                    append: false,
                    configuration: None,
                    parallel_level: None,
                    flags: None,
                    project_name: None,
                    target: None,
                    number_errors: None,
                    number_warnings: None,
                    return_value: None,
                    capture_cmake_error: None
                })),
                Command::CTestBuild(Box::new(CTestBuild {
                    build_dir: Some(token(b"build1")),
                    append: true,
                    configuration: Some(token(b"configuration1")),
                    parallel_level: Some(token(b"parallel_level1")),
                    flags: Some(token(b"flags1")),
                    project_name: Some(token(b"project_name1")),
                    target: Some(token(b"target1")),
                    number_errors: Some(token(b"number_errors1")),
                    number_warnings: Some(token(b"number_warnings1")),
                    return_value: Some(token(b"return_value1")),
                    capture_cmake_error: Some(token(b"capture_cmake_error1")),
                })),
            ])
        )
    }
}
