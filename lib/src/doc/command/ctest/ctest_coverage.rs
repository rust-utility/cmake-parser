use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Perform the CTest Coverage Step as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_coverage.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct CTestCoverage<'t> {
    #[cmake(rename = "BUILD")]
    pub build_dir: Option<Token<'t>>,
    pub append: bool,
    pub labels: Option<Vec<Token<'t>>>,
    pub return_value: Option<Token<'t>>,
    pub quiet: bool,
    pub capture_cmake_error: Option<Token<'t>>,
}

impl<'t> ToCommandScope for CTestCoverage<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_coverage() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_coverage");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestCoverage(Box::new(CTestCoverage {
                    build_dir: None,
                    append: false,
                    labels: None,
                    return_value: None,
                    quiet: false,
                    capture_cmake_error: None,
                })),
                Command::CTestCoverage(Box::new(CTestCoverage {
                    build_dir: Some(token(b"build1")),
                    append: true,
                    labels: Some(tokens_vec([b"label1", b"label2"])),
                    return_value: Some(token(b"return_value1")),
                    quiet: true,
                    capture_cmake_error: Some(token(b"capture_cmake_error1")),
                })),
            ])
        )
    }
}
