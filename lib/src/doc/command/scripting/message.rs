use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Log a message.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/message.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum Message<'t> {
    ReportingChecks(MessageReportingChecks<'t>),
    #[cmake(transparent)]
    ConfigureLog(Vec<Token<'t>>),
    General(MessageGeneral<'t>),
}

impl<'t> ToCommandScope for Message<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum MessageGeneral<'t> {
    #[cmake(transparent)]
    FatalError(Vec<Token<'t>>),
    #[cmake(transparent)]
    SendError(Vec<Token<'t>>),
    #[cmake(transparent)]
    Warning(Vec<Token<'t>>),
    #[cmake(transparent)]
    AuthorWarning(Vec<Token<'t>>),
    #[cmake(transparent)]
    Deprecation(Vec<Token<'t>>),
    #[cmake(transparent)]
    Notice(Vec<Token<'t>>),
    #[cmake(transparent)]
    Status(Vec<Token<'t>>),
    #[cmake(transparent)]
    Verbose(Vec<Token<'t>>),
    #[cmake(transparent)]
    Debug(Vec<Token<'t>>),
    #[cmake(transparent)]
    Trace(Vec<Token<'t>>),
    // TODO: implement `default` cmake attribute to add to [MessageGeneral::Notice]
    NoticeDefault(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum MessageReportingChecks<'t> {
    CheckStart(Vec<Token<'t>>),
    CheckPass(Vec<Token<'t>>),
    CheckFail(Vec<Token<'t>>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::quoted_tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn message() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/message");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::FatalError(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::SendError(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Warning(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::AuthorWarning(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Deprecation(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Notice(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Status(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Verbose(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Debug(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::Trace(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::General(
                    MessageGeneral::NoticeDefault(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::ReportingChecks(
                    MessageReportingChecks::CheckStart(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::ReportingChecks(
                    MessageReportingChecks::CheckPass(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::ReportingChecks(
                    MessageReportingChecks::CheckFail(quoted_tokens_vec([b"msg1"]))
                )))),
                Ok(Command::Message(Box::new(Message::ConfigureLog(
                    quoted_tokens_vec([b"msg1"])
                )))),
            ]
        )
    }
}
