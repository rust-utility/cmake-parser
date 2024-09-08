use std::f64::consts::PI;

use crate::{CMakeParse, CommandParseError, Token};

pub trait CMakePositional<'t>: 't + Sized {
    fn positional<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError>;

    fn positional_complete<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        Self::positional(default_name, tokens, has_keyword).and_then(|(result, tokens)| {
            if tokens.is_empty() {
                Ok((result, tokens))
            } else {
                Err(CommandParseError::NotEmpty)
            }
        })
    }

    fn in_range<'tv>(
        default_name: &'static [u8],
        to: &'static [u8],
        allow_empty: bool,
        tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        if let Some((range_to, range_from)) = tokens
            .iter()
            .position(|token| token.as_ref() == to)
            .map(|mid| tokens.split_at(mid))
        {
            Self::positional(default_name, range_to, has_keyword).map(|(res, _)| (res, range_from))
        } else if allow_empty {
            Self::positional(default_name, tokens, has_keyword)
        } else {
            Err(CommandParseError::TokenRequired)
        }
    }
}

impl<'t> CMakePositional<'t> for Token<'t> {
    fn positional<'tv>(
        default_name: &'static [u8],
        mut tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        if has_keyword {
            let (_, rest) = Keyword::positional(default_name, tokens, has_keyword)?;
            tokens = rest;
        }
        CMakeParse::parse(tokens)
    }
}

impl<'t, T> CMakePositional<'t> for Vec<T>
where
    T: CMakeParse<'t>,
{
    fn positional<'tv>(
        default_name: &'static [u8],
        mut tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        if has_keyword {
            let (_, rest) = Keyword::positional(default_name, tokens, has_keyword)?;
            tokens = rest;
        }
        CMakeParse::parse(tokens)
    }
}

impl<'t, T> CMakePositional<'t> for Option<T>
where
    T: CMakePositional<'t>,
{
    fn positional<'tv>(
        keyword: &'static [u8],
        tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        match T::positional(keyword, tokens, has_keyword).map(|(res, tokens)| (Some(res), tokens)) {
            Ok(result) => Ok(result),
            Err(_) => Ok((None, tokens)),
        }
    }
}

impl<'t, T> CMakePositional<'t> for Box<T>
where
    T: CMakePositional<'t>,
{
    fn positional<'tv>(
        keyword: &'static [u8],
        tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T::positional(keyword, tokens, has_keyword).map(|(res, tokens)| (Box::new(res), tokens))
    }
}

impl<'t> CMakePositional<'t> for bool {
    fn positional<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
        _: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        tokens
            .split_first()
            .filter(|(first, _)| first.as_bytes() == default_name)
            .map(|(_, rest)| (true, rest))
            .or(Some((false, tokens)))
            .ok_or(CommandParseError::TokenRequired)
    }
}

impl<'t> CMakePositional<'t> for () {
    fn positional<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
        has_keyword: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        <bool as CMakePositional>::positional(default_name, tokens, has_keyword)
            .map(|(_, tokens)| ((), tokens))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyword;

impl<'t> CMakePositional<'t> for Keyword {
    fn positional<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
        _: bool,
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        tokens
            .split_first()
            .filter(|(first, _)| first.as_bytes() == default_name)
            .map(|(_, rest)| (Keyword, rest))
            .ok_or(CommandParseError::TokenRequired)
    }
}
