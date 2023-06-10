use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Starts the testing for a given model
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_start.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "model")]
pub struct CTestStart<'t> {
    #[cmake(rename = "")]
    pub model: Option<Model<'t>>,
    pub group: Option<Token<'t>>,
    pub append: bool,
    pub quiet: bool,
}

impl<'t> ToCommandScope for CTestStart<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Model<'t> {
    pub model: Token<'t>,
    pub source: Option<Token<'t>>,
    pub binary: Option<Token<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_start() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_start");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestStart(Box::new(CTestStart {
                    model: Some(Model {
                        model: token(b"Experimental"),
                        source: None,
                        binary: None,
                    }),
                    group: Some(token(b"GroupExperimental")),
                    append: false,
                    quiet: false,
                })),
                Command::CTestStart(Box::new(CTestStart {
                    model: None,
                    group: None,
                    append: true,
                    quiet: false,
                })),
                Command::CTestStart(Box::new(CTestStart {
                    model: Some(Model {
                        model: token(b"Experimental"),
                        source: Some(token(b"path/to/source")),
                        binary: Some(token(b"path/to/binary")),
                    }),
                    group: Some(token(b"SomeGroup")),
                    append: true,
                    quiet: true,
                })),
                Command::CTestStart(Box::new(CTestStart {
                    model: Some(Model {
                        model: token(b"Experimental"),
                        source: Some(token(b"path/to/source")),
                        binary: Some(token(b"path/to/binary")),
                    }),
                    group: Some(token(b"SomeGroup")),
                    append: true,
                    quiet: true,
                })),
                Command::CTestStart(Box::new(CTestStart {
                    model: Some(Model {
                        model: token(b"Experimental"),
                        source: Some(token(b"path/to/source")),
                        binary: Some(token(b"path/to/binary")),
                    }),
                    group: Some(token(b"SomeGroup")),
                    append: true,
                    quiet: true,
                })),
            ])
        )
    }
}
