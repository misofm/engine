//! The one authorized descriptive capture for issue #602.
//!
//! Preparation, publication, PCM collection and hashing are deliberately outside the timing
//! closure. The only operation observed by the clock is `PreparedRenderPlan::render`.

use super::input_symmetry::{
    FIXTURE_ID, Owner, PREPARATION_BLOCKS, QUALIFICATION_BLOCKS, QUANTUM, RECORDS_PER_BLOCK,
    SAMPLE_RATE_HZ, SMOOTHING_SAMPLES, TRACKS,
};
use bench_support::alloc as bench_alloc;
use bench_support::json::escape;
use bench_support::metadata::Metadata;
use bench_support::timing;
use lane::Backend;

const ISSUE: u64 = 602;
const SCHEMA_VERSION: u64 = 1;
const ROUNDS: usize = 2;
const RENDER_DENOMINATOR: u64 = QUALIFICATION_BLOCKS * 2;
const REVIEWED_PHASE_DIGEST: &str =
    "75eeffae6a0116867d6d6fbe589834df53f3dce87ffd09092f22e5968bc28f87";

#[derive(Debug, Eq, PartialEq)]
struct TimedRound {
    elapsed_ns: u64,
    left: super::input_symmetry::PhaseResult,
    right: super::input_symmetry::PhaseResult,
}

fn checked_sum(left: u64, right: u64) -> u64 {
    left.checked_add(right)
        .expect("capture arithmetic overflow")
}

fn ns_per_render(elapsed_ns: u64) -> u64 {
    assert!(elapsed_ns > 0, "capture clock must be positive");
    let value = elapsed_ns
        .checked_div(RENDER_DENOMINATOR)
        .expect("capture denominator is nonzero");
    assert!(value > 0, "capture clock resolution is too small");
    value
}

fn timed_phase(left: &mut Owner, right: &mut Owner) -> TimedRound {
    left.begin_phase();
    right.begin_phase();
    let mut elapsed_ns = 0;
    for _ in 0..QUALIFICATION_BLOCKS {
        left.publish();
        let (left_elapsed, _) =
            left.render_observed(|plan, io, time| timing::timed(|| plan.render(io, time)));
        left.absorb();
        left.observe_nonzero();
        elapsed_ns = checked_sum(elapsed_ns, left_elapsed);

        right.publish();
        let (right_elapsed, _) =
            right.render_observed(|plan, io, time| timing::timed(|| plan.render(io, time)));
        right.absorb();
        right.observe_nonzero();
        elapsed_ns = checked_sum(elapsed_ns, right_elapsed);
    }
    let left_result = left.phase_snapshot();
    let right_result = right.phase_snapshot();
    assert_eq!(left_result, right_result, "capture owners diverged");
    for result in [&left_result, &right_result] {
        assert_eq!(
            result.attempted,
            QUALIFICATION_BLOCKS * RECORDS_PER_BLOCK as u64
        );
        assert_eq!(result.accepted, result.attempted);
        assert_eq!(result.rendered, QUALIFICATION_BLOCKS);
        assert_eq!(result.render_errors, 0);
        assert_eq!(
            result.output_words,
            QUALIFICATION_BLOCKS * (QUANTUM * 2) as u64
        );
        assert!(result.nonzero_samples > 0, "capture PCM is all zero");
        assert_eq!(result.digest, REVIEWED_PHASE_DIGEST);
    }
    TimedRound {
        elapsed_ns,
        left: left_result,
        right: right_result,
    }
}

fn required_env(name: &str) -> String {
    Metadata::gather()
        .var(name)
        .unwrap_or_else(|_| panic!("capture requires sealed {name}"))
}

