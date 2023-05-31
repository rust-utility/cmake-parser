use cmake_parser_derive::CMake;

use crate::{
    command::common::Append,
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add directories in which the linker will look for libraries.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/link_directories.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct LinkDirectories<'t> {
    pub append: Option<Append>,
    pub dirs: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for LinkDirectories<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_directories() {
        let src = include_bytes!("../../../../../fixture/commands/project/link_directories");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::LinkDirectories(Box::new(LinkDirectories {
                    append: None,
                    dirs: tokens_vec([b"include"])
                })),
                Command::LinkDirectories(Box::new(LinkDirectories {
                    append: Some(Append::Before),
                    dirs: tokens_vec([b"include1", b"include2"])
                })),
                Command::LinkDirectories(Box::new(LinkDirectories {
                    append: Some(Append::After),
                    dirs: tokens_vec([b"include1"])
                })),
            ])
        )
    }
}
