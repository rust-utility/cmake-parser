use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Adds options to the compilation of source files.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_compile_options.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddCompileOptions<'t> {
    #[cmake(positional)]
    pub compile_options: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for AddCompileOptions<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens;
    use crate::*;

    #[test]
    fn add_compile_options() {
        let src = include_bytes!("../../../../fixture/commands/add_compile_options");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[Command::AddCompileOptions(Box::new(AddCompileOptions {
                compile_options: tokens([b"-foo", b"-bar",]).to_vec()
            }))]
        )
    }
}
