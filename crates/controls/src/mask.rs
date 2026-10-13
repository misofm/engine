//! The value mask (decision 17, C6 and #1554 D1): the model fields that hold control values.
//!
//! A host that holds a running plan classifies a transaction by comparing the committed model
//! with the prospective one. [`mask_values`] says which parts of a model are control values: it
//! copies them from one model into the other, so what remains different is structure. The control
//! rows (`crate::diff`) then say whether each copied value is live.

use session::{
    Console, ConsoleEntry, DualMonoBuiltins, DualMonoFader, MatrixOrPan, Rack, SessionModel,
};

use crate::effect_rows;

/// The control-value parts of one strip of `from`.
struct Source<'a> {
    builtins: &'a DualMonoBuiltins,
    fader: &'a DualMonoFader,
    matrix_or_pan: &'a MatrixOrPan,
    console: &'a [ConsoleEntry],
    inserts: &'a Rack,
}

/// The control-value parts of one strip of `into`.
struct Target<'a> {
    builtins: &'a mut DualMonoBuiltins,
    fader: &'a mut DualMonoFader,
    matrix_or_pan: &'a mut MatrixOrPan,
    console: &'a mut [ConsoleEntry],
    inserts: &'a mut Rack,
}

/// Copy, for every owner present in both models with the same ID and kind, the model fields that
/// hold control values from `from` into `into`: each strip's `builtins`, `fader` and
/// `matrix_or_pan`, and each console entry's and insert's `params`, `bypass` and (an insert's)
/// `link_mode`; each route's `gain_db`, `mute`, `channel_matrix` and `follows_mute`; each VCA's
/// `fader`; and the session's `control_smoothing`.
///
/// It copies representations, not resolved values: an effect's `both` entry and the equal
/// per-lane entries it could be rewritten as are different fields, so a rewrite of one as the other
/// is a value change the row diff sees as none, and a structural difference is never hidden. An
/// owner that `from` lacks, or has with another kind, is left as it is; so is an effect value that
/// no row describes (a `cid` effect, an identity the launch registry lacks, an effect with a
/// single supported link mode's `link_mode`), so that a change to it stays a difference of
/// structure and a rebuild reports it.
///
/// After this call, `into` and `from` differ only in what is not a control value, and in the
/// owners they do not share.
pub fn mask_values(into: &mut SessionModel, from: &SessionModel) {
    into.control_smoothing = from.control_smoothing;
    let console = &into.console;
    for (index, track) in into.tracks.iter_mut().enumerate() {
        if let Some(source) = pair(&from.tracks, index, |candidate| candidate.id == track.id) {
            mask_strip(
                console,
                Target {
                    builtins: &mut track.builtins,
                    fader: &mut track.fader,
                    matrix_or_pan: &mut track.matrix_or_pan,
                    console: &mut track.console,
                    inserts: &mut track.inserts,
                },
                &Source {
                    builtins: &source.builtins,
                    fader: &source.fader,
                    matrix_or_pan: &source.matrix_or_pan,
                    console: &source.console,
                    inserts: &source.inserts,
                },
            );
        }
    }
    for (index, submix) in into.submixes.iter_mut().enumerate() {
        if let Some(source) = pair(&from.submixes, index, |candidate| candidate.id == submix.id) {
            mask_strip(
                console,
                Target {
                    builtins: &mut submix.builtins,
                    fader: &mut submix.fader,
                    matrix_or_pan: &mut submix.matrix_or_pan,
                    console: &mut submix.console,
                    inserts: &mut submix.inserts,
                },
                &Source {
                    builtins: &source.builtins,
                    fader: &source.fader,
                    matrix_or_pan: &source.matrix_or_pan,
                    console: &source.console,
                    inserts: &source.inserts,
                },
            );
        }
    }
    for (index, route) in into.routes.iter_mut().enumerate() {
        if let Some(source) = pair(&from.routes, index, |candidate| candidate.id == route.id) {
            route.gain_db = source.gain_db;
            route.mute = source.mute;
            route.channel_matrix = source.channel_matrix.clone();
            route.follows_mute = source.follows_mute;
        }
    }
    for (index, vca) in into.vcas.iter_mut().enumerate() {
        if let Some(source) = pair(&from.vcas, index, |candidate| candidate.id == vca.id) {
            vca.fader = source.fader.clone();
        }
    }
}

/// The item of `items` that `matches`, trying position `index` first: two models of one session
/// list their owners in one order, so the search is a single comparison on the common path.
fn pair<T>(items: &[T], index: usize, matches: impl Fn(&T) -> bool) -> Option<&T> {
    items
        .get(index)
        .filter(|item| matches(item))
        .or_else(|| items.iter().find(|item| matches(item)))
}

/// Mask one strip. `console` is the session's slot declaration, which gives each entry's identity.
fn mask_strip(console: &Console, into: Target<'_>, from: &Source<'_>) {
    into.builtins.clone_from(from.builtins);
    into.fader.clone_from(from.fader);
    into.matrix_or_pan.clone_from(from.matrix_or_pan);
    for (index, entry) in into.console.iter_mut().enumerate() {
        let Some(slot) = console.slots().find(|slot| slot.slot == entry.slot) else {
            continue;
        };
        if !effect_rows::has_rows(&slot.identity) {
            continue;
        }
        if let Some(prior) = pair(from.console, index, |candidate| {
            candidate.slot == entry.slot
        }) {
            entry.params.clone_from(&prior.params);
            entry.bypass = prior.bypass;
        }
    }
    for (index, effect) in into.inserts.effects.iter_mut().enumerate() {
        if !effect_rows::has_rows(&effect.identity) {
            continue;
        }
        let Some(prior) = pair(&from.inserts.effects, index, |candidate| {
            candidate.id == effect.id && candidate.identity == effect.identity
        }) else {
            continue;
        };
        effect.params.clone_from(&prior.params);
        effect.bypass = prior.bypass;
        if effect_rows::has_link_mode_row(&effect.identity) {
            effect.link_mode = prior.link_mode;
        }
    }
}
