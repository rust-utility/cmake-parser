use crate::{CommandParseError, Token};

pub trait CMakeParse<'t>: 't + Sized {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError>;

    fn complete(tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        let (result, tokens) = Self::parse(tokens)?;
        if !tokens.is_empty() {
            return Err(CommandParseError::Incomplete);
        }

        Ok(result)
    }

    fn default_value() -> Option<Self> {
        None
    }

    fn matches(&self, field_keyword: &[u8], keyword: &[u8], tokens: &[Token<'t>]) -> bool {
        Self::matches_type(field_keyword, keyword, tokens)
    }

    fn matches_type(
        field_keyword: &[u8],
        keyword: &[u8],
        #[allow(unused_variables)] tokens: &[Token<'t>],
    ) -> bool {
        field_keyword == keyword
    }

    fn need_update(
        #[allow(unused_variables)] field_keyword: &[u8],
        #[allow(unused_variables)] keyword: &Token<'t>,
        buffer: &[Token<'t>],
    ) -> bool {
        !buffer.is_empty()
    }

    fn start<'tv>(
        &mut self,
        field_keyword: &[u8],
        keyword: &Token<'t>,
        tokens: &'tv [Token<'t>],
        buffer: &mut Vec<Token<'t>>,
    ) -> Result<(bool, &'tv [Token<'t>]), CommandParseError> {
        if Self::need_update(field_keyword, keyword, buffer) {
            self.update(buffer)?;
            buffer.clear();
        }

        if Self::need_push_keyword(keyword) {
            buffer.push(keyword.clone());
        }

        Ok((Self::update_mode(keyword), Self::rest(tokens)))
    }

    fn rest<'tv>(tokens: &'tv [Token<'t>]) -> &'tv [Token<'t>] {
        tokens
    }

    fn need_push_keyword(keyword: &Token<'t>) -> bool {
        !Self::update_mode(keyword)
    }

    fn update_mode(#[allow(unused_variables)] keyword: &Token<'t>) -> bool {
        true
    }

    fn update<'tv>(&mut self, tokens: &'tv [Token<'t>]) -> Result<(), CommandParseError> {
        Self::complete(tokens).map(|res| *self = res)
    }

    fn end<'tv>(mut self, tokens: &'tv [Token<'t>]) -> Result<Self, CommandParseError> {
        self.update(tokens)?;

        Ok(self)
    }
}

impl<'t> CMakeParse<'t> for Token<'t> {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        tokens
            .split_first()
            .map(|(first, rest)| (first.clone(), rest))
            .ok_or(CommandParseError::TokenRequired)
    }
}

