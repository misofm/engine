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

use bench_support::metadata::Metadata;
use console_workload::{QUANTUM, Workload};

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
const COMPRESSOR_LANE_OPS: f64 = 81.5;
/// Required arithmetic per lane-sample, parametric-EQ stationary cascade, at the standing fixture's
/// one live section: a select-free depth-one pass (`svf_step` 24 + output mix 5) and the 4.4
/// boundary scan (3). Issue #976 dropped the identity padding section the depth-two pass used to
/// run beside it; issue #1328's joint SVF flush (`flush_pair`, 11 lane-ops for the two state words
/// against 6 for two `flush`) raised `svf_step` from 19 to 24 and this from 27 to 32.
///
/// `docs/rulings/effect-floor-accounting.md`, "EQ inventory".
const EQ_LANE_OPS: f64 = 32.0;
/// Required arithmetic per lane-sample, true-peak limiter, post-round-1 uniform-cohort shape.
///
/// `docs/rulings/effect-floor-accounting.md`, "Limiter inventory".
const LIMITER_LANE_OPS: f64 = 129.5;
/// Required arithmetic per lane-sample, the builtins chain and the fixture's routing, with both
/// SVF sections per channel carrying a real design. Each section is 29 since issue #1328's joint
/// SVF flush (24 before it), so the chain is 79 (69 before it).
///
/// `docs/rulings/effect-floor-accounting.md`, "Builtins inventory".
const BUILTINS_LANE_OPS: f64 = 79.0;

