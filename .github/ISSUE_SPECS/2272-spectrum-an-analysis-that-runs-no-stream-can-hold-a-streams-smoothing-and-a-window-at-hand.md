# Spectrum: an analysis that runs no stream can hold a stream's smoothing and a window at hand

Follow-up of #2255 (tracker #2258, "Follow-ups those attempts surfaced"). Terra attempt 1; five
attempts maximum.

## Problem

`SpectrumAnalysis` (`crates/host-core/src/spectrum_service.rs`, the worker-side half of a
spectrum stream) keeps its stream in four loose fields: `stream_active: bool`,
`cadence: Option<SpectrumCadence>`, `smoothing: Option<SpectrumSmoothingConfig>` and
`window_known: bool`. #2255 made the service's half an enum (`ServiceStream`) and left this one
out of scope. The fields encode one machine, Off or On { cadence, smoothing, window at hand }, but
two writers in the service touch them without asking which state the analysis is in:

- `SpectrumService::read_stream` sets `window_known` (false, then true on a ready window) on the
  analysis it is handed, running or not.
- `SpectrumService::select_stream` writes `smoothing = Some(..)` on a changed smoothing, running
  or not.

An analysis that runs no stream while its service does is a documented case:
`SpectrumAnalysis::adoption_needed` exists for "an analysis that is `None` or not running the
stream", which adopts the stream from a ready record with `configure_imported`. Handing such an
analysis to a read or a select (the API allows it) leaves `stream_active() == false` beside
`has_stream_window() == true` or `stream_smoothing() == Some(..)`. `analyze_stream_into` carries
a dead `(Some(cadence), Some(smoothing)) else StreamInactive` branch for the other illegal
combination, running without a cadence.

**Reachability.** No production caller reaches it: the only one, control-plane's spectrum monitor
(`crates/control-plane/src/control.rs`, `SpectrumMonitor` in `crates/control-plane/src/spectrum.rs`),
makes its analysis with its service and passes it to every stream call, so the two never
disagree. The stray facts are also never consumed today: every transition into a running
analysis (`begin_stream`, `configure_imported`) overwrites them, and every analysis call checks
`stream_active` first. It is reachable through host-core's public API, and the accessors report
it. Severity L (hardening): the fix makes the illegal combinations unrepresentable.

## Contract

- One private enum holds the analysis's stream: `Off`, or `On { cadence, smoothing,
  window_known }`. Only a running analysis has a cadence, a smoothing or a window at hand.
- The service changes a running analysis's window and smoothing through methods that do nothing
  on an `Off` analysis; it never makes an `Off` analysis hold a fact.
- The public accessors keep their names and signatures; they read the enum.
- No other behaviour changes: the stray facts were never consumed (see Reachability).

## Non-goals

- HC-10 (the one-shot render-local `armed`/`held`/`survived_block_end`/`latched`/`filled`
  fields in `crates/host-core/src/spectrum.rs`): a different struct on the render path, with its
  own outcome. It stays a tracker item.
- An analysis that runs a stream other than its service's (a caller that passed `None` to a stop
  and a start): the caller contract documented on `SpectrumService::adopt_successor` and
  `stop_stream`, not this machine.
- `crates/host-core/src/warm.rs`, `source.rs`, `meters.rs`.

## Gates

- A regression test red on the base, green after.
- Neither render-path state (`SpectrumAnalysis` runs on the control or worker side; the service
  calls it touches allocate nothing either way) nor cross-thread (it is owned by one thread and
  passed by `&mut`), so no no-bit-moved comparison and no loom model apply.
- `cargo test --locked -p host-core` and its dependents' spectrum tests; `cargo clippy --locked
  -p host-core --all-targets -- -D warnings`; `cargo fmt --check`;
  `bash scripts/check-workspace-policy.sh`; the realtime policy tool (the service calls sit in a
  marked region).

## Evidence

### Attempt 1 (Terra, branch `codex/issue-2272-spectrum-analysis-stream`)

**Model.** `SpectrumAnalysis.stream: AnalysisStream`, private:
`Stopped | Running { cadence, smoothing, window_known }`, the shape of the service's own
`ServiceStream`. `begin_stream` and `configure_imported` are the only ways into `Running`;
`end_stream` is the only way out. The service's two writers now go through
`set_window_known` and `set_stream_smoothing`, which do nothing on a `Stopped` analysis.
`analyze_stream_into` destructures `Running`, so the dead branch for "running without a
cadence" is gone. The accessors read the enum and keep their signatures. One `bool` and two
`Option`s become one enum with a `bool` inside it, so no allocation, lock or call is added on the
service's realtime-marked calls.

**Red on base.** `an_analysis_without_a_stream_holds_no_stream_facts`
(`crates/host-core/tests/spectrum_service.rs`). The service's stream was started without an
analysis, then a new `SpectrumAnalysis` is handed to a ready read and to a select that changes
the smoothing:

```
thread 'an_analysis_without_a_stream_holds_no_stream_facts' panicked at crates/host-core/tests/spectrum_service.rs:1233:5:
a read marked a window
```

and, with that assertion commented out for the run only, the second one:

```
assertion `left == right` failed: a select wrote a smoothing
  left: (None, Some(SpectrumSmoothingConfig { smoothing_ms: 50.0 }))
 right: (None, None)
