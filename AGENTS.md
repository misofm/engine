# Engine V2 agent guide

## Mission

`misofm/engine-v2` is a greenfield, headless Rust music-production engine for agents that work between musicians and the engine.  Agents require a deliberately broad, granular semantic API; the runtime ABI remains narrow, stable, and efficient.  Build for native/cloud embedding, iOS, Android, and browser WebAssembly.  PCM is the engine output; delivery codecs are external sidecars.

Never inspect, copy, benchmark against, or inherit an architecture from `misofm/engine` or any legacy/V1 source.  A useful idea may be independently re-derived and justified, but V2 is not a port.

## Product principles

- Sound quality and DSP correctness are first-class release criteria, not marketing language.  Prefer academically documented methods and verify them with fixtures and listening tests.
- Optimize ruthlessly but simply.  Ship a correct measurable implementation, record diminishing-return work as an issue, and reserve systematic optimization for the weekly performance pass.
- Make realtime processing allocation-free.  Allocate, parse, compile plans, perform I/O, decode, and load effects only on control/worker threads.
- Support arbitrary track counts constrained only by configured resources: never introduce a compiled `MAX_TRACKS`.
- Use SIMD at macro and micro levels: Wasm `simd128` and AArch64 NEON four-lane `f32` banks, AVX2 eight-lane specializations, and scalar/tail fallback; AVX2 and FMA require separate runtime feature dispatch.
- Sessions are strict, versioned TOML.  The control schema is expressive; session compilation is transactional.  Protocol mutations update the same typed session model and must be snapshot-able back to canonical TOML.
- Every Cargo package and crate directory uses the `miso-engine-` prefix.  Rust crate identifiers use the matching `miso_engine_` form; never introduce a bare `engine-*` package.

## Approved audio architecture

The realtime plane exclusively owns a preallocated `PreparedRenderPlan` at a caller-supplied explicit `u32` sample rate and render quantum.  Its graph, schedule, capacities, and parameter schema are structurally immutable while its preallocated DSP state is intentionally mutated by `render(&mut self, ...)`.  Render must perform **zero allocations/frees, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded calls**.  Every structural control-plane mutation produces and validates a replacement plan, then transfers ownership only at a documented block boundary.  The displaced plan goes to a bounded retirement queue and is reclaimed off the render thread; a full retirement queue defers the swap rather than dropping on render.

Tracks are dual-mono: left and right have independent state and parameters.  A processor that links channels must expose its detector-link mode.  Cross-channel operation uses an explicit smoothed 2x2 matrix; it is never implicit “stereo.”  “Builtins” is an umbrella over fixed input and output sections.  The required chain is:

`input -> polarity/trim/HPF/LPF -> SIMD rack 1 -> dynamic rack -> SIMD rack 2 -> fader/mute -> 2x2 matrix/pan -> routes`

Meters may observe any boundary without changing signal flow.  Send taps are explicit stable enum values: input, post-input-builtins, post-SIMD1, post-dynamic, post-SIMD2/pre-fader, post-fader, and post-matrix.  Graphs are typed and acyclic; compilation has deterministic topological ordering/reductions and exact integer-sample plugin-delay compensation (PDC), including sends and sidechains.  Prepared effect latency is fixed, and bypass must preserve it.  Feedback is a future capability and must have an explicit positive-latency edge.

Audio buffers are planar `f32`, banked AoSoA across tracks: at each sample, a vector contains the same dual-mono lane from four Wasm/NEON tracks or eight AVX2 tracks.  L and R use separate vectors and state.  A bank cohort shares an effect-program signature (slot types/order, quality, and compatible routing), while parameters/state remain per-track; absent slots are identity kernels and incompatible tracks form another cohort or take a documented scalar/dynamic fallback.  Scalar tails support every track count.  Base Wasm SIMD uses multiply plus add because fused multiply-add is not guaranteed; relaxed SIMD is optional and correctness must not depend on it.  Native launch may use preallocated deterministic dependency waves; browser launch is single render-thread unless a later platform-specific gate proves otherwise.

Multicore workers write disjoint outputs and reduce in stable node-ID order.  They are prestarted and cannot allocate, steal heap jobs, or make the render coordinator wait on an OS mutex.  Single-thread execution remains the correctness fallback.

