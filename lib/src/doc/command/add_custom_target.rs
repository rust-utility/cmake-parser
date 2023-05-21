use ::cmake_parser_derive::CMake;

use crate::{command::CustomCommand, CommandScope, ToCommandScope, Token};

/// Add a target with no output so it will always be built.
///
/// Adds a target with the given name that executes the given commands.
/// The target has no output file and is always considered out of date even
/// if the commands try to create a file with the name of the target. Use
/// the `add_custom_command()` command to generate a file with dependencies.
/// By default nothing depends on the custom target. Use the `add_dependencies()`
/// command to add dependencies to or from other targets.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_custom_target.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "commands")]
pub struct AddCustomTarget<'t> {
    #[cmake(positional)]
    name: Token<'t>,
    /// Indicate that this target should be added to the default build target so that it will be run every time (the command cannot be called ALL).
    #[cmake(positional)]
    all: bool,
    /// Specify the command-line(s) to execute at build time. If more than one COMMAND is specified they will be executed in order, but not necessarily composed into a stateful shell or batch script. (To run a full script, use the configure_file() command or the file(GENERATE) command to create it, and then specify a COMMAND to launch it.)
    #[cmake(rename = "COMMAND")]
    commands: Option<Vec<CustomCommand<'t>>>,
    /// Reference files and outputs of custom commands created with add_custom_command() command calls in the same directory (CMakeLists.txt file). They will be brought up to date when the target is built.
    depends: Option<Vec<Token<'t>>>,
    /// Specify the files the command is expected to produce but whose modification time may or may not be updated on subsequent builds. If a byproduct name is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory. Each byproduct file will be marked with the GENERATED source file property automatically.
    byproducts: Option<Vec<Token<'t>>>,
    /// Execute the command with the given current working directory. If it is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory.
    working_directory: Option<Token<'t>>,
    /// Display the given message before the commands are executed at build time.
    comment: Option<Token<'t>>,
    /// Specify a pool for the Ninja generator. Incompatible with USES_TERMINAL, which implies the console pool. Using a pool that is not defined by JOB_POOLS causes an error by ninja at build time.
    job_pool: Option<Token<'t>>,
    /// All arguments to the commands will be escaped properly for the build tool so that the invoked command receives each argument unchanged. Note that one level of escapes is still used by the CMake language processor before add_custom_target even sees the arguments. Use of VERBATIM is recommended as it enables correct behavior. When VERBATIM is not given the behavior is platform specific because there is no protection of tool-specific special characters.
    verbatim: bool,
    /// The command will be given direct access to the terminal if possible. With the Ninja generator, this places the command in the console pool.
    uses_terminal: bool,
    /// Lists in COMMAND arguments will be expanded, including those created with generator expressions, allowing COMMAND arguments such as ${CC} "-I$<JOIN:$<TARGET_PROPERTY:foo,INCLUDE_DIRECTORIES>,;-I>" foo.cc to be properly expanded.
    command_expand_lists: bool,
    /// Specify additional source files to be included in the custom target. Specified source files will be added to IDE project files for convenience in editing even if they have no build rules.
    sources: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for AddCustomTarget<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{assert_parse, tokens};
    use crate::*;

    #[test]
    fn custom_command() {
        let input = tokens([b"command", b"arg1", b"arg2", b"arg3"]);
        let (cmd, input): (CustomCommand, _) = CMakeParse::parse(&input).unwrap();
        assert!(input.is_empty());
        assert_eq!(
            cmd,
            CustomCommand {
                name: Token::text_node(b"command", false),
                args: Some(tokens([b"arg1", b"arg2", b"arg3"]).to_vec()),
            }
        );

        let vec_custom_command: Vec<CustomCommand> = assert_parse(
            [
                b"QQQ",
                b"command1",
                b"arg1",
                b"arg2",
                b"arg3",
                b"QQQ",
                b"command2",
                b"arg4",
                b"arg5",
                b"END",
            ],
            b"QQQ",
        );
        assert_eq!(
            vec_custom_command,
            vec![
                CustomCommand {
                    name: Token::text_node(b"command1", false),
                    args: Some(tokens([b"arg1", b"arg2", b"arg3"]).to_vec()),
                },
                CustomCommand {
                    name: Token::text_node(b"command2", false),
                    args: Some(tokens([b"arg4", b"arg5"]).to_vec()),
                }
            ]
        )
    }

    #[test]
    fn add_custom_target() {
        let src = include_bytes!("../../../../fixture/commands/add_custom_target");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        let commands = doc.commands().unwrap();
        dbg!(commands);
    }
}
