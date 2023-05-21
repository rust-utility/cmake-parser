use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add options to the link step for executable, shared library or module library targets in the current directory and below that are added after this command is invoked.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_link_options.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddLinkOptions<'t> {
    #[cmake(positional)]
    pub link_options: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for AddLinkOptions<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::quoted_tokens;
    use crate::*;

    #[test]
    fn add_link_options() {
        let src = include_bytes!("../../../../fixture/commands/add_link_options");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[
                Command::AddLinkOptions(Box::new(AddLinkOptions {
                    link_options: quoted_tokens([b"SHELL: --bind"]).to_vec(),
                })),
                Command::AddLinkOptions(Box::new(AddLinkOptions {
                    link_options: quoted_tokens([b"SHELL: -s MODULARIZE=1"]).to_vec(),
                }))
            ]
        )
    }
}
