//! End-to-end checks against the frozen canonical-PCM vector corpus.

use std::{
    ffi::OsString,
    fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
};

use stem_hasher::{CanonicalBitDepth, CanonicalPcmShape, canonicalize_raw_pcm, canonicalize_wave};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/stem-identity/v1"
);
const BINARY: &str = env!("CARGO_BIN_EXE_stem-hasher");

struct Vector<'a> {
    name: &'a str,
    bit_depth: &'a str,
    channels: u16,
    frames: u64,
    canonical_hex: &'a str,
    identity: &'a str,
    pcm_file: &'a str,
    wave_file: Option<&'a str>,
}

#[test]
fn vector_matrix_covers_integer_boundaries_and_float_edge_bits() {
    let manifest =
        fs::read_to_string(Path::new(FIXTURES).join("VECTORS.tsv")).expect("read vector manifest");
    let vectors = parse_vectors(&manifest);
    assert_eq!(vectors.len(), 6);
    assert_eq!(
        vectors
            .iter()
            .map(|vector| (vector.bit_depth, vector.channels))
            .collect::<Vec<_>>(),
        [
            ("32f", 1),
            ("32f", 2),
            ("16", 1),
            ("16", 2),
            ("24", 1),
            ("24", 2),
        ]
    );
}

#[test]
fn f32_mono_edge_bit_pin_matches_library_and_cli() {
    assert_vector("f32-mono-edge-bits");
}

#[test]
fn f32_stereo_edge_bit_pin_matches_library_cli_and_engine_wave_parser() {
    assert_vector("f32-stereo-edge-bits");
}

#[test]
fn pcm16_mono_boundary_pin_matches_library_and_cli() {
    assert_vector("pcm16-mono-boundaries");
}

#[test]
fn pcm16_stereo_boundary_pin_matches_library_cli_and_engine_wave_parser() {
    assert_vector("pcm16-stereo-boundaries");
}

#[test]
fn pcm24_mono_boundary_pin_matches_library_and_cli() {
    assert_vector("pcm24-mono-boundaries");
}

#[test]
fn pcm24_stereo_boundary_pin_matches_library_cli_and_engine_wave_parser() {
    assert_vector("pcm24-stereo-boundaries");
}

#[test]
fn stdout_mode_keeps_canonical_bytes_and_identity_separate() {
    let manifest =
        fs::read_to_string(Path::new(FIXTURES).join("VECTORS.tsv")).expect("read vector manifest");
    let vectors = parse_vectors(&manifest);
    let first = &vectors[0];
    let stdout_mode = Command::new(BINARY)
        .args(raw_arguments(
            first,
            &Path::new(FIXTURES).join(first.pcm_file),
            Some(Path::new("-")),
        ))
        .output()
        .expect("run stdout CLI");
    assert!(stdout_mode.status.success());
    assert_eq!(
        stdout_mode.stdout,
        fs::read(Path::new(FIXTURES).join(first.pcm_file)).expect("PCM")
    );
    assert_eq!(
        stdout_mode.stderr,
        format!("{}\n", first.identity).as_bytes()
    );
}