fn record(round: usize, result: &TimedRound) -> String {
    let metadata = Metadata::gather();
    let attempted = result
        .left
        .attempted
        .checked_add(result.right.attempted)
        .unwrap();
    let accepted = result
        .left
        .accepted
        .checked_add(result.right.accepted)
        .unwrap();
    let rendered = result
        .left
        .rendered
        .checked_add(result.right.rendered)
        .unwrap();
    let errors = result
        .left
        .render_errors
        .checked_add(result.right.render_errors)
        .unwrap();
    let words = result
        .left
        .output_words
        .checked_add(result.right.output_words)
        .unwrap();
    let nonzero = result
        .left
        .nonzero_samples
        .checked_add(result.right.nonzero_samples)
        .unwrap();
    let argv = required_env("MISO_ENGINE_CAPTURE_ARGV");
    let cwd = required_env("MISO_ENGINE_CAPTURE_CWD");
    format!(
        concat!(
            "{{\"schema_version\":{schema_version},\"issue\":{issue},",
            "\"kind\":\"input_symmetry_capture\",\"round\":{round},",
            "\"fixture_id\":\"{fixture}\",\"fixture_sha256\":\"{fixture_hash}\",",
            "\"sample_rate_hz\":{sample_rate},\"quantum_frames\":{quantum},",
            "\"lane_width\":8,\"track_count\":{tracks},\"records_per_block\":{records_per_block},",
            "\"target_pair_db\":[-6.0,-12.0],\"smoothing_samples\":{smoothing},",
            "\"preparation_blocks_per_owner\":{preparation},",
            "\"measured_blocks_per_owner\":{measured},\"owners\":2,",
            "\"render_denominator\":{denominator},\"attempted_records\":{attempted},",
            "\"accepted_records\":{accepted},\"successful_renders\":{rendered},",
            "\"render_errors\":{errors},\"output_words\":{words},\"nonzero_samples\":{nonzero},",
            "\"owner_digests\":[\"{left}\",\"{right}\"],",
            "\"elapsed_ns\":{elapsed},\"nanoseconds_per_plan_render\":{per_render},",
            "\"source_commit\":\"{commit}\",\"source_tree\":\"{tree}\",",
            "\"binary_sha256\":\"{binary}\",\"source_sha256\":\"{source}\",",
            "\"argv\":\"{argv}\",\"cwd\":\"{cwd}\",",
            "\"compiler\":\"{compiler}\",",
            "\"effective_build_flags\":\"{flags}\",\"backend\":\"Simd8\",",
            "{metadata}\"descriptive_only\":true}}"
        ),
        fixture = escape(FIXTURE_ID),
        schema_version = SCHEMA_VERSION,
        issue = ISSUE,
        sample_rate = SAMPLE_RATE_HZ,
        quantum = QUANTUM,
        tracks = TRACKS,
        records_per_block = RECORDS_PER_BLOCK,
        smoothing = SMOOTHING_SAMPLES,
        preparation = PREPARATION_BLOCKS,
        measured = QUALIFICATION_BLOCKS,
        denominator = RENDER_DENOMINATOR,
        round = round,
        attempted = attempted,
        accepted = accepted,
        rendered = rendered,
        errors = errors,
        words = words,
        nonzero = nonzero,
        fixture_hash = required_env("MISO_ENGINE_CAPTURE_FIXTURE_SHA256"),
        left = result.left.digest,
        right = result.right.digest,
        elapsed = result.elapsed_ns,
        per_render = ns_per_render(result.elapsed_ns),
        commit = required_env("MISO_ENGINE_CAPTURE_COMMIT"),
        tree = required_env("MISO_ENGINE_CAPTURE_TREE"),
        binary = required_env("MISO_ENGINE_CAPTURE_BINARY_SHA256"),
        source = required_env("MISO_ENGINE_CAPTURE_SOURCE_SHA256"),
        compiler = required_env("MISO_ENGINE_CAPTURE_COMPILER"),
        flags = required_env("MISO_ENGINE_CAPTURE_FLAGS"),
        argv = escape(&argv),
        cwd = escape(&cwd),
        metadata = metadata.record_fields(),
    )
}

pub(crate) fn main() {
    bench_alloc::assert_installed();
    assert!(
        std::env::args_os().nth(1).is_none(),
        "capture takes no arguments"
    );
    assert_eq!(
        Backend::current(),
        Backend::Simd8,
        "capture requires native W8 dispatch"
    );
    let mut owners = [Owner::prepare(), Owner::prepare()];
    owners[0].warmup();
    owners[1].warmup();
    eprintln!("MISO_ENGINE_CAPTURE_PHASE capture_started");
    let (left, right) = owners.split_at_mut(1);
    let left = &mut left[0];
    let right = &mut right[0];
    for round in 1..=ROUNDS {
        let result = timed_phase(left, right);
        println!("{}", record(round, &result));
        eprintln!("MISO_ENGINE_CAPTURE_PHASE round_{round}_complete");
    }
}

#[cfg(test)]
mod tests {
    use super::{checked_sum, ns_per_render};

    #[test]
    fn injected_clock_arithmetic_is_checked_and_untimed() {
        let elapsed = (1_000_000_u64..1_008_192).fold(0, checked_sum);
        assert_eq!(elapsed, 8_225_550_336);
        assert_eq!(ns_per_render(8_192 * 123), 123);
    }

    #[test]
    #[should_panic(expected = "capture clock must be positive")]
    fn zero_clock_is_rejected() {
        let _ = ns_per_render(0);
    }
}
