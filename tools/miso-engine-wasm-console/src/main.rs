//! Host half of the issue #163 phase 2 step 1 wasm console arm.
//!
//! # What this measures
//!
//! The nine console workloads -- the real 64-track fixture and its decomposition rows, through a
//! real `PreparedRenderPlan` -- rendered on three legs whose only difference is *what executes the
//! code*:
//!
//! * `native_simd8`, the production backend every recorded console number was taken at;
//! * `native_simd4`, the same source at the lane width `simd128` offers;
//! * `wasm_simd128`, the same source compiled to `wasm32-unknown-unknown` with `+simd128` and
//!   executed under the pinned wasmtime.
//!
//! All three drive [`miso_engine_console_workload`]. Not a port of it, not a stand-in for it: the
//! same crate, linked natively here and compiled to wasm in the guest. That is the only condition
//! under which a ratio between two of these legs is a statement about a target rather than about a
//! transcription.
//!
//! # Why all three legs live in one process
//!
//! Because the *ratio* is the deliverable, and a ratio between two numbers taken minutes apart is
//! partly a measurement of the minutes. #104's paired alternation: the three legs are interleaved
//! **observation by observation**, so every drift the run suffers -- a governor ramp, a tenant
//! landing on the sibling core, a thermal limit engaging -- is shared by all three, and the
//! per-observation ratio is a distribution rather than a quotient of two summaries.
//!
//! This is the discipline the console benchmark's own facility arms already use, applied across
//! targets instead of across plan configurations.
//!
//! # What the host-side clock can and cannot attribute
//!
//! Stated plainly, because the guest has no clock and everything below follows from that.
//!
//! **What it attributes.** The interval measured around a `wasm_simd128` observation is one
//! `Instance::call` into `miso_console_render` and its return. Inside it is exactly one call to
//! the production render entry on a prepared plan, which is exactly what the native legs' timed
//! region contains. Rendering and hashing are separate exports, so the digest update the native
//! subject keeps outside its clock is outside this one too -- structurally, not by assertion,
//! which is what replaces `timing::timed`'s #104 F1 guard across the ABI boundary.
//!
//! **What it does not attribute.** The wasm leg's interval also contains one host-to-guest
//! crossing that the native legs do not have: wasmtime's trampoline, the stack switch and the
//! return. That cost is *inside* every reported `wasm_simd128` number and is not subtracted from
//! it. It is measured instead: [`Guest::crossing`] times the same crossing into an export that
//! renders nothing, and every record carries the result as `guest_call_overhead_p50_ns`. A reader
//! who wants the crossing removed can remove it; this arm does not remove it silently, and does
//! not pretend it is zero.
//!
//! **What it cannot attribute at all.** Nothing inside the guest is separable from anything else
//! inside the guest. The audit counter that gives the native legs
//! `render_total_forbidden_operations` counts allocations on *this* process's heap; the guest
//! allocates inside its own linear memory, which that counter cannot see. The records say so
//! rather than reporting a zero that would look like a finding.
//!
//! # This is not a browser measurement
//!
//! wasmtime with Cranelift is an ahead-of-time compiler: a module is fully compiled at
//! `Module::from_file` and the code that runs at observation 1 is the code that runs at
//! observation 1000. A browser JIT is not that. It tiers, it deoptimises, it recompiles on
//! feedback, and it does so on a different microarchitecture inside a different thermal envelope.
//! These records are the *determinism-pinned reference*: same source, same runtime version, same
//! host, one target difference. The browser numbers remain the owner's field pass, and the record
//! family says so in a field rather than only in prose.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_bench_support::digest::{Sha256Sink, sha256_hex};
use miso_engine_bench_support::json::escape as json_escape;
use miso_engine_bench_support::metadata::Metadata;
use miso_engine_bench_support::stats;
use miso_engine_bench_support::timing;
use miso_engine_console_workload::{
    QUANTUM, SAMPLE_RATE_HZ, SessionRuntime, WORKLOADS, Workload, source_block,
};
use miso_engine_core::realtime::audit;
use miso_engine_lane::Backend;
use wasmtime::{Config, Engine, Instance, Module, Store, TypedFunc};

