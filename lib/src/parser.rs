use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_until},
    character::complete::{alpha1, alphanumeric1, char, not_line_ending, space1},
    combinator::{consumed, map, not, opt, recognize, value},
    multi::{many0, many0_count, many1},
    sequence::{delimited, pair, preceded, tuple},
};

use crate::Token;

pub fn parse_cmakelists(src: &[u8]) -> Result<CMakeListsTokens, CMakeListsParseError> {
    nom_parse_cmakelists(src)
        .map(|(_, cm)| cm)
        .map_err(From::from)
}

#[derive(Debug)]
pub struct CMakeListsTokens<'cmlist> {
    file: Vec<FileElement<'cmlist>>,
}

impl<'cmlist> CMakeListsTokens<'cmlist> {
    pub(crate) fn command_invocations(&self) -> impl Iterator<Item = &CommandInvocation<'cmlist>> {
        self.file.iter().filter_map(|file_element| {
            if let CMakeLanguage::CommandInvocation((command_invocation, _)) = &file_element.element
            {
                Some(command_invocation)
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
struct FileElement<'fe> {
    source: Source<'fe>,
    element: CMakeLanguage<'fe>,
}

struct Source<'s>(&'s [u8]);

type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

impl<'s> std::fmt::Debug for Source<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Source")
            .field(&String::from_utf8_lossy(self.0))
            .finish()
    }
}

#[derive(Debug)]
enum CMakeLanguage<'cml> {
    CommandInvocation((CommandInvocation<'cml>, LineEnding<'cml>)),
    Formatting((Vec<Formatting<'cml>>, LineEnding<'cml>)),
}

#[derive(Debug)]
enum Formatting<'f> {
    BracketComment(BracketComment<'f>),
    Spaces(Spaces),
}

#[derive(Debug)]
pub(crate) struct CommandInvocation<'ci> {
    spaces_before: Vec<Spaces>,
    pub(crate) identifier: &'ci [u8],
    spaces_after: Vec<Spaces>,
    arguments: Arguments<'ci>,
}

impl<'ci> CommandInvocation<'ci> {
    pub fn to_text_nodes(&'ci self) -> Vec<Token<'ci>> {
        self.arguments.to_text_nodes()
    }
}

#[derive(Debug)]
struct Arguments<'a> {
    argument: Option<Argument<'a>>,
    separated_arguments: Vec<SeparatedArguments<'a>>,
}

impl<'a> Arguments<'a> {
    pub fn to_text_nodes(&'a self) -> Vec<Token<'a>> {
        let mut text_nodes = vec![];
        if let Some(arg_tn) = self.argument.as_ref().map(|arg| arg.to_text_node()) {
            text_nodes.push(arg_tn);
        }
        text_nodes.extend(self.separated_arguments.iter().filter_map(|x| {
            if let SeparatedArguments::Single((_, Some(arg))) = x {
                Some(arg.to_text_node())
            } else {
                None
            }
        }));
        text_nodes
    }
}

#[derive(Debug)]
enum SeparatedArguments<'a> {
    Single((Vec<Separation<'a>>, Option<Argument<'a>>)),
    Multi((Vec<Separation<'a>>, Box<Arguments<'a>>)),
}

#[derive(Debug)]
enum Separation<'a> {
    Space(Spaces),
    LineEnding(LineEnding<'a>),
}

#[derive(Debug)]
enum Argument<'a> {
    Bracket(BracketArgument<'a>),
    Quoted(QuotedArgument),
    Unquoted(UnquotedArgument<'a>),
}

impl<'a> Argument<'a> {
    fn to_text_node(&'a self) -> Token<'a> {
        match self {
            Argument::Bracket(ba) => Token::text_node(ba.bracket_content, false),
            Argument::Quoted(qa) => Token::text_node(&qa.0, true),
            Argument::Unquoted(ua) => ua.to_text_node(),
        }
    }
}

#[derive(Debug)]
struct BracketComment<'bc>(BracketArgument<'bc>);

#[derive(Debug)]
struct BracketArgument<'ba> {
    len: usize,
    bracket_content: &'ba [u8],
}

#[derive(Debug)]
struct QuotedArgument(Vec<u8>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum UnquotedArgument<'ua> {
    Normal(Vec<u8>),
    Legacy(&'ua [u8]),
}

impl<'ua> UnquotedArgument<'ua> {
    fn to_text_node(&'ua self) -> Token<'ua> {
        match self {
            UnquotedArgument::Normal(n) => Token::text_node(n, false),
            UnquotedArgument::Legacy(l) => Token::text_node(l, false),
        }
    }
}

#[derive(Debug)]
struct LineComment<'lc>(&'lc [u8]);

#[derive(Debug)]
struct LineEnding<'le> {
    line_comment: Option<LineComment<'le>>,
}

