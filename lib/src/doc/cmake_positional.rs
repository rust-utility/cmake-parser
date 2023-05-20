use crate::{CMakeParse, CommandParseError, Token};

pub trait CMakePositional<'t>: 't + Sized {
    fn positional<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError>;
}

impl<'t> CMakePositional<'t> for Token<'t> {
    fn positional<'tv>(
        _: &'static [u8],
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        CMakeParse::parse(tokens)
    }
}

impl<'t, T> CMakePositional<'t> for Vec<T>
where
    T: CMakeParse<'t>,
{
    fn positional<'tv>(
        _: &'static [u8],
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
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
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        match T::positional(keyword, tokens).map(|(res, tokens)| (Some(res), tokens)) {
            Ok(result) => Ok(result),
            Err(CommandParseError::TokenRequired) => Ok((None, tokens)),
            Err(err) => Err(err),
        }
    }
}

impl<'t> CMakePositional<'t> for bool {
    fn positional<'tv>(
        default_name: &'static [u8],
        tokens: &'tv [Token<'t>],
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
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        <bool as CMakePositional>::positional(default_name, tokens).map(|(_, tokens)| ((), tokens))
    }
}
