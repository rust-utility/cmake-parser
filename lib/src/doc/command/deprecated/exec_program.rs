use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Run an executable program during the processing of the CMakeList.txt file.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/exec_program.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "dir")]
pub struct ExecProgram<'t> {
    #[cmake(positional)]
    pub executable: Token<'t>,
    #[cmake(rename = "")]
    pub dir: Option<Token<'t>>,
    pub args: Option<Vec<Token<'t>>>,
    pub output_variable: Option<Token<'t>>,
    pub return_value: Option<Token<'t>>,
}

impl<'t> ToCommandScope for ExecProgram<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn exec_program() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/exec_program");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::ExecProgram(Box::new(ExecProgram {
                    executable: token(b"exe1"),
                    dir: Some(token(b"dir1")),
                    args: Some(tokens_vec([b"arg1", b"arg2"])),
                    output_variable: Some(token(b"output_variable1")),
                    return_value: Some(token(b"return_value1")),
                })),
                Command::ExecProgram(Box::new(ExecProgram {
                    executable: token(b"exe1"),
                    dir: None,
                    args: Some(tokens_vec([b"arg1", b"arg2"])),
                    output_variable: None,
                    return_value: None,
                })),
            ])
        )
    }
}
