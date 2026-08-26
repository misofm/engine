//! Host half of gate G5: run the frozen corpus natively and under wasmtime, compare the digests.
//!
//! Master plan #83 D5 claims a rendered block is bit-identical across `Scalar`/`Simd4`/`Simd8` and
//! across `x86_64`/`aarch64`/`wasm32`. Gates G1–G4 and G6 prove the width half of that natively.
//! This crate proves the target half the only way it can be proven: by executing the identical
//! corpus on a second target and comparing bits, never tolerances.
//!
//! Two legs, one corpus:
//!
//! * **native** — [`miso_engine_wasm_gate_corpus`] linked as an `rlib` and run in-process at
//!   every width, compared against the pins.
//! * **wasm** — the same crate compiled to `wasm32-unknown-unknown` (with and without `simd128`)
//!   and executed under wasmtime, compared against the same pins.
//!
//! The runtime is configured to *reject* relaxed SIMD (`Config::wasm_relaxed_simd(false)`), so a
//! guest built with `-C target-feature=+relaxed-simd` that actually emits a relaxed instruction
//! fails module validation instead of quietly returning a different digest. Master plan D3 forbids
//! those instructions; this is where that is enforced against a built artifact and not a grep.

use std::fmt;
use std::path::Path;

use miso_engine_bench_support::stats;
use miso_engine_bench_support::timing;
use miso_engine_wasm_gate_corpus as corpus;
use wasmtime::{Config, Engine, Instance, Module, Store, TypedFunc};

/// The pinned WebAssembly runtime, reported in the evidence line.
///
/// Pinned exactly because a runtime upgrade may change which post-MVP proposals validate, and
/// rejecting a module that uses one is part of what this gate does.
pub const WASMTIME_VERSION: &str = "47.0.3";

/// Licence of the pinned runtime, reported alongside the version.
///
/// Wasmtime and the Cranelift backend it embeds are Apache-2.0 with the LLVM exception. This
/// crate is dev/tooling and links nothing into a shipped artifact.
pub const WASMTIME_LICENCE: &str = "Apache-2.0 WITH LLVM-exception";

/// Which production backend a run is expected to have used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedBackend {
    /// `f32`, one lane: a wasm build without `simd128`.
    Scalar,
    /// `Simd4`: wasm with `simd128`, or AArch64 NEON.
    Simd4,
    /// `Simd8`: one `__m256` on `x86-64-v3`.
    Simd8,
}

impl ExpectedBackend {
    /// Parses the `--expect-backend` argument.
    ///
    /// # Errors
    ///
    /// Returns the offending text if it names no backend.
    pub fn parse(text: &str) -> Result<Self, String> {
        match text {
            "scalar" => Ok(Self::Scalar),
            "simd4" => Ok(Self::Simd4),
            "simd8" => Ok(Self::Simd8),
            other => Err(other.to_string()),
        }
    }

    /// The code the guest's `miso_gate_backend` export reports.
    #[must_use]
    pub const fn code(self) -> u32 {
        match self {
            Self::Scalar => 0,
            Self::Simd4 => 1,
            Self::Simd8 => 2,
        }
    }

    /// Name used in reports.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Simd4 => "simd4",
            Self::Simd8 => "simd8",
        }
    }
}

/// One case whose digest did not equal its pin.
#[derive(Clone, Debug)]
pub struct Mismatch {
    /// Corpus case index.
    pub case: usize,
    /// Case name, so the report names a kernel and not a number.
    pub name: String,
    /// Width index the case ran at.
    pub width: usize,
    /// The pinned digest.
    pub expected: [u8; 32],
    /// What this run produced.
    pub actual: [u8; 32],
}

impl fmt::Display for Mismatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "case {} ({}) at {}: expected {}, got {}",
            self.case,
            self.name,
            corpus::width_name(self.width),
            hex(&self.expected),
            hex(&self.actual)
        )
    }
}