impl<'t, T> CMakeParse<'t> for Option<T>
where
    T: CMakeParse<'t>,
{
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T::parse(tokens).map(|(result, rest)| (Some(result), rest))
    }

    fn complete(tokens: &[Token<'t>]) -> Result<Self, CommandParseError> {
        match Self::parse(tokens) {
            Ok((result, _)) => Ok(result),
            Err(CommandParseError::TokenRequired) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn default_value() -> Option<Self> {
        Some(T::default_value())
    }

    fn matches_type(field_keyword: &[u8], keyword: &[u8], tokens: &[Token<'t>]) -> bool {
        T::matches_type(field_keyword, keyword, tokens)
    }

    fn update<'tv>(&mut self, tokens: &'tv [Token<'t>]) -> Result<(), CommandParseError> {
        if let Some(t) = self {
            t.update(tokens)
        } else {
            Self::complete(tokens).map(|res| *self = res)
        }
    }

    fn need_update(field_keyword: &[u8], keyword: &Token<'t>, buffer: &[Token<'t>]) -> bool {
        T::need_update(field_keyword, keyword, buffer)
    }

    fn need_push_keyword(keyword: &Token<'t>) -> bool {
        T::need_push_keyword(keyword)
    }

    fn rest<'tv>(tokens: &'tv [Token<'t>]) -> &'tv [Token<'t>] {
        T::rest(tokens)
    }

    fn update_mode(keyword: &Token<'t>) -> bool {
        T::update_mode(keyword)
    }
}

impl<'t> CMakeParse<'t> for bool {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        Ok(tokens
            .split_first()
            .map(|(_, rest)| (true, rest))
            .unwrap_or_else(|| (false, tokens)))
    }

    fn update_mode(#[allow(unused_variables)] keyword: &Token<'t>) -> bool {
        false
    }
}

impl<'t> CMakeParse<'t> for () {
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        Ok(tokens
            .split_first()
            .map(|(_, rest)| ((), rest))
            .unwrap_or_else(|| ((), &[])))
    }

    fn update_mode(#[allow(unused_variables)] keyword: &Token<'t>) -> bool {
        false
    }
}

impl<'t, T> CMakeParse<'t> for Vec<T>
where
    T: CMakeParse<'t>,
{
    fn parse<'tv>(
        mut tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        let mut result = vec![];
        loop {
            let (val, new_tokens) = T::parse(tokens)?;
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

    fn need_update(field_keyword: &[u8], keyword: &Token<'t>, buffer: &[Token<'t>]) -> bool {
        T::need_update(field_keyword, keyword, buffer)
    }

    fn need_push_keyword(keyword: &Token<'t>) -> bool {
        T::need_push_keyword(keyword)
    }

    fn matches_type(field_keyword: &[u8], keyword: &[u8], tokens: &[Token<'t>]) -> bool {
        T::matches_type(field_keyword, keyword, tokens)
    }

    fn update<'tv>(&mut self, tokens: &'tv [Token<'t>]) -> Result<(), CommandParseError> {
        Self::complete(tokens).map(|res| self.extend(res))
    }

    fn rest<'tv>(tokens: &'tv [Token<'t>]) -> &'tv [Token<'t>] {
        T::rest(tokens)
    }
}

impl<'t, T> CMakeParse<'t> for Box<T>
where
    T: CMakeParse<'t>,
{
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T::parse(tokens).map(|(result, rest)| (Box::new(result), rest))
    }

    fn matches_type(field_keyword: &[u8], keyword: &[u8], tokens: &[Token<'t>]) -> bool {
        T::matches_type(field_keyword, keyword, tokens)
    }

    fn need_update(field_keyword: &[u8], keyword: &Token<'t>, buffer: &[Token<'t>]) -> bool {
        T::need_update(field_keyword, keyword, buffer)
    }

    fn need_push_keyword(keyword: &Token<'t>) -> bool {
        T::need_push_keyword(keyword)
    }

    fn update_mode(keyword: &Token<'t>) -> bool {
        T::update_mode(keyword)
    }

    fn rest<'tv>(tokens: &'tv [Token<'t>]) -> &'tv [Token<'t>] {
        T::rest(tokens)
    }
}

impl<'t, T1, T2> CMakeParse<'t> for (T1, T2)
where
    T1: CMakeParse<'t>,
    T2: CMakeParse<'t>,
{
    fn parse<'tv>(tokens: &'tv [Token<'t>]) -> Result<(Self, &'tv [Token<'t>]), CommandParseError> {
        T1::parse(tokens)
            .and_then(|(t1, tokens)| T2::parse(tokens).map(|(t2, tokens)| ((t1, t2), tokens)))
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
    fn cmake_parse_vec_none() {
        let option_vec_token: Option<Vec<Token<'_>>> = assert_parse([b"END"], b"QQQ");
        assert_eq!(option_vec_token, None);
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

    #[test]
    fn cmake_parse_enum() {
        #[derive(CMake, Debug, PartialEq)]
        #[cmake(pkg = "crate")]
        enum Test {
            PostBuild,
            Compile,
        }

        let enm: Test = assert_parse([b"COMPILE", b"END"], b"WHEN");
        assert_eq!(enm, Test::Compile);

        let enm: Option<Test> = assert_parse([b"COMPILE", b"END"], b"WHEN");
        assert_eq!(enm, Some(Test::Compile));

        let enm: Option<Test> = assert_parse([b"END"], b"WHEN");
        assert_eq!(enm, None);
    }

    pub fn token(buf: &[u8]) -> Token<'_> {
        Token::text_node(buf, false)
    }

    pub fn tokens<const T: usize>(buf: [&[u8]; T]) -> [Token<'_>; T] {
        buf.map(token)
    }

    pub fn tokens_vec<const T: usize>(buf: [&[u8]; T]) -> Vec<Token<'_>> {
        tokens(buf).to_vec()
    }

    pub fn quoted_token(buf: &[u8]) -> Token<'_> {
        Token::text_node(buf, true)
    }

    pub fn quoted_tokens<const T: usize>(buf: [&[u8]; T]) -> [Token<'_>; T] {
        buf.map(quoted_token)
    }

    pub fn quoted_tokens_vec<const T: usize>(buf: [&[u8]; T]) -> Vec<Token<'_>> {
        quoted_tokens(buf).to_vec()
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

        #[derive(Debug, Copy, Clone)]
        enum CMakeParserMode {
            Field,
            Another,
        }

        let default_mode = if def_mode {
            Some(CMakeParserMode::Field)
        } else {
            None
        };

        let mut current_mode = default_mode;

        loop {
            let Some((first, rest)) = tokens.split_first() else {
                break;
            };
            tokens = rest;
            let keyword = first.as_bytes();
            if field.matches(field_keyword, keyword, tokens) {
                let (update_mode, rest) =
                    field.start(field_keyword, first, tokens, &mut buffers.field)?;
                tokens = rest;
                if update_mode {
                    current_mode = Some(CMakeParserMode::Field)
                } else {
                    current_mode = default_mode;
                }
            } else if another.matches(b"END", keyword, tokens) {
                let (update_mode, rest) =
                    another.start(b"END", first, tokens, &mut buffers.another)?;
                tokens = rest;
                if update_mode {
                    current_mode = Some(CMakeParserMode::Another);
                } else {
                    current_mode = default_mode;
                }
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
                .end(&buffers.field)?
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
