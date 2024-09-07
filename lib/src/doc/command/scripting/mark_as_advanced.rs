use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Mark cmake cached variables as advanced.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/mark_as_advanced.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "vars")]
pub struct MarkAsAdvanced<'t> {
    #[cmake(positional)]
    pub mode: Option<Mode>,
    #[cmake(rename = "")]
    pub vars: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for MarkAsAdvanced<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Mode {
    Clear,
    Force,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn mark_as_advanced() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/mark_as_advanced");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::MarkAsAdvanced(Box::new(MarkAsAdvanced {
                    mode: None,
                    vars: tokens_vec([b"name1", b"name2"]),
                }))),
                Ok(Command::MarkAsAdvanced(Box::new(MarkAsAdvanced {
                    mode: Some(Mode::Clear),
                    vars: tokens_vec([b"name1"]),
                }))),
            ]
        )
    }
}
