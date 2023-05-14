use ::cmake_parser_derive::CMake2;

use crate::Token;

use super::{CMakeParse, CMakePositional};

#[derive(CMake2, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(positional, pkg = "crate")]
pub struct CustomCommand<'t> {
    name: Token<'t>,
    args: Vec<Token<'t>>,
}

#[derive(CMake2, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddCustomTarget<'t> {
    #[cmake(positional)]
    name: Token<'t>,
    #[cmake(positional)]
    all: bool,
    #[cmake(rename = "COMMAND")]
    commands: Option<Vec<CustomCommand<'t>>>,
    byproducts: Option<Vec<Token<'t>>>,
    working_directory: Option<Token<'t>>,
    comment: Option<Token<'t>>,
    job_pool: Option<Token<'t>>,
    verbatim: bool,
    uses_terminal: bool,
    command_expand_lists: bool,
    sources: Option<Vec<Token<'t>>>,
}

/*
impl<'t> CMakeParse<'t> for AddCustomTarget<'t> {
    fn cmake_parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), crate::CommandParseError> {
        let (name, tokens) = CMakePositional::positional(b"NAME", tokens)?;
        let (all, mut tokens) = CMakePositional::positional(b"ALL", tokens)?;

        #[derive(Default)]
        struct Buffers<'b> {
            commands: Vec<Token<'b>>,
            byproducts: Vec<Token<'b>>,
            working_directory: Vec<Token<'b>>,
            comment: Vec<Token<'b>>,
            job_pool: Vec<Token<'b>>,
            verbatim: Vec<Token<'b>>,
            uses_terminal: Vec<Token<'b>>,
            command_expand_lists: Vec<Token<'b>>,
            sources: Vec<Token<'b>>,
        }
        let mut buffers = Buffers::default();

        let mut commands = CMakeParse::default_value();
        let mut byproducts = CMakeParse::default_value();
        let mut working_directory = CMakeParse::default_value();
        let mut comment = CMakeParse::default_value();
        let mut job_pool = CMakeParse::default_value();
        let mut verbatim = CMakeParse::default_value();
        let mut uses_terminal = CMakeParse::default_value();
        let mut command_expand_lists = CMakeParse::default_value();
        let mut sources = CMakeParse::default_value();

        enum CMakeParserMode {
            Commands,
            Byproducts,
            WorkingDirectory,
            Comment,
            JobPool,
            Verbatim,
            UsesTerminal,
            CommandExpandLists,
            Sources,
        }

        let mut current_mode = Some(CMakeParserMode::Commands);

        loop {
            let Some((first, rest)) = tokens.split_first() else {
                break;
            };
            tokens = rest;
            match first.as_bytes() {
                keyword => match &current_mode {
                    Some(mode) => match mode {
                        CMakeParserMode::Commands => buffers.commands.push(first.clone()),
                        CMakeParserMode::Byproducts => buffers.byproducts.push(first.clone()),
                        CMakeParserMode::WorkingDirectory => {
                            buffers.working_directory.push(first.clone())
                        }
                        CMakeParserMode::Comment => buffers.comment.push(first.clone()),
                        CMakeParserMode::JobPool => buffers.job_pool.push(first.clone()),
                        CMakeParserMode::Verbatim => buffers.verbatim.push(first.clone()),
                        CMakeParserMode::UsesTerminal => buffers.uses_terminal.push(first.clone()),
                        CMakeParserMode::CommandExpandLists => {
                            buffers.command_expand_lists.push(first.clone())
                        }
                        CMakeParserMode::Sources => buffers.sources.push(first.clone()),
                    },
                    None => {
                        return Err(crate::CommandParseError::UnknownOption(
                            String::from_utf8_lossy(keyword).to_string(),
                        ))
                    }
                },
            }
        }

        Ok((
            Self {
                name,
                all,
                commands: commands
                    .cmake_event_end(&buffers.commands)?
                    .ok_or_else(|| crate::CommandParseError::MissingToken("COMMAND".to_string()))?,
                byproducts: byproducts.ok_or_else(|| {
                    crate::CommandParseError::MissingToken("byproducts".to_string())
                })?,
                working_directory: working_directory.ok_or_else(|| {
                    crate::CommandParseError::MissingToken("working_directory".to_string())
                })?,
                comment: comment
                    .ok_or_else(|| crate::CommandParseError::MissingToken("comment".to_string()))?,
                job_pool: job_pool.ok_or_else(|| {
                    crate::CommandParseError::MissingToken("job_pool".to_string())
                })?,
                verbatim: verbatim.ok_or_else(|| {
                    crate::CommandParseError::MissingToken("verbatim".to_string())
                })?,
                uses_terminal: uses_terminal.ok_or_else(|| {
                    crate::CommandParseError::MissingToken("uses_terminal".to_string())
                })?,
                command_expand_lists: command_expand_lists.ok_or_else(|| {
                    crate::CommandParseError::MissingToken("command_expand_lists".to_string())
                })?,
                sources: sources
                    .ok_or_else(|| crate::CommandParseError::MissingToken("sources".to_string()))?,
            },
            tokens,
        ))
    }
}

 */

#[cfg(test)]
mod tests {
    use super::super::cmake_parse::tests::{assert_parse, tokens};
    use super::*;
    use crate::*;

    #[test]
    fn custom_command() {
        let input = tokens([b"command", b"arg1", b"arg2", b"arg3"]);
        let (cmd, input): (CustomCommand, _) = CMakeParse::cmake_parse(&input).unwrap();
        assert!(input.is_empty());
        assert_eq!(
            cmd,
            CustomCommand {
                name: Token::text_node(b"command", false),
                args: tokens([b"arg1", b"arg2", b"arg3"]).to_vec(),
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
                    args: tokens([b"arg1", b"arg2", b"arg3"]).to_vec(),
                },
                CustomCommand {
                    name: Token::text_node(b"command2", false),
                    args: tokens([b"arg4", b"arg5"]).to_vec(),
                }
            ]
        )
    }

    #[test]
    fn add_compile_options() {
        let src = include_bytes!("../../../../fixture/commands/add_custom_target");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        let commands = doc.commands().unwrap();
        dbg!(commands);
    }
}