```

Green after. The test ends by adopting the stream from the next ready record
(`adoption_needed` -> `configure_imported`): the adopted smoothing is the metadata's.

**Test value.** It turns red when a service call gives an analysis that runs no stream a stream
fact: a window at hand, a smoothing, or running state without `begin_stream` or an import. No
existing test hands a stream call an analysis that is not running the stream.

**Mutations** (each applied to the fixed tree, then reverted; 16 other tests in the file stayed
green under each):

- `keep_stream_window` promotes a stopped analysis to `Running` with the service's cadence and
  smoothing (a "helpful" adopt on read): red, `spectrum_service.rs:1232` (`!stream_active()`).
- `select_stream` promotes a stopped analysis to `Running` on a changed smoothing: red,
  `assertion failed: !analysis.stream_active()` at `:1244`.

**No-bit-moved and loom: not applicable.** `SpectrumAnalysis` is not render-path state. It is
owned by the control side (control-plane's `SpectrumMonitor`) or by a worker, and is passed by
`&mut`, so no other thread reads it. Its analysis arithmetic is unchanged; only its state shape
changed.

**Gates.**
- `cargo test --locked -p host-core`: 291 passed, 0 failed. With `--all-targets --features
  host-core/test-support`: 490 passed, 0 failed.
- `cargo test --locked --all-targets --no-fail-fast -p control-plane -p capi -p host-web -p host-core
  --features host-web/test-support,host-web/render-lock-probe,host-core/test-support,control-plane/test-support`
  (CI's features for these packages): 1140 passed, 0 failed.
- `cargo check --locked --workspace --all-targets`: clean, apart from one host-web warning
  (`set_state_for_test` is never used) that predates this change.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p host-core --features
  control-provider`: clean. Without the feature, the command fails on one unresolved link,
  `crate::warm::production_config` in `prepare.rs:262`, which this change did not touch: `warm`
  exists only under `control-provider` (`lib.rs:125-126`). CI's `--workspace` doc build turns that
  feature on.
- One observation, noted to be candid: the first dependents run (`cargo test -p control-plane -p
  capi -p host-web`, no explicit features, on a machine at load ~31 of 32 cores) stalled in
  `spectrum_monitor`'s `alternating_configures_against_a_rendering_thread_allocate_nothing_on_render`
  for 48 minutes and was killed. Two later runs of that binary passed it in seconds, 2.38 s for
  the whole binary. That test drives one-shot captures against a render thread and never takes the
  stream path this change edits. It was not rerun on the base.
- `cargo clippy --locked -p host-core --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `realtime_policy`: `ok (292 marked regions in 46 files)`.

**Follow-up (not folded).** HC-10, the one-shot render-local `armed`/`held`/`survived_block_end`/
`latched`/`filled` fields (`crates/host-core/src/spectrum.rs` ~1732-1748), is a different struct
on the render path with a different outcome. It needs its own issue and a no-bit-moved
comparison.
