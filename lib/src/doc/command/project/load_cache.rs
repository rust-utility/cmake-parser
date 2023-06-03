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
#[cmake(pkg = "crate", positional)]
pub struct LocalLoadCache<'t> {
    pub build_dir: Token<'t>,
    #[cmake(rename = "READ_WITH_PREFIX", transparent)]
    pub prefix: Token<'t>,
    pub entries: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ExternalLoadCache<'t> {
    #[cmake(positional)]
    pub build_dir: Token<'t>,
    pub exclude: Option<Vec<Token<'t>>>,
    pub include_internals: Option<Vec<Token<'t>>>,
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
            Ok(vec![
                Command::LoadCache(Box::new(LoadCache::Local(LocalLoadCache {
                    build_dir: token(b"qqq"),
                    prefix: token(b"prefix"),
                    entries: tokens_vec([b"entry1", b"entry2"]),
                }))),
                Command::LoadCache(Box::new(LoadCache::External(ExternalLoadCache {
                    build_dir: token(b"qqq"),
                    exclude: None,
                    include_internals: None,
                }))),
                Command::LoadCache(Box::new(LoadCache::External(ExternalLoadCache {
                    build_dir: token(b"qqq"),
                    exclude: Some(tokens_vec([b"abc", b"def"])),
                    include_internals: None,
                }))),
                Command::LoadCache(Box::new(LoadCache::External(ExternalLoadCache {
                    build_dir: token(b"qqq"),
                    exclude: None,
                    include_internals: Some(tokens_vec([b"abc", b"def", b"ghk"])),
                }))),
                Command::LoadCache(Box::new(LoadCache::External(ExternalLoadCache {
                    build_dir: token(b"qqq"),
                    exclude: Some(tokens_vec([b"abc", b"def"])),
                    include_internals: Some(tokens_vec([b"abc", b"def", b"ghk"])),
                }))),
            ])
        )
    }
}
