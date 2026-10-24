//! The cross-target corpus of the resident route fold (issue #952).
//!
//! The browser ships 4-lane banks, so `ArenaMembers::fold_resident_tiles::<4, Simd4>` is the fold
//! every browser session runs. Gate G5 (`tools/wasm-gate-corpus`) replays these cases natively and
//! under wasmtime with `simd128`, and compares both against [`DIGESTS`]; no other gate executes
//! that instantiation on `wasm32`.
//!
//! Each case folds [`LANES`] lanes of hostile stereo words over [`FRAMES`] frames into one
//! master through the production fold, reached by the one hook `runtime::run_resident_fold`. At
//! `L::WIDTH == 4` the lanes are two cohorts of four onto the same master -- the browser's shape --
//! the second continuing from the first's live master. At `L::WIDTH == 8` (only where `avx2` is
//! enabled) they are one cohort. At `L::WIDTH == 1` the case runs an independent scalar reference
//! written here, not the fold's own body: per frame and plane, each lane's routed word
//! `l' = lr * r + ll * l` and `r' = rr * r + rl * l` (two roundings each), then the lanes in
//! ascending order, each added on the right of the running value, which starts as lane 0's routed
//! word when lane 0 stores and as `master + lane 0` otherwise. Left association makes two
//! four-lane cohorts, one eight-lane cohort and this sum the same expression, so one pin per case
//! holds at every width.
//!
//! # Why the inputs are built from integers
//!
//! Every word comes from a `xorshift64*` and exact bit assembly, so a target cannot differ on the
//! inputs. Samples are signed zeros, signed subnormals and signed normals of magnitude
//! `2^-24 .. 2^25`; constants are signed zeros or signed normals of magnitude `2^-4 .. 2^2`. Both
//! are finite, so every sum stays finite and no NaN payload is ever digested.
//!
//! [`FRAMES`] is 131: 32 full four-frame tiles and 16 full eight-frame tiles, each with a ragged
//! tail of three frames.

use core::num::NonZeroUsize;

use effect_contract::BankWidth;
use engine::realtime::DisjointArena;
use lane::Lane;

use crate::runtime::{ARENA_BASE, run_resident_fold};

/// Lanes folded into the master in every case: two four-lane cohorts or one eight-lane cohort.
pub const LANES: usize = 8;

/// Frames per case: ragged at both bank widths.
pub const FRAMES: usize = 131;

/// Result words per case: the master's left plane, then its right plane.
pub const WORDS: usize = 2 * FRAMES;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 2;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = ["resident_fold/store", "resident_fold/accumulate"];

/// The pinned SHA-256 of each case's result words, little-endian, as
/// `wasm_gate_corpus::digest_case` hashes them. Generated from the scalar reference only.
pub const DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // resident_fold/store
    [
        0x83, 0xf3, 0x3f, 0x30, 0x5b, 0x11, 0xab, 0xa3, 0x08, 0x75, 0xf7, 0x4f, 0x48, 0xe6, 0x5f,
        0xde, 0x3d, 0x14, 0x47, 0x96, 0x30, 0xd6, 0x4e, 0x58, 0x7d, 0x10, 0x6e, 0xac, 0x34, 0xe8,
        0xb8, 0x4c,
    ],
    // resident_fold/accumulate
    [
        0x4a, 0x8b, 0xc4, 0x3f, 0x1a, 0x5b, 0x6c, 0x98, 0x15, 0x25, 0xb2, 0x48, 0x0c, 0x9a, 0xd2,
        0x1b, 0xda, 0x2a, 0x61, 0x13, 0x83, 0x90, 0xbd, 0xff, 0x0d, 0xd3, 0x85, 0x3c, 0x06, 0x50,
        0xf4, 0x81,
    ],
];

/// Seed of the corpus generator.
const SEED: u64 = 0x0952_f01d_5ee0_0952;

/// Runs one case at width `L::WIDTH` and writes its [`WORDS`] result words into `out`.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, `out.len() != WORDS`, `L::WIDTH` is not 1, 4 or (with `avx2`)
/// 8, or the production fold declines a cohort (which would leave the prior master in place and
/// digest it silently).
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert_eq!(out.len(), WORDS, "corpus output length");
    let store = case == 0;
    let corpus = Corpus::new();
    let (left, right) = match L::WIDTH {
        1 => corpus.scalar_reference(store),
        4 => corpus.fold(BankWidth::Four, store),
        #[cfg(target_feature = "avx2")]
        8 => corpus.fold(BankWidth::Eight, store),
        width => panic!("no resident fold at width {width}"),
    };
    for (word, value) in out.iter_mut().zip(left.iter().chain(&right)) {
        *word = value.to_bits();
    }
}

