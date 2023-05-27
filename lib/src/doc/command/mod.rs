mod add_compile_definitions;
mod add_compile_options;
mod add_custom_command;
mod add_custom_target;
mod add_definitions;
mod add_dependencies;
mod add_executable;
mod add_library;
mod add_link_options;
mod add_subdirectory;
mod add_test;
mod aux_source_directory;
mod build_command;
mod create_test_sourcelist;
mod define_property;
mod enable_language;
mod enable_testing;
mod export;
mod fltk_wrap_ui;

mod custom_command;

pub use add_compile_definitions::AddCompileDefinitions;
pub use add_compile_options::AddCompileOptions;
pub use add_custom_command::AddCustomCommand;
pub use add_custom_target::AddCustomTarget;
pub use add_definitions::AddDefinitions;
pub use add_dependencies::AddDependencies;
pub use add_executable::AddExecutable;
pub use add_library::AddLibrary;
pub use add_link_options::AddLinkOptions;
pub use add_subdirectory::AddSubdirectory;
pub use add_test::AddTest;
pub use aux_source_directory::AuxSourceDirectory;
pub use build_command::BuildCommand;
pub use create_test_sourcelist::CreateTestSourceList;
pub use define_property::DefineProperty;
pub use enable_language::EnableLanguage;
pub use export::Export;
pub use fltk_wrap_ui::FLTKWrapUI;

pub use custom_command::CustomCommand;

/// CMake command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/manual/cmake-commands.7.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<'t> {
    /// Add preprocessor definitions to the compilation of source files.
    AddCompileDefinitions(Box<AddCompileDefinitions<'t>>),
    /// Adds options to the compilation of source files.
    AddCompileOptions(Box<AddCompileOptions<'t>>),
    /// Add a custom build rule to the generated build system.
    AddCustomCommand(Box<AddCustomCommand<'t>>),
    /// Add a target with no output so it will always be built.
    AddCustomTarget(Box<AddCustomTarget<'t>>),
    /// Add -D define flags to the compilation of source files.
    AddDefinitions(Box<AddDefinitions<'t>>),
    /// Add a dependency between top-level targets.
    AddDependencies(Box<AddDependencies<'t>>),
    /// Add an executable to the project using the specified source files.
    AddExecutable(Box<AddExecutable<'t>>),
    /// Add a library to the project using the specified source files.
    AddLibrary(Box<AddLibrary<'t>>),
    /// Add options to the link step for executable, shared library or module library targets in the current directory and below that are added after this command is invoked.
    AddLinkOptions(Box<AddLinkOptions<'t>>),
    /// Add a subdirectory to the build.
    AddSubdirectory(Box<AddSubdirectory<'t>>),
    /// Add a test to the project to be run by ctest.
    AddTest(Box<AddTest<'t>>),
    /// Find all source files in a directory.
    AuxSourceDirectory(Box<AuxSourceDirectory<'t>>),
    /// Get a command line to build the current project.
    BuildCommand(Box<BuildCommand<'t>>),
    /// Create a test driver and source list for building test programs.
    CreateTestSourceList(Box<CreateTestSourceList<'t>>),
    /// Define and document custom properties.
    DefineProperty(Box<DefineProperty<'t>>),
    /// Enable languages (CXX/C/OBJC/OBJCXX/Fortran/etc)
    EnableLanguage(Box<EnableLanguage<'t>>),
    /// Enable testing for current directory and below.
    EnableTesting,
    /// Export targets or packages for outside projects to use them directly from the current project's build tree, without installation.
    Export(Box<Export<'t>>),
    /// Create FLTK user interfaces Wrappers.
    FLTKWrapUI(Box<FLTKWrapUI<'t>>),
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CommandParseError {
    #[error("required token is missing: {0}")]
    MissingToken(String),
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("unknown option: {0}")]
    UnknownOption(String),
    #[error("expected: {expected:?}, found: {found:?}")]
    UnexpectedToken { expected: String, found: String },
    #[error("token required")]
    TokenRequired,
    #[error("flag option must have no arguments")]
    NotFlag,
    #[error("all arguments must be parsed")]
    Incomplete,
}