fn assert_vector(name: &str) {
    let manifest =
        fs::read_to_string(Path::new(FIXTURES).join("VECTORS.tsv")).expect("read vector manifest");
    let vectors = parse_vectors(&manifest);
    let vector = vectors
        .iter()
        .find(|vector| vector.name == name)
        .expect("named vector");
    let temp = create_temp_dir();
    let pcm_path = Path::new(FIXTURES).join(vector.pcm_file);
    let pcm = fs::read(&pcm_path).expect("read PCM vector");
    assert_eq!(lowercase_hex(&pcm), vector.canonical_hex, "{}", vector.name);
    assert_eq!(
        u64::try_from(pcm.len()).expect("fixture length"),
        vector.frames
            * u64::from(vector.channels)
            * u64::from(bit_depth(vector.bit_depth).bytes_per_sample()),
        "{}",
        vector.name
    );

    let shape = CanonicalPcmShape::new(vector.channels, bit_depth(vector.bit_depth), vector.frames)
        .expect("shape");
    let mut canonical = Vec::new();
    let report = canonicalize_raw_pcm(&mut &pcm[..], shape, &mut canonical).expect("raw hash");
    assert_eq!(canonical, pcm, "{}", vector.name);
    assert_eq!(report.identity(), vector.identity, "{}", vector.name);

    let output = temp.join(format!("{}.pcm", vector.name));
    let cli = Command::new(BINARY)
        .args(raw_arguments(vector, &pcm_path, Some(&output)))
        .output()
        .expect("run raw CLI");
    assert!(cli.status.success(), "{}: {:?}", vector.name, cli.stderr);
    assert_eq!(
        cli.stdout,
        format!("{}\n", vector.identity).as_bytes(),
        "{}",
        vector.name
    );
    assert!(cli.stderr.is_empty(), "{}", vector.name);
    assert_eq!(
        fs::read(output).expect("CLI output"),
        pcm,
        "{}",
        vector.name
    );

    if let Some(wave_file) = vector.wave_file {
        let wave_path = Path::new(FIXTURES).join(wave_file);
        let mut wave = File::open(&wave_path).expect("open WAVE vector");
        let mut wave_canonical = Vec::new();
        let wave_report =
            canonicalize_wave(&mut wave, &mut wave_canonical).expect("engine WAVE path");
        assert_eq!(wave_report.shape, shape, "{}", vector.name);
        assert_eq!(wave_report.identity(), vector.identity, "{}", vector.name);
        assert_eq!(wave_canonical, pcm, "{}", vector.name);

        let wave_output = temp.join(format!("{}-wave.pcm", vector.name));
        let cli = Command::new(BINARY)
            .args([OsString::from("wave"), OsString::from("--input")])
            .arg(&wave_path)
            .arg("--output")
            .arg(&wave_output)
            .output()
            .expect("run wave CLI");
        assert!(cli.status.success(), "{}: {:?}", vector.name, cli.stderr);
        assert_eq!(cli.stdout, format!("{}\n", vector.identity).as_bytes());
        assert_eq!(fs::read(wave_output).expect("WAVE CLI output"), pcm);
    }
    fs::remove_dir_all(temp).expect("remove test directory");
}

fn parse_vectors(manifest: &str) -> Vec<Vector<'_>> {
    let mut lines = manifest.lines();
    assert_eq!(lines.next(), Some("schema_version\t1"));
    assert_eq!(
        lines.next(),
        Some(
            "name\tbit_depth\tchannels\tframes\tsamples_by_frame\tcanonical_hex\tidentity\tpcm_file\twave_file"
        )
    );
    lines
        .map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 9, "{line}");
            Vector {
                name: fields[0],
                bit_depth: fields[1],
                channels: fields[2].parse().expect("channels"),
                frames: fields[3].parse().expect("frames"),
                canonical_hex: fields[5],
                identity: fields[6],
                pcm_file: fields[7],
                wave_file: (fields[8] != "-").then_some(fields[8]),
            }
        })
        .collect()
}

fn raw_arguments(vector: &Vector<'_>, input: &Path, output: Option<&Path>) -> Vec<OsString> {
    let mut arguments = vec![
        OsString::from("raw"),
        OsString::from("--input"),
        input.as_os_str().to_owned(),
        OsString::from("--channels"),
        OsString::from(vector.channels.to_string()),
        OsString::from("--bit-depth"),
        OsString::from(vector.bit_depth),
        OsString::from("--frames"),
        OsString::from(vector.frames.to_string()),
    ];
    if let Some(output) = output {
        arguments.push(OsString::from("--output"));
        arguments.push(output.as_os_str().to_owned());
    }
    arguments
}

fn bit_depth(token: &str) -> CanonicalBitDepth {
    CanonicalBitDepth::from_token(token).expect("bit depth")
}

fn lowercase_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn create_temp_dir() -> PathBuf {
    for nonce in 0_u32..100 {
        let path = std::env::temp_dir().join(format!("stem-hasher-{}-{nonce}", std::process::id()));
        if fs::create_dir(&path).is_ok() {
            return path;
        }
    }
    panic!("could not create test directory");
}
