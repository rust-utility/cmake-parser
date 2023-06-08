use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add a list of header files to precompile.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_precompile_headers.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum TargetPrecompileHeaders<'t> {
    Main(MainTargetPrecompileHeaders<'t>),
    Reuse(ReuseTargetPrecompileHeaders<'t>),
}

impl<'t> ToCommandScope for TargetPrecompileHeaders<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct MainTargetPrecompileHeaders<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub headers: Vec<Header<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Header<'t> {
    Interface(Vec<Token<'t>>),
    Public(Vec<Token<'t>>),
    Private(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ReuseTargetPrecompileHeaders<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub reuse_from: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_tokens_vec, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_precompile_headers() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/target_precompile_headers");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetPrecompileHeaders(Box::new(TargetPrecompileHeaders::Main(
                    MainTargetPrecompileHeaders {
                        target: token(b"myTarget"),
                        headers: vec![
                            Header::Public(tokens_vec([b"project_header.h"])),
                            Header::Private(tokens_vec([
                                b"\"other_header.h\"",
                                b"<unordered_map>"
                            ]))
                        ]
                    }
                ))),
                Command::TargetPrecompileHeaders(Box::new(TargetPrecompileHeaders::Main(
                    MainTargetPrecompileHeaders {
                        target: token(b"mylib"),
                        headers: vec![Header::Private(quoted_tokens_vec([
                            b"$<$<COMPILE_LANGUAGE:CXX>:${CMAKE_CURRENT_SOURCE_DIR}/cxx_only.h>",
                            b"$<$<COMPILE_LANGUAGE:C>:<stddef.h$<ANGLE-R>>",
                            b"$<$<COMPILE_LANGUAGE:CXX>:<cstddef$<ANGLE-R>>"
                        ])),]
                    }
                ))),
                Command::TargetPrecompileHeaders(Box::new(TargetPrecompileHeaders::Reuse(
                    ReuseTargetPrecompileHeaders {
                        target: token(b"target"),
                        reuse_from: token(b"other_target"),
                    }
                ))),
            ])
        )
    }
}