/// The outcome of one leg.
#[derive(Clone, Debug)]
pub struct Report {
    /// Which leg produced it: `"native"` or `"wasm"`.
    pub leg: &'static str,
    /// The backend the run reported.
    pub backend: u32,
    /// Corpus cases compared.
    pub cases: usize,
    /// Digest comparisons performed (a lane case is compared at every width).
    pub comparisons: usize,
    /// Every case whose digest did not equal its pin.
    pub mismatches: Vec<Mismatch>,
    /// Lanes on which this leg's `Lane::max`/`Lane::min` disagreed with the scalar oracle over the
    /// per-backend lowering pool, summed over every width. Anything but zero fails the leg.
    ///
    /// Not a digest and not pinned: it is the wasm execution of the truth table behind the
    /// single-instruction `max`/`min` lowerings, which no native gate can reach (see
    /// `miso_engine_wasm_gate_corpus::minmax_lowering_mismatches`).
    pub minmax_lowering_mismatches: u32,
}

impl Report {
    /// One machine-readable evidence line, in the shape the other audit tools emit.
    #[must_use]
    pub fn json(&self) -> String {
        let mismatches: Vec<String> = self
            .mismatches
            .iter()
            .map(|mismatch| {
                format!(
                    "{{\"case\":{},\"name\":\"{}\",\"width\":\"{}\",\"expected\":\"{}\",\"actual\":\"{}\"}}",
                    mismatch.case,
                    mismatch.name,
                    corpus::width_name(mismatch.width),
                    hex(&mismatch.expected),
                    hex(&mismatch.actual)
                )
            })
            .collect();
        format!(
            "{{\"schema_version\":1,\"kind\":\"wasm_gates\",\"leg\":\"{}\",\"runtime\":\"wasmtime {}\",\"backend\":{},\"cases\":{},\"comparisons\":{},\"minmax_lowering_mismatches\":{},\"mismatches\":[{}]}}",
            self.leg,
            WASMTIME_VERSION,
            self.backend,
            self.cases,
            self.comparisons,
            self.minmax_lowering_mismatches,
            mismatches.join(",")
        )
    }
}

/// Lowercase hexadecimal of a digest.
#[must_use]
pub fn hex(bytes: &[u8; 32]) -> String {
    use fmt::Write as _;
    let mut text = String::with_capacity(64);
    for byte in bytes {
        let _ = write!(text, "{byte:02x}");
    }
    text
}

/// The widths a case is compared at: every width for a case with a lane instantiation, the scalar
/// run alone for a math case, whose functions have none.
fn widths_of(case: usize) -> std::ops::Range<usize> {
    if corpus::is_width_dependent(case) {
        0..corpus::WIDTHS
    } else {
        0..1
    }
}

/// The backend this process was compiled for, in the guest's numbering.
fn native_backend_code() -> u32 {
    match miso_engine_lane::Backend::current() {
        miso_engine_lane::Backend::Scalar => 0,
        miso_engine_lane::Backend::Simd4 => 1,
        miso_engine_lane::Backend::Simd8 => 2,
    }
}

/// Runs the corpus in this process and compares every digest against its pin.
///
/// This is the native leg of G5, and it is also what proves the pins still describe the corpus: if
/// this fails, the wasm leg has nothing meaningful to compare against.
#[must_use]
pub fn native_report() -> Report {
    let mut mismatches = Vec::new();
    let mut comparisons = 0;
    for case in 0..corpus::CASE_COUNT {
        let expected = corpus::expected_digest(case);
        for width in widths_of(case) {
            let actual = corpus::digest_case(case, width);
            comparisons += 1;
            if actual != expected {
                mismatches.push(Mismatch {
                    case,
                    name: corpus::case_name(case),
                    width,
                    expected,
                    actual,
                });
            }
        }
    }
    Report {
        leg: "native",
        backend: native_backend_code(),
        cases: corpus::CASE_COUNT,
        comparisons,
        mismatches,
        minmax_lowering_mismatches: (0..corpus::WIDTHS)
            .map(corpus::minmax_lowering_mismatches)
            .sum(),
    }
}

/// The guest exports this host drives.
struct Guest {
    /// Wasmtime store; the guest holds no host state, so its data is `()`.
    store: Store<()>,
    /// `miso_gate_digest_word(case, width, word) -> u32`.
    digest_word: TypedFunc<(u32, u32, u32), u32>,
    /// `miso_gate_minmax_lowering_mismatches(width) -> u32`.
    minmax_lowering_mismatches: TypedFunc<u32, u32>,
    /// What `miso_gate_backend()` reported.
    backend: u32,
    /// What `miso_gate_case_count()` reported.
    cases: usize,
}

