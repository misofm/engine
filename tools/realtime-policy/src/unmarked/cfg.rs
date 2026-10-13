//! Test code by attribute (#1443 D1, rule 2): `#[test]`, and `#[cfg(P)]` where `P` is false.
//!
//! `P` is evaluated with three-valued (Kleene) logic: the atom `test` is false, and every
//! other atom, every `feature = ".."` included, is unknown (root fix of 2026-10-13: reading a
//! feature as true made `#[cfg(not(feature = "x"))]` false and let its pops pass as test code). Several `#[cfg]` attributes on one
//! node are joined with AND. `cfg_attr` is ignored, which fails closed: a node whose attributes
//! the evaluator cannot read as false is not test code.

use syn::punctuated::Punctuated;

/// A value of three-valued logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tri {
    False,
    Unknown,
    True,
}

impl Tri {
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::Unknown,
        }
    }

    fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
            _ => Self::Unknown,
        }
    }

    fn not(self) -> Self {
        match self {
            Self::False => Self::True,
            Self::True => Self::False,
            Self::Unknown => Self::Unknown,
        }
    }
}

/// Whether a node with these attributes is test code: one of them is `#[test]`, or the AND of its
/// `#[cfg(..)]` attributes (outer and inner alike) is false.
pub(super) fn is_test(attributes: &[syn::Attribute]) -> bool {
    let mut cfg = Tri::True;
    let mut any_cfg = false;
    for attribute in attributes {
        if attribute.path().is_ident("test") && matches!(attribute.meta, syn::Meta::Path(_)) {
            return true;
        }
        if attribute.path().is_ident("cfg")
            && let syn::Meta::List(list) = &attribute.meta
        {
            any_cfg = true;
            let value = list
                .parse_args::<syn::Meta>()
                .map_or(Tri::Unknown, |meta| eval(&meta));
            cfg = cfg.and(value);
        }
    }
    any_cfg && cfg == Tri::False
}

fn eval(meta: &syn::Meta) -> Tri {
    match meta {
        syn::Meta::Path(path) if path.is_ident("test") => Tri::False,
        syn::Meta::List(list) => {
            let Ok(arguments) =
                list.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
            else {
                return Tri::Unknown;
            };
            let values = arguments.iter().map(eval);
            if list.path.is_ident("all") {
                values.fold(Tri::True, Tri::and)
            } else if list.path.is_ident("any") {
                values.fold(Tri::False, Tri::or)
            } else if list.path.is_ident("not") && arguments.len() == 1 {
                values.fold(Tri::Unknown, |_, value| value.not())
            } else {
                Tri::Unknown
            }
        }
        _ => Tri::Unknown,
    }
}