/// Timed observations per leg per workload. The console benchmark's count, unchanged.
const OBSERVATIONS: usize = 1_000;
/// Issue this arm belongs to.
const ISSUE: u32 = 163;
/// Untimed blocks every leg renders before the clock starts.
///
/// The native console runner warms only the idle row, because only that row has a decay to settle.
/// This arm warms every leg of every row by at least this much for a different reason: three legs
/// are interleaved, and a leg paying first-touch inside the clock would charge it to the *ratio*.
/// The idle row still gets its full settling period on top, through
/// [`Workload::warmup_blocks`].
const WARMUP_BLOCKS: usize = 64;
/// Crossing-cost observations taken per workload, outside the reported per-block samples.
const CROSSING_OBSERVATIONS: usize = 1_000;

fn main() -> ExitCode {
    // #104 F4: prove the shared audited allocator is the one serving this process.
    bench_alloc::assert_installed();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Parsed command line: the guest module to drive.
struct Invocation {
    guest: PathBuf,
}

fn usage() -> String {
    "usage: miso_engine_wasm_console <guest.wasm>\n\
     the round marker and host metadata arrive in the environment, from the fixed runner"
        .to_string()
}

fn parse_arguments() -> Result<Invocation, String> {
    let mut arguments = std::env::args_os().skip(1);
    let guest = arguments.next().ok_or_else(usage)?;
    if arguments.next().is_some() {
        return Err(usage());
    }
    Ok(Invocation {
        guest: PathBuf::from(guest),
    })
}

fn run() -> Result<(), String> {
    let invocation = parse_arguments()?;
    let round = round_from_runner()?;
    let metadata = Metadata::gather();

    // The native leg must be the backend the console records were taken at, or the ratio table
    // silently compares wasm against something other than the production native backend.
    let native = Backend::current();
    if native != Backend::Simd8 {
        return Err(format!(
            "the native leg of this arm must run at Simd8, the backend every recorded console \
             number was taken at; this build detected {}",
            backend_name(native)
        ));
    }

    let module_bytes = std::fs::read(&invocation.guest)
        .map_err(|error| format!("reading {}: {error}", invocation.guest.display()))?;
    let module_sha256 = sha256_hex(&module_bytes);
    let (engine, module) = Guest::compile(&invocation.guest)?;

    for workload in WORKLOADS {
        let measurement = WorkloadMeasurement::run(workload, &engine, &module)?;
        println!(
            "{}",
            measurement.record(workload, round, &module_sha256, metadata)
        );
    }
    Ok(())
}

/// The round marker, supplied by the fixed runner and never defaulted.
fn round_from_runner() -> Result<u32, String> {
    match Metadata::gather().var("MISO_ENGINE_BENCH_ROUND").as_deref() {
        Ok("warmup") => Ok(0),
        Ok("1") => Ok(1),
        Ok("2") => Ok(2),
        _ => Err("the wasm console arm must be launched by its fixed runner".to_string()),
    }
}

fn backend_name(backend: Backend) -> &'static str {
    match backend {
        Backend::Scalar => "Scalar",
        Backend::Simd4 => "Simd4",
        Backend::Simd8 => "Simd8",
    }
}

/// The three legs of the comparison, in the order they are rendered within one observation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Leg {
    /// The production native backend: the same source, dispatched at `Simd8`.
    NativeSimd8,
    /// The same source dispatched at `Simd4`, the width `simd128` offers.
    NativeSimd4,
    /// The same source compiled for `wasm32-unknown-unknown` with `+simd128`.
    WasmSimd128,
}

/// The legs in emission order. Fixed, and fixed deliberately -- see [`WorkloadMeasurement::run`].
const LEGS: [Leg; 3] = [Leg::NativeSimd8, Leg::NativeSimd4, Leg::WasmSimd128];

