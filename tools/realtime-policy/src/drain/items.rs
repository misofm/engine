//! The rules that read a region's tokens rather than its syntax tree: the items that can shadow a
//! name (D2, D9) and the constructors that loop without a loop keyword (D3).
//!
//! Both read leaves, so a macro's token tree and a `macro_rules!` body are read too: tokens in
//! them can declare an item or call a constructor as well. An item that a macro *expands to* is not
//! a token of the region; D10 refuses the macro itself (see [`super`]).

use crate::leaf::{Kind, Leaf};

/// The constructors that run a closure repeatedly with no loop keyword.
const CONSTRUCTORS: [&str; 3] = ["from_fn", "repeat_with", "successors"];

/// The word a leaf is, a raw identifier without its prefix.
fn word(leaf: &Leaf) -> Option<String> {
    match &leaf.kind {
        Kind::Word(word) => Some(word.strip_prefix("r#").unwrap_or(word).to_owned()),
        _ => None,
    }
}

/// Whether the leaf is the keyword `keyword` (a raw identifier of that spelling is no keyword).
fn is_keyword(leaf: &Leaf, keyword: &str) -> bool {
    matches!(&leaf.kind, Kind::Word(found) if found == keyword)
}

/// The keywords that declare an item whose name follows them. `static` may take a `mut` first.
const NAMING_ITEMS: [&str; 9] = [
    "const", "static", "struct", "enum", "union", "fn", "type", "trait", "mod",
];

/// D2, D9: whether an item among `leaves` (the scope of a read binding) can name `name`, a binding that a rule read: an item of any kind that declares it as
/// its own name (`const`, `static [mut]`, `struct`, `enum`, `union`, `fn`, `type`, `trait`,
/// `mod`), or a `use` whose tokens up to its `;` hold `name` or a glob `*`. A block item shadows a
/// `let` of an enclosing block for its whole block, wherever it is declared, so the position of
/// the item does not matter. A unit struct with comparison and `SubAssign` impls redirects an alias
/// as a `const` does, so every kind counts.
pub(super) fn names_item(leaves: &[Leaf], name: &str) -> bool {
    for (index, leaf) in leaves.iter().enumerate() {
        if NAMING_ITEMS.iter().any(|keyword| is_keyword(leaf, keyword)) {
            let mut next = index + 1;
            if is_keyword(leaf, "static")
                && leaves
                    .get(next)
                    .is_some_and(|after| is_keyword(after, "mut"))
            {
                next += 1;
            }
            if leaves
                .get(next)
                .and_then(word)
                .is_some_and(|found| found == name)
            {
                return true;
            }
        }
        if is_keyword(leaf, "use") {
            let declaration = leaves[index + 1..]
                .iter()
                .take_while(|after| !matches!(after.kind, Kind::Punct(';')));
            if declaration.into_iter().any(|after| {
                matches!(after.kind, Kind::Punct('*'))
                    || word(after).is_some_and(|found| found == name)
            }) {
                return true;
            }
        }
    }
    false
}

/// D3: the lines in which a region calls `from_fn`, `repeat_with` or `successors`, path-qualified
/// or bare. The token of `core::array::from_fn` and `std::array::from_fn` is the exception: the
/// closure of those runs a constant number of times. A pop in that closure is refused as a pop in
/// any closure is (B2a-D1), whatever the path means.
pub(super) fn constructor_lines(leaves: &[Leaf]) -> Vec<usize> {
    let mut lines = Vec::new();
    for (index, leaf) in leaves.iter().enumerate() {
        let Some(found) = word(leaf) else {
            continue;
        };
        if CONSTRUCTORS.contains(&found.as_str()) && !is_array_from_fn(leaves, index) {
            lines.push(leaf.start.line);
        }
    }
    lines.dedup();
    lines
}

/// Whether the `from_fn` at `leaves[index]` ends the path `core::array::from_fn` or
/// `std::array::from_fn`, with nothing but a leading `::` before it.
fn is_array_from_fn(leaves: &[Leaf], index: usize) -> bool {
    if word(&leaves[index]).is_none_or(|found| found != "from_fn") || index < 6 {
        return false;
    }
    let colons = |at: usize| {
        matches!(leaves[at].kind, Kind::Punct(':'))
            && matches!(leaves[at + 1].kind, Kind::Punct(':'))
    };
    let named = |at: usize, names: &[&str]| matches!(&leaves[at].kind, Kind::Word(found) if names.contains(&found.as_str()));
    if !(colons(index - 2)
        && named(index - 3, &["array"])
        && colons(index - 5)
        && named(index - 6, &["core", "std"]))
    {
        return false;
    }
    // A longer path (`other::core::array::from_fn`) is some other `core`: only a leading `::`,
    // or nothing, may come before.
    match index.checked_sub(9) {
        Some(before) if colons(before + 1) => !matches!(
            leaves[before].kind,
            Kind::Word(_) | Kind::Punct('>') | Kind::Close(_)
        ),
        _ => true,
    }
}
