use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add include directories to the build.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/include_directories.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct IncludeDirectories<'t> {
    pub append: Option<Append>,
    pub system: bool,
    pub dirs: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for IncludeDirectories<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Append {
    After,
    Before,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn include_directories() {
        let src = include_bytes!("../../../../../fixture/commands/project/include_directories");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::IncludeDirectories(Box::new(IncludeDirectories {
                    append: None,
                    system: false,
                    dirs: tokens_vec([b"include"])
                })),
                Command::IncludeDirectories(Box::new(IncludeDirectories {
                    append: Some(Append::Before),
                    system: false,
                    dirs: tokens_vec([b"include1", b"include2"])
                })),
                Command::IncludeDirectories(Box::new(IncludeDirectories {
                    append: Some(Append::After),
                    system: true,
                    dirs: tokens_vec([b"include1"])
                })),
            ])
        )
    }
}
