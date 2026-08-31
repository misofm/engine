//! Generates, verifies, or fingerprints the checked-in issue-006 graph fixtures.

use core::fmt::Write as _;
use graph_compiler::Backend;
use std::{
    env, fs,
    io::Write as _,
    path::{Path, PathBuf},
};

use effect_compiler::EffectPreparedSession;
use graph::{GraphCompileCaps, PreparedGraphPlan, reduce_left_to_right};
use graph_compiler::{GraphCompileRequest, GraphCompiler, GraphEvidence, PreparedGraphArtifact};
use session::{CompileCaps, compile_session, parse_session_toml};
use sha2::{Digest, Sha256};

const SESSION: &str = include_str!("../../../../fixtures/session/v1/canonical.toml");
const MANIFEST_HEADER: &str = "path\tlength\tsha256\n";

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    let root = default_root();
    match arguments.as_slice() {
        [] => {
            let artifact = compile_fixture();
            let evidence = GraphCompiler::evidence(&artifact.graph, &artifact.report);
            println!("{}", fingerprint(&artifact.graph, &evidence));
            Ok(())
        }
        [mode] if mode == "--check" => verify(&root, &generated()),
        [mode, supplied_root] if mode == "--check" => {
            verify(Path::new(supplied_root), &generated())
        }
        [mode] if mode == "--write" => write_and_verify(&root),
        [mode, supplied_root] if mode == "--write" => write_and_verify(Path::new(supplied_root)),
        [mode] if mode == "--manifest" => {
            print!("{}", manifest(&generated()));
            Ok(())
        }
        [mode, path] if mode == "--emit" => {
            let files = generated();
            let bytes = files
                .iter()
                .find_map(|(candidate, bytes)| (candidate == path).then_some(bytes))
                .ok_or_else(|| format!("unknown generated fixture path: {path}"))?;
            std::io::stdout()
                .write_all(bytes)
                .map_err(|error| format!("write stdout: {error}"))
        }
        _ => Err(concat!(
            "usage: graph_fixture [--check [ROOT] | --write [ROOT] | ",
            "--manifest | --emit PATH]"
        )
        .to_owned()),
    }
}

fn default_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/graph")
}

