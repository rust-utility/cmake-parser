use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Copy mesa headers for use in combination with system GL.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/use_mangled_mesa.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct UseMangledMesa<'t> {
    pub path_to_mesa: Token<'t>,
    pub output_directory: Token<'t>,
}

impl<'t> ToCommandScope for UseMangledMesa<'t> {
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
    fn use_mangled_mesa() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/use_mangled_mesa");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::UseMangledMesa(Box::new(UseMangledMesa {
                path_to_mesa: token(b"path_to_mesa1"),
                output_directory: token(b"output_directory1"),
            })),])
        )
    }
}
