//! Prints the `console_mixing_automation` row's controls, resolved against the mono console, as
//! one JSON object (issue #1003).
//!
//! The browser arm (`scripts/web-mixing-automation-benchmark.mjs`) rides the shipped
//! `host_web.wasm` with exactly these controls: the same tracks, rack slots, wire parameter ids,
//! held bases and per-block values the native row pushes. It reads them from here rather than
//! from a transcription, so the two arms cannot drift apart.
//!
//! `cargo run --locked --release -p console-workload --example mixing_automation_controls`

use console_workload::mixing_automation::{
    self, CONTROLS, MixingAutomation, PREFLIGHT_BLOCKS, PREROLL_BLOCKS, SMOOTHING_SAMPLES,
};
use console_workload::{
    QUANTUM, SessionRuntime, TONE_AMPLITUDE, TONE_RADIANS_PER_FRAME, TONE_TRACK_PHASE_RADIANS,
};

fn main() {
    let runtime = SessionRuntime::build(mixing_automation::WORKLOAD, mixing_automation::CONFIG);
    let automation = MixingAutomation::resolve(&runtime)
        .unwrap_or_else(|error| panic!("console_mixing_automation controls: {error:?}"));
    assert_eq!(automation.controls().len(), CONTROLS.len());
    let controls: Vec<String> = automation
        .controls()
        .iter()
        .map(|control| {
            format!(
                concat!(
                    "{{\"track_id\":\"{track}\",\"track_index\":{track_index},",
                    "\"slot_id\":\"{slot}\",\"effect\":\"{effect}\",\"rack\":{rack},",
                    "\"effect_index\":{effect_index},\"parameter\":\"{parameter}\",",
                    "\"parameter_index\":{parameter_index},\"parameter_id\":{parameter_id},",
                    "\"lowering\":\"{lowering}\",\"base\":{base},\"step\":{step},",
                    "\"even_value\":{even},\"odd_value\":{odd}}}"
                ),
                track = control.control.track_id,
                track_index = control.track_index,
                slot = control.slot_id,
                effect = control.control.effect.contract_id(),
                rack = control.rack,
                effect_index = control.effect_index,
                parameter = control.control.parameter,
                parameter_index = control.control.parameter_index,
                parameter_id = control.parameter_id,
                lowering = control.control.effect.lowering().name(),
                base = control.base,
                step = control.control.step,
                even = control.values[0],
                odd = control.values[1],
            )
        })
        .collect();
    // The native row's input (`console_workload::source_block`): per track, one frozen block of
    // the tone, phase-offset by track and repeated every block. The browser arm streams the same
    // tone continuously instead and states both in its record (issue #1011).
    let native_input_feed = format!(
        concat!(
            "{{\"waveform\":\"sine\",\"radians_per_frame\":{radians},\"amplitude\":{amplitude},",
            "\"track_phase_radians\":{phase},\"delivery\":\"frozen_block_per_track\",",
            "\"block_frames\":{block},\"continuous_across_blocks\":false}}"
        ),
        radians = TONE_RADIANS_PER_FRAME,
        amplitude = TONE_AMPLITUDE,
        phase = TONE_TRACK_PHASE_RADIANS,
        block = QUANTUM,
    );
    println!(
        "{{\"workload_kind\":\"{kind}\",\"fixture_id\":\"{fixture}\",\"preroll_blocks\":{preroll},\"preflight_blocks\":{preflight},\"smoothing_samples\":{smoothing},\"native_input_feed\":{native_input_feed},\"controls\":[{controls}]}}",
        kind = mixing_automation::WORKLOAD.kind(),
        fixture = mixing_automation::WORKLOAD.fixture_id(),
        preroll = PREROLL_BLOCKS,
        preflight = PREFLIGHT_BLOCKS,
        smoothing = SMOOTHING_SAMPLES,
        controls = controls.join(","),
    );
}
