//! Generates and verifies the deliberately small Issue-008 rack vertical corpus.

use core::fmt::Write as _;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

const TRACKS: usize = 12;
const FRAMES: usize = 128;
const MANIFEST_HEADER: &str = "path\tlength\tsha256\n";

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [] => verify(&default_root()),
        [flag] if flag == "--check" => verify(&default_root()),
        [flag, root] if flag == "--check" => verify(Path::new(root)),
        [flag] if flag == "--write" => write_and_verify(&default_root()),
        [flag, root] if flag == "--write" => write_and_verify(Path::new(root)),
        _ => Err("usage: rack_fixture [--check [ROOT] | --write [ROOT]]".to_owned()),
    }
}

fn default_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/rack/v1")
}

fn generated() -> Vec<(&'static str, Vec<u8>)> {
    let cases = concat!(
        "schema = 1\n",
        "sample_rate_hz = 48000\n",
        "quantum_frames = 128\n",
        "tracks = 12\n",
        "input_layout = \"track-major-planar-f32le:left,right\"\n",
        "expected_layout = \"planar-f32le:left,right\"\n",
        "effect = \"conformance.delay\"\n",
        "effect_latency_samples = 3\n",
        "simd_rack = \"simd1\"\n",
        "compatible_tracks = [\"rack00\", \"rack01\", \"rack02\", \"rack03\", \"rack04\", \"rack05\", \"rack06\", \"rack07\"]\n",
        "identity_missing_middle_slot_tracks = [\"rack08\", \"rack09\"]\n",
        "scalar_tail_tracks = [\"rack10\", \"rack11\"]\n",
        "scalar_only_sidechain_track = \"sidechain-fallback\"\n",
        "notes = \"The checked expected PCM is produced by the frozen scalar dual-mono delay operation: each lane has a separate three-sample state and gain. The graph vertical sums twelve independently routed tracks; the sidechain declaration is a compiler fallback case, not a bank member.\"\n",
    )
    .as_bytes()
    .to_vec();
    let input = input_bytes();
    let expected = scalar_expected_bytes();
    vec![
        ("cases.toml", cases),
        ("input.f32le", input),
        ("scalar-expected.f32le", expected),
    ]
}

fn input_sample(track: usize, channel: usize, frame: usize) -> f32 {
    let ramp = (frame as f32 - 31.0) / 97.0;
    let amplitude = (track + 1) as f32 * 0.003_125;
    if channel == 0 {
        ramp * amplitude
    } else {
        (0.75 - ramp) * -amplitude
    }
}

fn input_bytes() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(TRACKS * 2 * FRAMES * core::mem::size_of::<f32>());
    for track in 0..TRACKS {
        for channel in 0..2 {
            for frame in 0..FRAMES {
                bytes.extend_from_slice(&input_sample(track, channel, frame).to_le_bytes());
            }
        }
    }
    bytes
}

/// Independent, scalar reference for the accepted conformance-delay fixture. This preserves the
/// dual-mono three-sample state and per-track gain used by the graph vertical without using any
/// bank implementation to manufacture expected output.
fn scalar_expected_bytes() -> Vec<u8> {
    let mut channels = [[0.0_f32; FRAMES]; 2];
    for track in 0..TRACKS {
        let gains = [1.0 + track as f32 * 0.01, 1.0 - track as f32 * 0.0075];
        for channel in 0..2 {
            for (frame, output) in channels[channel].iter_mut().enumerate().skip(3) {
                *output += input_sample(track, channel, frame - 3) * gains[channel];
            }
        }
    }
    let mut bytes = Vec::with_capacity(2 * FRAMES * core::mem::size_of::<f32>());
    for channel in channels {
        for sample in channel {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
    }
    bytes
}

fn manifest(files: &[(&str, Vec<u8>)]) -> String {
    let mut result = String::from(MANIFEST_HEADER);
    for (path, bytes) in files {
        writeln!(result, "{path}\t{}\t{}", bytes.len(), sha256_hex(bytes)).expect("string");
    }
    result
}

fn write_and_verify(root: &Path) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|error| format!("create rack fixture directory: {error}"))?;
    let files = generated();
    for (path, bytes) in &files {
        fs::write(root.join(path), bytes)
            .map_err(|error| format!("write rack fixture {path}: {error}"))?;
    }
    fs::write(root.join("MANIFEST.tsv"), manifest(&files))
        .map_err(|error| format!("write rack fixture manifest: {error}"))?;
    verify(root)
}

fn verify(root: &Path) -> Result<(), String> {
    let files = generated();
    let expected_manifest = manifest(&files);
    let actual_manifest = fs::read(root.join("MANIFEST.tsv"))
        .map_err(|error| format!("read rack fixture manifest: {error}"))?;
    if actual_manifest != expected_manifest.as_bytes() {
        return Err("rack fixture manifest mismatch".to_owned());
    }
    for (path, expected) in &files {
        let actual = fs::read(root.join(path))
            .map_err(|error| format!("read rack fixture {path}: {error}"))?;
        if actual != *expected {
            return Err(format!("rack fixture content mismatch: {path}"));
        }
    }
    let mut actual_paths = fs::read_dir(root)
        .map_err(|error| format!("read rack fixture directory: {error}"))?
        .map(|entry| {
            let entry = entry.map_err(|error| format!("read rack fixture entry: {error}"))?;
            if !entry
                .file_type()
                .map_err(|error| format!("read rack fixture type: {error}"))?
                .is_file()
            {
                return Err("rack fixture directory contains a non-file".to_owned());
            }
            entry
                .file_name()
                .into_string()
                .map_err(|_| "rack fixture name is not UTF-8".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    actual_paths.sort();
    let mut expected_paths: Vec<_> = files.iter().map(|(path, _)| (*path).to_owned()).collect();
    expected_paths.push("MANIFEST.tsv".to_owned());
    expected_paths.sort();
    if actual_paths != expected_paths {
        return Err("rack fixture directory has missing or unlisted files".to_owned());
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(output, "{byte:02x}").expect("string");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_root() -> PathBuf {
        env::temp_dir().join(format!("rack-fixture-test-{}", std::process::id()))
    }

    #[test]
    fn checker_rejects_content_manifest_missing_and_unlisted_corruption() {
        let root = temporary_root();
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale fixture");
        }
        write_and_verify(&root).expect("write corpus");
        let files = generated();
        for (path, bytes) in &files {
            let mut corrupt = bytes.clone();
            corrupt.push(0);
            fs::write(root.join(path), corrupt).expect("corrupt content");
            assert!(
                verify(&root).is_err(),
                "accepted content corruption: {path}"
            );
            fs::write(root.join(path), bytes).expect("restore content");
        }
        let manifest_path = root.join("MANIFEST.tsv");
        fs::write(&manifest_path, b"corrupt\n").expect("corrupt manifest");
        assert!(verify(&root).is_err(), "accepted manifest corruption");
        fs::write(&manifest_path, manifest(&files)).expect("restore manifest");
        fs::write(root.join("unlisted"), []).expect("write unlisted");
        assert!(verify(&root).is_err(), "accepted unlisted file");
        fs::remove_file(root.join("unlisted")).expect("remove unlisted");
        fs::remove_file(root.join(files[0].0)).expect("remove listed file");
        assert!(verify(&root).is_err(), "accepted missing file");
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
