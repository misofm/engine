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
//! *number*, which is why only the inventories and the two machine constants are spelled here —
//! every row's floor is composed from them by [`FloorRow::cycles_per_lane_sample`] rather than
//! being written down a second time.
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

/// Required arithmetic per lane-sample, the session's own route matrix.
///
/// `mix2x2_block` writes each output channel as one `mul` plus one deliberately unfused `fma`: six
/// operations per frame, and a frame is two lane-samples. `docs/rulings/effect-floor-accounting.md`
/// counts it as the "route `mix2x2`" line of the builtins inventory, and it is spelled separately
/// here because the plumbing row is the builtins inventory *minus* everything but this and the
/// reduction.
const ROUTE_LANE_OPS: f64 = 3.0;
/// Required arithmetic per lane-sample, the output node's reduction, amortised per track.
///
/// Sixty-four contributors summed is sixty-three adds, which is 0.984 adds per track; the ruling
/// states it as 1, and the strip round's job 3 did not move it. The fold relocated the summation
/// into the cohort chain's epilogue -- the first contributor stores and the rest accumulate --
/// which is the same sixty-three adds over the same summands in the same order, by construction
/// (`route_fold` proves the order at bind). A fold is a dispatch and buffer saving, not an
/// arithmetic one, exactly as job 2's banking was.
const REDUCTION_LANE_OPS: f64 = 1.0;