impl Leg {
    const fn name(self) -> &'static str {
        match self {
            Self::NativeSimd8 => "native_simd8",
            Self::NativeSimd4 => "native_simd4",
            Self::WasmSimd128 => "wasm_simd128",
        }
    }
    const fn backend(self) -> &'static str {
        match self {
            Self::NativeSimd8 => "Simd8",
            Self::NativeSimd4 | Self::WasmSimd128 => "Simd4",
        }
    }
    const fn target(self) -> &'static str {
        match self {
            Self::NativeSimd8 | Self::NativeSimd4 => "native",
            Self::WasmSimd128 => "wasm32-unknown-unknown",
        }
    }
    /// Whether this process's allocation audit can see what the leg did.
    ///
    /// It cannot see inside the guest: a wasm module allocates in its own linear memory, and a
    /// zero from this process's counter would describe the host, not the guest.
    const fn audit_scope(self) -> &'static str {
        match self {
            Self::NativeSimd8 | Self::NativeSimd4 => "host_process_heap",
            Self::WasmSimd128 => "not_observable_guest_linear_memory",
        }
    }
}

/// The guest module, instantiated, with its exports resolved once.
struct Guest {
    store: Store<()>,
    reset_source: TypedFunc<(), u32>,
    push_source_word: TypedFunc<u32, u32>,
    staged_source_len: TypedFunc<(), u32>,
    prepare: TypedFunc<u32, u32>,
    render: TypedFunc<u32, u32>,
    hash_output: TypedFunc<(), u32>,
    finish_digest: TypedFunc<(), u32>,
    digest_word: TypedFunc<u32, u32>,
    render_errors: TypedFunc<(), u32>,
    backend: TypedFunc<(), u32>,
}

impl Guest {
    /// Compiles the guest module once, for the whole run.
    ///
    /// Compilation is Cranelift's whole job here and it is not cheap on a module this size. It is
    /// also not part of anything measured: doing it once and instantiating per workload keeps the
    /// nine rows from each paying for it, and -- more importantly -- guarantees every row was
    /// timed against *the same machine code*.
    fn compile(path: &Path) -> Result<(Engine, Module), String> {
        let mut config = Config::new();
        // `simd128` is the artifact the browser ships. Relaxed SIMD is out of scope for this
        // engine (spec 024/074) and is switched off so a guest that emitted one would fail to
        // instantiate rather than be timed and reported as a `simd128` number.
        config.wasm_simd(true);
        config.wasm_relaxed_simd(false);
        // Threads and shared memory are not merely disabled but absent: this crate pins wasmtime
        // with `default-features = false` and only `runtime`/`cranelift`/`std`, so the proposal is
        // not compiled into the runtime at all. That matches the atomics-free law the product
        // ships under, and it is why there is no `wasm_threads(false)` call to make here.
        let engine = Engine::new(&config).map_err(|error| format!("wasmtime engine: {error}"))?;
        let module = Module::from_file(&engine, path)
            .map_err(|error| format!("compiling {}: {error}", path.display()))?;
        Ok((engine, module))
    }

