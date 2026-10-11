//! Issue #1088 (console strip P2a), gate 1: the shape half of a bank request with an active mask.
//!
//! A bank request carries one request per lane and one active-mask entry per lane, and binds
//! `members <= lanes` tracks, `members` being the mask's active count. `validate_shape` is the
//! contract-violation half of `bind_homogeneous_bank` (issue #95): it must accept every non-empty
//! members-first mask of the right length, so that a planner can bind a padded group, and refuse
//! the three masks no correct planner produces -- one naming no member, one of the wrong length and
//! one with an active lane after a padded one -- each with its own typed code, so a factory can
//! never index a lane the mask does not have or meet a layout the graph cannot gather.
//!
//! Every mask is enumerated at both bank widths, so the oracle is the definition itself (a
//! population count) rather than a sample of it.

mod support;

use effect_contract::{
    BankWidth, EffectDescriptor, EffectQuality, LinkMode, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedPorts, PreparedSidechainPort,
};
use lane::Backend;

static DESCRIPTOR: EffectDescriptor = support::descriptor(1, support::unstated);

/// `validate_shape` never reads a member, so any well-typed request stands in for one; its tail
/// bound is a registry entry, the only kind there is (issue #1462).
fn member() -> PrepareEffectRequest<'static> {
    let tail_bound = support::registry(&DESCRIPTOR)
        .expect("admitted")
        .tail_bound(support::EFFECT_ID, 48_000, EffectQuality::Normal)
        .expect("a declared row");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: &[],
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        },
        tail_bound,
    }
}

/// Every bank width this build has, beside the backend that executes it (issue #1112).
fn widths() -> impl Iterator<Item = (Backend, BankWidth)> {
    BankWidth::ALL.iter().map(|&width| (width.backend(), width))
}

fn code(request: PrepareEffectBankRequest<'_>) -> Option<&'static str> {
    request.validate_shape().err().map(|error| error.code)
}

/// A non-empty mask of the right length is well formed exactly when its members come first, whatever
/// its active count; every other one is `effect.bank.mask_not_prefix`. A well-formed mask's active
/// count and padding are what the mask says.
///
/// Red if `validate_shape` refuses a partial prefix mask (the planner could never bind a padded
/// group, which is P2a's whole point), accepts a mask with an active lane after a padded one (the
/// graph's gather and scatter admit only members first, and an opted-in factory relies on it; P2a
/// verdict, L3), accepts only full masks, or if `active_lanes`/`is_padded` count anything but the
/// mask's `true` entries -- the guard every unpadded factory declines on.
#[test]
fn a_mask_is_well_formed_exactly_when_its_members_come_first() {
    for (backend, width) in widths() {
        let lanes = width.lanes() as usize;
        let requests = vec![member(); lanes];
        let mut accepted = vec![0_usize; lanes + 1];
        for bits in 1_u32..(1 << lanes) {
            let mask: Vec<bool> = (0..lanes).map(|lane| bits >> lane & 1 == 1).collect();
            let request = PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: &mask,
            };
            let members = bits.count_ones() as usize;
            if bits != (1 << members) - 1 {
                assert_eq!(
                    code(request),
                    Some("effect.bank.mask_not_prefix"),
                    "{width:?} mask {bits:#b} has an active lane after a padded one"
                );
                continue;
            }
            assert_eq!(
                code(request),
                None,
                "{width:?} mask {bits:#b} is well formed"
            );
            assert_eq!(request.active_lanes(), members, "{width:?} mask {bits:#b}");
            assert_eq!(
                request.is_padded(),
                members < lanes,
                "{width:?} mask {bits:#b}"
            );
            accepted[members] += 1;
        }
        // Every active count 1..=W was reached, including W - 1 and 1 (the console remainders).
        assert!(
            accepted[1..].iter().all(|count| *count > 0),
            "{width:?}: {accepted:?}"
        );
    }
}

/// A full bank's mask is [`BankWidth::full_mask`]: well formed, never padded.
///
/// Red if `full_mask` has the wrong length or a `false` entry, which would make every shipped
/// caller that binds a full bank (the graph planner's full groups, the benches and audits) either
/// malformed or padded, and so declined by every effect.
#[test]
fn the_full_mask_is_a_full_bank() {
    for (backend, width) in widths() {
        let requests = vec![member(); width.lanes() as usize];
        let request = PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        };
        assert_eq!(code(request), None, "{width:?}");
        assert_eq!(request.active_lanes(), width.lanes() as usize, "{width:?}");
        assert!(!request.is_padded(), "{width:?}");
    }
}

/// A mask that names no member is `effect.bank.mask_empty`.
///
/// Red if an empty mask is accepted: a bank of no members has no request to clone, and a factory
/// that accepted it would bind a bank no track can reach.
#[test]
fn an_empty_mask_is_refused_with_its_code() {
    for (backend, width) in widths() {
        let lanes = width.lanes() as usize;
        let requests = vec![member(); lanes];
        let mask = vec![false; lanes];
        let request = PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: &mask,
        };
        assert_eq!(code(request), Some("effect.bank.mask_empty"), "{width:?}");
    }
}

/// A mask without exactly one entry per lane is `effect.bank.mask_length`, whether it is short
/// or long, all active, all padded or empty, and it is refused before its active count is read.
///
/// Red if the length is not checked (a factory indexing `active_mask[lane]` would read past a
/// short mask, and a long one would claim a lane the bank does not have), or if a wrong-length
/// mask reports `mask_empty` or `requests` instead of its own code.
#[test]
fn a_wrong_length_mask_is_refused_with_its_code() {
    for (backend, width) in widths() {
        let lanes = width.lanes() as usize;
        let requests = vec![member(); lanes];
        for length in (0..=2 * lanes).filter(|length| *length != lanes) {
            for fill in [true, false] {
                let mask = vec![fill; length];
                let request = PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                    active_mask: &mask,
                };
                assert_eq!(
                    code(request),
                    Some("effect.bank.mask_length"),
                    "{width:?}: a {length}-entry mask of {fill}"
                );
            }
        }
    }
}

/// The request half of the shape keeps its code, and is checked before the mask.
///
/// Red if adding the mask moved the existing refusals: a request list that is not one per lane,
/// or a backend that disagrees with the width, is still `effect.bank.requests`, even when the mask
/// is also wrong. The conformance harness counts refusals by that code.
#[test]
fn the_request_refusals_keep_their_code_and_come_first() {
    for (backend, width) in widths() {
        let lanes = width.lanes() as usize;
        let short = vec![member(); lanes - 1];
        let empty_mask = vec![false; lanes];
        for mask in [width.full_mask(), &empty_mask[..], &[true][..]] {
            let request = PrepareEffectBankRequest {
                backend,
                width,
                requests: &short,
                active_mask: mask,
            };
            assert_eq!(code(request), Some("effect.bank.requests"), "{width:?}");
        }
        let requests = vec![member(); lanes];
        // A backend of another width, which only an 8-lane (AVX2) build has (issue #1112).
        for &other in Backend::VECTOR.iter().filter(|&&other| other != backend) {
            let request = PrepareEffectBankRequest {
                backend: other,
                width,
                requests: &requests,
                active_mask: &empty_mask,
            };
            assert_eq!(code(request), Some("effect.bank.requests"), "{width:?}");
        }
    }
}
