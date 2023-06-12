use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Manually create Qt user interfaces Wrappers.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/qt_wrap_ui.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct QtWrapUi<'t> {
    pub lib: Token<'t>,
    pub headers_dest: Token<'t>,
    pub sources_dest: Token<'t>,
    pub source_lists: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for QtWrapUi<'t> {
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
    fn qt_wrap_ui() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/qt_wrap_ui");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::QtWrapUi(Box::new(QtWrapUi {
                lib: token(b"lib1"),
                headers_dest: token(b"headers_dest1"),
                sources_dest: token(b"sources_dest1"),
                source_lists: tokens_vec([b"source_lists1", b"source_lists2"]),
            })),])
        )
    }
}