    /// Instantiates a fresh guest, and checks it is the artifact this arm claims.
    ///
    /// One instance per workload, never reused: a workload's prepared plan, its staged input table
    /// and its running digest all live in the guest's linear memory, and the cleanest way to be
    /// sure one row cannot inherit another's state is to give each row a new one.
    fn instantiate(engine: &Engine, module: &Module) -> Result<Self, String> {
        let mut store = Store::new(engine, ());
        // No imports: the guest cannot reach the host, the clock, or anything outside itself.
        let instance = Instance::new(&mut store, module, &[])
            .map_err(|error| format!("instantiating the guest module: {error}"))?;

        let export = |store: &mut Store<()>, name: &str| -> Result<(), String> {
            instance
                .get_func(&mut *store, name)
                .map(|_| ())
                .ok_or_else(|| format!("guest module exports no {name}"))
        };
        for name in [
            "miso_console_backend",
            "miso_console_workload_count",
            "miso_console_reset_source",
            "miso_console_push_source_word",
            "miso_console_staged_source_len",
            "miso_console_prepare",
            "miso_console_render",
            "miso_console_hash_output",
            "miso_console_finish_digest",
            "miso_console_digest_word",
            "miso_console_render_errors",
        ] {
            export(&mut store, name)?;
        }
        let typed = |store: &mut Store<()>, name: &str| {
            instance
                .get_typed_func::<u32, u32>(&mut *store, name)
                .map_err(|error| format!("{name}: {error}"))
        };
        let typed_unit = |store: &mut Store<()>, name: &str| {
            instance
                .get_typed_func::<(), u32>(&mut *store, name)
                .map_err(|error| format!("{name}: {error}"))
        };

        let backend = typed_unit(&mut store, "miso_console_backend")?;
        let reset_source = typed_unit(&mut store, "miso_console_reset_source")?;
        let push_source_word = typed(&mut store, "miso_console_push_source_word")?;
        let staged_source_len = typed_unit(&mut store, "miso_console_staged_source_len")?;
        let prepare = typed(&mut store, "miso_console_prepare")?;
        let workload_count = typed_unit(&mut store, "miso_console_workload_count")?;
        let render = typed(&mut store, "miso_console_render")?;
        let hash_output = typed_unit(&mut store, "miso_console_hash_output")?;
        let finish_digest = typed_unit(&mut store, "miso_console_finish_digest")?;
        let digest_word = typed(&mut store, "miso_console_digest_word")?;
        let render_errors = typed_unit(&mut store, "miso_console_render_errors")?;

        let guest_backend = backend
            .call(&mut store, ())
            .map_err(|error| format!("miso_console_backend: {error}"))?;
        if guest_backend != 1 {
            return Err(format!(
                "the guest reports backend {guest_backend}, not Simd4: it was built without \
                 `+simd128` and must not be reported as a simd128 measurement"
            ));
        }
        let guest_workloads = workload_count
            .call(&mut store, ())
            .map_err(|error| format!("miso_console_workload_count: {error}"))?
            as usize;
        if guest_workloads != WORKLOADS.len() {
            return Err(format!(
                "the guest carries {guest_workloads} workloads and this host {}: the two were \
                 built from different sources",
                WORKLOADS.len()
            ));
        }
        Ok(Self {
            store,
            reset_source,
            push_source_word,
            staged_source_len,
            prepare,
            render,
            hash_output,
            finish_digest,
            digest_word,
            render_errors,
            backend,
        })
    }

    /// Stages this host's input samples inside the guest, then prepares one workload from them.
    ///
    /// Untimed on both counts: the staging crossings and the four real compilers all run before
    /// any clock is held. The samples come from [`miso_engine_console_workload::source_block`],
    /// evaluated *here*, so the guest renders the identical input the native legs render -- see
    /// that function for why the tone cannot simply be recomputed on the other side.
    fn prepare(&mut self, workload: Workload, index: usize) -> Result<(), String> {
        self.reset_source
            .call(&mut self.store, ())
            .map_err(|error| format!("miso_console_reset_source: {error}"))?;
        let silent = workload.input_signal() == "silence";
        let mut staged = 0_u32;
        for track in 0..workload.tracks() as usize {
            for value in source_block(track, silent) {
                self.push_source_word
                    .call(&mut self.store, value.to_bits())
                    .map_err(|error| format!("miso_console_push_source_word: {error}"))?;
                staged += 1;
            }
        }
        let guest_staged = self
            .staged_source_len
            .call(&mut self.store, ())
            .map_err(|error| format!("miso_console_staged_source_len: {error}"))?;
        if guest_staged != staged {
            return Err(format!(
                "staged {staged} input samples but the guest holds {guest_staged}"
            ));
        }
        let ok = self
            .prepare
            .call(&mut self.store, index as u32)
            .map_err(|error| format!("miso_console_prepare: {error}"))?;
        if ok != 1 {
            return Err(format!("the guest refused to prepare workload {index}"));
        }
        Ok(())
    }

    /// Renders one block. The only guest call a clock is held around.
    fn render(&mut self, observation: u32) -> Result<bool, String> {
        self.render
            .call(&mut self.store, observation)
            .map(|ok| ok == 1)
            .map_err(|error| format!("miso_console_render: {error}"))
    }

