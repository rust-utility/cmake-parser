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