#[derive(Debug)]
struct Spaces(usize);

#[derive(Debug, thiserror::Error)]
pub enum CMakeListsParseError {
    #[error("unknown")]
    Unknown,
    #[error("parser: {0}")]
    Parser(String),
}

impl From<nom::Err<nom::error::VerboseError<&[u8]>>> for CMakeListsParseError {
    fn from(value: nom::Err<nom::error::VerboseError<&[u8]>>) -> Self {
        Self::Parser(value.to_string())
    }
}

fn nom_parse_cmakelists(src: &[u8]) -> IResult<&[u8], CMakeListsTokens<'_>> {
    many0(file_element)(src).map(|(src, file)| (src, CMakeListsTokens { file }))
}

fn file_element(src: &[u8]) -> IResult<&[u8], FileElement<'_>> {
    alt((
        map(
            consumed(tuple((command_invocation, line_ending))),
            |(source, command_invocation)| FileElement {
                source: Source(source),
                element: CMakeLanguage::CommandInvocation(command_invocation),
            },
        ),
        map(
            consumed(tuple((
                many0(alt((
                    map(bracket_comment, Formatting::BracketComment),
                    map(spaces, Formatting::Spaces),
                ))),
                line_ending,
            ))),
            |(source, formatting)| FileElement {
                source: Source(source),
                element: CMakeLanguage::Formatting(formatting),
            },
        ),
    ))(src)
}

fn command_invocation(src: &[u8]) -> IResult<&[u8], CommandInvocation> {
    map(
        tuple((many0(spaces), identifier, many0(spaces), scoped_arguments)),
        |(spaces_before, identifier, spaces_after, arguments)| CommandInvocation {
            spaces_before,
            identifier,
            spaces_after,
            arguments,
        },
    )(src)
}

fn scoped_arguments(src: &[u8]) -> IResult<&[u8], Arguments<'_>> {
    delimited(char('('), arguments, char(')'))(src)
}

fn arguments(src: &[u8]) -> IResult<&[u8], Arguments<'_>> {
    map(
        pair(opt(argument), many0(separated_arguments)),
        |(argument, separated_arguments)| Arguments {
            argument,
            separated_arguments,
        },
    )(src)
}

fn separated_arguments(src: &[u8]) -> IResult<&[u8], SeparatedArguments<'_>> {
    alt((
        map(
            pair(many1(separation), opt(argument)),
            SeparatedArguments::Single,
        ),
        map(
            pair(many0(separation), map(scoped_arguments, Box::new)),
            SeparatedArguments::Multi,
        ),
    ))(src)
}

fn separation(src: &[u8]) -> IResult<&[u8], Separation<'_>> {
    alt((
        map(spaces, Separation::Space),
        map(line_ending, Separation::LineEnding),
    ))(src)
}

fn argument(src: &[u8]) -> IResult<&[u8], Argument<'_>> {
    alt((
        map(bracket_argument, Argument::Bracket),
        map(quoted_argument, Argument::Quoted),
        map(unquoted_argument, Argument::Unquoted),
    ))(src)
}

fn bracket_argument(src: &[u8]) -> IResult<&[u8], BracketArgument> {
    let (src, _) = char('[')(src)?;
    let (src, len) = many0_count(char('='))(src)?;
    let bracket_close = format!("]{}]", "=".repeat(len));
    let (src, _) = char('[')(src)?;
    let (src, _) = opt(nom::character::complete::line_ending)(src)?;
    let (src, bracket_content) = take_until(bracket_close.as_bytes())(src)?;
    let (src, _) = tag(bracket_close.as_bytes())(src)?;
    Ok((
        src,
        BracketArgument {
            len,
            bracket_content,
        },
    ))
}

fn quoted_argument(src: &[u8]) -> IResult<&[u8], QuotedArgument> {
    map(
        delimited(tag(b"\""), many0(quoted_element), tag(b"\"")),
        |x| QuotedArgument(x.into_iter().flatten().collect()),
    )(src)
}

fn quoted_element(src: &[u8]) -> IResult<&[u8], Vec<u8>> {
    alt((
        map(is_not("\\\""), |x: &[u8]| x.to_vec()),
        map(escape_sequence, |x| x.to_vec()),
        value(
            Vec::default(),
            pair(char('\\'), nom::character::complete::line_ending),
        ),
    ))(src)
}

