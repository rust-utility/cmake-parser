use std::fmt::{self, Display};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'b> {
    bytes: &'b [u8],
    quoted: bool,
}

impl<'tn> Token<'tn> {
    pub fn text_node(bytes: &'tn [u8], quoted: bool) -> Self {
        Token { bytes, quoted }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn is_quoted(&self) -> bool {
        self.quoted
    }
}

impl<'b> Display for Token<'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.bytes))
    }
}

impl<'b> fmt::Debug for Token<'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.quoted {
            write!(f, "Token(\"{}\")", String::from_utf8_lossy(self.bytes))
        } else {
            write!(f, "Token({})", String::from_utf8_lossy(self.bytes))
        }
    }
}

impl<'b> AsRef<[u8]> for Token<'b> {
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl<'b, const N: usize> From<&'b [u8; N]> for Token<'b> {
    fn from(bytes: &'b [u8; N]) -> Self {
        Self {
            bytes,
            quoted: false,
        }
    }
}

pub struct TokenDeclarations<'kv, 'k, 'tnv, 'tn> {
    tokens: &'tnv [Token<'tn>],
    keywords: &'kv [&'k [u8]],
    finished: bool,
}

pub fn declarations_by_keywords<'kv, 'k, 'tnv, 'tn>(
    tokens: &'tnv [Token<'tn>],
    keywords: &'kv [&'k [u8]],
) -> TokenDeclarations<'kv, 'k, 'tnv, 'tn> {
    TokenDeclarations {
        tokens,
        keywords,
        finished: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextNodeDeclaration<'tnv, 'tn> {
    option: &'tnv Token<'tn>,
    args: &'tnv [Token<'tn>],
}

impl<'tnv, 'tn> TextNodeDeclaration<'tnv, 'tn> {
    pub fn from_text_nodes(value: &'tnv [Token<'tn>]) -> Option<Self> {
        value
            .split_first()
            .map(|(option, args)| Self { option, args })
    }

    pub fn option(&self) -> &Token<'tn> {
        self.option
    }

    pub fn args(&self) -> &[Token<'tn>] {
        self.args
    }
}

impl<'kv, 'k, 'tnv, 'tn> Iterator for TokenDeclarations<'kv, 'k, 'tnv, 'tn> {
    type Item = TextNodeDeclaration<'tnv, 'tn>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut text_nodes = self.tokens.iter();

        let start = text_nodes.position(|tn| self.keywords.iter().any(|&k| tn.as_bytes() == k));

        let Some(start) = start else {
            self.finished = true;
            return TextNodeDeclaration::from_text_nodes(self.tokens);
        };

        if start != 0 {
            self.finished = true;
            return TextNodeDeclaration::from_text_nodes(self.tokens);
        }

        let len = self.tokens.len();

        let end = text_nodes
            .position(|tn| self.keywords.iter().any(|&k| tn.as_bytes() == k))
            .map(|end| start + end + 1)
            .unwrap_or(len);

        if end >= len {
            self.finished = true;
        }

        let ret = TextNodeDeclaration::from_text_nodes(&self.tokens[start..end]);
        self.tokens = &self.tokens[end..];
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::{declarations_by_keywords, TextNodeDeclaration, Token};

    fn to_text_nodes<'tn>(tns: &[&'tn [u8]]) -> Vec<Token<'tn>> {
        tns.iter().map(|&x| Token::text_node(x, false)).collect()
    }

    #[test]
    fn check_split_by_keywords() {
        let tns: &[&[u8]] = &[b"HELLO", b"world", b"FLAG", b"FLAG", b"COMMAND", b"command"];
        let text_nodes: Vec<_> = to_text_nodes(tns);
        let mut iter = declarations_by_keywords(&text_nodes, &[b"FLAG", b"HELLO", b"COMMAND"]);
        assert_eq!(
            TextNodeDeclaration::from_text_nodes(&to_text_nodes(&[b"HELLO", b"world"])),
            iter.next()
        );
        assert_eq!(
            TextNodeDeclaration::from_text_nodes(&to_text_nodes(&[b"FLAG"])),
            iter.next()
        );
        assert_eq!(
            TextNodeDeclaration::from_text_nodes(&to_text_nodes(&[b"FLAG"])),
            iter.next()
        );
        assert_eq!(
            TextNodeDeclaration::from_text_nodes(&to_text_nodes(&[b"COMMAND", b"command"])),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }
}
