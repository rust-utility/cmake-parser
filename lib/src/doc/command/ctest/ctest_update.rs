use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Perform the CTest Update Step as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_update.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct CTestUpdate<'t> {
    #[cmake(rename = "SOURCE")]
    pub source_dir: Option<Token<'t>>,
    pub return_value: Option<Token<'t>>,
    pub capture_cmake_error: Option<Token<'t>>,
    pub quiet: bool,
}

impl<'t> ToCommandScope for CTestUpdate<'t> {
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
    fn ctest_update() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_update");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestUpdate(Box::new(CTestUpdate {
                    source_dir: None,
                    return_value: None,
                    capture_cmake_error: None,
                    quiet: false,
                })),
                Command::CTestUpdate(Box::new(CTestUpdate {
                    source_dir: Some(token(b"source1")),
                    return_value: Some(token(b"return_value1")),
                    capture_cmake_error: Some(token(b"capture_cmake_error1")),
                    quiet: true,
                })),
            ])
        )
    }
}
