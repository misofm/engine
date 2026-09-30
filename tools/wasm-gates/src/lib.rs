//! Host half of gate G5: run the frozen corpus natively and under wasmtime, compare the digests.
//!
//! Master plan #83 D5 claims a rendered block is bit-identical across `Scalar`/`Simd4`/`Simd8` and
//! across `x86_64`/`aarch64`/`wasm32`. Gates G1–G4 and G6 prove the width half of that natively.
//! This crate proves the target half the only way it can be proven: by executing the identical
//! corpus on a second target and comparing bits, never tolerances.
//!
//! Two legs, one corpus:
//!
//! * **native** — [`wasm_gate_corpus`] linked as an `rlib` and run in-process at
//!   every width (`Scalar`, `Simd4` and `Simd8`), compared against the pins.
//! * **wasm** — the same crate compiled to `wasm32-unknown-unknown` with `simd128`, the one wasm
//!   build that ships, and executed under wasmtime, compared against the same pins at the widths
//!   that build has: `Scalar` and `Simd4`. The browser runs four lanes only, and since #1110 the
//!   wasm build has no eight-lane type to digest. The scalar (non-`simd128`) wasm leg retired with
//!   #1062: `lane` refuses that build.
//!
//! The runtime is configured to *reject* relaxed SIMD (`Config::wasm_relaxed_simd(false)`), so a
//! guest built with `-C target-feature=+relaxed-simd` that actually emits a relaxed instruction
//! fails module validation instead of quietly returning a different digest. Master plan D3 forbids
//! those instructions; this is where that is enforced against a built artifact and not a grep.

use std::fmt;
use std::path::Path;

use wasm_gate_corpus as corpus;
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

/// Which production backend a wasm guest is expected to have used.
///
/// No `Scalar`: no target selects the whole-plan scalar backend, and the wasm build without
/// `simd128` that once reported it is refused at compile time (#1062). No `Simd8`: the wasm build
/// has one width, and the eight-lane measurement cfg that once reported it is gone (#1038).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedBackend {
    /// `Simd4`: wasm with `simd128`.
    Simd4,
}

impl ExpectedBackend {
    /// Parses the `--expect-backend` argument.
    ///
    /// # Errors
    ///
    /// Returns the offending text if it names no backend.
    pub fn parse(text: &str) -> Result<Self, String> {
        match text {
            "simd4" => Ok(Self::Simd4),
            other => Err(other.to_string()),
        }
    }

    /// The code the guest's `miso_gate_backend` export reports.
    #[must_use]
    pub const fn code(self) -> u32 {
        match self {
            Self::Simd4 => 1,
        }
    }

    /// How many widths a guest of this backend digests every lane case at, as the guest's
    /// `miso_gate_widths` export reports it: the scalar oracle and the backend's own width.
    ///
    /// Pinned rather than read from the guest, so a guest that stopped digesting its own width
    /// fails the leg instead of passing on the scalar run alone. It is not the host's
    /// `wasm_gate_corpus::WIDTHS`: the host also has `Simd8`, which the wasm build does not
    /// (issue #1110).
    #[must_use]
    pub const fn widths(self) -> usize {
        match self {
            Self::Simd4 => 2,
        }
    }

    /// Name used in reports.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Simd4 => "simd4",
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
    /// `wasm_gate_corpus::minmax_lowering_mismatches`).
    pub minmax_lowering_mismatches: u32,
    /// Lanes on which this leg's `f64` lanes (`lane::LaneF64`, `lane::Widen`) disagreed with the
    /// scalar `f64` oracle, summed over every width. Anything but zero fails the leg.
    ///
    /// Not a digest and not pinned, for the reason `minmax_lowering_mismatches` is not: it is the
    /// wasm execution of issue #949's exactness differential (see
    /// `wasm_gate_corpus::f64_lane_mismatches`).
    pub f64_lane_mismatches: u32,
    /// Lane fields on which this leg's full meter block pass (`lane::kernels::builtins::meter_block`)
    /// disagreed with the builtin meter's scalar loop, summed over every width. Anything but zero
    /// fails the leg.
    ///
    /// Not a digest and not pinned, for the same reason: it is the wasm execution of issue #950's
    /// kernel differential (see `wasm_gate_corpus::meter_block_mismatches`).
    pub meter_block_mismatches: u32,
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
            "{{\"schema_version\":1,\"kind\":\"wasm_gates\",\"leg\":\"{}\",\"runtime\":\"wasmtime {}\",\"backend\":{},\"cases\":{},\"comparisons\":{},\"minmax_lowering_mismatches\":{},\"f64_lane_mismatches\":{},\"meter_block_mismatches\":{},\"mismatches\":[{}]}}",
            self.leg,
            WASMTIME_VERSION,
            self.backend,
            self.cases,
            self.comparisons,
            self.minmax_lowering_mismatches,
            self.f64_lane_mismatches,
            self.meter_block_mismatches,
            mismatches.join(",")
        )
    }
}

/// Lowercase hexadecimal of a digest.
#[must_use]
pub fn hex(bytes: &[u8; 32]) -> String {
    bench_support::digest::hex(bytes)
}

