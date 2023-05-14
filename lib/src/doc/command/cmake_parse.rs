use crate::{CommandParseError, Token};

pub trait CMakeParse<'t>: 't + Sized {
    fn cmake_parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError>;

    fn cmake_field_matches(&self, field_keyword: &[u8], keyword: &[u8]) -> bool {
        Self::cmake_field_matches_type(field_keyword, keyword)
    }

    fn cmake_field_matches_type(field_keyword: &[u8], keyword: &[u8]) -> bool {
        field_keyword == keyword
    }

    fn cmake_event_start<'tv>(
        &mut self,
        _field_keyword: &[u8],
        _keyword: &'tv Token<'t>,
        tokens: &[Token<'t>],
    ) -> Result<bool, CommandParseError> {
        if !tokens.is_empty() {
            self.cmake_update(tokens)?;
        }

        Ok(true)
    }

    fn cmake_update(&mut self, tokens: &[Token<'t>]) -> Result<(), CommandParseError> {
        *self = Self::cmake_complete(tokens)?;

        Ok(())
    }

    fn cmake_event_end(mut self, tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        self.cmake_update(tokens)?;

        Ok(self)
    }

    fn cmake_complete(tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        let (result, tokens) = Self::cmake_parse(tokens)?;
        if !tokens.is_empty() {
            return Err(CommandParseError::Incomplete);
        }

        Ok(result)
    }

    fn default_value() -> Option<Self> {
        None
    }

    fn new_value() -> Option<Self> {
        Self::default_value()
    }
}

impl<'t> CMakeParse<'t> for Token<'t> {
    fn cmake_parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        tokens
            .split_first()
            .map(|(first, rest)| (first.clone(), rest))
            .ok_or(CommandParseError::TokenRequired)
    }
}

impl<'t> CMakeParse<'t> for bool {
    fn cmake_event_start<'tv>(
        &mut self,
        _field_keyword: &[u8],
        _keyword: &'tv Token<'t>,
        _tokens: &[Token<'t>],
    ) -> Result<bool, CommandParseError> {
        *self = true;

        Ok(false)
    }

    fn cmake_parse<'tv>(
        _: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        Err(CommandParseError::NotFlag)
    }

    fn cmake_event_end(self, tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        if !tokens.is_empty() {
            return Err(CommandParseError::Incomplete);
        }
        Ok(self)
    }

    fn default_value() -> Option<Self> {
        Some(false)
    }
}

impl<'t, T> CMakeParse<'t> for Option<T>
where
    T: CMakeParse<'t>,
{
    fn cmake_field_matches_type(field_keyword: &[u8], keyword: &[u8]) -> bool {
        T::cmake_field_matches_type(field_keyword, keyword)
    }

    fn cmake_event_start<'tv>(
        &mut self,
        field_keyword: &[u8],
        keyword: &'tv Token<'t>,
        tokens: &[Token<'t>],
    ) -> Result<bool, CommandParseError> {
        if self.is_none() {
            *self = T::new_value();
        }
        self.as_mut()
            .map(|t| t.cmake_event_start(field_keyword, keyword, tokens))
            .transpose()
            .map(|x| x.unwrap_or(true))
    }

    fn cmake_parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T::cmake_parse(tokens).map(|(result, rest)| (Some(result), rest))
    }

    fn cmake_event_end(mut self, tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        if let Some(x) = self {
            x.cmake_event_end(tokens).map(Some)
        } else {
            self.cmake_update(tokens)?;
            Ok(self)
        }
    }

    fn default_value() -> Option<Self> {
        Some(T::default_value())
    }
}

impl<'t, T> CMakeParse<'t> for Vec<T>
where
    T: CMakeParse<'t>,
{
    fn cmake_field_matches_type(field_keyword: &[u8], keyword: &[u8]) -> bool {
        T::cmake_field_matches_type(field_keyword, keyword)
    }

    fn cmake_parse<'tv>(
        mut tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        let mut result = vec![];
        loop {
            let (val, new_tokens) = T::cmake_parse(tokens)?;
            result.push(val);
            if new_tokens.len() == tokens.len() {
                break;
            }
            tokens = new_tokens;
            if tokens.is_empty() {
                break;
            }
        }
        Ok((result, tokens))
    }

    fn cmake_update(&mut self, tokens: &[Token<'t>]) -> Result<(), CommandParseError> {
        self.extend(Self::cmake_complete(tokens)?);

        Ok(())
    }

    fn new_value() -> Option<Self> {
        Some(vec![])
    }
}