    /// Folds the rendered block into the guest's digest. Never inside a timed region.
    fn hash_output(&mut self) -> Result<(), String> {
        let ok = self
            .hash_output
            .call(&mut self.store, ())
            .map_err(|error| format!("miso_console_hash_output: {error}"))?;
        if ok != 1 {
            return Err("the guest refused to hash its output".to_string());
        }
        Ok(())
    }

    /// The guest's finished output digest, as lowercase hex.
    fn digest_hex(&mut self) -> Result<String, String> {
        let ok = self
            .finish_digest
            .call(&mut self.store, ())
            .map_err(|error| format!("miso_console_finish_digest: {error}"))?;
        if ok != 1 {
            return Err("the guest refused to finish its digest".to_string());
        }
        let mut hex = String::with_capacity(64);
        for index in 0..8 {
            let word = self
                .digest_word
                .call(&mut self.store, index)
                .map_err(|error| format!("miso_console_digest_word: {error}"))?;
            hex.push_str(&format!("{word:08x}"));
        }
        Ok(hex)
    }

    /// Blocks the guest's plan refused.
    fn errors(&mut self) -> Result<u64, String> {
        self.render_errors
            .call(&mut self.store, ())
            .map(u64::from)
            .map_err(|error| format!("miso_console_render_errors: {error}"))
    }

    /// One host-to-guest crossing into an export that renders nothing.
    ///
    /// `miso_console_backend` reads a compile-time constant and returns. Timing it measures the
    /// trampoline, the stack switch and the return -- the part of every `wasm_simd128` observation
    /// that the native legs do not pay and that this arm does not subtract silently.
    fn crossing(&mut self) -> Result<(), String> {
        self.backend
            .call(&mut self.store, ())
            .map(|_| ())
            .map_err(|error| format!("miso_console_backend: {error}"))
    }
}

/// One leg's per-block samples and the identity of what it rendered.
struct LegSamples {
    ns_per_block: Vec<u64>,
    output_sha256: String,
    render_errors: u64,
}

/// One workload measured on all three legs, paired observation by observation.
struct WorkloadMeasurement {
    legs: Vec<LegSamples>,
    crossing_ns: Vec<u64>,
    audit: audit::AuditSnapshot,
}

impl WorkloadMeasurement {
    fn run(workload: Workload, engine: &Engine, module: &Module) -> Result<Self, String> {
        let mut native8 = SessionRuntime::new(workload);
        let mut native4 = SessionRuntime::build_with_dispatch(
            workload,
            miso_engine_console_workload::PlanConfig::BASELINE,
            Backend::Simd4,
        );
        let mut guest = Guest::instantiate(engine, module)?;
        let index = WORKLOADS
            .iter()
            .position(|candidate| *candidate == workload)
            .ok_or_else(|| "workload is not in the shared emission order".to_string())?;
        guest.prepare(workload, index)?;

        // Untimed settling, every leg equally. The idle row asks for a lot; see
        // `Workload::warmup_blocks`.
        let warmup = workload.warmup_blocks().max(WARMUP_BLOCKS);
        for observation in 0..warmup {
            let _ = native8.render(observation as u64);
            let _ = native4.render(observation as u64);
            guest.render(observation as u32)?;
        }

        let mut samples: Vec<Vec<u64>> = LEGS
            .iter()
            .map(|_| Vec::with_capacity(OBSERVATIONS))
            .collect();
        let mut hashes: Vec<Sha256Sink> = LEGS.iter().map(|_| Sha256Sink::new()).collect();
        let mut errors = [0_u64; 3];

        audit::warm_up();
        audit::reset();
        // Paired alternation (#104): one observation renders all three legs before any leg renders
        // its next. The leg *order* within an observation is fixed rather than rotated, which is
        // what the console benchmark's own facility arms do; the consequence is that leg 0 meets a
        // slightly different cache state than leg 2 does, and that residue is inside the ratio.
        // It is bounded by the native pair: `native_simd8` and `native_simd4` are the same code on
        // the same target in positions 0 and 1, so whatever position costs, it cannot exceed what
        // separates those two beyond their width difference.
        for observation in 0..OBSERVATIONS {
            let (native8_ns, native8_result) = timing::timed(|| native8.render(observation as u64));
            let (native4_ns, native4_result) = timing::timed(|| native4.render(observation as u64));
            let (wasm_ns, wasm_result) = timing::timed(|| guest.render(observation as u32));
            let wasm_ok = wasm_result?;

            if native8_result.is_err() {
                errors[0] += 1;
            }
            if native4_result.is_err() {
                errors[1] += 1;
            }
            if !wasm_ok {
                errors[2] += 1;
            }
            samples[0].push(native8_ns);
            samples[1].push(native4_ns);
            samples[2].push(wasm_ns);

            // Every evidence step is outside the clock and after the whole round-robin, so one
            // leg's bookkeeping never lands between another leg's two timed blocks.
            native8.hash_output(&mut hashes[0]);
            native4.hash_output(&mut hashes[1]);
            guest.hash_output()?;
        }
        let snapshot = audit::snapshot();

        // The crossing cost, measured after the reported samples so it cannot perturb them.
        let mut crossing_ns = Vec::with_capacity(CROSSING_OBSERVATIONS);
        for _ in 0..CROSSING_OBSERVATIONS {
            let (elapsed, result) = timing::timed(|| guest.crossing());
            result?;
            crossing_ns.push(elapsed);
        }

        let guest_digest = guest.digest_hex()?;
        let guest_errors = guest.errors()?;
        let mut digests: Vec<String> = hashes.into_iter().map(Sha256Sink::finish_hex).collect();
        digests[2] = guest_digest;
        errors[2] = errors[2].max(guest_errors);

        let legs = LEGS
            .iter()
            .enumerate()
            .map(|(index, _)| LegSamples {
                ns_per_block: std::mem::take(&mut samples[index]),
                output_sha256: digests[index].clone(),
                render_errors: errors[index],
            })
            .collect();

        Ok(Self {
            legs,
            crossing_ns,
            audit: snapshot,
        })
    }

