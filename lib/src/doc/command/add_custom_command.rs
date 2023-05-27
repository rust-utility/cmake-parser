use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add a custom build rule to the generated build system.
///
/// There are two main signatures for add_custom_command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_custom_command.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum AddCustomCommand<'t> {
    Output(AddCustomCommandOutput<'t>),
    Target(AddCustomCommandTarget<'t>),
}

impl<'t> ToCommandScope for AddCustomCommand<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddCustomCommandOutput<'t> {
    /// Specify the output files the command is expected to produce. Each output file will be marked with the GENERATED source file property automatically. If the output of the custom command is not actually created as a file on disk it should be marked with the SYMBOLIC source file property.
    ///
    /// If an output file name is a relative path, its absolute path is determined by interpreting it relative to:
    ///
    /// 1. the build directory corresponding to the current source directory (CMAKE_CURRENT_BINARY_DIR), or
    /// 1. the current source directory (CMAKE_CURRENT_SOURCE_DIR).
    ///
    /// The path in the build directory is preferred unless the path in the source tree is mentioned as an absolute source file path elsewhere in the current directory.
    pub output: Vec<Token<'t>>,
    /// Specify the command-line(s) to execute at build time. If more than one COMMAND is specified they will be executed in order, but not necessarily composed into a stateful shell or batch script. (To run a full script, use the configure_file() command or the file(GENERATE) command to create it, and then specify a COMMAND to launch it.) The optional ARGS argument is for backward compatibility and will be ignored.
    ///
    /// 1. If COMMAND specifies an executable target name (created by the add_executable() command), it will automatically be replaced by the location of the executable created at build time if either of the following is true:
    ///     - The target is not being cross-compiled (i.e. the CMAKE_CROSSCOMPILING variable is not set to true).
    ///     - New in version 3.6: The target is being cross-compiled and an emulator is provided (i.e. its CROSSCOMPILING_EMULATOR target property is set). In this case, the contents of CROSSCOMPILING_EMULATOR will be prepended to the command before the location of the target executable.
    /// 1. If neither of the above conditions are met, it is assumed that the command name is a program to be found on the PATH at build time.
    ///
    /// Arguments to COMMAND may use generator expressions. Use the TARGET_FILE generator expression to refer to the location of a target later in the command line (i.e. as a command argument rather than as the command to execute).
    ///
    /// Whenever one of the following target based generator expressions are used as a command to execute or is mentioned in a command argument, a target-level dependency will be added automatically so that the mentioned target will be built before any target using this custom command (see policy CMP0112).
    ///
    /// - TARGET_FILE
    /// - TARGET_LINKER_FILE
    /// - TARGET_SONAME_FILE
    /// - TARGET_PDB_FILE
    ///
    /// This target-level dependency does NOT add a file-level dependency that would cause the custom command to re-run whenever the executable is recompiled. List target names with the DEPENDS option to add such file-level dependencies.
    #[cmake(rename = "COMMAND")]
    pub commands: Vec<Vec<Token<'t>>>,
    /// Specify the primary input source file to the command. This is treated just like any value given to the DEPENDS option but also suggests to Visual Studio generators where to hang the custom command. Each source file may have at most one command specifying it as its main dependency. A compile command (i.e. for a library or an executable) counts as an implicit main dependency which gets silently overwritten by a custom command specification.
    pub main_dependency: Option<Token<'t>>,
    /// Specify files on which the command depends. Each argument is converted to a dependency as follows:
    ///
    /// 1. If the argument is the name of a target (created by the add_custom_target(), add_executable(), or add_library() command) a target-level dependency is created to make sure the target is built before any target using this custom command. Additionally, if the target is an executable or library, a file-level dependency is created to cause the custom command to re-run whenever the target is recompiled.
    /// 1. If the argument is an absolute path, a file-level dependency is created on that path.
    /// 1. If the argument is the name of a source file that has been added to a target or on which a source file property has been set, a file-level dependency is created on that source file.
    /// 1. If the argument is a relative path and it exists in the current source directory, a file-level dependency is created on that file in the current source directory.
    /// 1. Otherwise, a file-level dependency is created on that path relative to the current binary directory.
    ///
    /// If any dependency is an OUTPUT of another custom command in the same directory (CMakeLists.txt file), CMake automatically brings the other custom command into the target in which this command is built.
    ///
    /// New in version 3.16: A target-level dependency is added if any dependency is listed as BYPRODUCTS of a target or any of its build events in the same directory to ensure the byproducts will be available.
    ///
    /// If DEPENDS is not specified, the command will run whenever the OUTPUT is missing; if the command does not actually create the OUTPUT, the rule will always run.
    ///
    /// New in version 3.1: Arguments to DEPENDS may use generator expressions.
    pub depends: Option<Vec<Token<'t>>>,
    /// Specify the files the command is expected to produce but whose modification time may or may not be newer than the dependencies. If a byproduct name is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory. Each byproduct file will be marked with the GENERATED source file property automatically.
    ///
    /// See policy CMP0058 for the motivation behind this feature.
    ///
    /// Explicit specification of byproducts is supported by the Ninja generator to tell the ninja build tool how to regenerate byproducts when they are missing. It is also useful when other build rules (e.g. custom commands) depend on the byproducts. Ninja requires a build rule for any generated file on which another rule depends even if there are order-only dependencies to ensure the byproducts will be available before their dependents build.
    ///
    /// The Makefile Generators will remove BYPRODUCTS and other GENERATED files during make clean.
    ///
    /// New in version 3.20: Arguments to BYPRODUCTS may use a restricted set of generator expressions. Target-dependent expressions are not permitted.
    pub byproducts: Option<Vec<Token<'t>>>,
    /// Request scanning of implicit dependencies of an input file. The language given specifies the programming language whose corresponding dependency scanner should be used. Currently only C and CXX language scanners are supported. The language has to be specified for every file in the IMPLICIT_DEPENDS list. Dependencies discovered from the scanning are added to those of the custom command at build time. Note that the IMPLICIT_DEPENDS option is currently supported only for Makefile generators and will be ignored by other generators.
    ///
    /// Note: This option cannot be specified at the same time as DEPFILE option.
    pub implicit_depends: Option<Vec<(Token<'t>, Token<'t>)>>,
    /// Execute the command with the given current working directory. If it is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory.
    pub working_directory: Option<Token<'t>>,
    /// Display the given message before the commands are executed at build time.
    ///
    /// New in version 3.26: Arguments to COMMENT may use generator expressions.
    pub comment: Option<Token<'t>>,
    /// Specify a depfile which holds dependencies for the custom command. It is usually emitted by the custom command itself. This keyword may only be used if the generator supports it, as detailed below.
    ///
    /// The expected format, compatible with what is generated by gcc with the option -M, is independent of the generator or platform.
    ///
    /// Note: DEPFILE cannot be specified at the same time as the IMPLICIT_DEPENDS option for Makefile Generators.
    pub depfile: Option<Token<'t>>,
    /// Specify a pool for the Ninja generator. Incompatible with USES_TERMINAL, which implies the console pool. Using a pool that is not defined by JOB_POOLS causes an error by ninja at build time.
    pub job_pool: Option<Token<'t>>,
    /// All arguments to the commands will be escaped properly for the build tool so that the invoked command receives each argument unchanged. Note that one level of escapes is still used by the CMake language processor before add_custom_command even sees the arguments. Use of VERBATIM is recommended as it enables correct behavior. When VERBATIM is not given the behavior is platform specific because there is no protection of tool-specific special characters.
    pub verbatim: bool,
    /// Append the COMMAND and DEPENDS option values to the custom command for the first output specified. There must have already been a previous call to this command with the same output.
    ///
    /// If the previous call specified the output via a generator expression, the output specified by the current call must match in at least one configuration after evaluating generator expressions. In this case, the appended commands and dependencies apply to all configurations.
    ///
    /// The COMMENT, MAIN_DEPENDENCY, and WORKING_DIRECTORY options are currently ignored when APPEND is given, but may be used in the future.
    pub append: bool,
    /// The command will be given direct access to the terminal if possible. With the Ninja generator, this places the command in the console pool.
    pub uses_terminal: bool,
    /// Lists in COMMAND arguments will be expanded, including those created with generator expressions, allowing COMMAND arguments such as ${CC} "-I$<JOIN:$<TARGET_PROPERTY:foo,INCLUDE_DIRECTORIES>,;-I>" foo.cc to be properly expanded.
    pub command_expands_list: bool,
}

/// This defines a new command that will be associated with building the specified <target>. The <target> must be defined in the current directory; targets defined in other directories may not be specified.
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddCustomCommandTarget<'t> {
    pub target: Token<'t>,
    /// When the command will happen
    ///
    /// Projects should always specify one of the above three keywords when using the TARGET form. For backward compatibility reasons, POST_BUILD is assumed if no such keyword is given, but projects should explicitly provide one of the keywords to make clear the behavior they expect.
    pub when: AddCustomCommandTargetWhen,
    /// Specify the command-line(s) to execute at build time. If more than one COMMAND is specified they will be executed in order, but not necessarily composed into a stateful shell or batch script. (To run a full script, use the configure_file() command or the file(GENERATE) command to create it, and then specify a COMMAND to launch it.) The optional ARGS argument is for backward compatibility and will be ignored.
    ///
    /// 1. If COMMAND specifies an executable target name (created by the add_executable() command), it will automatically be replaced by the location of the executable created at build time if either of the following is true:
    ///     - The target is not being cross-compiled (i.e. the CMAKE_CROSSCOMPILING variable is not set to true).
    ///     - New in version 3.6: The target is being cross-compiled and an emulator is provided (i.e. its CROSSCOMPILING_EMULATOR target property is set). In this case, the contents of CROSSCOMPILING_EMULATOR will be prepended to the command before the location of the target executable.
    /// 1. If neither of the above conditions are met, it is assumed that the command name is a program to be found on the PATH at build time.
    ///
    /// Arguments to COMMAND may use generator expressions. Use the TARGET_FILE generator expression to refer to the location of a target later in the command line (i.e. as a command argument rather than as the command to execute).
    ///
    /// Whenever one of the following target based generator expressions are used as a command to execute or is mentioned in a command argument, a target-level dependency will be added automatically so that the mentioned target will be built before any target using this custom command (see policy CMP0112).
    ///
    /// - TARGET_FILE
    /// - TARGET_LINKER_FILE
    /// - TARGET_SONAME_FILE
    /// - TARGET_PDB_FILE
    ///
    /// This target-level dependency does NOT add a file-level dependency that would cause the custom command to re-run whenever the executable is recompiled. List target names with the DEPENDS option to add such file-level dependencies.
    #[cmake(rename = "COMMAND")]
    pub commands: Vec<Vec<Token<'t>>>,
    /// Specify the primary input source file to the command. This is treated just like any value given to the DEPENDS option but also suggests to Visual Studio generators where to hang the custom command. Each source file may have at most one command specifying it as its main dependency. A compile command (i.e. for a library or an executable) counts as an implicit main dependency which gets silently overwritten by a custom command specification.
    pub main_dependency: Option<Token<'t>>,
    /// Specify files on which the command depends. Each argument is converted to a dependency as follows:
    ///
    /// 1. If the argument is the name of a target (created by the add_custom_target(), add_executable(), or add_library() command) a target-level dependency is created to make sure the target is built before any target using this custom command. Additionally, if the target is an executable or library, a file-level dependency is created to cause the custom command to re-run whenever the target is recompiled.
    /// 1. If the argument is an absolute path, a file-level dependency is created on that path.
    /// 1. If the argument is the name of a source file that has been added to a target or on which a source file property has been set, a file-level dependency is created on that source file.
    /// 1. If the argument is a relative path and it exists in the current source directory, a file-level dependency is created on that file in the current source directory.
    /// 1. Otherwise, a file-level dependency is created on that path relative to the current binary directory.
    ///
    /// If any dependency is an OUTPUT of another custom command in the same directory (CMakeLists.txt file), CMake automatically brings the other custom command into the target in which this command is built.
    ///
    /// New in version 3.16: A target-level dependency is added if any dependency is listed as BYPRODUCTS of a target or any of its build events in the same directory to ensure the byproducts will be available.
    ///
    /// If DEPENDS is not specified, the command will run whenever the OUTPUT is missing; if the command does not actually create the OUTPUT, the rule will always run.
    ///
    /// New in version 3.1: Arguments to DEPENDS may use generator expressions.
    pub depends: Option<Vec<Token<'t>>>,
    /// Specify the files the command is expected to produce but whose modification time may or may not be newer than the dependencies. If a byproduct name is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory. Each byproduct file will be marked with the GENERATED source file property automatically.
    ///
    /// See policy CMP0058 for the motivation behind this feature.
    ///
    /// Explicit specification of byproducts is supported by the Ninja generator to tell the ninja build tool how to regenerate byproducts when they are missing. It is also useful when other build rules (e.g. custom commands) depend on the byproducts. Ninja requires a build rule for any generated file on which another rule depends even if there are order-only dependencies to ensure the byproducts will be available before their dependents build.
    ///
    /// The Makefile Generators will remove BYPRODUCTS and other GENERATED files during make clean.
    ///
    /// New in version 3.20: Arguments to BYPRODUCTS may use a restricted set of generator expressions. Target-dependent expressions are not permitted.
    pub byproducts: Option<Vec<Token<'t>>>,
    /// Execute the command with the given current working directory. If it is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory.
    pub working_directory: Option<Token<'t>>,
    /// Display the given message before the commands are executed at build time.
    ///
    /// New in version 3.26: Arguments to COMMENT may use generator expressions.
    pub comment: Option<Token<'t>>,
    /// All arguments to the commands will be escaped properly for the build tool so that the invoked command receives each argument unchanged. Note that one level of escapes is still used by the CMake language processor before add_custom_command even sees the arguments. Use of VERBATIM is recommended as it enables correct behavior. When VERBATIM is not given the behavior is platform specific because there is no protection of tool-specific special characters.
    pub verbatim: bool,
    /// The command will be given direct access to the terminal if possible. With the Ninja generator, this places the command in the console pool.
    pub uses_terminal: bool,
    /// Lists in COMMAND arguments will be expanded, including those created with generator expressions, allowing COMMAND arguments such as ${CC} "-I$<JOIN:$<TARGET_PROPERTY:foo,INCLUDE_DIRECTORIES>,;-I>" foo.cc to be properly expanded.
    pub command_expands_list: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum AddCustomCommandTargetWhen {
    /// On [Visual Studio Generators](https://cmake.org/cmake/help/v3.26/manual/cmake-generators.7.html#visual-studio-generators), run before any other rules are executed within the target. On other generators, run just before PRE_LINK commands.
    PreBuild,
    /// Run after sources have been compiled but before linking the binary or running the librarian or archiver tool of a static library. This is not defined for targets created by the [add_custom_target()](https://cmake.org/cmake/help/v3.26/command/add_custom_target.html#command:add_custom_target) command.
    PreLink,
    /// Run after all other rules within the target have been executed.
    PostBuild,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_custom_command() {
        let src = include_bytes!("../../../../fixture/commands/project/add_custom_command");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        let commands = doc.commands().unwrap();
        dbg!(commands);
    }
}
