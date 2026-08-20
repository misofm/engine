#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageError {
    Header,
    Length,
    Limit,
    Canonical,
    Text,
    Hash,
    Unavailable,
    State,
}
