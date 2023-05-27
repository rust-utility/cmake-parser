use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Create FLTK user interfaces Wrappers.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/fltk_wrap_ui.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FLTKWrapUI<'t> {
    pub resulting_library_name: Token<'t>,
    pub sources: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for FLTKWrapUI<'t> {
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
    fn fltk_wrap_ui() {
        let src = include_bytes!("../../../../../fixture/commands/project/fltk_wrap_ui");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[
                Command::FLTKWrapUI(Box::new(FLTKWrapUI {
                    resulting_library_name: token(b"Ui"),
                    sources: tokens_vec([b"ui.fl"]),
                })),
                Command::FLTKWrapUI(Box::new(FLTKWrapUI {
                    resulting_library_name: token(b"Ui2"),
                    sources: tokens_vec([b"ui1.fl", b"ui2.fl"]),
                })),
            ]
        )
    }
}
