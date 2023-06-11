use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Create rules to install the listed targets into the given directory.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/install_targets.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "targets")]
pub struct InstallTargets<'t> {
    #[cmake(positional)]
    pub dir: Token<'t>,
    pub runtime_directory: Option<Token<'t>>,
    pub targets: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for InstallTargets<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn install_targets() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/install_targets");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::InstallTargets(Box::new(InstallTargets {
                    dir: token(b"dir1"),
                    runtime_directory: Some(token(b"dir2")),
                    targets: tokens_vec([b"target1", b"target2"]),
                })),
                Command::InstallTargets(Box::new(InstallTargets {
                    dir: token(b"dir1"),
                    runtime_directory: None,
                    targets: tokens_vec([b"target1", b"target2"]),
                })),
            ])
        )
    }
}