/// Required arithmetic per lane-sample when every builtin section is the prepared identity.
///
/// The two rack-free rows no longer share a floor. A section whose prepared design is the exact
/// identity is not a recurrence the spec requires: it is the map `v |-> v + 0.0`, and a run of them
/// is one `add(+0.0)` (`input_chain_block_elided`, and the appendix to the ruling). So the class-A
/// arithmetic of the `dispatch_only` row is the 79 with both 29-op sections replaced by that single
/// add: 7 sanitise + 1 identity add + 4 boundary scan + 2 fader + 4 pan + 3 route + 1 reduction.
///
/// The fader and the pan matrix stay at their full cost: a 0 dB fader is still a multiply and a
/// mask clear, and the row's pan (hard right on both inputs, not the identity; see the workload's
/// doc) is a settled matrix with no identity lane, which since #944 takes the select-free arm. The
/// pan is still counted at 4 lane-ops here; it is recounted with the prepared-identity elision
/// successor of #944, not before. Only the input sections have a prepared-identity rewrite;
/// `docs/rulings/effect-floor-accounting.md`, "Builtins inventory".
///
/// It is the floor of the whole table (issue #956). Its last two lines, the route's `mix2x2` (3)
/// and the output node's reduction amortised per track (1), are the routing component every row
/// pays to reach the master; no row is costed at them alone, because the only row that was, the
/// builtins-less `sixty_four_track_plumbing_only`, measured a plan no host compiles and was
/// retired. `docs/rulings/effect-floor-accounting.md`, "Routing component".
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
/// fixture (`fixtures/session/v1/parametric-eq-nine-track.json`), whose band count and builtins
/// were never inventoried, and a floor stated for the wrong fixture would be worse than no floor
/// at all. The record says so: its `floor_basis` is `not_derived` and its floor columns are null.
///
/// `sixty_four_track_console_metered` (issue #881) is absent for the same reason. Its strip is the
/// standing console's, but its meters are arithmetic the rulings have never inventoried, and
/// costing it at the unmetered strip's floor would publish the meters' cost as a gap in the
/// strip's. It names no control either: a subtraction against the standing row would be an
/// isolate of the meters against a floor nobody derived.
///
/// The five console-strip rows (issue #1085) are absent too, each for a reason the rulings have not
/// settled. A remainder's width factor depends on whether it renders per node or as a padded bank,
/// which is what the console strip changes (the nine-track factor holds for a remainder of one
/// only, where the two cost the same). The app shape's bypassed lanes and the sparse row's silent
/// ones are arithmetic no inventory counts. The sixteen-track row could carry the whole strip's
/// inventory, but it is one of the strip-at-N set and is read with it, on measured cost alone.
/// H5's figures stay arithmetic until S4 reports, so these rows state no floor and name no
/// control.
///
/// `sixty_four_track_console_sends` (issue #1227) is absent because no inventory exists for a bus
/// or a send: its tracks carry the standing strip, but its submix strips, inserts and route mixes
/// are arithmetic the rulings have never counted, so it states no floor and names no control.
pub(crate) fn floor_row(workload: Workload) -> Option<FloorRow> {
    let full = 1.0_f64;
    Some(match workload {
        Workload::NineTrackBaseline
        | Workload::SixtyFourTrackConsoleMetered
        | Workload::SixtyFourTrackConsoleSends
        | Workload::TenTrackRaggedStrip
        | Workload::ThirteenTrackRaggedStrip
        | Workload::SixteenTrackStrip
        | Workload::SixtyFourTrackAppShape
        | Workload::SixtyFourTrackConsoleSparse => return None,
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
        // 0 dB and a pan of `left = right = 1.0`, and the inventory does not move. `gain_mute_block`
        // has no identity arm. The settled matrix has had one data-dependent path since issue #944
        // -- a bank with no identity lane skips the per-lane identity select -- but neither row
        // has an identity lane: `dispatch_only`'s pan routes both inputs hard right, which is not
        // the identity matrix, so both rows take the same select-free arm. One basis string for
        // both is the claim: a gap between the two rows' measurements would mean one of those two
        // kernels had grown another data-dependent path.
        Workload::SixtyFourTrackDispatchOnly => FloorRow {
            lane_ops: BUILTINS_IDENTITY_LANE_OPS,
            width_factor: full,
            control: None,
            basis: "docs/rulings/effect-floor-accounting.md: builtins, identity",
        },
        // The driver-fed twin (issue #928, re-based onto this session by #956) is the same session
        // with its track inputs claimed by a prepared source set, so it requires the same
        // arithmetic: moving a frozen block into the graph is a copy, not a lane-op, whichever feed
        // does it, and the production feed does not even copy -- each claim is gathered in place.
        // Both rows are costed at the identity inventory, the floor of the table, and neither names
        // a control nor is anybody's: a subtraction between the two feeds would publish the bound
        // feed's copies as an isolate against a floor that has no copy in it. The driver-fed row
        // is the native pure-path target, so its own `percent_of_floor` is the number the
        // pure-path work reads.
        Workload::SixtyFourTrackGainPanOnly | Workload::SixtyFourTrackGainPanRing => FloorRow {
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
        source = bench_support::json::escape(&clock.source),
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
    use std::collections::BTreeSet;
    use std::io::Write;
    use std::process::{Command, Stdio};

    use super::{
        BANK_WIDTH, BUILTINS_IDENTITY_LANE_OPS, COMPRESSOR_LANE_OPS, EQ_LANE_OPS, LIMITER_LANE_OPS,
        OPS_PER_CYCLE, floor_row, lane_samples_per_block,
    };
    use console_workload::{
        BUS_SEND_WORKLOADS, CONSOLE_STRIP_WORKLOADS, Workload, native_session_rows,
    };

    /// The rows the table states no floor for: the uninventoried nine-track fixture, the metered
    /// row, the bus-and-send row (issue #1227) and the console-strip rows (issue #1085).
    fn underived(workload: Workload) -> bool {
        matches!(
            workload,
            Workload::NineTrackBaseline | Workload::SixtyFourTrackConsoleMetered
        ) || BUS_SEND_WORKLOADS.contains(&workload)
            || CONSOLE_STRIP_WORKLOADS.contains(&workload)
    }

    fn rust_floor_table() -> String {
        let mut keys = BTreeSet::new();
        let mut table = String::from("{");
        for (index, workload) in native_session_rows().enumerate() {
            let key = workload.kind();
            assert!(keys.insert(key), "duplicate floor workload key: {key}");
            if index != 0 {
                table.push(',');
            }
            table.push('"');
            table.push_str(&bench_support::json::escape(key));
            table.push_str("\":[");
            match floor_row(workload) {
                Some(row) => {
                    assert!(row.lane_ops.is_finite());
                    assert!(row.width_factor.is_finite());
                    table.push_str(&format!("{:?},{:?},\"", row.lane_ops, row.width_factor));
                    table.push_str(&bench_support::json::escape(
                        row.control.map_or("none", Workload::kind),
                    ));
                    table.push_str("\",");
                    table.push('"');
                    table.push_str(&bench_support::json::escape(row.basis));
                    table.push('"');
                }
                None => {
                    assert!(underived(workload), "{}", workload.kind());
                    table.push_str("null,1,\"none\",\"not_derived\"");
                }
            }
            table.push(']');
        }
        table.push('}');
        table
    }

    fn jq_floor_comparison(rust_table: &str, mutation: &str) -> Result<bool, String> {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let scripts = manifest
            .parent()
            .and_then(std::path::Path::parent)
            .ok_or_else(|| "cannot resolve repository scripts directory".to_string())
            .map(|root| root.join("scripts"))?;
        let expression = r#"
include "console-benchmark-record-lib";
floor_pins as $expected |
input as $rust |
(if $mutation == "missing" then
   ($expected | del(.sixty_four_track_console))
 elif $mutation == "extra" then
   $expected + {"floor_parity_extra": [null, 1, "none", "not_derived"]}
 elif $mutation == "value" then
   ($expected | .sixty_four_track_console[0] = (.sixty_four_track_console[0] + 1))
 else $expected
 end) == $rust
"#;
        let mut child = Command::new("jq")
            .args([
                "-L",
                scripts
                    .to_str()
                    .ok_or_else(|| "non-UTF-8 scripts path".to_string())?,
            ])
            .args(["-n", "--arg", "mutation", mutation, expression])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("failed to spawn jq: {error}"))?;
        child
            .stdin
            .take()
            .ok_or_else(|| "jq stdin was not available".to_string())?
            .write_all(rust_table.as_bytes())
            .map_err(|error| format!("failed to write jq input: {error}"))?;
        let output = child
            .wait_with_output()
            .map_err(|error| format!("failed waiting for jq: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "jq failed with {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        match String::from_utf8_lossy(&output.stdout).trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            result => Err(format!("jq did not return exactly one boolean: {result:?}")),
        }
    }

    #[test]
    fn rust_and_jq_floor_tables_have_exact_key_value_parity() {
        let rust_table = rust_floor_table();
        assert!(jq_floor_comparison(&rust_table, "").expect("positive jq comparison"));
        for mutation in ["missing", "extra", "value"] {
            assert!(
                !jq_floor_comparison(&rust_table, mutation).expect("negative jq comparison"),
                "{mutation} mutation was not detected"
            );
        }
    }

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
        for workload in native_session_rows() {
            let Some(row) = floor_row(workload) else {
                assert!(underived(workload), "{}", workload.kind());
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
        for workload in native_session_rows() {
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

    /// The identity inventory is the floor of the whole table, and the rows that share it share it
    /// exactly (issue #956).
    ///
    /// Both halves matter. If some row were ever costed below the identity inventory, this table
    /// would be claiming a session can render with less than the arithmetic the D7 policy, the
    /// fader, the pan and the routing require of every block; and if `gain_pan_only` ever stopped
    /// matching `dispatch_only`, the claim that a 0 dB fader and the row's settled pan cost what
    /// real trims and pans cost would have been quietly abandoned in the table rather than argued
    /// in the ruling. The builtins-less plumbing row that used to anchor the first half was
    /// retired, because no host compiles such a plan; the identity inventory took its place.
    #[test]
    fn the_identity_inventory_is_the_floor_of_the_table_and_the_identity_pair_shares_it() {
        let identity = floor_row(Workload::SixtyFourTrackDispatchOnly).expect("a derived row");
        for workload in native_session_rows() {
            let Some(row) = floor_row(workload) else {
                continue;
            };
            assert!(
                row.cycles_per_lane_sample() >= identity.cycles_per_lane_sample(),
                "{} is costed below the identity inventory every row must pay",
                workload.kind()
            );
        }
        let gain_pan = floor_row(Workload::SixtyFourTrackGainPanOnly).expect("a derived row");
        assert_eq!(identity.basis, gain_pan.basis);
        assert!(
            (identity.cycles_per_lane_sample() - gain_pan.cycles_per_lane_sample()).abs() < 1.0e-9
        );
        assert!(
            (identity.cycles_per_lane_sample()
                - BUILTINS_IDENTITY_LANE_OPS / (BANK_WIDTH * OPS_PER_CYCLE))
                .abs()
                < 1.0e-12
        );
    }

    /// The driver-fed gain/pan row (issues #928 and #956) is costed at the identity inventory
    /// exactly, and it isolates nothing.
    ///
    /// Equal to its bound twin's floor, never below it: the floor-of-the-table test above holds
    /// with `>=`, and this pins that the second row at that inventory is the gain/pan row's
    /// arithmetic restated -- same basis, same cycles -- rather than a new, cheaper inventory. And
    /// like its twin it names no control and nothing names either of them: a feed change is not
    /// an arithmetic change, so a subtraction between the two feeds would publish the bound feed's
    /// copies as an isolate against a floor that has no copy in it.
    #[test]
    fn the_driver_fed_gain_pan_row_is_costed_at_the_identity_inventory_and_isolates_nothing() {
        let gain_pan = floor_row(Workload::SixtyFourTrackGainPanOnly).expect("a derived row");
        let ring = floor_row(Workload::SixtyFourTrackGainPanRing).expect("a derived row");
        assert_eq!(ring.basis, gain_pan.basis);
        assert_eq!(
            ring.basis,
            "docs/rulings/effect-floor-accounting.md: builtins, identity"
        );
        assert!(
            (ring.cycles_per_lane_sample() - gain_pan.cycles_per_lane_sample()).abs() < 1.0e-12
        );
        assert!(
            (ring.cycles_per_lane_sample()
                - BUILTINS_IDENTITY_LANE_OPS / (BANK_WIDTH * OPS_PER_CYCLE))
                .abs()
                < 1.0e-12
        );
        assert!(ring.control.is_none() && gain_pan.control.is_none());
        for workload in native_session_rows() {
            let control = floor_row(workload).and_then(|row| row.control);
            assert!(
                control != Some(Workload::SixtyFourTrackGainPanRing)
                    && control != Some(Workload::SixtyFourTrackGainPanOnly),
                "{} isolates against a gain/pan row",
                workload.kind()
            );
        }
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
    fn the_current_effect_recount_keeps_fractional_link_work_and_composes_the_strip() {
        // The linked stereo max is shared by the two channel samples, hence the .5 in the
        // compressor inventory. The limiter's shared link has the same accounting shape.
        assert_eq!(COMPRESSOR_LANE_OPS, 81.5);
        assert_eq!(LIMITER_LANE_OPS, 129.5);
        assert_eq!(EQ_LANE_OPS, 32.0);
        let console = floor_row(Workload::SixtyFourTrackConsole).expect("derived console row");
        let expected = (79.0 + 32.0 + 81.5 + 129.5) / (BANK_WIDTH * OPS_PER_CYCLE);
        assert!((console.cycles_per_lane_sample() - expected).abs() < 1.0e-12);
        let compressor = floor_row(Workload::SixtyFourTrackCompressorOnly).expect("compressor");
        let builtins = floor_row(Workload::SixtyFourTrackBuiltinsOnly).expect("builtins");
        assert!(
            ((compressor.cycles_per_lane_sample() - builtins.cycles_per_lane_sample())
                - COMPRESSOR_LANE_OPS / (BANK_WIDTH * OPS_PER_CYCLE))
                .abs()
                < 1.0e-12
        );
    }
}