impl Guest {
    /// Compiles and instantiates the module with an empty import object.
    fn load(path: &Path) -> wasmtime::Result<Self> {
        let mut config = Config::new();
        // `simd128` is the artifact the browser ships. Relaxed SIMD is forbidden by D3 and is
        // switched off here so a guest that emits one fails validation rather than the comparison.
        config.wasm_simd(true);
        config.wasm_relaxed_simd(false);
        let engine = Engine::new(&config)?;
        let module = Module::from_file(&engine, path)?;
        let mut store = Store::new(&engine, ());
        // No imports: the guest cannot reach the host, the clock, or anything outside itself.
        let instance = Instance::new(&mut store, &module, &[])?;

        let backend: TypedFunc<(), u32> =
            instance.get_typed_func(&mut store, "miso_gate_backend")?;
        let case_count: TypedFunc<(), u32> =
            instance.get_typed_func(&mut store, "miso_gate_case_count")?;
        let widths: TypedFunc<(), u32> = instance.get_typed_func(&mut store, "miso_gate_widths")?;
        let digest_word: TypedFunc<(u32, u32, u32), u32> =
            instance.get_typed_func(&mut store, "miso_gate_digest_word")?;
        let minmax_lowering_mismatches: TypedFunc<u32, u32> =
            instance.get_typed_func(&mut store, "miso_gate_minmax_lowering_mismatches")?;

        let backend = backend.call(&mut store, ())?;
        let cases = case_count.call(&mut store, ())? as usize;
        let guest_widths = widths.call(&mut store, ())? as usize;
        if cases != corpus::CASE_COUNT || guest_widths != corpus::WIDTHS {
            wasmtime::bail!(
                "guest corpus shape ({cases} cases, {guest_widths} widths) differs from this \
                 host's ({} cases, {} widths): the two were built from different sources",
                corpus::CASE_COUNT,
                corpus::WIDTHS
            );
        }

        Ok(Self {
            store,
            digest_word,
            minmax_lowering_mismatches,
            backend,
            cases,
        })
    }

    /// Runs the `max`/`min` lowering truth table inside the guest, at every width.
    fn minmax_lowering_mismatches(&mut self) -> wasmtime::Result<u32> {
        let mut total = 0;
        for width in 0..corpus::WIDTHS {
            total += self
                .minmax_lowering_mismatches
                .call(&mut self.store, width as u32)?;
        }
        Ok(total)
    }

