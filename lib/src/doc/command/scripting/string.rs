use cmake_parser_derive::CMake;

use crate::{
    command::common::HashAlgorithm,
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// String operations.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/string.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum String<'t> {
    SearchAndReplace(StringSearchAndReplace<'t>),
    Manipulation(StringManipulation<'t>),
    Comparison(StringComparison<'t>),
    Hash(StringHash<'t>),
    Generation(StringGeneration<'t>),
    #[cmake(transparent)]
    Json(StringJson<'t>),
}

impl<'t> ToCommandScope for String<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum StringSearchAndReplace<'t> {
    Find(StringFind<'t>),
    Replace(StringReplace<'t>),
    Regex(StringRegex<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringFind<'t> {
    pub string: Token<'t>,
    pub substring: Token<'t>,
    pub output_variable: Token<'t>,
    pub reverse: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringReplace<'t> {
    pub match_string: Token<'t>,
    pub replace_string: Token<'t>,
    pub output_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum StringRegex<'t> {
    Match(StringRegexMatch<'t>),
    #[cmake(rename = "MATCHALL")]
    MatchAll(StringRegexMatchAll<'t>),
    Replace(StringRegexReplace<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringRegexMatch<'t> {
    pub regular_expression: Token<'t>,
    pub output_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringRegexMatchAll<'t> {
    pub regular_expression: Token<'t>,
    pub output_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringRegexReplace<'t> {
    pub regular_expression: Token<'t>,
    pub replacement_expression: Token<'t>,
    pub output_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum StringManipulation<'t> {
    Append(StringAppend<'t>),
    Prepend(StringPrepend<'t>),
    Concat(StringConcat<'t>),
    Join(StringJoin<'t>),
    #[cmake(rename = "TOLOWER")]
    ToLower(StringToLower<'t>),
    #[cmake(rename = "TOUPPER")]
    ToUpper(StringToUpper<'t>),
    Length(StringLength<'t>),
    Substring(StringSubstring<'t>),
    Strip(StringStrip<'t>),
    GenexStrip(StringGenexStrip<'t>),
    Repeat(StringRepeat<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringAppend<'t> {
    pub string_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringPrepend<'t> {
    pub string_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringConcat<'t> {
    pub output_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJoin<'t> {
    pub glue: Token<'t>,
    pub output_variable: Token<'t>,
    pub input: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringToLower<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringToUpper<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringLength<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringSubstring<'t> {
    pub string: Token<'t>,
    pub begin: Token<'t>,
    pub length: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringStrip<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringGenexStrip<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringRepeat<'t> {
    pub string: Token<'t>,
    pub count: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum StringComparison<'t> {
    Less(StringLess<'t>),
    Greater(StringGreater<'t>),
    Equal(StringEqual<'t>),
    #[cmake(rename = "NOTEQUAL")]
    NotEqual(StringNotEqual<'t>),
    LessEqual(StringLessEqual<'t>),
    GreaterEqual(StringGreaterEqual<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringLess<'t> {
    pub string1: Token<'t>,
    pub string2: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringGreater<'t> {
    pub string1: Token<'t>,
    pub string2: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringEqual<'t> {
    pub string1: Token<'t>,
    pub string2: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringNotEqual<'t> {
    pub string1: Token<'t>,
    pub string2: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringLessEqual<'t> {
    pub string1: Token<'t>,
    pub string2: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringGreaterEqual<'t> {
    pub string1: Token<'t>,
    pub string2: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringHash<'t> {
    pub hash_algorithm: HashAlgorithm,
    pub output_variable: Token<'t>,
    pub input: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum StringGeneration<'t> {
    Ascii(StringAscii<'t>),
    Hex(StringHex<'t>),
    Configure(StringConfigure<'t>),
    #[cmake(rename = "MAKE_C_IDENTIFIER")]
    MakeCIdentifier(StringMakeCIdentifier<'t>),
    Random(StringRandom<'t>),
    Timestamp(StringTimestamp<'t>),
    Uuid(StringUuid<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringAscii<'t> {
    pub number: Vec<Token<'t>>,
    #[cmake(last)]
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringHex<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringConfigure<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
    #[cmake(rename = "@ONLY")]
    pub only: bool,
    pub escape_quotes: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringMakeCIdentifier<'t> {
    pub string: Token<'t>,
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "output_variable")]
pub struct StringRandom<'t> {
    pub length: Option<Token<'t>>,
    pub alphabet: Option<Token<'t>>,
    pub random_seed: Option<Token<'t>>,
    #[cmake(rename = "")]
    pub output_variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringTimestamp<'t> {
    pub output_variable: Token<'t>,
    #[cmake(allow_empty, in_range)]
    pub format_string: Option<Token<'t>>,
    pub utc: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringUuid<'t> {
    pub output_variable: Token<'t>,
    #[cmake(transparent)]
    pub namespace: Token<'t>,
    #[cmake(transparent)]
    pub name: Token<'t>,
    #[cmake(transparent, rename = "TYPE")]
    pub hash_algorithm: UuidHashAlgorithm,
    pub upper: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum UuidHashAlgorithm {
    MD5,
    SHA1,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJson<'t> {
    pub output_variable: Token<'t>,
    #[cmake(transparent)]
    pub error_variable: Option<Token<'t>>,
    pub command: JsonCommand<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum JsonCommand<'t> {
    Get(StringJsonGet<'t>),
    Type(StringJsonType<'t>),
    Member(StringJsonMember<'t>),
    Length(StringJsonLength<'t>),
    Remove(StringJsonRemove<'t>),
    Set(StringJsonSet<'t>),
    Equal(StringJsonEqual<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonGet<'t> {
    pub json_string: Token<'t>,
    pub member_index: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonType<'t> {
    pub json_string: Token<'t>,
    pub member_index: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonMember<'t> {
    pub json_string: Token<'t>,
    pub member_index: Option<Vec<Token<'t>>>,
    #[cmake(last)]
    pub index: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonLength<'t> {
    pub json_string: Token<'t>,
    pub member_index: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonRemove<'t> {
    pub json_string: Token<'t>,
    pub member_index: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonSet<'t> {
    pub json_string: Token<'t>,
    pub member_index: Vec<Token<'t>>,
    #[cmake(last)]
    pub value: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct StringJsonEqual<'t> {
    pub json_string1: Token<'t>,
    pub json_string2: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn string() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/string");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::String(Box::new(String::SearchAndReplace(
                    StringSearchAndReplace::Find(StringFind {
                        string: token(b"str1"),
                        substring: token(b"substr1"),
                        output_variable: token(b"out_var1"),
                        reverse: false,
                    })
                )))),
                Ok(Command::String(Box::new(String::SearchAndReplace(
                    StringSearchAndReplace::Find(StringFind {
                        string: token(b"str1"),
                        substring: token(b"substr1"),
                        output_variable: token(b"out_var1"),
                        reverse: true,
                    })
                )))),
                Ok(Command::String(Box::new(String::SearchAndReplace(
                    StringSearchAndReplace::Replace(StringReplace {
                        match_string: token(b"match_str1"),
                        replace_string: token(b"replace_str1"),
                        output_variable: token(b"out_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    })
                )))),
                Ok(Command::String(Box::new(String::SearchAndReplace(
                    StringSearchAndReplace::Regex(StringRegex::Match(StringRegexMatch {
                        regular_expression: token(b"regex1"),
                        output_variable: token(b"out_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    }))
                )))),
                Ok(Command::String(Box::new(String::SearchAndReplace(
                    StringSearchAndReplace::Regex(StringRegex::MatchAll(StringRegexMatchAll {
                        regular_expression: token(b"regex1"),
                        output_variable: token(b"out_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    }))
                )))),
                Ok(Command::String(Box::new(String::SearchAndReplace(
                    StringSearchAndReplace::Regex(StringRegex::Replace(StringRegexReplace {
                        regular_expression: token(b"regex1"),
                        replacement_expression: token(b"replace_ex1"),
                        output_variable: token(b"out_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    }))
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Append(StringAppend {
                        string_variable: token(b"str_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Prepend(StringPrepend {
                        string_variable: token(b"str_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Concat(StringConcat {
                        output_variable: token(b"out_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Join(StringJoin {
                        glue: token(b","),
                        output_variable: token(b"out_var1"),
                        input: tokens_vec([b"input1", b"input2"]),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::ToLower(StringToLower {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::ToUpper(StringToUpper {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Length(StringLength {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Substring(StringSubstring {
                        string: token(b"str1"),
                        begin: token(b"begin1"),
                        length: token(b"length1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Strip(StringStrip {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::GenexStrip(StringGenexStrip {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Manipulation(
                    StringManipulation::Repeat(StringRepeat {
                        string: token(b"str1"),
                        count: token(b"count1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Comparison(
                    StringComparison::Less(StringLess {
                        string1: token(b"str1"),
                        string2: token(b"str2"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Comparison(
                    StringComparison::Greater(StringGreater {
                        string1: token(b"str1"),
                        string2: token(b"str2"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Comparison(
                    StringComparison::Equal(StringEqual {
                        string1: token(b"str1"),
                        string2: token(b"str2"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Comparison(
                    StringComparison::NotEqual(StringNotEqual {
                        string1: token(b"str1"),
                        string2: token(b"str2"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Comparison(
                    StringComparison::LessEqual(StringLessEqual {
                        string1: token(b"str1"),
                        string2: token(b"str2"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Comparison(
                    StringComparison::GreaterEqual(StringGreaterEqual {
                        string1: token(b"str1"),
                        string2: token(b"str2"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Hash(StringHash {
                    hash_algorithm: HashAlgorithm::MD5,
                    output_variable: token(b"out_var1"),
                    input: token(b"input1"),
                })))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Ascii(StringAscii {
                        number: tokens_vec([b"number1", b"number2"]),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Hex(StringHex {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Configure(StringConfigure {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                        only: false,
                        escape_quotes: false,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Configure(StringConfigure {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                        only: true,
                        escape_quotes: false,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Configure(StringConfigure {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                        only: false,
                        escape_quotes: true,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Configure(StringConfigure {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                        only: true,
                        escape_quotes: true,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::MakeCIdentifier(StringMakeCIdentifier {
                        string: token(b"str1"),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Random(StringRandom {
                        length: Some(token(b"len1")),
                        alphabet: Some(token(b"abc1")),
                        random_seed: Some(token(b"seed1")),
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Random(StringRandom {
                        length: None,
                        alphabet: None,
                        random_seed: None,
                        output_variable: token(b"out_var1"),
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Timestamp(StringTimestamp {
                        output_variable: token(b"out_var1"),
                        format_string: None,
                        utc: false,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Timestamp(StringTimestamp {
                        output_variable: token(b"out_var1"),
                        format_string: Some(token(b"format1")),
                        utc: false,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Timestamp(StringTimestamp {
                        output_variable: token(b"out_var1"),
                        format_string: Some(token(b"format1")),
                        utc: true,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Timestamp(StringTimestamp {
                        output_variable: token(b"out_var1"),
                        format_string: None,
                        utc: true,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Uuid(StringUuid {
                        output_variable: token(b"out_var1"),
                        namespace: token(b"namespace1"),
                        name: token(b"name1"),
                        hash_algorithm: UuidHashAlgorithm::MD5,
                        upper: true,
                    })
                )))),
                Ok(Command::String(Box::new(String::Generation(
                    StringGeneration::Uuid(StringUuid {
                        output_variable: token(b"out_var1"),
                        namespace: token(b"namespace1"),
                        name: token(b"name1"),
                        hash_algorithm: UuidHashAlgorithm::SHA1,
                        upper: false,
                    })
                )))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Get(StringJsonGet {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Get(StringJsonGet {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Type(StringJsonType {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Type(StringJsonType {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Member(StringJsonMember {
                        json_string: token(b"json_str1"),
                        member_index: Some(tokens_vec([b"member1", b"member2"])),
                        index: token(b"idx1"),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Member(StringJsonMember {
                        json_string: token(b"json_str1"),
                        member_index: None,
                        index: token(b"idx1"),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Length(StringJsonLength {
                        json_string: token(b"json_str1"),
                        member_index: Some(tokens_vec([b"member1", b"member2"])),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Length(StringJsonLength {
                        json_string: token(b"json_str1"),
                        member_index: None,
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Remove(StringJsonRemove {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Remove(StringJsonRemove {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Set(StringJsonSet {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1", b"member2"]),
                        value: token(b"value1"),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Set(StringJsonSet {
                        json_string: token(b"json_str1"),
                        member_index: tokens_vec([b"member1"]),
                        value: token(b"value1"),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: None,
                    command: JsonCommand::Equal(StringJsonEqual {
                        json_string1: token(b"json_str1"),
                        json_string2: token(b"json_str2"),
                    }),
                })))),
                Ok(Command::String(Box::new(String::Json(StringJson {
                    output_variable: token(b"out_var1"),
                    error_variable: Some(token(b"err_var1")),
                    command: JsonCommand::Equal(StringJsonEqual {
                        json_string1: token(b"json_str1"),
                        json_string2: token(b"json_str2"),
                    }),
                })))),
            ]
        )
    }
}
