use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Set the regular expression used for dependency checking.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/include_regular_expression.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct IncludeRegularExpression<'t> {
    pub regex_match: Token<'t>,
    pub regex_complain: Option<Token<'t>>,
}

impl<'t> ToCommandScope for IncludeRegularExpression<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::quoted_token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn include_regular_expression() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/include_regular_expression");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::IncludeRegularExpression(Box::new(IncludeRegularExpression {
                    regex_match: quoted_token(b"^[^.]+$|[.]h$|[.]icc$|[.]hxx$|[.]hpp$"),
                    regex_complain: None,
                })),
                Command::IncludeRegularExpression(Box::new(IncludeRegularExpression {
                    regex_match: quoted_token(b"^[^.]+$|[.]h$|[.]icc$|[.]hxx$|[.]hpp$"),
                    regex_complain: Some(quoted_token(b"^[^.]+$|[.]h$|[.]icc$|[.]hxx$|[.]hpp$")),
                })),
            ])
        )
    }
}
