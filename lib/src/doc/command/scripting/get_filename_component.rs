use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a specific component of a full filename.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_filename_component.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct GetFilenameComponent<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
    #[cmake(positional)]
    pub filename: Token<'t>,
    pub mode: Mode<'t>,
    pub cache: bool,
}

impl<'t> ToCommandScope for GetFilenameComponent<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Mode<'t> {
    /// Directory without file name
    Directory,
    /// File name without directory
    Name,
    /// File name longest extension (.b.c from d/a.b.c)
    Ext,
    /// File name with neither the directory nor the longest extension
    NameWe,
    /// File name last extension (.c from d/a.b.c)
    LastExt,
    /// File name with neither the directory nor the last extension
    NameWle,
    /// Legacy alias for DIRECTORY (use for CMake <= 2.8.11)
    Path,
    /// Full path to file
    Absolute(BaseDir<'t>),
    /// Full path to existing file with symlinks resolved
    #[cmake(rename = "REALPATH")]
    RealPath(BaseDir<'t>),
    Program(Program<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct BaseDir<'t> {
    pub base_dir: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct Program<'t> {
    pub program_args: Option<Token<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_filename_component() {
        let src =
            include_bytes!("../../../../../fixture/commands/scripting/get_filename_component");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::Directory,
                        cache: false,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::Name,
                        cache: true,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::NameWe,
                        cache: false,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::Absolute(BaseDir { base_dir: None }),
                        cache: false,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::RealPath(BaseDir {
                            base_dir: Some(token(b"base_dir1"))
                        }),
                        cache: false,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::Absolute(BaseDir {
                            base_dir: Some(token(b"base_dir1"))
                        }),
                        cache: true,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::RealPath(BaseDir { base_dir: None }),
                        cache: true,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::Program(Program { program_args: None }),
                        cache: false,
                    }
                ))),
                Ok(Command::GetFilenameComponent(Box::new(
                    GetFilenameComponent {
                        variable: token(b"var1"),
                        filename: token(b"filename1"),
                        mode: Mode::Program(Program {
                            program_args: Some(token(b"program_args1"))
                        }),
                        cache: true,
                    }
                ))),
            ]
        )
    }
}
