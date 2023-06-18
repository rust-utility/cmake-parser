use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Parse function or macro arguments.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/cmake_parse_arguments.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum CMakeParseArguments<'t> {
    #[cmake(rename = "PARSE_ARGV", transparent)]
    Function(CMakeParseArgumentsFunction<'t>),
    Regular(CMakeParseArgumentsRegular<'t>),
}

impl<'t> ToCommandScope for CMakeParseArguments<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakeParseArgumentsRegular<'t> {
    pub prefix: Token<'t>,
    pub options: Token<'t>,
    pub one_value_keywords: Token<'t>,
    pub multi_value_keywords: Token<'t>,
    pub args: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakeParseArgumentsFunction<'t> {
    pub n: Token<'t>,
    pub prefix: Token<'t>,
    pub options: Token<'t>,
    pub one_value_keywords: Token<'t>,
    pub multi_value_keywords: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cmake_parse_arguments() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/cmake_parse_arguments");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CMakeParseArguments(Box::new(CMakeParseArguments::Regular(
                    CMakeParseArgumentsRegular {
                        prefix: token(b"prefix1"),
                        options: token(b"options1"),
                        one_value_keywords: token(b"one_value_keywords1"),
                        multi_value_keywords: token(b"multi_value_keywords1"),
                        args: Some(tokens_vec([b"arg1", b"arg2"])),
                    }
                ))),
                Command::CMakeParseArguments(Box::new(CMakeParseArguments::Function(
                    CMakeParseArgumentsFunction {
                        n: token(b"5"),
                        prefix: token(b"prefix1"),
                        options: token(b"options1"),
                        one_value_keywords: token(b"one_value_keywords1"),
                        multi_value_keywords: token(b"multi_value_keywords1"),
                    }
                ))),
            ])
        )
    }
}
