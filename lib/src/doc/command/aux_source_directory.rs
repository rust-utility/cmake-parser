use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Find all source files in a directory.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/aux_source_directory.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AuxSourceDirectory<'t> {
    pub dir: Token<'t>,
    pub variable: Token<'t>,
}

impl<'t> ToCommandScope for AuxSourceDirectory<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn aux_source_directory() {
        let src = include_bytes!("../../../../fixture/commands/project/aux_source_directory");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::AuxSourceDirectory(Box::new(
                AuxSourceDirectory {
                    dir: b"src/".into(),
                    variable: b"sources".into(),
                }
            )),])
        )
    }
}
