use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Write content into a file.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/write_file.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct WriteFile<'t> {
    pub filename: Token<'t>,
    pub content: Token<'t>,
    pub append: bool,
}

impl<'t> ToCommandScope for WriteFile<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn write_file() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/write_file");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::WriteFile(Box::new(WriteFile {
                    filename: token(b"filename1"),
                    content: token(b"content1"),
                    append: false,
                })),
                Command::WriteFile(Box::new(WriteFile {
                    filename: token(b"filename2"),
                    content: token(b"content2"),
                    append: true,
                })),
            ])
        )
    }
}
