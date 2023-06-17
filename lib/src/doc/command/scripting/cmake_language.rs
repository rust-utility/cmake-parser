use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Call meta-operations on CMake commands.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/cmake_language.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum CMakeLanguage<'t> {
    Call(CMakeLanguageCall<'t>),
    Eval(CMakeLanguageEval<'t>),
    Defer(CMakeLanguageDefer<'t>),
    SetDependencyProvider(CMakeLanguageSetDependencyProvider<'t>),
    GetMessageLogLevel(CMakeLanguageGetMessageLogLevel<'t>),
}

impl<'t> ToCommandScope for CMakeLanguage<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakeLanguageCall<'t> {
    pub command: Token<'t>,
    pub args: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CMakeLanguageEval<'t> {
    pub code: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum CMakeLanguageDefer<'t> {
    Call(CMakeLanguageDeferCall<'t>),
    GetCallIds(CMakeLanguageDeferGetCallIds<'t>),
    GetCall(CMakeLanguageDeferGetCall<'t>),
    CancelCall(CMakeLanguageDeferCancelCall<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "options")]
pub struct CMakeLanguageDeferCall<'t> {
    #[cmake(rename = "")]
    pub options: Option<Vec<DeferCallOption<'t>>>,
    pub call: CMakeLanguageCall<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list, transparent)]
pub enum DeferCallOption<'t> {
    Directory(Token<'t>),
    Id(Token<'t>),
    IdVar(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, transparent)]
pub struct CMakeLanguageDeferGetCallIds<'t> {
    pub directory: Option<Token<'t>>,
    #[cmake(rename = "GET_CALL_IDS")]
    pub var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakeLanguageDeferGetCall<'t> {
    #[cmake(transparent)]
    pub directory: Option<Token<'t>>,
    #[cmake(transparent, rename = "GET_CALL")]
    pub id: Token<'t>,
    pub var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, transparent)]
pub struct CMakeLanguageDeferCancelCall<'t> {
    pub directory: Option<Token<'t>>,
    #[cmake(rename = "CANCEL_CALL")]
    pub ids: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakeLanguageSetDependencyProvider<'t> {
    pub command: Token<'t>,
    #[cmake(transparent)]
    pub supported_methods: Vec<SuportedMethod>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum SuportedMethod {
    FindPackage,
    #[cmake(rename = "FETCHCONTENT_MAKEAVAILABLE_SERIAL")]
    FetchContentMakeAvailableSerial,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CMakeLanguageGetMessageLogLevel<'t> {
    pub out_var: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, quoted_tokens_vec, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cmake_language() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/cmake_language");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CMakeLanguage(Box::new(CMakeLanguage::Call(CMakeLanguageCall {
                    command: token(b"${message_command}"),
                    args: Some(vec![token(b"STATUS"), quoted_token(b"Hello World!")]),
                }))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Eval(CMakeLanguageEval {
                    code: quoted_tokens_vec([br###"
  if (${condition})
    message(STATUS TRUE)
  else()
    message(STATUS FALSE)
  endif()"###]),
                }))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(CMakeLanguageDefer::Call(
                    CMakeLanguageDeferCall {
                        options: None,
                        call: CMakeLanguageCall {
                            command: token(b"message"),
                            args: Some(vec![quoted_token(b"${deferred_message}")]),
                        }
                    }
                )))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(CMakeLanguageDefer::Call(
                    CMakeLanguageDeferCall {
                        options: Some(vec![DeferCallOption::IdVar(token(b"id"))]),
                        call: CMakeLanguageCall {
                            command: token(b"message"),
                            args: Some(vec![quoted_token(b"Canceled Message")]),
                        }
                    }
                )))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(
                    CMakeLanguageDefer::GetCallIds(CMakeLanguageDeferGetCallIds {
                        directory: None,
                        var: token(b"ids1"),
                    })
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(
                    CMakeLanguageDefer::GetCallIds(CMakeLanguageDeferGetCallIds {
                        directory: Some(token(b"dir1")),
                        var: token(b"ids1"),
                    })
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(
                    CMakeLanguageDefer::GetCall(CMakeLanguageDeferGetCall {
                        directory: None,
                        id: token(b"id1"),
                        var: token(b"var1"),
                    })
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(
                    CMakeLanguageDefer::GetCall(CMakeLanguageDeferGetCall {
                        directory: Some(token(b"dir1")),
                        id: token(b"id1"),
                        var: token(b"var1"),
                    })
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(
                    CMakeLanguageDefer::CancelCall(CMakeLanguageDeferCancelCall {
                        directory: None,
                        ids: tokens_vec([b"${id}"]),
                    })
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::Defer(
                    CMakeLanguageDefer::CancelCall(CMakeLanguageDeferCancelCall {
                        directory: Some(token(b"dir1")),
                        ids: tokens_vec([b"id1", b"id2"]),
                    })
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::SetDependencyProvider(
                    CMakeLanguageSetDependencyProvider {
                        command: token(b"cmd1"),
                        supported_methods: vec![
                            SuportedMethod::FindPackage,
                            SuportedMethod::FetchContentMakeAvailableSerial
                        ],
                    }
                ))),
                Command::CMakeLanguage(Box::new(CMakeLanguage::GetMessageLogLevel(
                    CMakeLanguageGetMessageLogLevel {
                        out_var: token(b"out_var1"),
                    }
                ))),
            ])
        )
    }
}
