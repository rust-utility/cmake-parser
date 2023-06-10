use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// This command generates an old-style library dependencies file.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/export_library_dependencies.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ExportLibraryDependencies<'t> {
    #[cmake(positional)]
    pub file: Token<'t>,
    pub append: bool,
}

impl<'t> ToCommandScope for ExportLibraryDependencies<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn export_library_dependencies() {
        let src = include_bytes!(
            "../../../../../fixture/commands/deprecated/export_library_dependencies"
        );
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::ExportLibraryDependencies(Box::new(ExportLibraryDependencies {
                    file: token(b"file1"),
                    append: false,
                })),
                Command::ExportLibraryDependencies(Box::new(ExportLibraryDependencies {
                    file: token(b"file1"),
                    append: true,
                })),
            ])
        )
    }
}
