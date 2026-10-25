//! The duck overlay of a strip removal's phase 1 (issue #1325 D6): what the running plan's cells
//! hold while its successor waits. Preparation composes it into a successor's base
//! ([`crate::prepare::SuccessorBase::duck`]), so it is available on every build; the transition
//! step that extends it (`DuckOverlay::joined`, in `crate::transition`) is the control
//! provider's.

use session::SessionModel;

/// What phase 1 wrote to a running plan while its successor waits (#1325 D6): kept with that
/// plan's epoch and dropped when the epoch stops running.
///
/// Every list is sorted and holds no duplicate. An empty overlay is "not in phase 1".
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DuckOverlay {
    pub(crate) strips: Vec<Box<str>>,
    pub(crate) routes: Vec<Box<str>>,
    pub(crate) sources: Vec<Box<str>>,
    pub(crate) not_before: u64,
}

impl DuckOverlay {
    /// The empty overlay: nothing ducks.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            strips: Vec::new(),
            routes: Vec::new(),
            sources: Vec::new(),
            not_before: 0,
        }
    }

    /// The most an overlay of a plan of a session with `strips` strips, `routes` routes and
    /// `sources` sources can retain, whose IDs total `strip_id_bytes`, `route_id_bytes` and
    /// `source_id_bytes` (#1325): the three ID vectors at those counts and one copy of each ID's
    /// text. An overlay names only that plan's strips, routes and sources, each once, and its
    /// vectors are kept at their length. A ceiling of the session's shape, not of its edits.
    #[must_use]
    pub fn retained_ceiling(
        [strips, routes, sources]: [usize; 3],
        [strip_id_bytes, route_id_bytes, source_id_bytes]: [usize; 3],
    ) -> Option<usize> {
        strips
            .checked_add(routes)?
            .checked_add(sources)?
            .checked_mul(size_of::<Box<str>>())?
            .checked_add(strip_id_bytes)?
            .checked_add(route_id_bytes)?
            .checked_add(source_id_bytes)
    }

    /// The heap bytes the overlay retains: its three vectors at their capacity and their ID text.
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        [&self.strips, &self.routes, &self.sources]
            .into_iter()
            .map(|ids| {
                ids.capacity() * size_of::<Box<str>>()
                    + ids.iter().map(|id| id.len()).sum::<usize>()
            })
            .sum()
    }

    /// Whether the overlay holds nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strips.is_empty() && self.routes.is_empty() && self.sources.is_empty()
    }

    /// The strips whose mute phase 1 wrote, sorted.
    #[must_use]
    pub fn strips(&self) -> &[Box<str>] {
        &self.strips
    }

    /// The routes whose mute or gate phase 1 wrote, sorted.
    #[must_use]
    pub fn routes(&self) -> &[Box<str>] {
        &self.routes
    }

    /// The sources the successor removes that still feed the running plan (#1325 D4), sorted.
    #[must_use]
    pub fn sources(&self) -> &[Box<str>] {
        &self.sources
    }

    /// `S`, the earliest block start at which a successor may be adopted, or `None` for an empty
    /// overlay.
    #[must_use]
    pub fn not_before(&self) -> Option<u64> {
        (!self.is_empty()).then_some(self.not_before)
    }

    /// Whether `id` names a strip in the overlay.
    #[must_use]
    pub fn has_strip(&self, id: &str) -> bool {
        self.strips
            .binary_search_by(|held| (**held).cmp(id))
            .is_ok()
    }

    /// Whether `id` names a route in the overlay.
    #[must_use]
    pub fn has_route(&self, id: &str) -> bool {
        self.routes
            .binary_search_by(|held| (**held).cmp(id))
            .is_ok()
    }

    /// Whether `id` names a source in the overlay: one binary search, no allocation, so a source
    /// call may route by it on any thread the facade runs on.
    #[must_use]
    pub fn has_source(&self, id: &[u8]) -> bool {
        self.sources
            .binary_search_by(|held| held.as_bytes().cmp(id))
            .is_ok()
    }

    /// Lay the overlay over `model`: each overlay strip's two channels muted and each overlay
    /// route muted, which is what the running plan's cells hold once the ramps settle. A name
    /// the model does not hold is skipped.
    pub fn apply_to(&self, model: &mut SessionModel) {
        for track in &mut model.tracks {
            if self.has_strip(track.id.as_str()) {
                track.fader.left_mute = true;
                track.fader.right_mute = true;
            }
        }
        for submix in &mut model.submixes {
            if self.has_strip(submix.id.as_str()) {
                submix.fader.left_mute = true;
                submix.fader.right_mute = true;
            }
        }
        for route in &mut model.routes {
            if self.has_route(route.id.as_str()) {
                route.mute = true;
            }
        }
    }

    /// Raise `S` to `not_before` if it is later (#1325 D6: the overlay's `S` is the maximum of its
    /// own and the one the new writes give).
    pub fn raise_not_before(&mut self, not_before: u64) {
        self.not_before = self.not_before.max(not_before);
    }
}