/// One case's inputs, independent of the width that folds them.
struct Corpus {
    /// `samples[lane][plane][frame]`.
    samples: [[[f32; FRAMES]; 2]; LANES],
    /// Each lane's `[ll, lr, rl, rr]`.
    constants: [[f32; 4]; LANES],
    /// The master before the fold: `[left, right]`.
    prior: [[f32; FRAMES]; 2],
}

impl Corpus {
    fn new() -> Self {
        let mut bits = Xorshift64Star(SEED);
        let constants = core::array::from_fn(|_| core::array::from_fn(|_| bits.constant()));
        let samples = core::array::from_fn(|_| {
            core::array::from_fn(|_| core::array::from_fn(|_| bits.sample()))
        });
        let prior = core::array::from_fn(|_| core::array::from_fn(|_| bits.sample()));
        Self {
            samples,
            constants,
            prior,
        }
    }

    /// The production fold at `width`, `LANES / width` cohorts in lane order onto one master.
    fn fold(&self, width: BankWidth, store: bool) -> ([f32; FRAMES], [f32; FRAMES]) {
        let lanes = width.lanes() as usize;
        let mut arena = DisjointArena::try_new(
            NonZeroUsize::new(2).expect("stereo planes"),
            NonZeroUsize::new(FRAMES).expect("frames"),
            1,
        )
        .expect("corpus arena");
        let (master_left, master_right) = arena.write_stereo(ARENA_BASE);
        master_left.copy_from_slice(&self.prior[0]);
        master_right.copy_from_slice(&self.prior[1]);
        for (cohort, constants) in self.constants.chunks_exact(lanes).enumerate() {
            let first = cohort * lanes;
            let resident = |plane: usize| -> Vec<f32> {
                (0..FRAMES * lanes)
                    .map(|word| self.samples[first + word % lanes][plane][word / lanes])
                    .collect()
            };
            let (left, right) = (resident(0), resident(1));
            assert!(
                run_resident_fold(
                    width,
                    (&left, &right),
                    FRAMES,
                    constants,
                    store && cohort == 0,
                    &mut arena,
                    ARENA_BASE,
                ),
                "the resident fold declined cohort {cohort} at {width:?}"
            );
        }
        let (left, right) = arena.read_stereo(ARENA_BASE);
        (
            left.try_into().expect("arena frames"),
            right.try_into().expect("arena frames"),
        )
    }

    /// The independent scalar reference: see the module documentation.
    fn scalar_reference(&self, store: bool) -> ([f32; FRAMES], [f32; FRAMES]) {
        let mut master = self.prior;
        for frame in 0..FRAMES {
            let routed = |lane: usize| -> [f32; 2] {
                let [ll, lr, rl, rr] = self.constants[lane];
                let (l, r) = (self.samples[lane][0][frame], self.samples[lane][1][frame]);
                let (left_product, right_product) = (ll * l, rl * l);
                [lr * r + left_product, rr * r + right_product]
            };
            for plane in 0..2 {
                let mut sum = if store {
                    routed(0)[plane]
                } else {
                    master[plane][frame] + routed(0)[plane]
                };
                for lane in 1..LANES {
                    sum += routed(lane)[plane];
                }
                master[plane][frame] = sum;
            }
        }
        (master[0], master[1])
    }
}

/// Xorshift64\* (Vigna 2016): a seeded, portable bit source defined by wrapping integer
/// operations only.
struct Xorshift64Star(u64);

impl Xorshift64Star {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// A signed zero one time in eight, a signed subnormal one time in eight, otherwise a signed
    /// normal of magnitude `2^-24 .. 2^25` with a random mantissa.
    fn sample(&mut self) -> f32 {
        let bits = self.next_u64();
        let sign = ((bits >> 63) as u32) << 31;
        let mantissa = (bits as u32) & 0x007f_ffff;
        match (bits >> 32) % 8 {
            0 => f32::from_bits(sign),
            1 => f32::from_bits(sign | mantissa.max(1)),
            _ => {
                // Biased exponents 103..=151 are the unbiased -24..=24.
                let exponent = 103 + ((bits >> 40) % 49) as u32;
                f32::from_bits(sign | (exponent << 23) | mantissa)
            }
        }
    }

    /// A signed zero one time in eight, otherwise a signed normal of magnitude `2^-4 .. 2^2`.
    fn constant(&mut self) -> f32 {
        let bits = self.next_u64();
        let sign = ((bits >> 63) as u32) << 31;
        if (bits >> 32).is_multiple_of(8) {
            return f32::from_bits(sign);
        }
        // Biased exponents 123..=128 are the unbiased -4..=1.
        let exponent = 123 + ((bits >> 40) % 6) as u32;
        f32::from_bits(sign | (exponent << 23) | ((bits as u32) & 0x007f_ffff))
    }
}
