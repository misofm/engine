//! Generates, verifies, or fingerprints the checked-in issue-006 graph fixtures.

use core::fmt::Write as _;
use std::{
    env, fs,
    io::Write as _,
    path::{Path, PathBuf},
};

use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::GraphCompileCaps;
use miso_engine_graph_compiler::{GraphCompileReport, GraphCompileRequest, GraphCompiler};
use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};
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
            println!("{}", fingerprint(&compile_fixture()));
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
            "usage: miso_engine_graph_fixture [--check [ROOT] | --write [ROOT] | ",
            "--manifest | --emit PATH]"
        )
        .to_owned()),
    }
}

fn default_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/graph")
}

fn compile_fixture() -> GraphCompileReport {
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
    .report
}

fn generated() -> Vec<(String, Vec<u8>)> {
    let report = compile_fixture();
    let fingerprint = format!("{}\n", fingerprint(&report)).into_bytes();
    vec![
        (
            "v1/direct-route.canonical.txt".to_owned(),
            report.canonical_debug_bytes,
        ),
        ("v1/direct-route.dot".to_owned(), report.dot.into_bytes()),
        ("v1/direct-route.report.json".to_owned(), fingerprint),
    ]
}

fn fingerprint(report: &GraphCompileReport) -> String {
    format!(
        concat!(
            "{{\"schema\":1,\"fixture\":\"direct-route\",",
            "\"canonical_bytes\":{},\"graph_sha256\":\"{}\",",
            "\"dot_bytes\":{},\"dot_sha256\":\"{}\",",
            "\"nodes\":{},\"edges\":{},\"schedule_items\":{},",
            "\"levels\":{},\"route_timings\":{},\"buffer_assignments\":{}}}"
        ),
        report.canonical_debug_bytes.len(),
        report.sha256,
        report.dot.len(),
        sha256_hex(report.dot.as_bytes()),
        report.nodes.len(),
        report.edges.len(),
        report.sequential_schedule.len(),
        report.dependency_levels.len(),
        report.route_timings.len(),
        report.buffer_assignments.len(),
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
        env::temp_dir().join(format!(
            "miso-engine-graph-fixture-test-{}",
            std::process::id()
        ))
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
