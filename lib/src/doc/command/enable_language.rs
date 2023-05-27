use ::cmake_parser_derive::CMake;

use crate::{CommandScope, ToCommandScope, Token};

/// Enable languages (CXX/C/OBJC/OBJCXX/Fortran/etc)
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/enable_language.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "langs")]
pub struct EnableLanguage<'t> {
    #[cmake(rename = "")]
    pub langs: Vec<Token<'t>>,
    pub optional: bool,
}

impl<'t> ToCommandScope for EnableLanguage<'t> {
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
    fn enable_language() {
        let src = include_bytes!("../../../../fixture/commands/project/enable_language");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);

        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::EnableLanguage(Box::new(EnableLanguage {
                    langs: tokens([b"CUDA"]).to_vec(),
                    optional: false,
                })),
                Command::EnableLanguage(Box::new(EnableLanguage {
                    langs: tokens([b"CSharp", b"ASM_NASM",]).to_vec(),
                    optional: true,
                })),
            ])
        )
    }
}
