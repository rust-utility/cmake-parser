use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Runs a script or scripts much like if it was run from ctest -S.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_run_script.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty, default = "scripts")]
pub struct CTestRunScript<'t> {
    pub new_process: bool,
    #[cmake(rename = "")]
    pub scripts: Option<Vec<Token<'t>>>,
    pub return_value: Option<Token<'t>>,
}

impl<'t> ToCommandScope for CTestRunScript<'t> {
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
    fn ctest_run_script() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_run_script");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestRunScript(Box::new(CTestRunScript {
                    new_process: false,
                    scripts: None,
                    return_value: None,
                })),
                Command::CTestRunScript(Box::new(CTestRunScript {
                    new_process: true,
                    scripts: Some(tokens_vec([b"script1", b"script2", b"script3"])),
                    return_value: Some(token(b"return_value1")),
                })),
            ])
        )
    }
}
