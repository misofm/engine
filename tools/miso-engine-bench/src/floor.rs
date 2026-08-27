//! Issue #184: per-effect class-A floor accounting for the console session rows.
//!
//! # What a floor is here
//!
//! A *class-A floor* is the cost of the arithmetic the frozen spec requires, and nothing else. It
//! is derived, never measured: the op inventory of a kernel is counted from the source that
//! implements the frozen operation order, and divided by the measured throughput of the machine
//! the benchmark is pinned to. `docs/rulings/effect-floor-accounting.md` is the derivation; this
//! module is the single authoritative table the records are built from, and the jq validator is
//! an independent restatement of the same composition. Three copies is two too many for a
//! *number*, which is why only the four inventories and the two machine constants are spelled
//! here — every row's floor is composed from them by [`FloorRow::cycles_per_lane_sample`] rather
//! than being written down a second time.
//!
//! # Why the columns are optional
//!
//! A cycle count needs a core clock, and a core clock needs a performance counter. The runner
//! measures one with `perf stat` under the same pinned-core preconditions it already enforces and
//! exports it as [`CORE_CLOCK_HZ_VAR`]; on a host with no usable counter it exports nothing and
//! every record in the run omits the whole group. That is what makes the extension *additive*:
//! the sealed records under `artifacts/` predate the columns and must still validate, so absence
//! is a legal shape rather than a defect. Absence is all-or-nothing per run, which the aggregate
//! validator enforces — a stream with the columns on some rows and not others would be a runner
//! that lost its counter half way through.
//!
//! # What the columns are not
//!
//! `cycles_per_lane_sample` is the *row's* cost, control plane included, so `percent_of_floor` is
//! a lower bound on how close the row's arithmetic is to its floor. The per-effect number is the
//! isolated one: every decomposition row names the control row whose cost is subtracted from it
//! ([`FloorRow::control`]), and that subtraction is what turns a row into an effect. Both are in
//! the record because reporting only one of them would hide which is which.

use miso_engine_bench_support::metadata::Metadata;
use miso_engine_console_workload::{QUANTUM, Workload};

/// The core clock the runner measured for the pinned core, in hertz.
pub(crate) const CORE_CLOCK_HZ_VAR: &str = "MISO_ENGINE_BENCH_CORE_CLOCK_HZ";
/// How the runner obtained [`CORE_CLOCK_HZ_VAR`]. Free text; the record carries it verbatim.
pub(crate) const CORE_CLOCK_SOURCE_VAR: &str = "MISO_ENGINE_BENCH_CORE_CLOCK_SOURCE";

/// Channels every console track renders.
///
/// A lane-sample is one channel of one track for one frame, which is the unit
/// `docs/rulings/fast-db-tier-boundaries.md` already counts in ("the 16,384 lane-samples in a
/// sixty-four-track block" is 64 tracks x 128 frames x 2 channels).
const CHANNELS: u64 = 2;

/// Lanes one native bank renders per vector operation on the measured backend (`Simd8`, AVX2).
const BANK_WIDTH: f64 = 8.0;

/// Sustained 256-bit vector-ALU operations per cycle, measured on the pinned benchmark core.
///
/// `docs/rulings/effect-floor-accounting.md`, "The machine": a multiply-and-add stream measured at
/// 3.695 ops/cycle and a compare-and-select stream at 3.763, against 1.99 for a stream of adds
/// alone. This is the only number in this module that describes the host rather than the spec, and
/// it is the one an adversarial reader should re-measure first: every floor below is inversely
/// proportional to it.
const OPS_PER_CYCLE: f64 = 3.7;

/// Required arithmetic per lane-sample, compressor kernel.
///
/// `docs/rulings/effect-floor-accounting.md`, "Compressor inventory".
const COMPRESSOR_LANE_OPS: f64 = 94.0;
/// Required arithmetic per lane-sample, parametric-EQ kernel, at the standing fixture's band count.
///
/// `docs/rulings/effect-floor-accounting.md`, "EQ inventory".
const EQ_LANE_OPS: f64 = 51.0;
/// Required arithmetic per lane-sample, true-peak limiter, post-round-1 uniform-cohort shape.
///
/// `docs/rulings/effect-floor-accounting.md`, "Limiter inventory".
const LIMITER_LANE_OPS: f64 = 138.0;
/// Required arithmetic per lane-sample, the builtins chain and the fixture's routing, with both
/// SVF sections per channel carrying a real design.
///
/// `docs/rulings/effect-floor-accounting.md`, "Builtins inventory".
const BUILTINS_LANE_OPS: f64 = 69.0;