fn escape_sequence(src: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(
        char('\\'),
        alt((
            is_a("()#\" \\$@^;"),
            value(&b"\t"[..], char('t')),
            value(&b"\r"[..], char('r')),
            value(&b"\n"[..], char('n')),
        )),
    )(src)
}

fn unquoted_argument(src: &[u8]) -> IResult<&[u8], UnquotedArgument> {
    alt((
        map(unquoted_legacy, UnquotedArgument::Legacy),
        map(many1(unquoted_element), |x| {
            UnquotedArgument::Normal(x.iter().flat_map(|x| x.to_vec()).collect())
        }),
    ))(src)
}

fn unquoted_element(src: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((is_not(" \t\r\n()#\"\\"), escape_sequence))(src)
}

fn unquoted_legacy(src: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(pair(
        alt((
            value((), is_not(" \t\r\n()#\"\\$")),
            value((), delimited(tag(b"$("), is_not(")"), tag(b")"))),
        )),
        many1(alt((
            value((), is_not(" \t\r\n()#\"\\$")),
            value((), delimited(tag(b"$("), is_not(")"), tag(b")"))),
            value((), delimited(char('"'), is_not("\""), char('"'))),
        ))),
    ))(src)
}

fn identifier(src: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(src)
}

fn line_ending(src: &[u8]) -> IResult<&[u8], LineEnding> {
    map(
        tuple((opt(line_comment), nom::character::complete::line_ending)),
        |(line_comment, _)| LineEnding { line_comment },
    )(src)
}

fn line_comment(src: &[u8]) -> IResult<&[u8], LineComment> {
    preceded(
        char('#'),
        map(
            recognize(tuple((
                not(tuple((char('['), many0(char('=')), char('[')))),
                not_line_ending,
            ))),
            LineComment,
        ),
    )(src)
}

fn bracket_comment(src: &[u8]) -> IResult<&[u8], BracketComment> {
    map(preceded(char('#'), bracket_argument), BracketComment)(src)
}

fn spaces(src: &[u8]) -> IResult<&[u8], Spaces> {
    map(space1, |spaces: &[u8]| Spaces(spaces.len()))(src)
}

#[cfg(test)]
mod tests {
    trait CheckNomError<O> {
        fn debug_unwrap(self) -> (&'static [u8], O);
    }

    impl<O> CheckNomError<O> for super::IResult<&'static [u8], O> {
        fn debug_unwrap(self) -> (&'static [u8], O) {
            match self {
                Ok(ok) => ok,
                Err(err) => match err {
                    nom::Err::Incomplete(_e) => panic!("Incomplete: {err}"),
                    nom::Err::Error(e) => {
                        let mut msgs = vec![];
                        for (src, knd) in e.errors {
                            msgs.push(format!(
                                "{knd:?}: '{}'",
                                String::from_utf8_lossy(&src[..src.len().min(50)])
                            ));
                        }
                        panic!("Error: {}", msgs.join("\n"));
                    }
                    nom::Err::Failure(e) => {
                        let mut msgs = vec![];
                        for (src, knd) in e.errors {
                            msgs.push(format!(
                                "{knd:?}: '{}'",
                                String::from_utf8_lossy(&src[..src.len().min(50)])
                            ));
                        }
                        panic!("Failure: {}", msgs.join("\n"));
                    }
                },
            }
        }
    }

    #[test]
    fn parse_cmakelists() {
        let ex1 = include_bytes!("../../fixture/CMakeLists.txt.ex1");
        let _ = super::parse_cmakelists(ex1).unwrap();

        let ex2 = include_bytes!("../../fixture/CMakeLists.txt.ex2");
        let _ = super::parse_cmakelists(ex2).unwrap();

        let ex3 = include_bytes!("../../fixture/CMakeLists.txt.ex3");
        let _ = super::parse_cmakelists(ex3).unwrap();

        let ex4 = include_bytes!("../../fixture/CMakeLists.txt.ex4");
        let _ = super::parse_cmakelists(ex4).unwrap();
    }

    #[test]
    fn file_element() {
        use super::file_element;

        let input = include_bytes!("../../fixture/CMakeLists.txt.ex2");
        let (src, _) = file_element(input).debug_unwrap();
        let (src, _) = file_element(src).unwrap();
        let (_, _) = file_element(src).unwrap();
    }

