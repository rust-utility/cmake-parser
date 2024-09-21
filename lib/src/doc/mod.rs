mod cmake_parse;
mod cmake_positional;
pub mod command;
mod command_scope;
mod token;

use crate::CMakeListsTokens;

pub use cmake_parse::CMakeParse;
pub use cmake_positional::{CMakePositional, Keyword};
use command::CommandParseError;

pub use command::Command;
pub use command_scope::{CommandScope, ToCommandScope};
pub use token::{declarations_by_keywords, TextNodeDeclaration, Token, TokenDeclarations};

pub struct Doc<'t> {
    tokens: CMakeListsTokens<'t>,
}

impl<'t> Doc<'t> {
    pub fn to_commands_iter<'a: 't>(
        &'a self,
    ) -> impl Iterator<Item = Result<Command<'t>, CommandParseError>> {
        self.tokens
            .command_invocations()
            .map(|ci| (ci.identifier(), ci.to_text_nodes()))
            .map(move |(identifier, tokens)| match &identifier[..] {
                b"add_compile_definitions" => to_command(tokens, Command::AddCompileDefinitions),
                b"add_compile_options" => to_command(tokens, Command::AddCompileOptions),
                b"add_custom_command" => to_command(tokens, Command::AddCustomCommand),
                b"add_custom_target" => to_command(tokens, Command::AddCustomTarget),
                b"add_definitions" => to_command(tokens, Command::AddDefinitions),
                b"add_dependencies" => to_command(tokens, Command::AddDependencies),
                b"add_executable" => to_command(tokens, Command::AddExecutable),
                b"add_library" => to_command(tokens, Command::AddLibrary),
                b"add_link_options" => to_command(tokens, Command::AddLinkOptions),
                b"add_subdirectory" => to_command(tokens, Command::AddSubdirectory),
                b"add_test" => to_command(tokens, Command::AddTest),
                b"aux_source_directory" => to_command(tokens, Command::AuxSourceDirectory),
                b"build_command" => to_command(tokens, Command::BuildCommand),
                b"create_test_sourcelist" => to_command(tokens, Command::CreateTestSourceList),
                b"define_property" => to_command(tokens, Command::DefineProperty),
                b"enable_language" => to_command(tokens, Command::EnableLanguage),
                b"enable_testing" => Ok(Command::EnableTesting),
                b"export" => to_command(tokens, Command::Export),
                b"fltk_wrap_ui" => to_command(tokens, Command::FLTKWrapUI),
                b"get_source_file_property" => to_command(tokens, Command::GetSourceFileProperty),
                b"get_target_property" => to_command(tokens, Command::GetTargetProperty),
                b"get_test_property" => to_command(tokens, Command::GetTestProperty),
                b"include_directories" => to_command(tokens, Command::IncludeDirectories),
                b"include_external_msproject" => {
                    to_command(tokens, Command::IncludeExternalMSProject)
                }
                b"include_regular_expression" => {
                    to_command(tokens, Command::IncludeRegularExpression)
                }
                b"install" => to_command(tokens, Command::Install),
                b"link_directories" => to_command(tokens, Command::LinkDirectories),
                b"link_libraries" => to_command(tokens, Command::LinkLibraries),
                b"load_cache" => to_command(tokens, Command::LoadCache),
                b"project" => to_command(tokens, Command::Project),
                b"remove_definitions" => to_command(tokens, Command::RemoveDefinitions),
                b"set_source_files_properties" => {
                    to_command(tokens, Command::SetSourceFileProperties)
                }
                b"set_target_properties" => to_command(tokens, Command::SetTargetProperties),
                b"set_tests_properties" => to_command(tokens, Command::SetTestsProperties),
                b"source_group" => to_command(tokens, Command::SourceGroup),
                b"target_compile_definitions" => {
                    to_command(tokens, Command::TargetCompileDefinitions)
                }
                b"target_compile_features" => to_command(tokens, Command::TargetCompileFeatures),
                b"target_compile_options" => to_command(tokens, Command::TargetCompileOptions),
                b"target_include_directories" => {
                    to_command(tokens, Command::TargetIncludeDirectories)
                }
                b"target_link_directories" => to_command(tokens, Command::TargetLinkDirectories),
                b"target_link_libraries" => to_command(tokens, Command::TargetLinkLibraries),
                b"target_link_options" => to_command(tokens, Command::TargetLinkOptions),
                b"target_precompile_headers" => {
                    to_command(tokens, Command::TargetPrecompileHeaders)
                }
                b"target_sources" => to_command(tokens, Command::TargetSources),
                b"try_compile" => to_command(tokens, Command::TryCompile),
                b"try_run" => to_command(tokens, Command::TryRun),
                b"ctest_build" => to_command(tokens, Command::CTestBuild),
                b"ctest_configure" => to_command(tokens, Command::CTestConfigure),
                b"ctest_coverage" => to_command(tokens, Command::CTestCoverage),
                b"ctest_empty_binary_directory" => {
                    to_command(tokens, Command::CTestEmptyBinaryDirectory)
                }
                b"ctest_memcheck" => to_command(tokens, Command::CTestMemCheck),
                b"ctest_read_custom_files" => to_command(tokens, Command::CTestReadCustomFiles),
                b"ctest_run_script" => to_command(tokens, Command::CTestRunScript),
                b"ctest_sleep" => to_command(tokens, Command::CTestSleep),
                b"ctest_start" => to_command(tokens, Command::CTestStart),
                b"ctest_submit" => to_command(tokens, Command::CTestSubmit),
                b"ctest_test" => to_command(tokens, Command::CTestTest),
                b"ctest_update" => to_command(tokens, Command::CTestUpdate),
                b"ctest_upload" => to_command(tokens, Command::CTestUpload),
                b"build_name" => to_command(tokens, Command::BuildName),
                b"exec_program" => to_command(tokens, Command::ExecProgram),
                b"export_library_dependencies" => {
                    to_command(tokens, Command::ExportLibraryDependencies)
                }
                b"install_files" => to_command(tokens, Command::InstallFiles),
                b"install_programs" => to_command(tokens, Command::InstallPrograms),
                b"install_targets" => to_command(tokens, Command::InstallTargets),
                b"load_command" => to_command(tokens, Command::LoadCommand),
                b"make_directory" => to_command(tokens, Command::MakeDirectory),
                b"output_required_files" => to_command(tokens, Command::OutputRequiredFiles),
                b"qt_wrap_cpp" => to_command(tokens, Command::QtWrapCpp),
                b"qt_wrap_ui" => to_command(tokens, Command::QtWrapUi),
                b"remove" => to_command(tokens, Command::Remove),
                b"subdir_depends" => to_command(tokens, Command::SubdirDepends),
                b"subdirs" => to_command(tokens, Command::Subdirs),
                b"use_mangled_mesa" => to_command(tokens, Command::UseMangledMesa),
                b"utility_source" => to_command(tokens, Command::UtilitySource),
                b"variable_requires" => to_command(tokens, Command::VariableRequires),
                b"write_file" => to_command(tokens, Command::WriteFile),
                b"block" => to_command(tokens, Command::Block),
                b"break" => to_command(tokens, Command::Break),
                b"cmake_host_system_information" => {
                    to_command(tokens, Command::CMakeHostSystemInformation)
                }
                b"cmake_language" => to_command(tokens, Command::CMakeLanguage),
                b"cmake_minimum_required" => to_command(tokens, Command::CMakeMinimumRequired),
                b"cmake_parse_arguments" => to_command(tokens, Command::CMakeParseArguments),
                b"cmake_path" => to_command(tokens, Command::CMakePath),
                b"cmake_policy" => to_command(tokens, Command::CMakePolicy),
                b"configure_file" => to_command(tokens, Command::ConfigureFile),
                b"continue" => to_command(tokens, Command::Continue),
                b"else" => to_command(tokens, Command::Else),
                b"elseif" => to_command(tokens, Command::ElseIf),
                b"endblock" => to_command(tokens, Command::EndBlock),
                b"endforeach" => to_command(tokens, Command::EndForEach),
                b"endfunction" => to_command(tokens, Command::EndFunction),
                b"endif" => to_command(tokens, Command::EndIf),
                b"endmacro" => to_command(tokens, Command::EndMacro),
                b"endwhile" => to_command(tokens, Command::EndWhile),
                b"execute_process" => to_command(tokens, Command::ExecuteProcess),
                b"file" => to_command(tokens, Command::File),
                b"find_file" => to_command(tokens, Command::FindFile),
                b"find_library" => to_command(tokens, Command::FindLibrary),
                b"find_package" => to_command(tokens, Command::FindPackage),
                b"find_path" => to_command(tokens, Command::FindPath),
                b"find_program" => to_command(tokens, Command::FindProgram),
                b"foreach" => to_command(tokens, Command::ForEach),
                b"function" => to_command(tokens, Command::Function),
                b"get_cmake_property" => to_command(tokens, Command::GetCMakeProperty),
                b"get_directory_property" => to_command(tokens, Command::GetDirectoryProperty),
                b"get_filename_component" => to_command(tokens, Command::GetFilenameComponent),
                b"get_property" => to_command(tokens, Command::GetProperty),
                b"if" => to_command(tokens, Command::If),
                b"include" => to_command(tokens, Command::Include),
                b"include_guard" => to_command(tokens, Command::IncludeGuard),
                b"list" => to_command(tokens, Command::List),
                b"macro" => to_command(tokens, Command::Macro),
                b"mark_as_advanced" => to_command(tokens, Command::MarkAsAdvanced),
                b"math" => to_command(tokens, Command::Math),
                b"message" => to_command(tokens, Command::Message),
                b"option" => to_command(tokens, Command::Option),
                b"return" => to_command(tokens, Command::Return),
                b"separate_arguments" => to_command(tokens, Command::SeparateArguments),
                b"set" => to_command(tokens, Command::Set),
                b"set_directory_properties" => to_command(tokens, Command::SetDirectoryProperties),
                b"set_property" => to_command(tokens, Command::SetProperty),
                b"site_name" => to_command(tokens, Command::SiteName),
                b"string" => to_command(tokens, Command::String),
                unknown => Err(CommandParseError::UnknownCommand(
                    String::from_utf8_lossy(unknown).to_string(),
                )),
            })
    }

    pub fn commands<'a: 't>(&'a self) -> Result<Vec<Command<'t>>, CommandParseError> {
        self.to_commands_iter().collect()
    }
}

impl<'t> From<CMakeListsTokens<'t>> for Doc<'t> {
    fn from(tokens: CMakeListsTokens<'t>) -> Self {
        Self { tokens }
    }
}
fn to_command<'t, C, F>(tokens: Vec<Token<'t>>, f: F) -> Result<Command<'t>, CommandParseError>
where
    C: CMakeParse<'t>,
    F: Fn(Box<C>) -> Command<'t>,
{
    CMakeParse::complete(&tokens).map(f)
}
