use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Specify the source tree of a third-party utility.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/utility_source.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct UtilitySource<'t> {
    pub cache_entry: Token<'t>,
    pub executable_name: Token<'t>,
    pub path_to_source: Token<'t>,
    pub files: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for UtilitySource<'t> {
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
    fn utility_source() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/utility_source");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::UtilitySource(Box::new(UtilitySource {
                cache_entry: token(b"cache_entry1"),
                executable_name: token(b"executable_name1"),
                path_to_source: token(b"path_to_source1"),
                files: Some(tokens_vec([b"file1", b"file2"])),
            })),])
        )
    }
}
