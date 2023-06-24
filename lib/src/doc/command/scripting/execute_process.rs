use cmake_parser_derive::CMake;

use crate::{
    command::common::CustomCommand,
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Execute one or more child processes.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/execute_process.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cmake(pkg = "crate")]
pub struct ExecuteProcess<'t> {
    #[cmake(rename = "COMMAND")]
    pub commands: Vec<CustomCommand<'t>>,
    pub working_directory: Option<Token<'t>>,
    pub timeout: Option<Token<'t>>,
    pub result_variable: Option<Token<'t>>,
    pub results_variable: Option<Token<'t>>,
    pub output_variable: Option<Token<'t>>,
    pub error_variable: Option<Token<'t>>,
    pub input_file: Option<Token<'t>>,
    pub output_file: Option<Token<'t>>,
    pub error_file: Option<Token<'t>>,
    pub output_quiet: bool,
    pub error_quiet: bool,
    pub command_echo: Option<Token<'t>>,
    pub output_strip_trailing_whitespace: bool,
    pub error_strip_trailing_whitespace: bool,
    pub encoding: Option<WindowsEncoding>,
    pub echo_output_variable: bool,
    pub echo_error_variable: bool,
    pub command_error_is_fatal: Option<ErrorFatal>,
}

impl<'t> ToCommandScope for ExecuteProcess<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum ErrorFatal {
    Any,
    Last,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum WindowsEncoding {
    None,
    Auto,
    Ansi,
    Oem,
    #[cmake(rename = ["UTF8", "UTF-8"])]
    Utf8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn execute_process() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/execute_process");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: Some(tokens_vec([b"arg1", b"arg2"])),
                    }],
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    encoding: Some(WindowsEncoding::None),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    encoding: Some(WindowsEncoding::Auto),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    encoding: Some(WindowsEncoding::Ansi),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    encoding: Some(WindowsEncoding::Oem),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    encoding: Some(WindowsEncoding::Utf8),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    encoding: Some(WindowsEncoding::Utf8),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    command_error_is_fatal: Some(ErrorFatal::Any),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![CustomCommand {
                        name: token(b"cmd1"),
                        args: None,
                    }],
                    command_error_is_fatal: Some(ErrorFatal::Last),
                    ..Default::default()
                })),
                Command::ExecuteProcess(Box::new(ExecuteProcess {
                    commands: vec![
                        CustomCommand {
                            name: token(b"cmd1"),
                            args: Some(tokens_vec([b"arg1", b"arg2"])),
                        },
                        CustomCommand {
                            name: token(b"cmd2"),
                            args: None,
                        }
                    ],
                    working_directory: Some(token(b"working_directory1")),
                    timeout: Some(token(b"timeout1")),
                    result_variable: Some(token(b"result_variable1")),
                    results_variable: Some(token(b"results_variable1")),
                    output_variable: Some(token(b"output_variable1")),
                    error_variable: Some(token(b"error_variable1")),
                    input_file: Some(token(b"input_file1")),
                    output_file: Some(token(b"output_file1")),
                    error_file: Some(token(b"error_file1")),
                    output_quiet: true,
                    error_quiet: true,
                    command_echo: Some(token(b"command_echo1")),
                    output_strip_trailing_whitespace: true,
                    error_strip_trailing_whitespace: true,
                    encoding: Some(WindowsEncoding::Utf8),
                    echo_output_variable: true,
                    echo_error_variable: true,
                    command_error_is_fatal: Some(ErrorFatal::Any),
                })),
            ])
        )
    }
}
