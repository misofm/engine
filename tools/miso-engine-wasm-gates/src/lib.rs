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
            "{{\"schema_version\":1,\"kind\":\"wasm_gates\",\"leg\":\"{}\",\"runtime\":\"wasmtime {}\",\"backend\":{},\"cases\":{},\"comparisons\":{},\"mismatches\":[{}]}}",
            self.leg,
            WASMTIME_VERSION,
            self.backend,
            self.cases,
            self.comparisons,
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

/// The widths a case is compared at: every width for a lane case, the scalar run alone for a math
/// case, whose functions have no lane instantiation.
fn widths_of(case: usize) -> std::ops::Range<usize> {
    if corpus::is_lane_case(case) {
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
    }
}

/// The guest exports this host drives.
struct Guest {
    /// Wasmtime store; the guest holds no host state, so its data is `()`.
    store: Store<()>,
    /// `miso_gate_digest_word(case, width, word) -> u32`.
    digest_word: TypedFunc<(u32, u32, u32), u32>,
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
            backend,
            cases,
        })
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

    Ok(Report {
        leg: "wasm",
        backend: guest.backend,
        cases: guest.cases,
        comparisons,
        mismatches,
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
