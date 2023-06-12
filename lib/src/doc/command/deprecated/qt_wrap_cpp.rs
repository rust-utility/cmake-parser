use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Manually create Qt Wrappers.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/qt_wrap_cpp.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct QtWrapCpp<'t> {
    pub lib: Token<'t>,
    pub dest: Token<'t>,
    pub source_lists: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for QtWrapCpp<'t> {
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
    fn qt_wrap_cpp() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/qt_wrap_cpp");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::QtWrapCpp(Box::new(QtWrapCpp {
                lib: token(b"lib1"),
                dest: token(b"dest1"),
                source_lists: tokens_vec([b"source_lists1", b"source_lists2"]),
            })),])
        )
    }
}
