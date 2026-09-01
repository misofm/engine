//! Issue #175: rack placement regroups lanes and must never move a rendered bit.
//!
//! AGENTS.md states the property and #166 made it structural: "Bank eligibility is decided by the
//! effect's homogeneous bank kernel contract, never by which rack the session placed it in ...
//! Banking regroups lanes; it never changes per-lane arithmetic, so a placement change must not
//! move a rendered bit."
//!
//! The owner's intended production layout is a placement change of exactly that kind: the
//! compressor moves out of its own one-slot `dynamic` chain and becomes the second slot of the
//! `simd1` chain the EQ already occupies. The signal still meets the EQ and then the compressor,
//! with the same coefficients, in the same order -- the strip order is
//! `builtins -> simd1 -> dynamic -> simd2`, so appending to `simd1` and emptying `dynamic`
//! preserves traversal order exactly. What changes is how many bank chains the plan runs, and
//! therefore how many planar/AoSoA transpose round-trips it pays.
//!
//! That is the whole basis for reading the chain-shape row-pair as a cost difference rather than
//! as two different sessions, so it is asserted here rather than assumed by the benchmark.

use bench_support::digest::Sha256Sink;
use console_workload::{SessionRuntime, Workload};

/// Enough blocks for every recursive filter and detector to be well past its transient.
const BLOCKS: u64 = 64;

/// Renders `workload` for [`BLOCKS`] blocks and returns the digest of everything it emitted.
fn render_digest(workload: Workload) -> String {
    let mut runtime = SessionRuntime::new(workload);
    let mut digest = Sha256Sink::new();
    for block in 0..BLOCKS {
        runtime.render(block).expect("console render");
        runtime.hash_output(&mut digest);
    }
    digest.finish_hex()
}

/// The #166 property, on the two placements #175 exists to compare.
///
/// `sixty_four_track_console_legacy` is the retired fixture as written: EQ on `simd1`, compressor
/// in `dynamic`, two one-slot chains. `sixty_four_track_eq_comp_simd1` is the standing fixture
/// with its limiter removed: the same EQ and the same compressor, the same coefficients, the same
/// order, as one two-slot chain on `simd1`.
///
/// If these two digests ever differ, the chain-shape row-pair is not measuring chain shape and
/// the benchmark's headline number is meaningless -- so this failing is a stop, not a re-pin.
#[test]
fn merging_the_compressor_into_the_simd1_chain_moves_no_rendered_bit() {
    let split = render_digest(Workload::SixtyFourTrackConsoleLegacy);
    let merged = render_digest(Workload::SixtyFourTrackEqCompSimd1);
    assert_eq!(
        split, merged,
        "placement is a layout choice, not an arithmetic one: EQ+compressor as two one-slot \
         chains (simd1 + dynamic) must render exactly what the same two effects render as one \
         two-slot chain on simd1"
    );
}

/// The limiter is the one thing in the standing fixture that is genuinely new arithmetic.
///
/// Stated as a test because the row-pair's honesty depends on it in the other direction: if the
/// full intended strip rendered the same bits as the strip without the limiter, the limiter would
/// not be doing anything and its measured cost would be the cost of nothing.
#[test]
fn the_intended_strips_limiter_moves_rendered_bits() {
    assert_ne!(
        render_digest(Workload::SixtyFourTrackEqCompSimd1),
        render_digest(Workload::SixtyFourTrackConsole),
        "the simd2 true-peak limiter must actually process the signal"
    );
}

/// The two placements agree on every block, not merely on the last one.
///
/// The digest tests above fold every block into one hash, which would let a difference in an
/// early block cancel against a difference in a later one -- vanishingly unlikely under SHA-256,
/// but the property being defended here is bit-exactness, and a test for bit-exactness should not
/// rest on a collision argument. This compares the two placements block by block and names the
/// first block that disagrees.
#[test]
fn the_two_placements_agree_block_by_block() {
    let mut split = SessionRuntime::new(Workload::SixtyFourTrackConsoleLegacy);
    let mut merged = SessionRuntime::new(Workload::SixtyFourTrackEqCompSimd1);
    for block in 0..BLOCKS {
        split.render(block).expect("split-chain render");
        merged.render(block).expect("merged-chain render");
        let mut left = Sha256Sink::new();
        let mut right = Sha256Sink::new();
        split.hash_output(&mut left);
        merged.hash_output(&mut right);
        assert_eq!(
            left.finish_hex(),
            right.finish_hex(),
            "block {block}: the two placements diverged"
        );
    }
}
