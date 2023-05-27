use ::cmake_parser_derive::CMake;

use crate::{CommandScope, ToCommandScope, Token};

/// Add a subdirectory to the build.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_subdirectory.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "binary_dir")]
pub struct AddSubdirectory<'t> {
    #[cmake(positional)]
    source_dir: Token<'t>,
    #[cmake(rename = "")]
    binary_dir: Option<Token<'t>>,
    exclude_from_all: bool,
    system: bool,
}

impl<'t> ToCommandScope for AddSubdirectory<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_subdirectory() {
        let src = include_bytes!("../../../../../fixture/commands/project/add_subdirectory");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::AddSubdirectory(Box::new(AddSubdirectory {
                    source_dir: b"libs".into(),
                    binary_dir: Some(b"qqq".into()),
                    exclude_from_all: false,
                    system: false,
                })),
                Command::AddSubdirectory(Box::new(AddSubdirectory {
                    source_dir: b"lib/${EDGE_SOURCES_DIR_NAME}/nano-stack".into(),
                    binary_dir: None,
                    exclude_from_all: false,
                    system: false,
                })),
                Command::AddSubdirectory(Box::new(AddSubdirectory {
                    source_dir: b"libs".into(),
                    binary_dir: None,
                    exclude_from_all: true,
                    system: true,
                })),
                Command::AddSubdirectory(Box::new(AddSubdirectory {
                    source_dir: b"libs".into(),
                    binary_dir: Some(b"qqq".into()),
                    exclude_from_all: false,
                    system: true,
                })),
            ])
        )
    }
}
