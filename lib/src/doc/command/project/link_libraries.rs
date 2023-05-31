use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Link libraries to all targets added later.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/link_libraries.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct LinkLibraries<'t> {
    pub libs: Vec<LinkLibrary<'t>>,
}

impl<'t> ToCommandScope for LinkLibraries<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct LinkLibrary<'t> {
    build_configuraion: Option<BuildConfiguration>,
    lib: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum BuildConfiguration {
    #[cmake(rename = "debug")]
    Debug,
    #[cmake(rename = "optimized")]
    Optimized,
    #[cmake(rename = "general")]
    General,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_libraries() {
        let src = include_bytes!("../../../../../fixture/commands/project/link_libraries");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::LinkLibraries(Box::new(LinkLibraries {
                libs: vec![
                    LinkLibrary {
                        build_configuraion: None,
                        lib: token(b"lib1"),
                    },
                    LinkLibrary {
                        build_configuraion: Some(BuildConfiguration::Debug),
                        lib: token(b"lib2"),
                    },
                    LinkLibrary {
                        build_configuraion: None,
                        lib: token(b"lib3"),
                    },
                ],
            })),])
        )
    }
}