/// Required arithmetic per lane-sample when every builtin section is the prepared identity.
///
/// The two rack-free rows no longer share a floor. A section whose prepared design is the exact
/// identity is not a recurrence the spec requires: it is the map `v |-> v + 0.0`, and a run of them
/// is one `add(+0.0)` (`input_chain_block_elided`, and the appendix to the ruling). So the class-A
/// arithmetic of the `dispatch_only` row is the 69 with both 24-op sections replaced by that single
/// add: 7 sanitise + 1 identity add + 4 boundary scan + 2 fader + 4 pan + 3 route + 1 reduction.
///
/// The fader and the pan matrix stay at their full cost even though this row asks both for their
/// identity: a 0 dB fader is still a multiply and a mask clear, and a settled identity matrix still
/// evaluates both arms of its per-lane select. Only the input sections have a prepared-identity
/// rewrite; `docs/rulings/effect-floor-accounting.md`, "Builtins inventory".
const BUILTINS_IDENTITY_LANE_OPS: f64 = 22.0;

/// The width penalty of a ragged track count, as a multiple of the full-bank floor.
///
/// Nine tracks is one full eight-lane bank plus a one-track tail, and the tail's lane-samples cost
/// a whole vector operation each. Eight of nine tracks run at [`BANK_WIDTH`] and one runs at width
/// one, so the mean lane-sample costs `(8 + 8) / 9` of the full-bank floor.
const RAGGED_NINE_TRACK_WIDTH_FACTOR: f64 = 16.0 / 9.0;

/// One row of the floor table.
pub(crate) struct FloorRow {
    /// Required arithmetic per lane-sample, summed over what this row's strip carries.
    lane_ops: f64,
    /// Multiple of the full-bank floor this row's bank shape costs. `1.0` for a full-bank row.
    width_factor: f64,
    /// The row whose cost is subtracted to isolate this row's subject, when there is one.
    pub(crate) control: Option<Workload>,
    /// The inventories this row's floor is composed from, named as the ruling doc names them.
    pub(crate) basis: &'static str,
}

impl FloorRow {
    /// The derived class-A floor for one lane-sample of this row, in core clock cycles.
    ///
    /// Required arithmetic per lane-sample divided by the lane-ops the machine retires per cycle,
    /// which is [`BANK_WIDTH`] lanes times [`OPS_PER_CYCLE`] vector operations.
    pub(crate) fn cycles_per_lane_sample(&self) -> f64 {
        self.lane_ops * self.width_factor / (BANK_WIDTH * OPS_PER_CYCLE)
    }
}

