//! The host's mirror of every live send (issue #1222 D3).
//!
//! A [`crate::RouteControlProducer`] builds a record from a send's *whole* value set -- gain,
//! matrix, mute and the follow-zeroed source lanes -- because the record's target is the four
//! folded coefficients. A command that moves one of them therefore needs the others, and the
//! render plane has no readback. [`LiveRouteState`] is that memory: one [`LiveRoute`] per live
//! route, in the order of `HostLiveControlHandles::route_controls`, seeded from the session the
//! plan was prepared from.
//!
//! # Which routes are live
//!
//! Every route whose destination is a submix, in canonical route-ID order -- exactly the routes
//! preparation attaches a control lane to. [`LiveRouteState::live_routes`] is that selection, so a
//! host can check the producers it was handed against it rather than assume the two orders agree.
//!
//! # Following a strip's mute
//!
//! A send with `follows_mute` zeroes its source strip's effectively muted lanes (issue #1224).
//! [`LiveRouteMuteFollow::delta`] names the sends a batch's strip mutes moved, and
//! [`LiveRouteState::follow`] records the new lanes; both read the effective mute through the one
//! function a host hands them.
//!
//! # The transaction
//!
//! Command admission is all-or-nothing across every queue in a submission, and the mirror changes
//! while a submission is still being validated. So, like `LiveControlSoloState`, it carries its
//! own shadow: the first mutation of a transaction copies the entries aside,
//! [`LiveRouteState::rollback`] restores them on any refusal, and [`LiveRouteState::commit`]
//! closes the transaction once the records are in their queues. Every array, the shadow included,
//! is allocated at preparation; nothing here allocates afterwards, and nothing runs on a render
//! thread.

use std::collections::TryReserveError;

use session::{Route, RouteDestination, RouteSource, SessionModel};

/// One live route's mirrored values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiveRoute {
    /// Send gain in decibels.
    pub gain_db: f32,
    /// The 2x2 matrix, `[ll, lr, rl, rr]`.
    pub matrix: [f32; 4],
    /// The send's own on/off switch.
    pub mute: bool,
    /// Whether the send follows its source strip's lane mutes. Fixed for the plan.
    pub follows_mute: bool,
    /// The strip index of the route's source: the tracks, then the submixes, each in canonical ID
    /// order. Fixed for the plan.
    pub source_strip: usize,
    /// The source lanes whose matrix column the send zeroes: the source strip's effective mute
    /// when `follows_mute` is set, else `[false; 2]`.
    pub source_lane_muted: [bool; 2],
}

/// Why [`LiveRouteState::try_new`] could not build the mirror.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveRouteStateError {
    /// An array could not be reserved.
    Allocation(TryReserveError),
    /// A live route's source names no track or submix of the model.
    UnknownSource,
}

/// The mirror of every live route, with a transaction shadow.
#[derive(Debug)]
pub struct LiveRouteState {
    routes: Box<[LiveRoute]>,
    shadow: Box<[LiveRoute]>,
    open: bool,
}

impl LiveRouteState {
    /// The routes that are live in a plan prepared with live controls from `model`: every route
    /// into a submix, in the model's order. For a normalized model (`CompiledSession::
    /// normalized_model`) that is canonical route-ID order, the order of
    /// `HostLiveControlHandles::route_controls`.
    pub fn live_routes(model: &SessionModel) -> impl Iterator<Item = &Route> {
        model
            .routes
            .iter()
            .filter(|route| matches!(route.destination, RouteDestination::SubmixInput { .. }))
    }