fn compile_fixture() -> PreparedGraphArtifact {
    let mut model = parse_session_toml(SESSION).unwrap_or_else(|diagnostics| {
        panic!("session parse diagnostics: {diagnostics:?}");
    });
    model.tracks[0].dynamic.effects.clear();
    model.automation.clear();
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .unwrap_or_else(|diagnostics| panic!("session compile diagnostics: {diagnostics:?}"));
    GraphCompiler::compile(GraphCompileRequest {
        dispatch: Backend::Scalar,
        plan_id: 0,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        caps: GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|failure| panic!("graph compile diagnostics: {:?}", failure.diagnostics))
}

fn generated() -> Vec<(String, Vec<u8>)> {
    let artifact = compile_fixture();
    let graph = &artifact.graph;
    let report = &artifact.report;
    // #99 F5: evidence is produced here, off the compile path, exactly once.
    let evidence = GraphCompiler::evidence(graph, report);
    let fingerprint = format!("{}\n", fingerprint(graph, &evidence)).into_bytes();
    let colored_buffers = graph
        .buffer_assignments
        .iter()
        .map(|assignment| assignment.buffer_index)
        .max()
        .map_or(0, |maximum| maximum + 1);
    let resource_report = format!(
        concat!(
            "{{\"schema\":1,\"fixture\":\"direct-route\",\"logical_nodes\":{},",
            "\"materialized_nodes\":{},\"edges\":{},\"schedule_items\":{},",
            "\"dependency_levels\":{},\"colored_output_buffers\":{},",
            "\"audio_buffer_samples\":{},\"delay_bytes\":{},",
            "\"graph_metadata_bytes\":{},\"declared_effect_bytes\":{},",
            "\"largest_allocation_bytes\":{},\"incremental_plan_bytes\":{},",
            "\"session_plus_plan_bytes\":{}}}\n"
        ),
        report.estimate.logical_nodes,
        report.estimate.materialized_nodes,
        report.estimate.edges,
        report.estimate.schedule_items,
        report.estimate.dependency_levels,
        colored_buffers,
        report.estimate.audio_buffer_samples,
        report.estimate.delay_bytes,
        report.estimate.graph_metadata_bytes,
        report.estimate.declared_effect_bytes,
        report.estimate.largest_allocation_bytes,
        report.estimate.incremental_plan_bytes,
        report.estimate.session_plus_plan_bytes,
    )
    .into_bytes();
    let mut files = vec![
        (
            "v1/direct-route.canonical.txt".to_owned(),
            evidence.canonical_bytes,
        ),
        ("v1/direct-route.dot".to_owned(), evidence.dot.into_bytes()),
        (
            "v1/invalid-scc-diagnostics.json".to_owned(),
            concat!(
                "{\"schema\":1,\"diagnostics\":[",
                "{\"code\":\"graph.cycle\",\"path\":\"$.routes[id=ab]\",",
                "\"cycle\":[\"submix:a\",\"submix:b\",\"submix:a\"],",
                "\"cycle_edge_paths\":[\"$.routes[id=ab]\",\"$.routes[id=ba]\"]},",
                "{\"code\":\"graph.cycle\",\"path\":\"$.routes[id=cc]\",",
                "\"cycle\":[\"submix:c\",\"submix:c\"],",
                "\"cycle_edge_paths\":[\"$.routes[id=cc]\"]}]}\n"
            )
            .as_bytes()
            .to_vec(),
        ),
        (
            "v1/main-sidechain-pdc.csv".to_owned(),
            concat!(
                "fixture,sample_rate_hz,quantum_frames,delayed_port,frame,left,right\n",
                "faster-main,48000,4,main,0,0,0\n",
                "faster-main,48000,4,main,1,0,0\n",
                "faster-main,48000,4,main,2,2,20\n",
                "faster-main,48000,4,main,3,0,0\n",
                "faster-sidechain,48000,4,sidechain,0,0,0\n",
                "faster-sidechain,48000,4,sidechain,1,0,0\n",
                "faster-sidechain,48000,4,sidechain,2,2,20\n",
                "faster-sidechain,48000,4,sidechain,3,0,0\n"
            )
            .as_bytes()
            .to_vec(),
        ),
        ("v1/direct-route.report.json".to_owned(), fingerprint),
        ("v1/direct-route.resources.json".to_owned(), resource_report),
        (
            "v1/summation-residuals.json".to_owned(),
            summation_report().into_bytes(),
        ),
    ];
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

fn summation_report() -> String {
    let fixtures = [
        ("positive", vec![1.0_f32; 257]),
        (
            "alternating",
            (0..257)
                .map(|index| if index % 2 == 0 { 1.0 } else { -1.0 })
                .collect(),
        ),
        (
            "descending",
            (0..257)
                .map(|index| 2.0_f32.powi(-index.min(120)))
                .collect(),
        ),
        (
            "adversarial-cancellation",
            vec![1.0e20, 1.0, -1.0e20, 3.0, -2.0, 0.5, -0.5],
        ),
    ];
    let mut records = Vec::new();
    let mut maximum_absolute = 0.0_f64;
    let mut maximum_bound = 0.0_f64;
    let mut squared_residual = 0.0_f64;
    for (name, values) in fixtures {
        let reference = values.iter().map(|value| f64::from(*value)).sum::<f64>();
        let sum_abs = values
            .iter()
            .map(|value| f64::from(value.abs()))
            .sum::<f64>();
        // Master plan #83 D9 is recursive left-to-right summation, so the analytic bound is
        // `gamma_{n-1} * sum|x_i|` with `gamma_k = k u / (1 - k u)` and `u = 2^-24` (Higham,
        // *Accuracy and Stability of Numerical Algorithms*, 2nd ed., eq. 4.4): `n - 1` additions
        // instead of the balanced tree's `log2 n` levels.
        let steps = (values.len() - 1) as f64;
        let unit_roundoff = 2.0_f64.powi(-24);
        let gamma = steps * unit_roundoff / (1.0 - steps * unit_roundoff);
        let bound = gamma * sum_abs + values.len() as f64 * f64::from(f32::MIN_POSITIVE);
        let actual = f64::from(reduce_left_to_right(&values));
        let absolute = (actual - reference).abs();
        maximum_absolute = maximum_absolute.max(absolute);
        maximum_bound = maximum_bound.max(bound);
        squared_residual += absolute * absolute;
        records.push(format!(
            concat!(
                "{{\"name\":\"{}\",\"contributions\":{},\"actual\":{:e},",
                "\"reference_f64\":{:e},\"absolute_residual\":{:e},",
                "\"analytic_bound\":{:e},\"sanitized_samples\":{}}}"
            ),
            name,
            values.len(),
            actual,
            reference,
            absolute,
            bound,
            0,
        ));
    }
    let rms = (squared_residual / records.len() as f64).sqrt();
    format!(
        concat!(
            "{{\"schema\":1,\"strategy\":\"left-to-right-f32\",",
            "\"reference\":\"independent-linear-f64\",\"fixtures\":[{}],",
            "\"maximum_absolute_residual\":{:e},",
            "\"rms_residual\":{:e},\"maximum_analytic_bound\":{:e}}}\n"
        ),
        records.join(","),
        maximum_absolute,
        rms,
        maximum_bound,
    )
}

fn fingerprint(graph: &PreparedGraphPlan, evidence: &GraphEvidence) -> String {
    format!(
        concat!(
            "{{\"schema\":1,\"fixture\":\"direct-route\",",
            "\"canonical_bytes\":{},\"graph_sha256\":\"{}\",",
            "\"dot_bytes\":{},\"dot_sha256\":\"{}\",",
            "\"nodes\":{},\"edges\":{},\"schedule_items\":{},",
            "\"levels\":{},\"route_timings\":{},\"buffer_assignments\":{}}}"
        ),
        evidence.canonical_bytes.len(),
        evidence.sha256,
        evidence.dot.len(),
        sha256_hex(evidence.dot.as_bytes()),
        graph.spec.nodes.len(),
        graph.spec.edges.len(),
        graph.sequential_schedule.len(),
        graph.dependency_levels.len(),
        graph.route_timings.len(),
        graph.buffer_assignments.len(),
    )
}

fn manifest(files: &[(String, Vec<u8>)]) -> String {
    let mut output = String::from(MANIFEST_HEADER);
    for (path, bytes) in files {
        writeln!(output, "{path}\t{}\t{}", bytes.len(), sha256_hex(bytes)).expect("String write");
    }
    output
}

fn write_and_verify(root: &Path) -> Result<(), String> {
    let files = generated();
    fs::create_dir_all(root.join("v1"))
        .map_err(|error| format!("create graph fixture directory: {error}"))?;
    for (path, bytes) in &files {
        fs::write(root.join(path), bytes)
            .map_err(|error| format!("write graph fixture {path}: {error}"))?;
    }
    fs::write(root.join("MANIFEST.tsv"), manifest(&files))
        .map_err(|error| format!("write graph fixture manifest: {error}"))?;
    verify(root, &files)
}

fn verify(root: &Path, expected: &[(String, Vec<u8>)]) -> Result<(), String> {
    let expected_manifest = manifest(expected);
    let actual_manifest = fs::read(root.join("MANIFEST.tsv"))
        .map_err(|error| format!("read graph fixture manifest: {error}"))?;
    if actual_manifest != expected_manifest.as_bytes() {
        return Err("graph fixture manifest mismatch".to_owned());
    }
    for (path, generated) in expected {
        let actual = fs::read(root.join(path))
            .map_err(|error| format!("read graph fixture {path}: {error}"))?;
        if actual != *generated {
            return Err(format!("graph fixture content mismatch: {path}"));
        }
    }
    let mut actual_paths = fs::read_dir(root.join("v1"))
        .map_err(|error| format!("read graph fixture directory: {error}"))?
        .map(|entry| {
            let entry = entry.map_err(|error| format!("read graph fixture entry: {error}"))?;
            if !entry
                .file_type()
                .map_err(|error| format!("read graph fixture type: {error}"))?
                .is_file()
            {
                return Err("graph fixture directory contains a non-file".to_owned());
            }
            entry
                .file_name()
                .into_string()
                .map(|name| format!("v1/{name}"))
                .map_err(|_| "graph fixture name is not UTF-8".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    actual_paths.sort();
    let expected_paths: Vec<_> = expected.iter().map(|(path, _)| path.clone()).collect();
    if actual_paths != expected_paths {
        return Err("graph fixture directory has missing or unlisted files".to_owned());
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(&mut output, "{byte:02x}").expect("String write");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_root() -> PathBuf {
        env::temp_dir().join(format!("graph-fixture-test-{}", std::process::id()))
    }

    #[test]
    fn check_rejects_fixture_manifest_missing_and_unlisted_corruption() {
        let root = temporary_root();
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale fixture test directory");
        }
        write_and_verify(&root).expect("write valid fixture corpus");
        let files = generated();
        for (path, bytes) in &files {
            let mut corrupted = bytes.clone();
            corrupted.push(0);
            fs::write(root.join(path), corrupted).expect("corrupt fixture");
            assert!(
                verify(&root, &files).is_err(),
                "accepted corruption in {path}"
            );
            fs::write(root.join(path), bytes).expect("restore fixture");
        }

        let manifest_path = root.join("MANIFEST.tsv");
        let valid_manifest = manifest(&files);
        let mut corrupted_manifest = valid_manifest.as_bytes().to_vec();
        corrupted_manifest[0] ^= 1;
        fs::write(&manifest_path, corrupted_manifest).expect("corrupt manifest");
        assert!(verify(&root, &files).is_err());
        fs::write(&manifest_path, valid_manifest).expect("restore manifest");

        fs::write(root.join("v1/unlisted"), []).expect("write unlisted fixture");
        assert!(verify(&root, &files).is_err());
        fs::remove_file(root.join("v1/unlisted")).expect("remove unlisted fixture");
        fs::remove_file(root.join(&files[0].0)).expect("remove fixture");
        assert!(verify(&root, &files).is_err());
        fs::remove_dir_all(root).expect("remove fixture test directory");
    }
}
