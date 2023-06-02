use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Load in the values from another project's CMake cache.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/load_cache.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum LoadCache<'t> {
    Local(LocalLoadCache<'t>),
    External(ExternalLoadCache<'t>),
}

impl<'t> ToCommandScope for LoadCache<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "entries")]
pub struct LocalLoadCache<'t> {
    #[cmake(positional)]
    pub build_dir: Token<'t>,
    #[cmake(rename = "READ_WITH_PREFIX")]
    pub prefix: Token<'t>,
    #[cmake(rename = "")]
    pub entries: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ExternalLoadCache<'t> {
    #[cmake(positional)]
    build_dir: Token<'t>,
    exclude: Option<Vec<Token<'t>>>,
    include_internals: Option<Vec<Token<'t>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn load_cache() {
        let src = include_bytes!("../../../../../fixture/commands/project/load_cache");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::LoadCache(Box::new(LoadCache::Local(
                LocalLoadCache {
                    build_dir: token(b"qqq"),
                    prefix: token(b"prefix"),
                    entries: tokens_vec([b"entry1", b"entry2"]),
                }
            ))),])
        )
    }
}