/// The widths a case is compared at on a leg that has `widths` of them: every width for a case with
/// a lane instantiation, the scalar run alone for a math case, whose functions have none.
fn widths_of(case: usize, widths: usize) -> std::ops::Range<usize> {
    if corpus::is_width_dependent(case) {
        0..widths
    } else {
        0..1
    }
}

/// The backend this process was compiled for, in the guest's numbering.
fn native_backend_code() -> u32 {
    // By width, not by variant: `Backend::Scalar` exists only in `lane/test-support` builds
    // (#1059), so a match on the variants compiles in only one of them.
    match lane::Backend::current().width() {
        4 => 1,
        8 => 2,
        _ => 0,
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
        for width in widths_of(case, corpus::WIDTHS) {
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
        f64_lane_mismatches: (0..corpus::WIDTHS).map(corpus::f64_lane_mismatches).sum(),
        meter_block_mismatches: (0..corpus::WIDTHS)
            .map(corpus::meter_block_mismatches)
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
    /// `miso_gate_f64_lane_mismatches(width) -> u32`.
    f64_lane_mismatches: TypedFunc<u32, u32>,
    /// `miso_gate_meter_block_mismatches(width) -> u32`.
    meter_block_mismatches: TypedFunc<u32, u32>,
    /// What `miso_gate_backend()` reported.
    backend: u32,
    /// What `miso_gate_case_count()` reported.
    cases: usize,
    /// What `miso_gate_widths()` reported: the widths this guest digests at.
    widths: usize,
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
        let f64_lane_mismatches: TypedFunc<u32, u32> =
            instance.get_typed_func(&mut store, "miso_gate_f64_lane_mismatches")?;
        let meter_block_mismatches: TypedFunc<u32, u32> =
            instance.get_typed_func(&mut store, "miso_gate_meter_block_mismatches")?;

        let backend = backend.call(&mut store, ())?;
        let cases = case_count.call(&mut store, ())? as usize;
        let guest_widths = widths.call(&mut store, ())? as usize;
        // The width count is the guest's own (no `Simd8` on wasm32, issue #1110), so only the
        // case list is compared here; `wasm_report` holds the widths to the expected backend's.
        if cases != corpus::CASE_COUNT || guest_widths > corpus::WIDTHS {
            wasmtime::bail!(
                "guest corpus shape ({cases} cases, {guest_widths} widths) differs from this \
                 host's ({} cases, at most {} widths): the two were built from different sources",
                corpus::CASE_COUNT,
                corpus::WIDTHS
            );
        }

        Ok(Self {
            store,
            digest_word,
            minmax_lowering_mismatches,
            f64_lane_mismatches,
            meter_block_mismatches,
            backend,
            cases,
            widths: guest_widths,
        })
    }

    /// Runs the `max`/`min` lowering truth table inside the guest, at every width it has.
    fn minmax_lowering_mismatches(&mut self) -> wasmtime::Result<u32> {
        let mut total = 0;
        for width in 0..self.widths {
            total += self
                .minmax_lowering_mismatches
                .call(&mut self.store, width as u32)?;
        }
        Ok(total)
    }

    /// Runs the `f64` lane exactness differential inside the guest, at every width it has.
    fn f64_lane_mismatches(&mut self) -> wasmtime::Result<u32> {
        let mut total = 0;
        for width in 0..self.widths {
            total += self
                .f64_lane_mismatches
                .call(&mut self.store, width as u32)?;
        }
        Ok(total)
    }

    /// Runs the full meter block pass differential inside the guest, at every width it has.
    fn meter_block_mismatches(&mut self) -> wasmtime::Result<u32> {
        let mut total = 0;
        for width in 0..self.widths {
            total += self
                .meter_block_mismatches
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
/// the backend it reports, or the number of widths it digests at, is not `expected`'s.
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
    if guest.widths != expected.widths() {
        wasmtime::bail!(
            "guest digests at {} widths but a {} guest has {}: the scalar oracle and its own",
            guest.widths,
            expected.name(),
            expected.widths()
        );
    }

    let mut mismatches = Vec::new();
    let mut comparisons = 0;
    for case in 0..guest.cases {
        let expected_digest = corpus::expected_digest(case);
        for width in widths_of(case, guest.widths) {
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
    let f64_lane_mismatches = guest.f64_lane_mismatches()?;
    let meter_block_mismatches = guest.meter_block_mismatches()?;

    Ok(Report {
        leg: "wasm",
        backend: guest.backend,
        cases: guest.cases,
        comparisons,
        mismatches,
        minmax_lowering_mismatches,
        f64_lane_mismatches,
        meter_block_mismatches,
    })
}

/// Regenerates the lane pins from the scalar `Lane` oracle, in the include-file form.
///
/// Master plan §8: a pin comes from the oracle, never from copying whatever the production path
/// currently prints. Only width 0 — the scalar implementation — is read here, and the gate then
/// requires `Simd4` on every target, and `Simd8` on every native one, to reproduce it.
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