/// The floor table: what every session workload's strip requires, and what isolates it.
///
/// `nine_track_baseline` is deliberately absent. It is the one row rendered from a different
/// fixture (`fixtures/session/v1/parametric-eq-nine-track.toml`), whose band count and builtins
/// were never inventoried, and a floor stated for the wrong fixture would be worse than no floor
/// at all. The record says so: its `floor_basis` is `not_derived` and its floor columns are null.
pub(crate) fn floor_row(workload: Workload) -> Option<FloorRow> {
    let full = 1.0_f64;
    Some(match workload {
        Workload::NineTrackBaseline => return None,
        Workload::NineTrackRaggedStrip => FloorRow {
            lane_ops: BUILTINS_LANE_OPS + EQ_LANE_OPS + COMPRESSOR_LANE_OPS + LIMITER_LANE_OPS,
            width_factor: RAGGED_NINE_TRACK_WIDTH_FACTOR,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins+eq+compressor+limiter, ragged",
        },
        Workload::SixtyFourTrackConsole | Workload::OneTwentyEightTrackStretch => FloorRow {
            lane_ops: BUILTINS_LANE_OPS + EQ_LANE_OPS + COMPRESSOR_LANE_OPS + LIMITER_LANE_OPS,
            width_factor: full,
            // The limiter is the one effect the intended strip adds to the chain-shape row, so
            // this subtraction is the limiter and nothing else (#175 states the same pairing).
            control: match workload {
                Workload::SixtyFourTrackConsole => Some(Workload::SixtyFourTrackEqCompSimd1),
                _ => None,
            },
            basis: "docs/rulings/effect-floor-accounting.md: builtins+eq+compressor+limiter",
        },
        Workload::SixtyFourTrackEqOnly => FloorRow {
            lane_ops: BUILTINS_LANE_OPS + EQ_LANE_OPS,
            width_factor: full,
            control: Some(Workload::SixtyFourTrackBuiltinsOnly),
            basis: "docs/rulings/effect-floor-accounting.md: builtins+eq",
        },
        Workload::SixtyFourTrackCompressorOnly => FloorRow {
            lane_ops: BUILTINS_LANE_OPS + COMPRESSOR_LANE_OPS,
            width_factor: full,
            control: Some(Workload::SixtyFourTrackBuiltinsOnly),
            basis: "docs/rulings/effect-floor-accounting.md: builtins+compressor",
        },
        Workload::SixtyFourTrackConsoleLegacy | Workload::SixtyFourTrackEqCompSimd1 => FloorRow {
            lane_ops: BUILTINS_LANE_OPS + EQ_LANE_OPS + COMPRESSOR_LANE_OPS,
            width_factor: full,
            control: Some(Workload::SixtyFourTrackBuiltinsOnly),
            basis: "docs/rulings/effect-floor-accounting.md: builtins+eq+compressor",
        },
        // The idle row is the full console strip rendering silence, and all three rack effects
        // hold a silence fixed point (`silent_fixed_point` in the EQ, the compressor and the
        // limiter alike), so all three kernels are skipped on every timed block of this row. What
        // remains of the strip's arithmetic is the builtins chain, which has no such claim.
        Workload::SixtyFourTrackIdle => FloorRow {
            lane_ops: BUILTINS_LANE_OPS,
            width_factor: full,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins, silent",
        },
        Workload::SixtyFourTrackBuiltinsOnly => FloorRow {
            lane_ops: BUILTINS_LANE_OPS,
            width_factor: full,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins",
        },
        // The two rack-free rows no longer share a floor, and the split is the whole point of the
        // row: every builtin section on this one is the prepared identity, and the prepared
        // identity is elided rather than executed. Its class-A arithmetic is what is left --
        // sanitisation, one identity add, the boundary scan, the fader, the pan and the routing.
        Workload::SixtyFourTrackDispatchOnly => FloorRow {
            lane_ops: BUILTINS_IDENTITY_LANE_OPS,
            width_factor: full,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins, identity",
        },
    })
}

/// Lane-samples one block of this workload renders: tracks x frames x channels.
pub(crate) fn lane_samples_per_block(workload: Workload) -> u64 {
    u64::from(workload.tracks()) * QUANTUM as u64 * CHANNELS
}

/// The pinned core's clock, as the runner measured it.
pub(crate) struct CoreClock {
    /// Cycles per second.
    hertz: f64,
    /// How the runner obtained it.
    source: String,
}

impl CoreClock {
    /// Reads the runner's measurement, or `None` when this run has no counter behind it.
    ///
    /// Both names or neither: a clock with no provenance is not a measurement, and a provenance
    /// with no clock is a runner defect. Either alone is treated as absent, which the runner's own
    /// export discipline makes unreachable and this function refuses to paper over.
    pub(crate) fn from_runner(metadata: &Metadata) -> Option<Self> {
        let hertz: f64 = metadata.var(CORE_CLOCK_HZ_VAR).ok()?.parse().ok()?;
        let source = metadata.var(CORE_CLOCK_SOURCE_VAR).ok()?;
        if !hertz.is_finite() || hertz <= 0.0 || source.is_empty() {
            return None;
        }
        Some(Self { hertz, source })
    }

    /// Core clock cycles in `nanoseconds` of wall time on the pinned core.
    fn cycles(&self, nanoseconds: u64) -> f64 {
        nanoseconds as f64 * self.hertz / 1.0e9
    }
}