/// Required arithmetic per lane-sample when **no builtins are prepared at all**.
///
/// The overhead floor row. `sixty_four_track_dispatch_only` is not this: an identity strip still
/// pays the D7 sanitise and boundary passes, the fader's multiply and mask clear, and the pan
/// matrix's per-lane select -- 22 lane-ops the spec requires of every block. What is left when the
/// input stage, the fader and the matrix are not *bound* is the session's own route and the master
/// reduction, and nothing else: the track stages lower to elided aliases, so a lane-sample passes
/// from the source binding to the route with no arithmetic in between.
///
/// Both terms are already lines of [`BUILTINS_LANE_OPS`] and [`BUILTINS_IDENTITY_LANE_OPS`], which
/// is what makes `gain_pan_only - plumbing_only` an exact subtraction rather than an estimate: it
/// is 22 - 4 = 18, the sanitise, the collapsed identity section, the boundary scan, the fader and
/// the pan matrix.
const PLUMBING_LANE_OPS: f64 = ROUTE_LANE_OPS + REDUCTION_LANE_OPS;

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
        // The mono rows carry the whole intended strip and are costed at the whole intended strip's
        // inventory, exactly as the standing console row is. Their fixture differs from it only in
        // per-channel *values* -- one source channel instead of two, and the left channel's
        // designed words on both sides -- and a floor is an inventory of operations, not of
        // operands. The mono collapse now exists and `sixty_four_track_console_mono` takes it, so
        // that row's measured cost falls while this inventory stands and its %-of-floor rises
        // above what a stereo row can reach. Whether a collapsed row's floor *should* halve is a
        // ruling this table still does not make; the open question is stated in the ruling doc's
        // "mono rows" note and this pin is what keeps answering it a deliberate edit.
        Workload::SixtyFourTrackConsole
        | Workload::OneTwentyEightTrackStretch
        | Workload::SixtyFourTrackConsoleMono
        | Workload::SixtyFourTrackConsoleMonoDual
        | Workload::SixtyFourTrackConsoleHalfMono => FloorRow {
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
        // The two rows that share the identity inventory, and share it on purpose. `gain_pan_only`
        // asks for the fixture's real fader trims and pan positions where `dispatch_only` asks for
        // 0 dB and hard identity, and the inventory does not move -- because `gain_mute_block` has
        // no identity arm and `matrix2x2_block` evaluates both arms of its per-lane select
        // unconditionally. One basis string for both is the claim: a gap between the two rows'
        // measurements would mean one of those two kernels had grown a data-dependent path.
        Workload::SixtyFourTrackDispatchOnly => FloorRow {
            lane_ops: BUILTINS_IDENTITY_LANE_OPS,
            width_factor: full,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins, identity",
        },
        Workload::SixtyFourTrackGainPanOnly => FloorRow {
            lane_ops: BUILTINS_IDENTITY_LANE_OPS,
            width_factor: full,
            // **No control, and the plumbing row is deliberately not one.** The inventories do
            // subtract -- 22 - 4 is the sanitise, the collapsed identity section, the boundary
            // scan, the fader and the pan -- but the *rows* do not, because they realise different
            // plumbing. This row binds eight bank chains, so job 3's route fold fires and its route
            // and reduction cost almost nothing; the plumbing row binds none, so it pays 64
            // individually dispatched route ops and an unfolded reduction. Subtracting the second
            // from the first removes the fold's saving as well as the plumbing's arithmetic, and
            // the result comes in *below* the 18-lane-op floor it is supposed to be measured
            // against -- which is the floor table saying, correctly, that the subtraction is not
            // the quantity it names. See the ruling's "why these two rows are not a control pair".
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins, identity",
        },
        // The floor of the whole stream. Nothing in this table is cheaper, and nothing can be: a
        // row that renders sixty-four tracks into one master pays a route and a share of the
        // reduction whatever else it does or does not prepare.
        //
        // It has no control and is nobody's control. What its own `percent_of_floor` reports is the
        // most interesting number the row carries: it is the *unfolded* plumbing -- 64 dispatched
        // route ops and a reduction over 64 separate buffers -- against the four lane-ops that
        // plumbing requires, so it is the worst standing in the table by a wide margin and that
        // gap is the dispatch job 3's fold removed from every banked row.
        Workload::SixtyFourTrackPlumbingOnly => FloorRow {
            lane_ops: PLUMBING_LANE_OPS,
            width_factor: full,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: plumbing",
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
        BANK_WIDTH, BUILTINS_IDENTITY_LANE_OPS, COMPRESSOR_LANE_OPS, OPS_PER_CYCLE,
        PLUMBING_LANE_OPS, floor_row, lane_samples_per_block,
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

    /// The overhead *inventories* subtract to 18 lane-ops, and neither row claims that as an
    /// isolate.
    ///
    /// Both halves are the assertion. The arithmetic difference between the identity strip and the
    /// bare plumbing is exactly the sanitise (7), the collapsed identity section (1), the boundary
    /// scan (4), the fader (2) and the pan (4) -- so the two inventories are consistent with each
    /// other and with the ruling. But the two *rows* are not a control pair, because they realise
    /// different plumbing: a banked row's route and reduction fold into its chain's epilogue and an
    /// unbanked row's do not. `floor_control_row` on both is `none`, and this test is what keeps a
    /// future edit from quietly turning an inventory identity into a measured isolate.
    #[test]
    fn the_overhead_inventories_differ_by_the_scaffolding_and_neither_row_claims_an_isolate() {
        let expected =
            (BUILTINS_IDENTITY_LANE_OPS - PLUMBING_LANE_OPS) / (BANK_WIDTH * OPS_PER_CYCLE);
        assert!((expected - 18.0 / (BANK_WIDTH * OPS_PER_CYCLE)).abs() < 1.0e-9);
        for workload in [
            Workload::SixtyFourTrackGainPanOnly,
            Workload::SixtyFourTrackPlumbingOnly,
        ] {
            let row = floor_row(workload).expect("a derived row");
            assert!(
                row.control.is_none(),
                "{}: the unbanked plumbing row is not a control for a banked row",
                workload.kind()
            );
        }
    }

    /// The plumbing row is the floor of the whole table, and the two rows that share the identity
    /// inventory share it exactly.
    ///
    /// Both halves matter. If some row were ever costed below the route and the reduction it must
    /// pay to reach the master at all, this table would be claiming a session can render for less
    /// than it can be summed; and if `gain_pan_only` ever stopped matching `dispatch_only`, the
    /// claim that a 0 dB fader and a settled identity matrix cost what a real one costs would have
    /// been quietly abandoned in the table rather than argued in the ruling.
    #[test]
    fn the_plumbing_row_is_the_floor_of_the_table_and_the_identity_pair_shares_one_inventory() {
        let plumbing = floor_row(Workload::SixtyFourTrackPlumbingOnly).expect("a derived row");
        for workload in WORKLOADS {
            let Some(row) = floor_row(workload) else {
                continue;
            };
            assert!(
                row.cycles_per_lane_sample() >= plumbing.cycles_per_lane_sample(),
                "{} is costed below the route and reduction every row must pay",
                workload.kind()
            );
        }
        let identity = floor_row(Workload::SixtyFourTrackDispatchOnly).expect("a derived row");
        let gain_pan = floor_row(Workload::SixtyFourTrackGainPanOnly).expect("a derived row");
        assert_eq!(identity.basis, gain_pan.basis);
        assert!(
            (identity.cycles_per_lane_sample() - gain_pan.cycles_per_lane_sample()).abs() < 1.0e-9
        );
    }

    /// The three mono rows are the standing console row's inventory, restated for their fixture.
    ///
    /// The mono fixture differs from the standing one in per-channel *values* only, and a floor is
    /// an inventory of operations. Pinning the equality here is what makes a future change that
    /// halves a collapsed row's floor a deliberate, visible decision rather than a table edit.
    #[test]
    fn the_mono_rows_carry_the_standing_strips_floor() {
        let console = floor_row(Workload::SixtyFourTrackConsole).expect("a derived row");
        for workload in [
            Workload::SixtyFourTrackConsoleMono,
            Workload::SixtyFourTrackConsoleMonoDual,
            Workload::SixtyFourTrackConsoleHalfMono,
        ] {
            let row = floor_row(workload).expect("a derived row");
            assert_eq!(row.basis, console.basis, "{}", workload.kind());
            assert!(
                (row.cycles_per_lane_sample() - console.cycles_per_lane_sample()).abs() < 1.0e-9,
                "{}",
                workload.kind()
            );
            assert!(row.control.is_none(), "{}", workload.kind());
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
