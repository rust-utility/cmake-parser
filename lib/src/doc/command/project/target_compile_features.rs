use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add expected compiler features to a target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_compile_features.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetCompileFeatures<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub features: Vec<Feature<'t>>,
}

impl<'t> ToCommandScope for TargetCompileFeatures<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Feature<'t> {
    Interface(Vec<Token<'t>>),
    Public(Vec<Token<'t>>),
    Private(Vec<Token<'t>>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_compile_features() {
        let src = include_bytes!("../../../../../fixture/commands/project/target_compile_features");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetCompileFeatures(Box::new(TargetCompileFeatures {
                    target: token(b"LibXml2"),
                    features: vec![Feature::Private(tokens_vec([
                        b"SYSCONFDIR=\"${CMAKE_INSTALL_FULL_SYSCONFDIR}\""
                    ]))]
                })),
                Command::TargetCompileFeatures(Box::new(TargetCompileFeatures {
                    target: token(b"LibXml2"),
                    features: vec![
                        Feature::Interface(tokens_vec([b"LIBXML_STATIC"])),
                        Feature::Private(tokens_vec([b"qqq", b"bbb"]))
                    ]
                })),
            ])
        )
    }
}
