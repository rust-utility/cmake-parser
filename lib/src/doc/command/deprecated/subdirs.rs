use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add a list of subdirectories to the build.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/subdirs.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "dirs")]
pub struct Subdirs<'t> {
    #[cmake(rename = "")]
    pub dirs: Vec<Token<'t>>,
    pub exclude_from_all: Option<Vec<Token<'t>>>,
    pub preorder: bool,
}

impl<'t> ToCommandScope for Subdirs<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn subdirs() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/subdirs");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::Subdirs(Box::new(Subdirs {
                dirs: tokens_vec([b"dir1", b"dir2"]),
                exclude_from_all: Some(tokens_vec([b"exclude_dir1", b"exclude_dir2"])),
                preorder: true,
            })),])
        )
    }
}
