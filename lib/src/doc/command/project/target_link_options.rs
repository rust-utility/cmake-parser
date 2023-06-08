use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add options to the link step for an executable, shared library or module library target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_link_options.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetLinkOptions<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub before: bool,
    pub options: Vec<Option<'t>>,
}

impl<'t> ToCommandScope for TargetLinkOptions<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Option<'t> {
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
    fn target_link_options() {
        let src = include_bytes!("../../../../../fixture/commands/project/target_link_options");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetLinkOptions(Box::new(TargetLinkOptions {
                    target: token(b"LibXml2"),
                    before: true,
                    options: vec![Option::Private(tokens_vec([
                        b"SYSCONFDIR=\"${CMAKE_INSTALL_FULL_SYSCONFDIR}\""
                    ]))]
                })),
                Command::TargetLinkOptions(Box::new(TargetLinkOptions {
                    target: token(b"LibXml2"),
                    before: false,
                    options: vec![
                        Option::Interface(tokens_vec([b"LIBXML_STATIC"])),
                        Option::Private(tokens_vec([b"qqq", b"bbb"]))
                    ]
                })),
            ])
        )
    }
}