Sources stream just-in-time: native WAV/RF64 decode workers fill bounded SPSC PCM rings, seeks are generation-tagged, and underrun emits zero plus a counter.  Browser/mobile hosts provide decoded chunks to equivalent bounded rings.  Never load whole stems solely to render a session.  Launch-supported session/render rates are exactly 44.1, 48, 88.2, and 96 kHz; 176.4, 192, 352.8, and 384 kHz are extended compatibility/research evidence only, not host or release support.  There is no implicit SRC in this sprint; host-rate mismatch is rejected or made explicit in a later plan.

## Effects and plugins

The initial native effect library is EQ, compressor, gate/expander, de-esser, true-peak limiter, dynamic EQ, multiband compressor, antialiased saturator/clipper, transient shaper, and dual-mono/stereo delay.  Every effect exposes stable numeric parameter IDs, units, domains, mappings, defaults, automation rate, smoothing, state, ports/sidechains, link modes, latency, tail, quality modes, reset semantics, and safe NaN/denormal behavior.  A native effect may run track-locally in the dynamic rack; SIMD-rack eligibility additionally requires its homogeneous bank kernel contract.

Third-party effects are designed now but do not execute at launch.  Sessions identify them by a deterministic CIDv1 over a canonical package byte sequence; a CID identifies exact bytes, not trust, quality, or cross-CPU bit identity.  Resolution, download, signature policy and cache verification happen off render, with no installation or licensing-dongle dependency; a public repository/resolver is future product scope.  Third-party core Wasm has no WASI/syscalls, bounded memory/state/scratch, declared latency/tail, and is compiled/validated off the render thread.  It is permitted only in the dynamic rack: opaque per-instance Wasm breaks the known homogeneous/fused SIMD bank contract.  A future executor must run it on sandbox workers through a bounded, at-least-one-quantum latency pipeline; the callback never invokes a general-purpose Wasm runtime and uses deterministic latency-preserving bypass on faults or late blocks.  Promotion to native requires audit, licensing, conformance, sound-quality, and performance evidence.

## Interfaces and transports

Expose a broad semantic control model and a narrow C ABI.  The control protocol is versioned, binary, transport-neutral, request-id/revision aware, uses absolute sample-time parameter events, and never transports PCM.  High-rate automation may use bounded point batches or step/linear/exponential segments.  Queue saturation returns typed backpressure; commands are never silently lost, while explicitly noncritical telemetry may be coalesced or dropped with counters.  In-process calls and shared/ring interfaces serve embedded hosts; a local sidecar may use local IPC; binary WebSocket is optional only at cloud/browser network boundaries, never in a render path.

## Realtime, quality, and research evidence

Every effect issue and implementation note must state: equations/algorithm; coefficient and update rules; numerical and stability limits; latency and tail; units and smoothing; denormal/NaN behavior; primary/official citations; fixtures; objective tests; benchmarks; and listening evidence.  Cite concise paraphrases, not copied prose.  Start with the repo research corpus and primary/official material, including:

- [RBJ Audio EQ Cookbook](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html)
- [Giannoulis, Massberg and Reiss, dynamic range compression](https://eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf)
- [ITU-R BS.1770-5](https://www.itu.int/rec/R-REC-BS.1770-5-202311-I) and [EBU R 128](https://tech.ebu.ch/publications/r128)
- Sophocles Orfanidis, *Introduction to Signal Processing*; P. P. Vaidyanathan, *Multirate Systems and Filter Banks*; and Julius O. Smith, *Spectral Audio Signal Processing*
- AES17 measurement methods and peer-reviewed antiderivative/oversampled antialiasing literature for nonlinear processors
- [Rust browser Wasm target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html), [Web Audio](https://www.w3.org/TR/webaudio-1.1/), and [WebAssembly core spec](https://webassembly.github.io/spec/core/)
- [Apple render-block guidance](https://developer.apple.com/documentation/audiotoolbox/auaudiounit/renderblock) and [Android low-latency guidance](https://developer.android.com/games/sdk/oboe/low-latency-audio)
- [CIDv1 specification](https://specs.ipfs.tech/cid/), [RFC 6455](https://www.rfc-editor.org/rfc/rfc6455.html), and [VST3 latency contract](https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html)

The corpus must also compare official routing/channel/bus/latency/automation documentation for at least three current large-format hardware consoles and two production DAWs.  DiGiCo, SSL, Lawo, Avid, Logic, and Ableton are candidate families, not designs to copy.  Record common patterns, conflicting choices, and the measured reason for every V2 adoption.

Use objective gates: allocation counters, deterministic fixtures, SIMD/scalar tolerances, PDC sample counts, memory ceilings independent of stem duration, cycle/CPU measurements, fault injection, and documented blinded listening evidence.  Subjective superlatives and unevidenced sound-quality claims are never acceptance gates.

## Issue-first execution and review workflow

Work only from a stateless issue body in `.github/ISSUE_SPECS/`; update its evidence/decision record as implementation learns facts.  Do not make cross-cutting architecture changes without a new or amended issue.

Create local Git checkpoint commits frequently at coherent, compiling or otherwise explicitly documented milestones so work is recoverable, and push those checkpoints to the configured upstream promptly when the user has authorized pushes.  A failed attempt may be committed when its evidence is candid and the tree is a useful checkpoint.  Never commit `target/`, fuzz artifacts, secrets, or unrelated generated output; do not rewrite or discard another agent's/user's history.

1. **Sol briefs** the issue and approves its scope, decision record, and objective gates.
2. **Terra implements attempt 1** and attaches required evidence.
3. **Sol adversarially reviews** the implementation against the issue, architecture, realtime rules, portability, and evidence.
4. If needed, **Sol performs up to two implementation/revision attempts**, each re-reviewed adversarially by Sol.
5. After three failed attempts total, stop.  Do not weaken gates to declare success.  Rescope/rebrief the issue, then restart the same workflow.

Research delegation should use Terra agents where possible to preserve Sol review capacity.  The workflow records who supplied evidence but never substitutes authority for testable gates.

### Delivery-control rules

GitHub state is part of the deliverable, not optional bookkeeping.  The local spec and its matching
GitHub issue must stay synchronized:

- When a local numbered issue spec is created, create the matching GitHub issue in the same
  checkpoint.  Confirm that its GitHub number and title match the local filename/index before
  starting implementation.
- After every pushed implementation checkpoint, add or refresh concise GitHub evidence when it
  materially changes the issue's status.  Never let a completed issue exist only as a local
  decision record.
- As soon as Sol records PASS and the evidence commit is upstream, close the GitHub issue in that
  same workflow.  Verify the remote state after closing it.  A task does not count as complete in
  progress reporting until GitHub is synchronized.
- At every issue boundary, compare `.github/ISSUE_SPECS/` with `gh issue list --state all` and fix
  missing, stale, or incorrectly closed entries before starting the next issue.

Keep feature issues small enough to ship.  A feature issue owns the minimum implementation and
evidence needed to prove its product contract; generic harness hardening, artifact promotion,
extended compatibility research, and human scheduling belong in separate stateless issues.  Split
the work before implementation when any of these are true:

- the brief combines more than one independently useful product outcome;
- new review findings require a second benchmark framework, a second large fixture corpus, or an
  unrelated host/tooling subsystem;
- the correction would materially expand the issue beyond its original crates or dependency
  boundary; or
- a benchmark-runner defect remains after one bounded correction.

The three-attempt rule is a hard delivery stop, not permission for an issue to consume unlimited
sub-rounds.  Each attempt gets one coherent implementation pass and one adversarial verdict.
Progress-only agent turns do not create extra attempts, but they must be consolidated or reassigned
when they stop producing commit-ready checkpoints.  After attempt three fails, preserve evidence,
split or rescope once, and move to the newly bounded issue; never perform a disguised fourth retry.

Benchmarks are descriptive during feature development.  Freeze the workload and validator before
timing, run exactly one invocation with one warmup and two measured rounds, and do not tune or
retry.  Benchmark infrastructure must preflight arguments, schema, output persistence, shell exit
semantics, and overwrite refusal without launching the timed workload.  If post-workload tooling
fails, preserve the raw output, record the failure, and move runner repair/promotion to a tooling
issue rather than blocking unrelated engine implementation.  Do not optimize merely to improve a
descriptive number; open a weekly optimization issue unless a named release budget is actually
missed.

Prefer a working vertical slice over exhaustive evidence scaffolding that delays downstream
features.  Launch-critical correctness, realtime safety, deterministic behavior, target builds,
and academically grounded DSP remain mandatory; additional matrix expansion should be recorded as
a follow-up once representative adversarial gates cover the frozen contract.  Report progress using
closed GitHub issues and deployable capabilities, not local file counts or unpushed evidence.

## Scope boundaries for this sprint

Deliver a mixing/mastering engine, session compiler, effect foundation, PCM runner, host adapters, streaming, and control foundation.  Do not deliver a timeline editor, human-oriented DAW UI, delivery codecs, unlimited in-memory stem cache, implicit feedback graph, third-party Wasm execution before its post-launch issue, or a general remote audio-streaming protocol.
