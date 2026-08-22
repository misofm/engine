use core::mem::size_of;

use miso_engine_session::{canonical_session_toml, parse_session_toml};

use super::*;

fn one_track_session(quantum: u32) -> String {
    let mut model = parse_session_toml(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.toml"
    ))
    .expect("accepted fixture");
    model.quantum_frames = quantum;
    model.limits.pcm_ring_frames = u64::from(quantum);
    model.sources[0].mapping.region.length_samples = u64::from(quantum) * 2;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    canonical_session_toml(&model).expect("canonical one-track session")
}

fn prepared_host(quantum: u32) -> AudioWorkletEngineHost {
    let mut host =
        AudioWorkletEngineHost::new(WebPrepareConfigV1::launch_defaults(48_000, quantum));
    assert_eq!(host.prepare(), RESULT_OK);
    host
}

fn ready_host(quantum: u32) -> AudioWorkletEngineHost {
    let toml = one_track_session(quantum);
    let mut host = prepared_host(quantum);
    host.session_toml_mut().expect("prepared TOML buffer")[..toml.len()]
        .copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );
    host
}

#[test]
fn frozen_layouts_and_values_are_exact() {
    assert_eq!(size_of::<WebPrepareConfigV1>(), 192);
    assert_eq!(size_of::<WebStatusV1>(), 80);
    assert_eq!(size_of::<WebResourceReportV1>(), 224);
    assert_eq!(
        [
            RESULT_OK,
            RESULT_INVALID_ARGUMENT,
            RESULT_ABI_MISMATCH,
            RESULT_WRONG_STATE,
            RESULT_BUFFER_TOO_SMALL,
            RESULT_PREPARE_REJECTED,
            RESULT_BACKPRESSURE,
            RESULT_UNSUPPORTED,
            RESULT_RENDER_REJECTED,
            RESULT_REPREPARE_REQUIRED,
            RESULT_INTERNAL,
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255]
    );
    assert_eq!(
        [
            STATE_CONFIG,
            STATE_PREPARED,
            STATE_READY,
            STATE_FAILED,
            STATE_DISPOSED
        ],
        [0, 1, 2, 3, 4]
    );
    assert_eq!([BACKEND_SCALAR, BACKEND_SIMD128], [0, 1]);
    assert_eq!(
        [
            BUFFER_SESSION_TOML,
            BUFFER_SOURCE_ID,
            BUFFER_SOURCE_PCM,
            BUFFER_DIAGNOSTIC,
            BUFFER_OUTPUT_PCM
        ],
        [1, 2, 3, 4, 5]
    );
}

#[test]
fn preparation_accepts_explicit_64_128_and_256_quanta_with_stable_buffers() {
    for quantum in [64, 128, 256] {
        let mut host = prepared_host(quantum);
        assert_eq!(host.status().state, STATE_PREPARED);
        let toml_ptr = host.session_toml_mut().expect("TOML").as_ptr();
        let source_id_ptr = host.source_id_mut().expect("ID").as_ptr();
        let source_pcm_ptr = host.source_pcm_mut().expect("PCM").as_ptr();
        let output_ptr = host.output_pcm().expect("output").as_ptr();
        assert_eq!(
            host.source_pcm_mut().expect("PCM").len(),
            8 * quantum as usize
        );
        assert_eq!(
            host.output_pcm().expect("output").len(),
            2 * quantum as usize
        );
        assert_eq!(toml_ptr, host.session_toml_mut().expect("TOML").as_ptr());
        assert_eq!(source_id_ptr, host.source_id_mut().expect("ID").as_ptr());
        assert_eq!(source_pcm_ptr, host.source_pcm_mut().expect("PCM").as_ptr());
        assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    }
}

#[test]
fn malformed_config_and_atomic_compile_failure_are_sticky() {
    let mut bad = WebPrepareConfigV1::launch_defaults(48_000, 128);
    bad.abi_version = 0;
    let mut host = AudioWorkletEngineHost::new(bad);
    assert_eq!(host.prepare(), RESULT_ABI_MISMATCH);
    assert_eq!(host.status().state, STATE_CONFIG);

    let mut host = prepared_host(128);
    host.session_toml_mut().expect("TOML")[..3].copy_from_slice(b"no=");
    assert_eq!(host.compile(3), RESULT_PREPARE_REJECTED);
    assert_eq!(host.status().state, STATE_FAILED);
    assert!(!host.diagnostic().is_empty());
    assert_eq!(host.compile(3), RESULT_WRONG_STATE);
}

