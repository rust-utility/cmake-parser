use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// List operations.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/list.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum List<'t> {
    Reading(ListReading<'t>),
    Search(ListSearch<'t>),
    Modification(ListModification<'t>),
    Ordering(ListOrdering<'t>),
}

impl<'t> ToCommandScope for List<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum ListReading<'t> {
    Length(ListLength<'t>),
    Get(ListGet<'t>),
    Join(ListJoin<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListLength<'t> {
    pub list: Token<'t>,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListGet<'t> {
    pub list: Token<'t>,
    pub element_index: Vec<Token<'t>>,
    #[cmake(last)]
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListJoin<'t> {
    pub list: Token<'t>,
    pub glue: Token<'t>,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum ListSearch<'t> {
    Find(ListFind<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListFind<'t> {
    pub list: Token<'t>,
    pub value: Token<'t>,
    pub out_var: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum ListModification<'t> {
    Append(ListAppend<'t>),
    Filter(ListFilter<'t>),
    Insert(ListInsert<'t>),
    PopBack(ListPopBack<'t>),
    PopFront(ListPopFront<'t>),
    Prepend(ListPrepend<'t>),
    RemoveItem(ListRemoveItem<'t>),
    RemoveAt(ListRemoveAt<'t>),
    RemoveDuplicates(ListRemoveDuplicates<'t>),
    Transform(ListTransform<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListAppend<'t> {
    pub list: Token<'t>,
    pub element: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListFilter<'t> {
    pub list: Token<'t>,
    pub mode: FilterMode,
    #[cmake(transparent, rename = "REGEX")]
    pub regular_expression: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum FilterMode {
    Include,
    Exclude,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListInsert<'t> {
    pub list: Token<'t>,
    pub index: Token<'t>,
    pub element: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListPopBack<'t> {
    pub list: Token<'t>,
    pub out_var: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListPopFront<'t> {
    pub list: Token<'t>,
    pub out_var: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListPrepend<'t> {
    pub list: Token<'t>,
    pub element: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListRemoveItem<'t> {
    pub list: Token<'t>,
    pub value: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListRemoveAt<'t> {
    pub list: Token<'t>,
    pub index: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListRemoveDuplicates<'t> {
    pub list: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ListTransform<'t> {
    #[cmake(positional)]
    pub list: Token<'t>,
    #[cmake(positional)]
    pub action: TransformAction<'t>,
    pub selector: Option<TransformSelector<'t>>,
    pub output_variable: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list, transparent)]
pub enum TransformAction<'t> {
    Append(Token<'t>),
    Prepend(Token<'t>),
    #[cmake(rename = "TOLOWER")]
    ToLower,
    #[cmake(rename = "TOUPPER")]
    ToUpper,
    Strip,
    GenexStrip,
    Replace(ReplaceAction<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ReplaceAction<'t> {
    pub regular_expression: Token<'t>,
    pub replace_expression: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum TransformSelector<'t> {
    At(SelectorAt<'t>),
    For(SelectorFor<'t>),
    Regex(SelectorRegex<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SelectorAt<'t> {
    pub index: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SelectorFor<'t> {
    pub start: Token<'t>,
    pub stop: Token<'t>,
    pub step: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SelectorRegex<'t> {
    pub regular_expression: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum ListOrdering<'t> {
    Reverse(ListReverse<'t>),
    Sort(ListSort<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ListReverse<'t> {
    pub list: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ListSort<'t> {
    #[cmake(positional)]
    pub list: Token<'t>,
    pub compare: Option<SortCompare>,
    pub case: Option<SortCase>,
    pub order: Option<SortOrder>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum SortCompare {
    String,
    FileBasename,
    Natural,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum SortCase {
    Sensitive,
    Insensitive,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn list() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/list");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::List(Box::new(List::Reading(ListReading::Length(
                    ListLength {
                        list: token(b"list1"),
                        out_var: token(b"out_var1"),
                    }
                ))))),
                Ok(Command::List(Box::new(List::Reading(ListReading::Get(
                    ListGet {
                        list: token(b"list1"),
                        element_index: tokens_vec([b"index1"]),
                        out_var: token(b"out_var1"),
                    }
                ))))),
                Ok(Command::List(Box::new(List::Reading(ListReading::Get(
                    ListGet {
                        list: token(b"list1"),
                        element_index: tokens_vec([b"index1", b"index2"]),
                        out_var: token(b"out_var1"),
                    }
                ))))),
                Ok(Command::List(Box::new(List::Reading(ListReading::Join(
                    ListJoin {
                        list: token(b"list1"),
                        glue: token(b"glue1"),
                        out_var: token(b"out_var1"),
                    }
                ))))),
                Ok(Command::List(Box::new(List::Search(ListSearch::Find(
                    ListFind {
                        list: token(b"list1"),
                        value: token(b"value1"),
                        out_var: token(b"out_var1"),
                    }
                ))))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Append(ListAppend {
                        list: token(b"list1"),
                        element: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Append(ListAppend {
                        list: token(b"list1"),
                        element: Some(tokens_vec([b"element1", b"element2"])),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Filter(ListFilter {
                        list: token(b"list1"),
                        mode: FilterMode::Include,
                        regular_expression: token(b"regex1"),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Filter(ListFilter {
                        list: token(b"list1"),
                        mode: FilterMode::Exclude,
                        regular_expression: token(b"regex1"),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Insert(ListInsert {
                        list: token(b"list1"),
                        index: token(b"5"),
                        element: tokens_vec([b"element1", b"element2"]),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::PopBack(ListPopBack {
                        list: token(b"list1"),
                        out_var: Some(tokens_vec([b"out_var1", b"out_var2"])),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::PopFront(ListPopFront {
                        list: token(b"list1"),
                        out_var: Some(tokens_vec([b"out_var1", b"out_var2"])),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Prepend(ListPrepend {
                        list: token(b"list1"),
                        element: Some(tokens_vec([b"element1", b"element2"])),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::RemoveItem(ListRemoveItem {
                        list: token(b"list1"),
                        value: tokens_vec([b"value1", b"value2"]),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::RemoveAt(ListRemoveAt {
                        list: token(b"list1"),
                        index: tokens_vec([b"index1", b"index2"]),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::RemoveDuplicates(ListRemoveDuplicates {
                        list: token(b"list1"),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::Append(token(b"value1")),
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::Prepend(token(b"value1")),
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToLower,
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::Strip,
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::GenexStrip,
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::Replace(ReplaceAction {
                            regular_expression: token(b"regular_expression1"),
                            replace_expression: token(b"replace_expression1"),
                        }),
                        selector: None,
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::At(SelectorAt {
                            index: token(b"10"),
                        })),
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::At(SelectorAt {
                            index: token(b"10"),
                        })),
                        output_variable: Some(token(b"out_var1")),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::For(SelectorFor {
                            start: token(b"10"),
                            stop: token(b"20"),
                            step: None,
                        })),
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::For(SelectorFor {
                            start: token(b"10"),
                            stop: token(b"20"),
                            step: None,
                        })),
                        output_variable: Some(token(b"out_var1")),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::For(SelectorFor {
                            start: token(b"10"),
                            stop: token(b"20"),
                            step: Some(token(b"30")),
                        })),
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::For(SelectorFor {
                            start: token(b"10"),
                            stop: token(b"20"),
                            step: Some(token(b"30")),
                        })),
                        output_variable: Some(token(b"out_var1")),
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::Regex(SelectorRegex {
                            regular_expression: token(b"regex1"),
                        })),
                        output_variable: None,
                    })
                )))),
                Ok(Command::List(Box::new(List::Modification(
                    ListModification::Transform(ListTransform {
                        list: token(b"list1"),
                        action: TransformAction::ToUpper,
                        selector: Some(TransformSelector::Regex(SelectorRegex {
                            regular_expression: token(b"regex1"),
                        })),
                        output_variable: Some(token(b"out_var1")),
                    })
                )))),
                Ok(Command::List(Box::new(List::Ordering(
                    ListOrdering::Reverse(ListReverse {
                        list: token(b"list1"),
                    })
                )))),
                Ok(Command::List(Box::new(List::Ordering(ListOrdering::Sort(
                    ListSort {
                        list: token(b"list1"),
                        compare: Some(SortCompare::Natural),
                        case: Some(SortCase::Sensitive),
                        order: Some(SortOrder::Ascending),
                    }
                ))))),
            ]
        )
    }
}