    /// Seed one entry per live route of `model` (a normalized model) from the session's values.
    ///
    /// `effective_mute(strip, lane)` is the source strip's effective lane mute at preparation; a
    /// following route's `source_lane_muted` is read from it, so a host seeds the mirror from
    /// whatever owns its strip mutes (the browser's solo state, or a committed model's mutes)
    /// without this type knowing which.
    ///
    /// # Errors
    ///
    /// [`LiveRouteStateError::Allocation`] when an array cannot be reserved, and
    /// [`LiveRouteStateError::UnknownSource`] when a live route's source is not a strip of the
    /// model (a model that never validated).
    pub fn try_new(
        model: &SessionModel,
        effective_mute: &dyn Fn(usize, usize) -> bool,
    ) -> Result<Self, LiveRouteStateError> {
        let count = Self::live_routes(model).count();
        let mut routes = Vec::new();
        routes
            .try_reserve_exact(count)
            .map_err(LiveRouteStateError::Allocation)?;
        for route in Self::live_routes(model) {
            let source_strip = source_strip(model, &route.source)?;
            let source_lane_muted = if route.follows_mute {
                followed_lanes(source_strip, effective_mute)
            } else {
                [false; 2]
            };
            let matrix = &route.channel_matrix;
            routes.push(LiveRoute {
                gain_db: route.gain_db,
                matrix: [matrix.ll, matrix.lr, matrix.rl, matrix.rr],
                mute: route.mute,
                follows_mute: route.follows_mute,
                source_strip,
                source_lane_muted,
            });
        }
        let mut shadow = Vec::new();
        shadow
            .try_reserve_exact(count)
            .map_err(LiveRouteStateError::Allocation)?;
        shadow.extend_from_slice(&routes);
        Ok(Self {
            routes: routes.into_boxed_slice(),
            shadow: shadow.into_boxed_slice(),
            open: false,
        })
    }

    /// The mirror of a plan with no live route, as a plan prepared without live controls has.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            routes: Box::new([]),
            shadow: Box::new([]),
            open: false,
        }
    }

    /// Live routes mirrored: the length of `HostLiveControlHandles::route_controls`.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.routes.len()
    }

    /// Whether the plan has no live route.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    /// One live route's current values, or `None` for an index past the last live route.
    #[must_use]
    pub fn get(&self, route: usize) -> Option<&LiveRoute> {
        self.routes.get(route)
    }

    /// Whether a transaction has mutated anything since the last commit or rollback.
    #[must_use]
    pub const fn transaction_open(&self) -> bool {
        self.open
    }

    /// Set one route's gain. `false`, changing nothing, for an unknown index.
    pub fn set_gain_db(&mut self, route: usize, gain_db: f32) -> bool {
        self.update(route, |entry| entry.gain_db = gain_db)
    }

    /// Set one route's matrix, `[ll, lr, rl, rr]`. `false`, changing nothing, for an unknown
    /// index.
    pub fn set_matrix(&mut self, route: usize, matrix: [f32; 4]) -> bool {
        self.update(route, |entry| entry.matrix = matrix)
    }

    /// Set one route's own mute. `false`, changing nothing, for an unknown index.
    pub fn set_mute(&mut self, route: usize, mute: bool) -> bool {
        self.update(route, |entry| entry.mute = mute)
    }

    /// Set a following route's source lanes to its source strip's effective mute now (issue #1224
    /// D1): what [`LiveRouteMuteFollow::delta`] yielded for it, read through the same function.
    /// `false`, changing nothing, for an unknown index or a route without `follows_mute`, whose
    /// lanes stay `[false; 2]`.
    pub fn follow(&mut self, route: usize, effective_mute: &dyn Fn(usize, usize) -> bool) -> bool {
        let Some(entry) = self.routes.get(route) else {
            return false;
        };
        if !entry.follows_mute {
            return false;
        }
        let lanes = followed_lanes(entry.source_strip, effective_mute);
        self.update(route, |entry| entry.source_lane_muted = lanes)
    }

    /// Close the transaction: the staged records reached their queues.
    pub const fn commit(&mut self) {
        self.open = false;
    }

    /// Undo everything this transaction did. A refused submission leaves no trace.
    pub fn rollback(&mut self) {
        if !self.open {
            return;
        }
        self.routes.copy_from_slice(&self.shadow);
        self.open = false;
    }

    fn update(&mut self, route: usize, change: impl FnOnce(&mut LiveRoute)) -> bool {
        if route >= self.routes.len() {
            return false;
        }
        if !self.open {
            self.shadow.copy_from_slice(&self.routes);
            self.open = true;
        }
        change(&mut self.routes[route]);
        true
    }
}