/// The floor-accounting record fields for one session row, with the trailing comma a splice needs.
///
/// `control_p50_ns` is the p50 of the row named by [`FloorRow::control`], measured in the same
/// process on the same host in the same round — which is the only way the subtraction means
/// anything, and the reason the subject measures every workload before it emits any record.
pub(crate) fn record_fields(
    workload: Workload,
    p50_ns: u64,
    clock: &CoreClock,
    control_p50_ns: Option<u64>,
) -> String {
    let lane_samples = lane_samples_per_block(workload);
    let cycles_per_block = clock.cycles(p50_ns);
    let cycles_per_lane_sample = cycles_per_block / lane_samples as f64;
    let row = floor_row(workload);
    let floor = row.as_ref().map(FloorRow::cycles_per_lane_sample);
    let basis = row.as_ref().map_or("not_derived", |row| row.basis);
    let control = row.as_ref().and_then(|row| row.control);
    let isolated = control
        .and(control_p50_ns)
        .map(|control_p50| (cycles_per_block - clock.cycles(control_p50)) / lane_samples as f64);
    let isolated_floor = control.zip(floor).and_then(|(control, floor)| {
        floor_row(control).map(|control| floor - control.cycles_per_lane_sample())
    });
    format!(
        concat!(
            "\"lane_samples_per_block\":{lane_samples},\"core_clock_hz\":{clock:.3},",
            "\"core_clock_source\":\"{source}\",\"cycles_per_block_p50\":{cycles_per_block:.3},",
            "\"cycles_per_lane_sample\":{per_lane_sample:.3},",
            "\"floor_cycles_per_lane_sample\":{floor},\"percent_of_floor\":{percent},",
            "\"floor_basis\":\"{basis}\",\"floor_control_row\":\"{control}\",",
            "\"isolated_cycles_per_lane_sample\":{isolated},",
            "\"isolated_percent_of_floor\":{isolated_percent},"
        ),
        lane_samples = lane_samples,
        clock = clock.hertz,
        source = miso_engine_bench_support::json::escape(&clock.source),
        cycles_per_block = cycles_per_block,
        per_lane_sample = cycles_per_lane_sample,
        floor = optional(floor),
        percent = optional(floor.map(|floor| 100.0 * floor / cycles_per_lane_sample)),
        basis = basis,
        control = control.map_or("none", Workload::kind),
        isolated = optional(isolated),
        isolated_percent = optional(
            isolated
                .zip(isolated_floor)
                .map(|(isolated, floor)| 100.0 * floor / isolated)
        ),
    )
}

/// A derived number that this row does not have, rendered as JSON `null` rather than as a zero.
fn optional(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), |value| format!("{value:.3}"))
}

#[cfg(test)]
mod tests {
    use super::{
        BANK_WIDTH, COMPRESSOR_LANE_OPS, OPS_PER_CYCLE, floor_row, lane_samples_per_block,
    };
    use miso_engine_console_workload::{WORKLOADS, Workload};

    #[test]
    fn the_sixty_four_track_block_has_the_lane_sample_count_the_rulings_quote() {
        assert_eq!(
            lane_samples_per_block(Workload::SixtyFourTrackConsole),
            16_384
        );
        assert_eq!(lane_samples_per_block(Workload::NineTrackBaseline), 2_304);
    }

    #[test]
    fn every_derived_row_names_its_ruling_and_composes_a_positive_floor() {
        for workload in WORKLOADS {
            let Some(row) = floor_row(workload) else {
                assert!(matches!(workload, Workload::NineTrackBaseline));
                continue;
            };
            assert!(
                row.basis
                    .starts_with("docs/rulings/effect-floor-accounting.md: "),
                "{} does not name its ruling",
                workload.kind()
            );
            assert!(row.cycles_per_lane_sample() > 0.0);
        }
    }

    #[test]
    fn a_control_row_is_always_cheaper_than_the_row_it_isolates() {
        for workload in WORKLOADS {
            let Some(row) = floor_row(workload) else {
                continue;
            };
            let Some(control) = row.control else {
                continue;
            };
            let control = floor_row(control).expect("a control row carries a floor of its own");
            assert!(
                control.cycles_per_lane_sample() < row.cycles_per_lane_sample(),
                "{} does not isolate anything",
                workload.kind()
            );
        }
    }

    #[test]
    fn the_compressor_isolate_is_the_compressor_inventory() {
        let row = floor_row(Workload::SixtyFourTrackCompressorOnly).expect("a derived row");
        let control = floor_row(Workload::SixtyFourTrackBuiltinsOnly).expect("a derived row");
        let isolated = row.cycles_per_lane_sample() - control.cycles_per_lane_sample();
        let expected = COMPRESSOR_LANE_OPS / (BANK_WIDTH * OPS_PER_CYCLE);
        assert!((isolated - expected).abs() < 1.0e-9);
    }
}