    /// Reads one digest out of the guest, eight little-endian words at a time.
    fn digest(&mut self, case: usize, width: usize) -> wasmtime::Result<[u8; 32]> {
        let mut digest = [0_u8; 32];
        for word in 0..8_usize {
            let value = self
                .digest_word
                .call(&mut self.store, (case as u32, width as u32, word as u32))?;
            digest[word * 4..word * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        Ok(digest)
    }
}

/// Runs the corpus inside `path` under wasmtime and compares every digest against its pin.
///
/// # Errors
///
/// Returns an error if the module fails to compile, validate or instantiate (which is what a guest
/// that emits a relaxed-SIMD instruction does), if an export is missing, if the guest traps, or if
/// the backend it reports is not `expected`.
pub fn wasm_report(path: &Path, expected: ExpectedBackend) -> wasmtime::Result<Report> {
    let mut guest = Guest::load(path)?;
    if guest.backend != expected.code() {
        wasmtime::bail!(
            "guest reports backend {} but {} was expected: the artifact was built with the wrong \
             simd128 setting",
            guest.backend,
            expected.name()
        );
    }

    let mut mismatches = Vec::new();
    let mut comparisons = 0;
    for case in 0..guest.cases {
        let expected_digest = corpus::expected_digest(case);
        for width in widths_of(case) {
            let actual = guest.digest(case, width)?;
            comparisons += 1;
            if actual != expected_digest {
                mismatches.push(Mismatch {
                    case,
                    name: corpus::case_name(case),
                    width,
                    expected: expected_digest,
                    actual,
                });
            }
        }
    }

    let minmax_lowering_mismatches = guest.minmax_lowering_mismatches()?;

    Ok(Report {
        leg: "wasm",
        backend: guest.backend,
        cases: guest.cases,
        comparisons,
        mismatches,
        minmax_lowering_mismatches,
    })
}

/// Regenerates the lane pins from the scalar `Lane` oracle, in the include-file form.
///
/// Master plan §8: a pin comes from the oracle, never from copying whatever the production path
/// currently prints. Only width 0 — the scalar implementation — is read here, and the gate then
/// requires `Simd4` and `Simd8`, on every target, to reproduce it.
#[must_use]
pub fn print_lane_pins() -> String {
    use fmt::Write as _;
    let mut text = String::from("[\n");
    for case in 0..corpus::LANE_CASE_COUNT {
        let digest = corpus::digest_case(case, 0);
        let _ = writeln!(text, "    // {}", corpus::case_name(case));
        text.push_str("    [");
        for (index, byte) in digest.iter().enumerate() {
            if index > 0 {
                text.push_str(", ");
            }
            let _ = write!(text, "0x{byte:02X}");
        }
        text.push_str("],\n");
    }
    text.push_str("]\n");
    text
}

// =============================================================================================
// Issue #163 phase 0b: the wasm kernel timing arm.
// =============================================================================================
//
// # What this is, and what it deliberately is not
//
// Full 0b is the *console* benchmark running under a wasm runtime, so that the 222 µs block the
// native records report has a browser-shaped twin. That is not what this is. `console.rs` is
// `#[cfg(not(target_arch = "wasm32"))]` and every compiler it drives -- session, builtins, effect
// and graph -- is a `cfg(not(wasm32))` dependency of the bench crate; `wasm32-unknown-unknown` has
// no clock, so `Instant` cannot even be constructed inside the guest; and the guest takes no
// arguments and reads no environment, so the runner's round marker and host metadata have nowhere
// to go. Reaching the console workload under wasm is a port of the bench tool, not a flag.
//
// This is the smallest honest measurement that can be taken *today*, through the harness gate G5
// already owns: the same frozen lane kernels, built for `wasm32-unknown-unknown` with `+simd128`,
// executed under the same pinned wasmtime, timed on the host clock.
//
// # How the SHA-256 is cancelled rather than ignored
//
// The only thing the guest exports is a digest, and a digest is a kernel followed by a SHA-256 of
// its output. Timing one call therefore measures the kernel *plus* a hash that is far larger than
// it. The way out is a difference rather than a ratio of totals.
//
// `corpus::digest_case` routes every lane case and every element-wise case down one identical
// path: `digest_lanes(&lane_values(index, width, true))`. `lane_values` always produces `LANES`
// signals of `FRAMES` frames, so **every case in this arm hashes exactly the same 32,768 bytes**.
// The SHA-256 term, the eight host-to-guest crossings that read the digest words, and the guest's
// memoisation bookkeeping are all constant across the arms, so they cancel in
//
//     T(heavy case) - T(baseline case) = kernel(heavy) - kernel(baseline)
//
// taken per observation. That difference is what this arm publishes. `gain_block/noise` is the
// baseline because a per-sample multiply is the cheapest lane kernel the corpus has.
//
// # Why the arms are alternated
//
// The #104 protocol, for the usual reason -- and here it is also load bearing for a second one.
// The guest memoises the last `(case, width)` it digested, so a loop that ran one case a thousand
// times would compute it once and then time a cache hit nine hundred and ninety-nine times.
// Alternating the arms busts that cache on every call by construction.

/// Observations per arm in the timing run.
pub const TIMING_OBSERVATIONS: usize = 500;
/// Untimed calls per arm before the clock starts.
const TIMING_WARMUP: usize = 16;
/// The cheapest lane kernel in the corpus: one multiply per sample. Every reported delta is a
/// difference from this arm, which is what cancels the common SHA-256 term.
pub const TIMING_BASELINE_CASE: &str = "gain_block/noise";
/// The kernels this arm reports, by their frozen corpus names.
pub const TIMING_CASES: [&str; 3] = [
    // The state-variable filter the parametric EQ and every builtin filter section ride.
    "svf_block_ramped/noise",
    // The one-pole follower a compressor's detector rides.
    "one_pole_block/noise",
    // Master plan §3.5's software FMA: the `v128` body no native gate can execute, and the reason
    // #163 says every recorded number is native while the product ships wasm.
    "lane_fma",
];

/// One timed arm of the wasm kernel timing run.
#[derive(Clone, Debug)]
pub struct TimingArm {
    /// The frozen corpus case name.
    pub name: String,
    /// Its case index in this build of the corpus.
    pub case: usize,
    /// Nearest-rank percentiles of the per-call wall time.
    pub p50_ns: u64,
    /// 95th percentile.
    pub p95_ns: u64,
    /// 99th percentile.
    pub p99_ns: u64,
    /// Median of the per-observation difference from the baseline arm. The kernel cost.
    pub paired_delta_median_ns: i64,
    /// The digest this arm produced, so a timing run also proves it ran the pinned computation.
    pub digest: [u8; 32],
}

/// The outcome of one leg at one width.
#[derive(Clone, Debug)]
pub struct TimingReport {
    /// The runner-supplied round marker: `0` for the discarded warmup, then `1` and `2`.
    ///
    /// Supplied by `scripts/run-wasm-kernel-timing.sh` and recorded here, so a record taken by a
    /// direct invocation cannot pass as one of the two frozen measured rounds.
    pub round: u32,
    /// `"native"` or `"wasm"`.
    pub leg: &'static str,
    /// The record-family label. Never comparable with a native console record.
    pub backend: String,
    /// Which lane width the corpus was driven at.
    pub width: usize,
    /// Observations per arm.
    pub observations: usize,
    /// The baseline arm every delta is measured from.
    pub baseline: TimingArm,
    /// The measured kernels.
    pub arms: Vec<TimingArm>,
}

impl TimingReport {
    /// One machine-readable evidence line.
    #[must_use]
    pub fn json(&self) -> String {
        let arm = |arm: &TimingArm| {
            format!(
                "{{\"case\":\"{}\",\"case_index\":{},\"p50_ns\":{},\"p95_ns\":{},\"p99_ns\":{},\
                 \"paired_delta_median_ns\":{},\"digest\":\"{}\"}}",
                arm.name,
                arm.case,
                arm.p50_ns,
                arm.p95_ns,
                arm.p99_ns,
                arm.paired_delta_median_ns,
                hex(&arm.digest)
            )
        };
        let arms: Vec<String> = self.arms.iter().map(arm).collect();
        format!(
            "{{\"schema_version\":1,\"issue\":163,\"phase\":\"0b\",\"record\":\"wasm_kernel_timing\",\
             \"round\":{},\"leg\":\"{}\",\"backend\":\"{}\",\"width\":\"{}\",\"runtime\":\"wasmtime {}\",\
             \"comparable_with_console_records\":false,\
             \"observations\":{},\"pairing\":\"alternating_per_observation\",\
             \"percentile_method\":\"nearest_rank\",\"units\":\"ns_per_case\",\
             \"common_term\":\"every arm hashes the same {} bytes, so the SHA-256 cancels in the paired delta\",\
             \"baseline\":{},\"arms\":[{}],\"descriptive_only\":true,\
             \"statistical_method\":\"arms alternated per observation; nearest-rank percentiles \
over per-call nanoseconds; paired delta is the arm minus the baseline arm per observation; \
descriptive only; no threshold\"}}",
            self.round,
            self.leg,
            self.backend,
            corpus::width_name(self.width),
            WASMTIME_VERSION,
            self.observations,
            corpus::LANES * corpus::FRAMES * 4,
            arm(&self.baseline),
            arms.join(",")
        )
    }
}

/// The index of a frozen corpus case, by name.
fn case_by_name(name: &str) -> usize {
    (0..corpus::LANE_CASE_COUNT)
        .find(|index| corpus::case_name(*index) == name)
        .unwrap_or_else(|| {
            panic!("the frozen corpus no longer carries the case {name}: the timing arm names it")
        })
}

/// Median of the per-observation differences `arm[i] - baseline[i]`.
fn paired_median(arm: &[u64], baseline: &[u64]) -> i64 {
    let mut paired: Vec<i64> = arm
        .iter()
        .zip(baseline)
        .map(|(arm, baseline)| *arm as i64 - *baseline as i64)
        .collect();
    paired.sort_unstable();
    paired[paired.len() / 2]
}

/// Alternates the baseline and every named case, timing each call on the host clock.
fn timing_run(
    round: u32,
    leg: &'static str,
    backend: String,
    width: usize,
    mut run: impl FnMut(usize) -> [u8; 32],
) -> TimingReport {
    let names: Vec<String> = std::iter::once(TIMING_BASELINE_CASE.to_owned())
        .chain(TIMING_CASES.iter().map(|name| (*name).to_owned()))
        .collect();
    let cases: Vec<usize> = names.iter().map(|name| case_by_name(name)).collect();

    for &case in &cases {
        for _ in 0..TIMING_WARMUP {
            let _ = run(case);
        }
    }

    let mut samples: Vec<Vec<u64>> = cases
        .iter()
        .map(|_| Vec::with_capacity(TIMING_OBSERVATIONS))
        .collect();
    let mut digests = vec![[0_u8; 32]; cases.len()];
    for _ in 0..TIMING_OBSERVATIONS {
        for (index, &case) in cases.iter().enumerate() {
            let (elapsed_ns, digest) = timing::timed(|| run(case));
            samples[index].push(elapsed_ns);
            digests[index] = digest;
        }
    }

    let arm = |index: usize| {
        let mut sorted = samples[index].clone();
        sorted.sort_unstable();
        TimingArm {
            name: names[index].clone(),
            case: cases[index],
            p50_ns: stats::nearest_rank(&sorted, 50, 100),
            p95_ns: stats::nearest_rank(&sorted, 95, 100),
            p99_ns: stats::nearest_rank(&sorted, 99, 100),
            paired_delta_median_ns: paired_median(&samples[index], &samples[0]),
            digest: digests[index],
        }
    };
    TimingReport {
        round,
        leg,
        backend,
        width,
        observations: TIMING_OBSERVATIONS,
        baseline: arm(0),
        arms: (1..cases.len()).map(arm).collect(),
    }
}

/// The native leg of the timing arm: the corpus run in this process at `width`.
///
/// # Panics
///
/// Panics if `width >= corpus::WIDTHS`, or if a named case has left the frozen corpus.
#[must_use]
pub fn native_timing_report(round: u32, width: usize) -> TimingReport {
    assert!(width < corpus::WIDTHS, "width index out of range");
    let backend = format!("native-{}", corpus::width_name(width));
    timing_run(round, "native", backend, width, |case| {
        corpus::digest_case(case, width)
    })
}

/// The wasm leg of the timing arm: the same corpus inside `path`, under the pinned wasmtime.
///
/// # Errors
///
/// Returns an error if the module fails to compile, validate or instantiate, if an export is
/// missing, if the guest traps, or if the backend it reports is not `expected`.
///
/// # Panics
///
/// Panics if `width >= corpus::WIDTHS`, or if a named case has left the frozen corpus.
pub fn wasm_timing_report(
    round: u32,
    path: &Path,
    expected: ExpectedBackend,
    width: usize,
) -> wasmtime::Result<TimingReport> {
    assert!(width < corpus::WIDTHS, "width index out of range");
    let mut guest = Guest::load(path)?;
    if guest.backend != expected.code() {
        wasmtime::bail!(
            "guest reports backend {} but {} was expected",
            guest.backend,
            expected.name()
        );
    }
    // The label the record family carries. Named for the artifact the browser ships, and it is
    // never comparable with a native console record: different target, different width, and a
    // software FMA where the native build has a hardware one.
    let backend = match expected {
        ExpectedBackend::Simd4 => "wasm-simd128".to_owned(),
        other => format!("wasm-{}", other.name()),
    };
    Ok(timing_run(round, "wasm", backend, width, |case| {
        guest
            .digest(case, width)
            .expect("the guest must not trap inside the timing arm")
    }))
}
