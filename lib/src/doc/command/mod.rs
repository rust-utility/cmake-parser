pub mod common;
pub mod ctest;
pub mod deprecated;
pub mod project;
pub mod scripting;

/// CMake command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/manual/cmake-commands.7.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<'t> {
    /// Add preprocessor definitions to the compilation of source files.
    AddCompileDefinitions(Box<project::AddCompileDefinitions<'t>>),
    /// Adds options to the compilation of source files.
    AddCompileOptions(Box<project::AddCompileOptions<'t>>),
    /// Add a custom build rule to the generated build system.
    AddCustomCommand(Box<project::AddCustomCommand<'t>>),
    /// Add a target with no output so it will always be built.
    AddCustomTarget(Box<project::AddCustomTarget<'t>>),
    /// Add -D define flags to the compilation of source files.
    AddDefinitions(Box<project::AddDefinitions<'t>>),
    /// Add a dependency between top-level targets.
    AddDependencies(Box<project::AddDependencies<'t>>),
    /// Add an executable to the project using the specified source files.
    AddExecutable(Box<project::AddExecutable<'t>>),
    /// Add a library to the project using the specified source files.
    AddLibrary(Box<project::AddLibrary<'t>>),
    /// Add options to the link step for executable, shared library or module library targets in the current directory and below that are added after this command is invoked.
    AddLinkOptions(Box<project::AddLinkOptions<'t>>),
    /// Add a subdirectory to the build.
    AddSubdirectory(Box<project::AddSubdirectory<'t>>),
    /// Add a test to the project to be run by ctest.
    AddTest(Box<project::AddTest<'t>>),
    /// Find all source files in a directory.
    AuxSourceDirectory(Box<project::AuxSourceDirectory<'t>>),
    /// Get a command line to build the current project.
    BuildCommand(Box<project::BuildCommand<'t>>),
    /// Create a test driver and source list for building test programs.
    CreateTestSourceList(Box<project::CreateTestSourceList<'t>>),
    /// Define and document custom properties.
    DefineProperty(Box<project::DefineProperty<'t>>),
    /// Enable languages (CXX/C/OBJC/OBJCXX/Fortran/etc)
    EnableLanguage(Box<project::EnableLanguage<'t>>),
    /// Enable testing for current directory and below.
    EnableTesting,
    /// Export targets or packages for outside projects to use them directly from the current project's build tree, without installation.
    Export(Box<project::Export<'t>>),
    /// Create FLTK user interfaces Wrappers.
    FLTKWrapUI(Box<project::FLTKWrapUI<'t>>),
    /// Get a property for a source file.
    GetSourceFileProperty(Box<project::GetSourceFileProperty<'t>>),
    /// Get a property from a target.
    GetTargetProperty(Box<project::GetTargetProperty<'t>>),
    /// Get a property of the test.
    GetTestProperty(Box<project::GetTestProperty<'t>>),
    /// Add include directories to the build.
    IncludeDirectories(Box<project::IncludeDirectories<'t>>),
    /// Include an external Microsoft project file in a workspace.
    IncludeExternalMSProject(Box<project::IncludeExternalMSProject<'t>>),
    /// Set the regular expression used for dependency checking.
    IncludeRegularExpression(Box<project::IncludeRegularExpression<'t>>),
    /// Specify rules to run at install time.
    Install(Box<project::Install<'t>>),
    /// Add directories in which the linker will look for libraries.
    LinkDirectories(Box<project::LinkDirectories<'t>>),
    /// Link libraries to all targets added later.
    LinkLibraries(Box<project::LinkLibraries<'t>>),
    /// Load in the values from another project's CMake cache.
    LoadCache(Box<project::LoadCache<'t>>),
    /// Set the name of the project.
    Project(Box<project::Project<'t>>),
    /// Remove -D define flags added by add_definitions().
    RemoveDefinitions(Box<project::RemoveDefinitions<'t>>),
    /// Sets properties associated with source files using a key/value paired list.
    SetSourceFileProperties(Box<project::SetSourceFileProperties<'t>>),
    /// Sets properties on targets.
    SetTargetProperties(Box<project::SetTargetProperties<'t>>),
    /// Set a property of the tests.
    SetTestsProperties(Box<project::SetTestsProperties<'t>>),
    /// Define a grouping for source files in IDE project generation.
    SourceGroup(Box<project::SourceGroup<'t>>),
    /// Add compile definitions to a target.
    TargetCompileDefinitions(Box<project::TargetCompileDefinitions<'t>>),
    /// Add expected compiler features to a target.
    TargetCompileFeatures(Box<project::TargetCompileFeatures<'t>>),
    /// Add compile options to a target.
    TargetCompileOptions(Box<project::TargetCompileOptions<'t>>),
    /// Add include directories to a target.
    TargetIncludeDirectories(Box<project::TargetIncludeDirectories<'t>>),
    /// Add link directories to a target.
    TargetLinkDirectories(Box<project::TargetLinkDirectories<'t>>),
    /// Specify libraries or flags to use when linking a given target and/or its dependents.
    TargetLinkLibraries(Box<project::TargetLinkLibraries<'t>>),
    /// Add options to the link step for an executable, shared library or module library target.
    TargetLinkOptions(Box<project::TargetLinkOptions<'t>>),
    /// Add a list of header files to precompile.
    TargetPrecompileHeaders(Box<project::TargetPrecompileHeaders<'t>>),
    /// Add sources to a target.
    TargetSources(Box<project::TargetSources<'t>>),
    /// Try building some code.
    TryCompile(Box<project::TryCompile<'t>>),
    /// Try compiling and then running some code.
    TryRun(Box<project::TryRun<'t>>),
    /// Perform the CTest Build Step as a Dashboard Client.
    CTestBuild(Box<ctest::CTestBuild<'t>>),
    /// Perform the CTest Configure Step as a Dashboard Client.
    CTestConfigure(Box<ctest::CTestConfigure<'t>>),
    /// Perform the CTest Coverage Step as a Dashboard Client.
    CTestCoverage(Box<ctest::CTestCoverage<'t>>),
    /// Removes a binary directory.
    CTestEmptyBinaryDirectory(Box<ctest::CTestEmptyBinaryDirectory<'t>>),
    /// Perform the CTest MemCheck Step as a Dashboard Client.
    CTestMemCheck(Box<ctest::CTestMemCheck<'t>>),
    /// Read all the CTestCustom.ctest or CTestCustom.cmake files from the given directory.
    CTestReadCustomFiles(Box<ctest::CTestReadCustomFiles<'t>>),
    /// Runs a script or scripts much like if it was run from ctest -S.
    CTestRunScript(Box<ctest::CTestRunScript<'t>>),
    /// Sleeps for some amount of time
    CTestSleep(Box<ctest::CTestSleep<'t>>),
    /// Starts the testing for a given model
    CTestStart(Box<ctest::CTestStart<'t>>),
    /// Perform the CTest Submit Step as a Dashboard Client.
    CTestSubmit(Box<ctest::CTestSubmit<'t>>),
    /// Perform the CTest Test Step as a Dashboard Client.
    CTestTest(Box<ctest::CTestTest<'t>>),
    /// Perform the CTest Update Step as a Dashboard Client.
    CTestUpdate(Box<ctest::CTestUpdate<'t>>),
    /// Upload files to a dashboard server as a Dashboard Client.
    CTestUpload(Box<ctest::CTestUpload<'t>>),
    /// Sets the specified variable to a string representing the platform and compiler settings.
    BuildName(Box<deprecated::BuildName<'t>>),
    /// Run an executable program during the processing of the CMakeList.txt file.
    ExecProgram(Box<deprecated::ExecProgram<'t>>),
    /// This command generates an old-style library dependencies file.
    ExportLibraryDependencies(Box<deprecated::ExportLibraryDependencies<'t>>),
    /// Specifies rules for installing files for a project.
    InstallFiles(Box<deprecated::InstallFiles<'t>>),
    /// Specifies rules for installing programs for a project.
    InstallPrograms(Box<deprecated::InstallPrograms<'t>>),
    /// Create rules to install the listed targets into the given directory.
    InstallTargets(Box<deprecated::InstallTargets<'t>>),
    /// Load a command into a running CMake.
    LoadCommand(Box<deprecated::LoadCommand<'t>>),
    /// Creates the specified directory.
    MakeDirectory(Box<deprecated::MakeDirectory<'t>>),
    /// Approximate C preprocessor dependency scanning.
    OutputRequiredFiles(Box<deprecated::OutputRequiredFiles<'t>>),
    /// Manually create Qt Wrappers.
    QtWrapCpp(Box<deprecated::QtWrapCpp<'t>>),
    /// Manually create Qt user interfaces Wrappers.
    QtWrapUi(Box<deprecated::QtWrapUi<'t>>),
    /// Removes value from the variable.
    Remove(Box<deprecated::Remove<'t>>),
    /// Does nothing.
    SubdirDepends(Box<deprecated::SubdirDepends<'t>>),
    /// Add a list of subdirectories to the build.
    Subdirs(Box<deprecated::Subdirs<'t>>),
    /// Copy mesa headers for use in combination with system GL.
    UseMangledMesa(Box<deprecated::UseMangledMesa<'t>>),
    /// Specify the source tree of a third-party utility.
    UtilitySource(Box<deprecated::UtilitySource<'t>>),
    /// Assert satisfaction of an option's required variables.
    VariableRequires(Box<deprecated::VariableRequires<'t>>),
    /// Write content into a file.
    WriteFile(Box<deprecated::WriteFile<'t>>),
    /// Evaluate a group of commands with a dedicated variable and/or policy scope.
    Block(Box<scripting::Block<'t>>),
    /// Break from an enclosing foreach or while loop.
    Break(Box<scripting::Break>),
    /// Query various host system information.
    CMakeHostSystemInformation(Box<scripting::CMakeHostSystemInformation<'t>>),
    /// Call meta-operations on CMake commands.
    CMakeLanguage(Box<scripting::CMakeLanguage<'t>>),
    /// Require a minimum version of cmake.
    CMakeMinimumRequired(Box<scripting::CMakeMinimumRequired<'t>>),
    /// Parse function or macro arguments.
    CMakeParseArguments(Box<scripting::CMakeParseArguments<'t>>),
    /// This command is for the manipulation of paths.
    CMakePath(Box<scripting::CMakePath<'t>>),
    /// Manage CMake Policy settings.
    CMakePolicy(Box<scripting::CMakePolicy<'t>>),
    /// Copy a file to another location and modify its contents.
    ConfigureFile(Box<scripting::ConfigureFile<'t>>),
    /// Continue to the top of enclosing foreach or while loop.
    Continue(Box<scripting::Continue>),
    /// Starts the else portion of an if block.
    Else(Box<scripting::Else<'t>>),
    /// Starts an elseif portion of an if block.
    ElseIf(Box<scripting::ElseIf<'t>>),
    /// Ends a list of commands in a block() and removes the scopes created by the block() command.
    EndBlock(Box<scripting::EndBlock>),
    /// Ends a list of commands in a foreach block.
    EndForEach(Box<scripting::EndForEach<'t>>),
    /// Ends a list of commands in a function block.
    EndFunction(Box<scripting::EndFunction<'t>>),
    /// Ends a list of commands in an if block.
    EndIf(Box<scripting::EndIf<'t>>),
    /// Ends a list of commands in a macro block.
    EndMacro(Box<scripting::EndMacro<'t>>),
    /// Ends a list of commands in a while block.
    EndWhile(Box<scripting::EndWhile<'t>>),
    /// Execute one or more child processes.
    ExecuteProcess(Box<scripting::ExecuteProcess<'t>>),
    /// File manipulation command.
    File(Box<scripting::File<'t>>),
    /// This command is used to find a full path to named file.
    FindFile(Box<scripting::FindFile<'t>>),
    /// This command is used to find a library.
    FindLibrary(Box<scripting::FindLibrary<'t>>),
    /// Find a package
    FindPackage(Box<scripting::FindPackage<'t>>),
    /// This command is used to find a directory containing the named file.
    FindPath(Box<scripting::FindPath<'t>>),
    /// This command is used to find a program.
    FindProgram(Box<scripting::FindProgram<'t>>),
    /// Evaluate a group of commands for each value in a list.
    ForEach(Box<scripting::ForEach<'t>>),
    /// Start recording a function for later invocation as a command.
    Function(Box<scripting::Function<'t>>),
    /// Get a global property of the CMake instance.
    GetCMakeProperty(Box<scripting::GetCMakeProperty<'t>>),
    /// Get a property of DIRECTORY scope.
    GetDirectoryProperty(Box<scripting::GetDirectoryProperty<'t>>),
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
    #[error("no tokens expected")]
    NotEmpty,
    #[error("flag option must have no arguments")]
    NotFlag,
    #[error("all arguments must be parsed")]
    Incomplete,
}
