use std::borrow::Cow;
use std::fmt::{self, Display};

pub trait TextNode<'tn>: Display + PartialEq<&'tn [u8]> {
    fn text_node<T>(bytes: T) -> Self
    where
        T: Into<Cow<'tn, [u8]>>;
    fn as_bytes(&self) -> &[u8];
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8TextNode<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> TextNode<'a> for Utf8TextNode<'a> {
    fn text_node<T>(bytes: T) -> Self
    where
        T: Into<Cow<'a, [u8]>>,
    {
        Utf8TextNode {
            bytes: bytes.into(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<'a> PartialEq<&'a [u8]> for Utf8TextNode<'a> {
    fn eq(&self, other: &&'a [u8]) -> bool {
        &self.bytes == other
    }
}

impl<'a> Display for Utf8TextNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

pub struct TextNodeDeclarations<'tn, 'k, TN> {
    text_nodes: &'tn [TN],
    keywords: &'k [&'k [u8]],
    finished: bool,
}

pub fn declarations_by_keywords<'tn, 'k, TN>(
    text_nodes: &'tn [TN],
    keywords: &'k [&[u8]],
) -> TextNodeDeclarations<'tn, 'k, TN> {
    TextNodeDeclarations {
        text_nodes,
        keywords,
        finished: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextNodeDeclaration<'tn, TN> {
    option: &'tn TN,
    args: &'tn [TN],
}

impl<'tn, TN> TextNodeDeclaration<'tn, TN> {
    pub fn from_text_nodes(value: &'tn [TN]) -> Option<Self> {
        value
            .split_first()
            .map(|(option, args)| Self { option, args })
    }

    pub fn option(&self) -> &TN {
        self.option
    }

    pub fn args(&self) -> &[TN] {
        self.args
    }
}

impl<'tn, 'k: 'tn, TN> Iterator for TextNodeDeclarations<'tn, 'k, TN>
where
    TN: TextNode<'tn>,
{
    type Item = TextNodeDeclaration<'tn, TN>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut text_nodes = self.text_nodes.iter();

        let start = text_nodes.position(|tn| self.keywords.iter().any(|k| *tn == k));

        let Some(start) = start else {
            self.finished = true;
            return TextNodeDeclaration::from_text_nodes(self.text_nodes);
        };

        if start != 0 {
            self.finished = true;
            return TextNodeDeclaration::from_text_nodes(self.text_nodes);
        }

        let len = self.text_nodes.len();

        let end = text_nodes
            .position(|tn| self.keywords.iter().any(|k| *tn == k))
            .map(|end| start + end + 1)
            .unwrap_or(len);

        if end >= len {
            self.finished = true;
        }

        let ret = TextNodeDeclaration::from_text_nodes(&self.text_nodes[start..end]);
        self.text_nodes = &self.text_nodes[end..];
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::TextNode;

    use super::{declarations_by_keywords, TextNodeDeclaration, Utf8TextNode};

    fn to_text_nodes<'tn>(tns: &[&'tn [u8]]) -> Vec<Utf8TextNode<'tn>> {
        tns.iter().map(|&x| Utf8TextNode::text_node(x)).collect()
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