#[test]
fn compile_resource_caps_are_inclusive_and_one_below_rejects() {
    let reference = ready_host(128);
    let expected_host = reference.resources().bridge_retained_bytes;
    let expected_named = reference.resources().largest_named_allocation_bytes;
    drop(reference);

    let toml = one_track_session(128);
    for (host_cap, named_cap, prepare_result, compile_result) in [
        (expected_host, expected_named, RESULT_OK, RESULT_OK),
        (
            expected_host - 1,
            expected_named,
            RESULT_OK,
            RESULT_PREPARE_REJECTED,
        ),
        (
            expected_host,
            expected_named - 1,
            RESULT_PREPARE_REJECTED,
            RESULT_WRONG_STATE,
        ),
    ] {
        let mut config = WebPrepareConfigV1::launch_defaults(48_000, 128);
        config.maximum_host_retained_bytes = host_cap;
        config.maximum_named_allocation_bytes = named_cap;
        let mut host = AudioWorkletEngineHost::new(config);
        assert_eq!(host.prepare(), prepare_result);
        if prepare_result == RESULT_OK {
            host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
        }
        assert_eq!(
            host.compile(toml.len()),
            compile_result,
            "diagnostic={:?} host_cap={host_cap} named_cap={named_cap}",
            host.diagnostic()
        );
        assert_eq!(
            host.status().state,
            if compile_result == RESULT_OK {
                STATE_READY
            } else if prepare_result == RESULT_OK {
                STATE_FAILED
            } else {
                STATE_CONFIG
            }
        );
    }
}

#[test]
fn source_backpressure_seek_render_and_stable_output_are_bounded() {
    let quantum = 128_usize;
    let toml = one_track_session(quantum as u32);
    let mut config = WebPrepareConfigV1::launch_defaults(48_000, quantum as u32);
    config.source_ring_frames = quantum as u32;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    let toml_ptr = host.session_toml_mut().expect("TOML").as_ptr();
    let source_id_ptr = host.source_id_mut().expect("ID").as_ptr();
    let source_pcm_ptr = host.source_pcm_mut().expect("PCM").as_ptr();
    let output_ptr = host.output_pcm().expect("output").as_ptr();
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );
    assert_eq!(toml_ptr, host.session_toml_mut().expect("TOML").as_ptr());
    assert_eq!(source_id_ptr, host.source_id_mut().expect("ID").as_ptr());
    assert_eq!(source_pcm_ptr, host.source_pcm_mut().expect("PCM").as_ptr());
    assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    let left = vec![0.25_f32; quantum];
    let right = vec![-0.5_f32; quantum];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            0,
            48_000,
            &planes,
            quantum as u32,
            false
        ),
        RESULT_OK
    );
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            quantum as u64,
            48_000,
            &planes,
            quantum as u32,
            true
        ),
        RESULT_BACKPRESSURE
    );
    assert_eq!(host.render_next(), RESULT_OK);
    assert_eq!(host.status().next_absolute_sample, quantum as u64);
    assert_eq!(host.status().rendered_quanta, 1);
    assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            quantum as u64,
            48_000,
            &planes,
            quantum as u32,
            true
        ),
        RESULT_OK
    );
    assert_eq!(host.seek_source(b"fixture-source", 2, 0), RESULT_OK);
    assert_eq!(
        host.seek_source(b"fixture-source", 3, 0),
        RESULT_BACKPRESSURE
    );
    assert_eq!(host.render_next(), RESULT_OK);
    assert_eq!(host.status().rendered_quanta, 2);
}

#[test]
fn output_mismatch_and_disposal_are_sticky_and_idempotent() {
    let mut host = ready_host(64);
    assert_eq!(host.reject_output_quantum(128), RESULT_REPREPARE_REQUIRED);
    assert_eq!(host.status().state, STATE_FAILED);
    assert_eq!(host.render_next(), RESULT_WRONG_STATE);
    assert_eq!(host.dispose(), RESULT_OK);
    assert_eq!(host.dispose(), RESULT_OK);
    assert_eq!(host.status().state, STATE_DISPOSED);
    assert!(host.output_pcm().is_none());
}
