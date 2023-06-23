use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Copy a file to another location and modify its contents.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/configure_file.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ConfigureFile<'t> {
    #[cmake(positional)]
    pub input: Token<'t>,
    #[cmake(positional)]
    pub output: Token<'t>,
    pub permissions: Option<Permissions<'t>>,
    #[cmake(rename = "COPYONLY")]
    pub copy_only: bool,
    pub escape_quotes: bool,
    #[cmake(rename = "@ONLY")]
    pub only: bool,
    pub newline_style: Option<NewlineStyle>,
}

impl<'t> ToCommandScope for ConfigureFile<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Permissions<'t> {
    #[cmake(rename = "NO_SOURCE_PERMISSIONS")]
    NoSource,
    #[cmake(rename = "USE_SOURCE_PERMISSIONS")]
    UseSource,
    #[cmake(rename = "FILE_PERMISSIONS", transparent)]
    File(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum NewlineStyle {
    Unix,
    Dos,
    Win32,
    Lf,
    #[cmake(rename = "CRLF")]
    CrLf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn configure_file() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/configure_file");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: None,
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: None,
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: Some(Permissions::NoSource),
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: None,
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: Some(Permissions::UseSource),
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: None,
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: Some(Permissions::File(tokens_vec([b"file1", b"file2"]))),
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: None,
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: Some(Permissions::File(tokens_vec([b"file1", b"file2"]))),
                    copy_only: true,
                    escape_quotes: true,
                    only: true,
                    newline_style: Some(NewlineStyle::Unix),
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: None,
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: Some(NewlineStyle::Dos),
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: None,
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: Some(NewlineStyle::Win32),
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: None,
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: Some(NewlineStyle::Lf),
                })),
                Command::ConfigureFile(Box::new(ConfigureFile {
                    input: token(b"input1"),
                    output: token(b"output1"),
                    permissions: None,
                    copy_only: false,
                    escape_quotes: false,
                    only: false,
                    newline_style: Some(NewlineStyle::CrLf),
                })),
            ])
        )
    }
}