/// The follow-mute composition (issue #1224 D2): which live sends a change of strip mutes moves.
///
/// A send with `follows_mute` zeroes the matrix columns of its source strip's effectively muted
/// lanes (DESIGN P11). The effective mute is handed in as a function, so the browser reads its solo
/// state and the C ABI its committed model's mutes, and neither spells the composition again.
pub struct LiveRouteMuteFollow;

impl LiveRouteMuteFollow {
    /// Live routes whose follow-mute input changed: `(route index, new source_lane_muted)`, in
    /// live-route order.
    ///
    /// A route is yielded only when it has `follows_mute` and its source strip's effective lane
    /// mutes differ from the mirror's `source_lane_muted`. A redundant record would restart a
    /// settled ramp, which is digest visible, so nothing unchanged is ever yielded; a route without
    /// `follows_mute` never is.
    pub fn delta<'a>(
        routes: &'a LiveRouteState,
        effective_mute: &'a dyn Fn(usize, usize) -> bool,
    ) -> impl Iterator<Item = (usize, [bool; 2])> + 'a {
        routes
            .routes
            .iter()
            .enumerate()
            .filter_map(move |(route, entry)| {
                if !entry.follows_mute {
                    return None;
                }
                let lanes = followed_lanes(entry.source_strip, effective_mute);
                (lanes != entry.source_lane_muted).then_some((route, lanes))
            })
    }
}

/// A following route's source lanes: its source strip's effective mute, lane by lane (D1).
fn followed_lanes(source_strip: usize, effective_mute: &dyn Fn(usize, usize) -> bool) -> [bool; 2] {
    [
        effective_mute(source_strip, 0),
        effective_mute(source_strip, 1),
    ]
}

/// The strip index of a route source: its position among the tracks, or `tracks + j` for submix
/// `j`. Each segment of a normalized model is sorted, so each is searched apart.
fn source_strip(model: &SessionModel, source: &RouteSource) -> Result<usize, LiveRouteStateError> {
    match source {
        RouteSource::Track { track_id, .. } => model
            .tracks
            .binary_search_by(|track| track.id.cmp(track_id))
            .map_err(|_| LiveRouteStateError::UnknownSource),
        RouteSource::Submix { submix_id, .. } => model
            .submixes
            .binary_search_by(|submix| submix.id.cmp(submix_id))
            .map(|submix| model.tracks.len() + submix)
            .map_err(|_| LiveRouteStateError::UnknownSource),
    }
}

#[cfg(test)]
mod tests {
    use session::{ChannelMatrix, SendTap, StableId, Submix, parse_session_json};

    use super::*;

    fn id(text: &str) -> StableId {
        StableId::parse(text).expect("stable id")
    }

    fn track(text: &str) -> RouteSource {
        RouteSource::Track {
            track_id: id(text),
            tap: SendTap::PostPan,
        }
    }

    fn submix(text: &str) -> RouteSource {
        RouteSource::Submix {
            submix_id: id(text),
            tap: SendTap::PostPan,
        }
    }