impl<'t, T1, T2> CMakeParse<'t> for (T1, T2)
where
    T1: CMakeParse<'t>,
    T2: CMakeParse<'t>,
{
    fn cmake_parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T1::cmake_parse(tokens)
            .and_then(|(t1, tokens)| T2::cmake_parse(tokens).map(|(t2, tokens)| ((t1, t2), tokens)))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn cmake_parse_token() {
        let token: Token<'_> = assert_parse([b"QQQ", b"aa", b"END"], b"QQQ");
        assert_eq!(token.as_bytes(), b"aa");

        let option_token: Option<Token<'_>> = assert_parse([b"QQQ", b"aa", b"END"], b"QQQ");
        assert_eq!(option_token.unwrap().as_bytes(), b"aa");
    }

    #[test]
    fn cmake_parse_bool() {
        let token_bool_true: bool = assert_parse([b"QQQ", b"END"], b"QQQ");
        assert!(token_bool_true);

        let token_bool_false: bool = assert_parse([b"END"], b"QQQ");
        assert!(!token_bool_false);

        let token_option_bool_true: Option<bool> = assert_parse([b"QQQ", b"END"], b"QQQ");
        assert_eq!(token_option_bool_true, Some(true));

        let token_option_bool_false: Option<bool> = assert_parse([b"END"], b"QQQ");
        assert_eq!(token_option_bool_false, Some(false));
    }

    #[test]
    fn cmake_parse_vec() {
        let vec_token: Vec<Token<'_>> = assert_parse([b"QQQ", b"aa", b"bb", b"END"], b"QQQ");
        assert_eq!(vec_token, tokens([b"aa", b"bb"]).to_vec());

        let vec_vec_token: Vec<Vec<Token<'_>>> = assert_parse(
            [
                b"QQQ", b"aa", b"bb", b"QQQ", b"cc", b"dd", b"QQQ", b"ee", b"ff", b"END",
            ],
            b"QQQ",
        );
        assert_eq!(
            vec_vec_token,
            vec![
                tokens([b"aa", b"bb"]).to_vec(),
                tokens([b"cc", b"dd"]).to_vec(),
                tokens([b"ee", b"ff"]).to_vec(),
            ]
        );

        let def_vec_token: Vec<Vec<Token<'_>>> = assert_parse(
            [
                b"aa", b"bb", b"CMD", b"cc", b"dd", b"CMD", b"ee", b"ff", b"END",
            ],
            b"CMD",
        );
        assert_eq!(
            def_vec_token,
            vec![
                tokens([b"aa", b"bb"]).to_vec(),
                tokens([b"cc", b"dd"]).to_vec(),
                tokens([b"ee", b"ff"]).to_vec(),
            ]
        );

        let option_vec_token: Option<Vec<Token<'_>>> =
            assert_parse([b"QQQ", b"aa", b"bb", b"END"], b"QQQ");
        assert_eq!(option_vec_token, Some(tokens([b"aa", b"bb"]).to_vec()));

        let option_vec_vec_token: Option<Vec<Vec<Token<'_>>>> = assert_parse(
            [
                b"QQQ", b"aa", b"bb", b"QQQ", b"cc", b"dd", b"QQQ", b"ee", b"ff", b"END",
            ],
            b"QQQ",
        );
        assert_eq!(
            option_vec_vec_token,
            Some(vec![
                tokens([b"aa", b"bb"]).to_vec(),
                tokens([b"cc", b"dd"]).to_vec(),
                tokens([b"ee", b"ff"]).to_vec(),
            ])
        );
    }

    pub fn tokens<const T: usize>(buf: [&[u8]; T]) -> [Token<'_>; T] {
        buf.map(|t| Token::text_node(t, false))
    }

    pub fn parse<'t, 'tv, T, E>(
        mut tokens: &'tv [Token<'t>],
        field_keyword: &[u8],
        def_mode: bool,
    ) -> Result<(T, &'tv [Token<'t>]), CommandParseError>
    where
        T: CMakeParse<'t> + std::fmt::Debug,
        E: CMakeParse<'t>,
    {
        #[derive(Default)]
        struct Buffers<'b> {
            field: Vec<Token<'b>>,
            another: Vec<Token<'b>>,
        }
        let mut buffers = Buffers::default();

        let mut field = CMakeParse::default_value();
        let mut another: Option<E> = CMakeParse::default_value();

        #[derive(Debug)]
        enum CMakeParserMode {
            Field,
            Another,
        }

        let mut current_mode = if def_mode {
            Some(CMakeParserMode::Field)
        } else {
            None
        };

        loop {
            let Some((first, rest)) = tokens.split_first() else {
                break;
            };
            tokens = rest;
            let keyword = first.as_bytes();
            if field.cmake_field_matches(field_keyword, keyword) {
                current_mode = if field.cmake_event_start(b"FIELD", first, &buffers.field)? {
                    Some(CMakeParserMode::Field)
                } else {
                    None
                };
                buffers.field.clear();
            } else if another.cmake_field_matches(b"END", keyword) {
                current_mode = if another.cmake_event_start(b"END", first, &buffers.another)? {
                    Some(CMakeParserMode::Another)
                } else {
                    None
                };
                buffers.another.clear();
            } else {
                match &current_mode {
                    Some(mode) => match mode {
                        CMakeParserMode::Field => buffers.field.push(first.clone()),
                        CMakeParserMode::Another => buffers.another.push(first.clone()),
                    },
                    None => {
                        return Err(crate::CommandParseError::UnknownOption(
                            String::from_utf8_lossy(keyword).to_string(),
                        ))
                    }
                }
            }
        }
        Ok((
            field
                .cmake_event_end(&buffers.field)?
                .ok_or_else(|| crate::CommandParseError::MissingToken("field".to_string()))?,
            tokens,
        ))
    }

    pub fn assert_parse<'t, 'tv, T, const C: usize>(ts: [&'t [u8]; C], keyword: &[u8]) -> T
    where
        T: CMakeParse<'t> + std::fmt::Debug,
    {
        let tokens = tokens(ts);
        let (res, tokens) =
            parse::<_, bool>(&tokens[..], keyword, keyword == b"CMD").expect("parse result");
        assert!(tokens.is_empty());
        res
    }
}