    #[test]
    fn bracket_argument() {
        use super::bracket_argument;
        let (_, ba) = bracket_argument(b"[[hello]]").unwrap();
        assert_eq!(ba.bracket_content, b"hello");
        let (_, ba) = bracket_argument(b"[=[hel]]lo]=]").unwrap();
        assert_eq!(ba.bracket_content, b"hel]]lo");
        let (_, ba) = bracket_argument(b"[=[hel]]\r\nlo]=]").unwrap();
        assert_eq!(ba.bracket_content, b"hel]]\r\nlo");
        let (_, ba) = bracket_argument(b"[=[\r\nhel]]\r\nlo]=]").unwrap();
        assert_eq!(ba.bracket_content, b"hel]]\r\nlo");
        let (_, ba) = bracket_argument(b"[=[\nhel]]\r\nlo]=]").unwrap();
        assert_eq!(ba.bracket_content, b"hel]]\r\nlo");
    }

    #[test]
    fn line_comment() {
        use super::line_comment;

        let (_, lc) = line_comment(b"#").unwrap();
        assert_eq!(lc.0, b"");
        let (_, lc) = line_comment(b"#hello").unwrap();
        assert_eq!(lc.0, b"hello");
        let (_, lc) = line_comment(b"# [[hello").unwrap();
        assert_eq!(lc.0, b" [[hello");
        let (_, lc) = line_comment(b"#\r\n").unwrap();
        assert_eq!(lc.0, b"");

        let res = line_comment(b"#[[hello");
        assert!(res.is_err());
        let res = line_comment(b"#[=[hello");
        assert!(res.is_err());
    }

    #[test]
    fn quoted_argument() {
        use super::quoted_argument;

        let (_, qa) = quoted_argument(br#""hello""#).unwrap();
        assert_eq!(&qa.0, b"hello");
        let (_, qa) = quoted_argument(
            br#""hello\
, world""#,
        )
        .unwrap();
        assert_eq!(&qa.0, b"hello, world");
        let (_, qa) = quoted_argument(br#""hello\nworld""#).unwrap();
        assert_eq!(&qa.0, b"hello\nworld");
    }

    #[test]
    fn unquoted_argument() {
        use super::{unquoted_argument, UnquotedArgument};

        let (_, ua) = unquoted_argument(b"hello").unwrap();
        assert_eq!(ua, UnquotedArgument::Normal(b"hello".to_vec()));

        let (_, ua) = unquoted_argument(b"a=\"b\"").unwrap();
        assert_eq!(ua, UnquotedArgument::Legacy(b"a=\"b\""));

        let (_, ua) = unquoted_argument(b"-Da=\"b c\"").unwrap();
        assert_eq!(ua, UnquotedArgument::Legacy(b"-Da=\"b c\""));

        let (_, ua) = unquoted_argument(b"-Da=$(v)").unwrap();
        assert_eq!(ua, UnquotedArgument::Legacy(b"-Da=$(v)"));

        let (_, ua) = unquoted_argument(br#"a" "b"c"d"#).unwrap();
        assert_eq!(ua, UnquotedArgument::Legacy(br#"a" "b"c"d"#));
    }

    #[test]
    fn unquoted_legacy() {
        use super::unquoted_legacy;
        let (_, ua) = unquoted_legacy(b"a=\"b\"").unwrap();
        assert_eq!(ua, b"a=\"b\"");

        let (_, ua) = unquoted_legacy(b"-Da=\"b c\"").unwrap();
        assert_eq!(ua, b"-Da=\"b c\"");

        let (_, ua) = unquoted_legacy(b"-Da=$(v)").unwrap();
        assert_eq!(ua, b"-Da=$(v)");

        let (_, ua) = unquoted_legacy(br#"a" "b"c"d"#).unwrap();
        assert_eq!(ua, br#"a" "b"c"d"#);
    }

    #[test]
    fn scoped_arguments() {
        use super::scoped_arguments;

        let (_, _sa) = scoped_arguments(b"(hello)").debug_unwrap();

        let (_, _sa) = scoped_arguments(b"(hello world)").debug_unwrap();

        let (_, _sa) =
            scoped_arguments(b"(LibXml2 PRIVATE SYSCONFDIR=\"${CMAKE_INSTALL_FULL_SYSCONFDIR}\")")
                .debug_unwrap();
    }

    #[test]
    fn arguments() {
        use super::arguments;

        let (_, _) = arguments(b"hello").debug_unwrap();

        let (_, _) = arguments(b"hello world").debug_unwrap();
    }
}