    /// The observation fixture (tracks `t0..t2`, three routes into the output) plus submixes `bx`
    /// and `by` and four sends, in route-ID order `a-send`, `b-send`, `c-send`, then the fixture's
    /// `t*-main`, then `zz-send`. `t1` has its left lane muted and `by` its right lane.
    fn model() -> SessionModel {
        let mut model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/observation-frame-shape.json"
        ))
        .expect("observation fixture");
        model.tracks[1].fader.left_mute = true;
        let mut by = Submix::unity(id("by"), &model.console);
        by.fader.right_mute = true;
        model.submixes = vec![Submix::unity(id("bx"), &model.console), by];
        let template = model.routes[0].clone();
        let send = |name: &str, source, bus: &str, gain_db, matrix: [f32; 4], mute, follows| {
            let mut route = template.clone();
            route.id = id(name);
            route.source = source;
            route.destination = RouteDestination::SubmixInput { submix_id: id(bus) };
            route.channel_matrix = ChannelMatrix {
                ll: matrix[0],
                lr: matrix[1],
                rl: matrix[2],
                rr: matrix[3],
            };
            route.gain_db = gain_db;
            route.mute = mute;
            route.follows_mute = follows;
            route
        };
        let mut routes = vec![
            send(
                "a-send",
                track("t1"),
                "bx",
                -3.0,
                [0.5, 0.25, -0.125, 0.75],
                false,
                true,
            ),
            send(
                "b-send",
                track("t1"),
                "by",
                2.0,
                [0.25, -0.5, 0.75, 1.0],
                false,
                false,
            ),
            send(
                "c-send",
                submix("by"),
                "bx",
                -9.0,
                [1.0, 0.5, 0.25, -1.0],
                true,
                true,
            ),
            send(
                "zz-send",
                track("t2"),
                "bx",
                0.0,
                [0.125, 0.5, 0.375, 0.25],
                false,
                true,
            ),
        ];
        routes.append(&mut model.routes);
        routes.sort_by(|left, right| left.id.cmp(&right.id));
        model.routes = routes;
        model
    }

    /// The model's fader mutes by strip index, except that track `t2` (strip 2) reads as
    /// solo-muted on its left lane: what a host's composed effective mute can say and the session
    /// cannot.
    fn effective_mute(model: &SessionModel) -> impl Fn(usize, usize) -> bool + '_ {
        move |strip, lane| {
            let fader = model.tracks.get(strip).map_or_else(
                || &model.submixes[strip - model.tracks.len()].fader,
                |track| &track.fader,
            );
            let session = [fader.left_mute, fader.right_mute][lane];
            session || (strip == 2 && lane == 0)
        }
    }

    fn entry(gain_db: f32, matrix: [f32; 4], mute: bool, follows: bool) -> LiveRoute {
        LiveRoute {
            gain_db,
            matrix,
            mute,
            follows_mute: follows,
            source_strip: 0,
            source_lane_muted: [false; 2],
        }
    }

    /// Issue #1222 gate 4: construction keeps exactly the routes into submixes, in route-ID order,
    /// with the session's values, each source's strip index, and a following route's lanes from
    /// the effective mute it is handed.
    ///
    /// Test value: red if the mirror keeps an output route, follows declaration order, resolves a
    /// submix source without the track offset, seeds a non-following route's lanes, or reads the
    /// session mutes instead of the effective mute (`zz-send` would start unmuted).
    #[test]
    fn construction_seeds_every_field_from_the_session_and_the_effective_mute() {
        let model = model();
        let mute = effective_mute(&model);
        let state = LiveRouteState::try_new(&model, &mute).expect("mirror");
        let ids: Vec<&str> = LiveRouteState::live_routes(&model)
            .map(|route| route.id.as_str())
            .collect();
        assert_eq!(ids, ["a-send", "b-send", "c-send", "zz-send"]);
        let expected = [
            LiveRoute {
                source_strip: 1,
                source_lane_muted: [true, false],
                ..entry(-3.0, [0.5, 0.25, -0.125, 0.75], false, true)
            },
            LiveRoute {
                source_strip: 1,
                ..entry(2.0, [0.25, -0.5, 0.75, 1.0], false, false)
            },
            LiveRoute {
                source_strip: 4,
                source_lane_muted: [false, true],
                ..entry(-9.0, [1.0, 0.5, 0.25, -1.0], true, true)
            },
            LiveRoute {
                source_strip: 2,
                source_lane_muted: [true, false],
                ..entry(0.0, [0.125, 0.5, 0.375, 0.25], false, true)
            },
        ];
        assert_eq!(state.len(), expected.len());
        for (route, want) in expected.iter().enumerate() {
            assert_eq!(state.get(route), Some(want), "live route {route}");
        }
        assert_eq!(state.get(expected.len()), None);
        assert!(!state.transaction_open());
    }

    /// Issue #1222 gate 4: a rollback restores every field the transaction moved, a commit keeps
    /// them, and the next transaction's rollback returns to the committed values, not the seeds.
    ///
    /// Test value: red if a refused batch leaks a field into the next one -- a setter that skips
    /// the shadow, a shadow taken once at construction rather than per transaction, or a commit
    /// that leaves the transaction open.
    #[test]
    fn a_rollback_restores_every_field_and_a_commit_keeps_them() {
        let model = model();
        let mute = effective_mute(&model);
        let mut state = LiveRouteState::try_new(&model, &mute).expect("mirror");
        let seeded: Vec<LiveRoute> = (0..state.len()).map(|r| *state.get(r).unwrap()).collect();
        let snapshot = |state: &LiveRouteState| -> Vec<LiveRoute> {
            (0..state.len()).map(|r| *state.get(r).unwrap()).collect()
        };

        assert!(state.set_gain_db(0, -12.0));
        assert!(state.set_matrix(3, [0.0, 1.0, 1.0, 0.0]));
        assert!(state.set_mute(2, false));
        assert!(state.set_gain_db(0, 4.0), "a second edit of one field");
        assert!(!state.set_mute(4, true), "past the last live route");
        assert!(state.transaction_open());
        state.rollback();
        assert!(!state.transaction_open());
        assert_eq!(snapshot(&state), seeded, "rolled back to the seeds");

        assert!(state.set_mute(1, true));
        assert!(state.set_matrix(0, [0.25, 0.25, 0.25, 0.25]));
        state.commit();
        assert!(!state.transaction_open());
        state.rollback();
        let committed = snapshot(&state);
        assert!(committed[1].mute, "a commit keeps the mute");
        assert_eq!(committed[0].matrix, [0.25; 4], "a commit keeps the matrix");

        assert!(state.set_gain_db(1, -40.0));
        state.rollback();
        assert_eq!(
            snapshot(&state),
            committed,
            "rolled back to the committed values"
        );
    }

    /// `model()` with live route `route`'s `follows_mute` set to `follows`.
    fn model_with_follow(route: &str, follows: bool) -> SessionModel {
        let mut model = model();
        model
            .routes
            .iter_mut()
            .find(|candidate| candidate.id.as_str() == route)
            .expect("declared route")
            .follows_mute = follows;
        model
    }

    /// [`effective_mute`] with the listed `(strip, lane)` mutes flipped.
    fn flipped<'a>(
        model: &'a SessionModel,
        flips: &'a [(usize, usize)],
    ) -> impl Fn(usize, usize) -> bool + 'a {
        let base = effective_mute(model);
        move |strip, lane| base(strip, lane) != flips.contains(&(strip, lane))
    }

    fn delta(
        state: &LiveRouteState,
        mute: &dyn Fn(usize, usize) -> bool,
    ) -> Vec<(usize, [bool; 2])> {
        LiveRouteMuteFollow::delta(state, mute).collect()
    }

    /// Issue #1224 gate 7: one strip's change moves every following send it sources, and only
    /// those. Here `a-send` and `b-send` both follow `t1` (strip 1, seeded `[true, false]`);
    /// muting its right lane yields both with `[true, true]`, while `c-send` (`by`) and `zz-send`
    /// (`t2`) keep their lanes and are not yielded. `t0` and `t3` source no live route, so flipping
    /// both of their lanes yields nothing.
    ///
    /// Test value: red if `delta` stops at a strip's first following send, yields a send whose
    /// source did not change, or reads another strip's mute for a send.
    #[test]
    fn delta_yields_every_following_send_of_a_changed_strip_and_nothing_else() {
        let model = model_with_follow("b-send", true);
        let seed = effective_mute(&model);
        let state = LiveRouteState::try_new(&model, &seed).expect("mirror");
        assert_eq!(delta(&state, &seed), [], "seeded: nothing changed");

        let right = flipped(&model, &[(1, 1)]);
        assert_eq!(
            delta(&state, &right),
            [(0, [true, true]), (1, [true, true])]
        );

        let unrouted = flipped(&model, &[(0, 0), (0, 1), (3, 0), (3, 1)]);
        assert_eq!(
            delta(&state, &unrouted),
            [],
            "strips that source no live route"
        );
    }

    /// Issue #1224 gate 7: a one-lane flip yields only that lane's change, per lane and per
    /// source: `t2`'s right lane moves `zz-send` from `[true, false]` to `[true, true]`; `by`'s
    /// left lane moves `c-send` from `[false, true]` to `[true, true]`; clearing `t1`'s left lane
    /// moves `a-send` to `[false, false]`.
    ///
    /// Test value: red if `delta` maps a lane to the other column, reads one lane for both, or
    /// resolves a submix source without the track offset.
    #[test]
    fn delta_follows_one_lane_at_a_time() {
        let model = model();
        let seed = effective_mute(&model);
        let state = LiveRouteState::try_new(&model, &seed).expect("mirror");
        assert_eq!(
            delta(&state, &flipped(&model, &[(2, 1)])),
            [(3, [true, true])]
        );
        assert_eq!(
            delta(&state, &flipped(&model, &[(4, 0)])),
            [(2, [true, true])]
        );
        assert_eq!(
            delta(&state, &flipped(&model, &[(1, 0)])),
            [(0, [false, false])]
        );
    }

    /// Issue #1224 gate 7: a route without `follows_mute` is never yielded and never follows.
    /// `b-send` (from `t1`, not following) stays out of `delta` when both of `t1`'s lanes flip;
    /// [`LiveRouteState::follow`] refuses it and leaves its lanes `[false; 2]`. Following every
    /// yielded route then leaves `delta` empty, so an unchanged mute yields nothing again; and a
    /// rollback restores the lanes.
    ///
    /// Test value: red if a non-following send gets a follow record or zeroed columns, if
    /// `follow` records lanes other than the ones `delta` yielded (the next batch would re-emit
    /// them), or if a follow escapes the transaction shadow.
    #[test]
    fn a_send_without_follow_never_follows_and_a_followed_change_is_not_yielded_twice() {
        let model = model();
        let seed = effective_mute(&model);
        let mut state = LiveRouteState::try_new(&model, &seed).expect("mirror");
        let both = flipped(&model, &[(1, 0), (1, 1), (4, 1)]);
        let moved = delta(&state, &both);
        assert_eq!(moved, [(0, [false, true]), (2, [false, false])]);
        assert!(!state.follow(1, &both), "b-send does not follow");
        assert_eq!(
            state.get(1).map(|route| route.source_lane_muted),
            Some([false; 2])
        );
        assert!(!state.follow(4, &both), "past the last live route");
        for (route, lanes) in &moved {
            assert!(state.follow(*route, &both));
            assert_eq!(
                state.get(*route).map(|entry| entry.source_lane_muted),
                Some(*lanes)
            );
        }
        assert!(state.transaction_open());
        assert_eq!(
            delta(&state, &both),
            [],
            "a followed change is not yielded again"
        );
        state.rollback();
        assert_eq!(delta(&state, &seed), [], "rolled back to the seeded lanes");
        assert_eq!(delta(&state, &both), moved);
    }
}