    /// `true` when every leg rendered byte-identical output.
    fn digests_agree(&self) -> bool {
        self.legs
            .iter()
            .all(|leg| leg.output_sha256 == self.legs[0].output_sha256)
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        module_sha256: &str,
        metadata: &Metadata,
    ) -> String {
        let tracks = f64::from(workload.tracks());
        let legs = LEGS
            .iter()
            .zip(&self.legs)
            .map(|(leg, samples)| {
                let percentiles = Percentiles::from_samples(&samples.ns_per_block);
                format!(
                    concat!(
                        "{{\"leg\":\"{name}\",\"target\":\"{target}\",\"backend\":\"{backend}\",",
                        "\"min_ns_per_block\":{min_ns},\"p50_ns_per_block\":{p50_ns},",
                        "\"p95_ns_per_block\":{p95_ns},\"p99_ns_per_block\":{p99_ns},",
                        "\"max_ns_per_block\":{max_ns},\"p50_us_per_block\":{p50_us},",
                        "\"p50_us_per_block_per_track\":{per_track},",
                        "\"output_sha256\":\"{digest}\",\"render_errors\":{errors},",
                        "\"audit_scope\":\"{scope}\"}}"
                    ),
                    name = leg.name(),
                    target = leg.target(),
                    backend = leg.backend(),
                    min_ns = percentiles.min,
                    p50_ns = percentiles.p50,
                    p95_ns = percentiles.p95,
                    p99_ns = percentiles.p99,
                    max_ns = percentiles.max,
                    p50_us = microseconds(percentiles.p50),
                    per_track = format_f64(percentiles.p50 as f64 / 1_000.0 / tracks),
                    digest = samples.output_sha256,
                    errors = samples.render_errors,
                    scope = leg.audit_scope(),
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        let ratios = [(2_usize, 0_usize), (2, 1)]
            .iter()
            .map(|(numerator, denominator)| {
                let top = &self.legs[*numerator];
                let bottom = &self.legs[*denominator];
                let top_p50 = Percentiles::from_samples(&top.ns_per_block).p50;
                let bottom_p50 = Percentiles::from_samples(&bottom.ns_per_block).p50;
                format!(
                    concat!(
                        "{{\"numerator\":\"{top}\",\"denominator\":\"{bottom}\",",
                        "\"ratio_of_p50\":{ratio},\"paired_ratio_median\":{paired}}}"
                    ),
                    top = LEGS[*numerator].name(),
                    bottom = LEGS[*denominator].name(),
                    ratio = format_f64(top_p50 as f64 / bottom_p50 as f64),
                    paired =
                        format_f64(paired_ratio_median(&top.ns_per_block, &bottom.ns_per_block)),
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        let crossing = Percentiles::from_samples(&self.crossing_ns);
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"phase\":\"2-step1\",",
                "\"record\":\"wasm_console_session\",\"workload_kind\":\"{kind}\",",
                "\"tracks\":{tracks},\"synthetic_fixture\":{synthetic},",
                "\"strip_content\":\"{strip}\",\"strip_layout\":\"{layout}\",",
                "\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},",
                "\"observations\":{obs},\"warmup_blocks\":{warmup},",
                "\"units\":\"us_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"runtime\":\"wasmtime {runtime}\",\"guest_target\":\"wasm32-unknown-unknown\",",
                "\"guest_target_features\":\"+simd128\",",
                "\"guest_module_sha256\":\"{module}\",",
                "\"guest_call_overhead_p50_ns\":{crossing},",
                "\"legs\":[{legs}],\"ratios\":[{ratios}],",
                "\"digest_identity\":\"{identity}\",",
                "\"render_total_forbidden_operations\":{forbidden},",
                "\"comparable_with_console_records\":false,",
                "\"browser_field_measurement\":false,",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"nearest-rank percentiles over per-block nanoseconds; ",
                "three legs interleaved observation by observation (#104); ratios reported both ",
                "as a quotient of per-leg p50 and as the median of the per-observation quotient; ",
                "one warmup pass and two measured rounds; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            layout = workload.strip_layout(),
            signal = workload.input_signal(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = OBSERVATIONS,
            warmup = workload.warmup_blocks().max(WARMUP_BLOCKS),
            runtime = wasmtime_version(),
            module = module_sha256,
            crossing = crossing.p50,
            legs = legs,
            ratios = ratios,
            identity = if self.digests_agree() {
                "all_legs_identical"
            } else {
                "divergent"
            },
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
        )
    }
}

/// The pinned wasmtime version, from the dependency this crate compiled against.
fn wasmtime_version() -> &'static str {
    "47.0.3"
}

/// Median of the per-observation quotient of two equally long sample vectors.
///
/// The paired statistic, and the reason all three legs render inside one observation: this is a
/// distribution over a thousand paired quotients rather than a quotient of two summaries, so a
/// drift that moved both legs together does not move it.
fn paired_ratio_median(numerator: &[u64], denominator: &[u64]) -> f64 {
    assert_eq!(
        numerator.len(),
        denominator.len(),
        "paired legs must have equal observation counts"
    );
    let mut quotients: Vec<f64> = numerator
        .iter()
        .zip(denominator)
        .filter(|(_, bottom)| **bottom > 0)
        .map(|(top, bottom)| *top as f64 / *bottom as f64)
        .collect();
    quotients.sort_by(f64::total_cmp);
    assert!(!quotients.is_empty(), "paired observations");
    stats::nearest_rank(&quotients, 50, 100)
}

/// Nearest-rank percentiles over one leg's per-block nanoseconds.
struct Percentiles {
    min: u64,
    p50: u64,
    p95: u64,
    p99: u64,
    max: u64,
}

impl Percentiles {
    fn from_samples(samples: &[u64]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_unstable();
        assert!(!sorted.is_empty(), "measured observations");
        let rank = |numerator: usize, denominator: usize| {
            stats::nearest_rank(&sorted, numerator, denominator)
        };
        Self {
            min: sorted[0],
            p50: rank(50, 100),
            p95: rank(95, 100),
            p99: rank(99, 100),
            max: *sorted.last().expect("nonempty"),
        }
    }
}

fn microseconds(nanoseconds: u64) -> String {
    format_f64(nanoseconds as f64 / 1_000.0)
}

fn format_f64(value: f64) -> String {
    format!("{value:.3}")
}
