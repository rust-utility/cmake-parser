use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Evaluate a group of commands for each value in a list.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/foreach.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum ForEach<'t> {
    RangeStop(RangeStop<'t>),
    RangeStartStop(RangeStartStop<'t>),
    InZipLists(InZipLists<'t>),
    In(In<'t>),
}

impl<'t> ToCommandScope for ForEach<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, complete)]
pub struct RangeStop<'t> {
    #[cmake(keyword_after = "RANGE")]
    pub loop_var: Token<'t>,
    pub stop: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, complete)]
pub struct RangeStartStop<'t> {
    #[cmake(keyword_after = "RANGE")]
    pub loop_var: Token<'t>,
    pub start: Token<'t>,
    pub stop: Token<'t>,
    pub step: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct In<'t> {
    #[cmake(positional, keyword_after = "IN")]
    pub loop_var: Token<'t>,
    pub lists: Option<Vec<Token<'t>>>,
    pub items: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "loop_var")]
pub struct InZipLists<'t> {
    #[cmake(rename = b"")]
    pub loop_var: Vec<Token<'t>>,
    pub zip_lists: ZipLists<'t>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZipLists<'t> {
    pub lists: Vec<Token<'t>>,
}

impl<'t> crate::CMakeParse<'t> for ZipLists<'t> {
    fn matches_type(_: &[u8], keyword: &[u8], tokens: &[Token<'t>]) -> bool {
        keyword == b"IN" && tokens.first().map(|x| x.as_bytes()) == Some(b"ZIP_LISTS")
    }

    fn need_push_keyword(_: &Token<'t>) -> bool {
        false
    }

    fn rest<'tv>(tokens: &'tv [Token<'t>]) -> &'tv [Token<'t>] {
        &tokens[1..]
    }

    fn parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), crate::CommandParseError> {
        crate::CMakeParse::parse(tokens).map(|(lists, tokens)| (Self { lists }, tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn foreach() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/foreach");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::ForEach(Box::new(ForEach::RangeStop(RangeStop {
                    loop_var: token(b"variable1"),
                    stop: token(b"10"),
                })))),
                Ok(Command::ForEach(Box::new(ForEach::RangeStartStop(
                    RangeStartStop {
                        loop_var: token(b"variable1"),
                        start: token(b"1"),
                        stop: token(b"10"),
                        step: None,
                    }
                )))),
                Ok(Command::ForEach(Box::new(ForEach::RangeStartStop(
                    RangeStartStop {
                        loop_var: token(b"variable1"),
                        start: token(b"1"),
                        stop: token(b"10"),
                        step: Some(token(b"3")),
                    }
                )))),
                Ok(Command::ForEach(Box::new(ForEach::In(In {
                    loop_var: token(b"variable1"),
                    lists: Some(tokens_vec([b"A", b"B", b"C", b"D", b"E", b"F"])),
                    items: None,
                })))),
                Ok(Command::ForEach(Box::new(ForEach::In(In {
                    loop_var: token(b"variable1"),
                    lists: None,
                    items: Some(tokens_vec([b"${A}", b"${B}"])),
                })))),
                Ok(Command::ForEach(Box::new(ForEach::In(In {
                    loop_var: token(b"variable1"),
                    lists: Some(tokens_vec([b"A", b"B", b"C", b"D", b"E", b"F"])),
                    items: Some(tokens_vec([b"${A}", b"${B}"])),
                })))),
                Ok(Command::ForEach(Box::new(ForEach::InZipLists(
                    InZipLists {
                        loop_var: tokens_vec([b"variable1"]),
                        zip_lists: ZipLists {
                            lists: tokens_vec([b"English", b"Bahasa"]),
                        }
                    }
                )))),
                Ok(Command::ForEach(Box::new(ForEach::InZipLists(
                    InZipLists {
                        loop_var: tokens_vec([b"en", b"ba"]),
                        zip_lists: ZipLists {
                            lists: tokens_vec([b"English", b"Bahasa"]),
                        }
                    }
                )))),
            ]
        )
    }
}
