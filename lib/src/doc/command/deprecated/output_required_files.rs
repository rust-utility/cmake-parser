use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Approximate C preprocessor dependency scanning.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/output_required_files.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct OutputRequiredFiles<'t> {
    pub src_file: Token<'t>,
    pub output_file: Token<'t>,
}

impl<'t> ToCommandScope for OutputRequiredFiles<'t> {
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
    fn output_required_files() {
        let src =
            include_bytes!("../../../../../fixture/commands/deprecated/output_required_files");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::OutputRequiredFiles(Box::new(
                OutputRequiredFiles {
                    src_file: token(b"src_file1"),
                    output_file: token(b"output_file1"),
                }
            )),])
        )
    }
}
