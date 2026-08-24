//! Offline, zero-playback preparation renderer for Issue 033.

use std::{
    collections::BTreeMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameters, ChannelParameters, DualMonoBlock, Matrix2x2,
};
use miso_engine_conformance::{FixtureLimits, PcmFixtureV1};
use sha2::{Digest, Sha256};

const RATE: u32 = 48_000;
const FRAMES: usize = 480_000;
const QUANTUM: usize = 128;
const SILENCE_FRAMES: usize = 480;
const WAVE_HEADER_BYTES: usize = 44;
const EVENT_FRAMES: [usize; 9] = [
    48_000, 96_000, 144_000, 192_000, 240_000, 288_000, 336_000, 384_000, 432_000,
];
const MATRIX_INITIAL: Matrix2x2 = Matrix2x2 {
    ll: 0.7,
    lr: 0.3,
    rl: -0.2,
    rr: 0.8,
};
const MATRIX_A: Matrix2x2 = Matrix2x2 {
    ll: 0.6,
    lr: 0.4,
    rl: -0.4,
    rr: 0.6,
};
const MATRIX_B: Matrix2x2 = Matrix2x2 {
    ll: 0.9,
    lr: -0.1,
    rl: 0.2,
    rr: 0.8,
};

#[derive(Clone, Debug, PartialEq)]
struct Stereo {
    left: Vec<f32>,
    right: Vec<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Metrics {
    rms: f64,
    peak: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct RoleOutput {
    role: &'static str,
    wave: Vec<u8>,
    metrics: Metrics,
}

pub(crate) fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    let [mode, source, probe, provenance, seed, output] = arguments.as_slice() else {
        return Err(
            "usage: miso_engine_builtins_fixture_listening --render SOURCE.mepcm PROBE.mepcm PROVENANCE.json SEED_FILE OUTPUT_DIRECTORY"
                .to_owned(),
        );
    };
    if mode != "--render" {
        return Err("the only supported mode is --render".to_owned());
    }
    render_files(
        Path::new(source),
        Path::new(probe),
        Path::new(provenance),
        Path::new(seed),
        Path::new(output),
    )
}

fn render_files(
    source: &Path,
    probe: &Path,
    provenance: &Path,
    seed: &Path,
    output: &Path,
) -> Result<(), String> {
    let output_metadata = fs::symlink_metadata(output).map_err(io_error)?;
    if output_metadata.file_type().is_symlink()
        || !output_metadata.is_dir()
        || fs::read_dir(output).map_err(io_error)?.next().is_some()
    {
        return Err("output directory must be an existing empty directory".to_owned());
    }
    let source_bytes = fs::read(source).map_err(io_error)?;
    let source_fixture = parse_fixture(&source_bytes, FRAMES as u64)?;
    let source = source_stereo(&source_fixture)?;
    let probe_bytes = fs::read(probe).map_err(io_error)?;
    let probe_fixture = parse_fixture(&probe_bytes, 1_000_000)?;
    let probe = probe_stereo(&probe_fixture)?;
    let provenance_bytes = fs::read(provenance).map_err(io_error)?;
    if provenance_bytes.is_empty() || !provenance_bytes.ends_with(b"\n") {
        return Err("provenance must be nonempty canonical LF data".to_owned());
    }
    let seed_bytes = fs::read(seed).map_err(io_error)?;
    let seed_text = std::str::from_utf8(&seed_bytes).map_err(|_| "seed must be UTF-8")?;
    let seed_value = seed_text
        .strip_suffix('\n')
        .filter(|value| {
            !value.is_empty()
                && value.bytes().all(|byte| byte.is_ascii_digit())
                && (*value == "0" || !value.starts_with('0'))
        })
        .ok_or_else(|| "seed must be one canonical unsigned decimal line".to_owned())?
        .parse::<u64>()
        .map_err(|_| "seed exceeds u64".to_owned())?;

    let rendered = render_roles(&source)?;
    let repeated = render_roles(&source)?;
    if rendered != repeated {
        return Err("nondeterministic source render".to_owned());
    }
    let probe_rendered = render_roles(&probe)?;
    let probe_repeated = render_roles(&probe)?;
    if probe_rendered != probe_repeated {
        return Err("nondeterministic probe render".to_owned());
    }

    let public = output.join("public");
    let private = output.join("private");
    fs::create_dir(&public).map_err(io_error)?;
    fs::create_dir(&private).map_err(io_error)?;
    set_mode(&public, 0o755)?;
    set_mode(&private, 0o700)?;

    let mut generator = SplitMix64::new(seed_value);
    let mut tokens = Vec::new();
    while tokens.len() < 4 {
        let token = format!("{:016x}{:016x}.wav", generator.next(), generator.next());
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    shuffle(&mut tokens, &mut generator);
    let mut token_roles = BTreeMap::new();
    let mut stimuli = Vec::new();
    for (role, token) in rendered.iter().zip(&tokens) {
        write_new_mode(&public.join(token), &role.wave, 0o444)?;
        token_roles.insert(token.clone(), role.role);
        stimuli.push((
            token.clone(),
            role.wave.len(),
            role.metrics.rms,
            role.metrics.peak,
            sha256(&role.wave),
        ));
    }
    stimuli.sort_by(|left, right| left.0.cmp(&right.0));

    let mut filter_assignments = [false; 20];
    filter_assignments[..10].fill(true);
    shuffle(&mut filter_assignments, &mut generator);
    let mut matrix_assignments = [false; 20];
    matrix_assignments[..10].fill(true);
    shuffle(&mut matrix_assignments, &mut generator);
    let schedule = format!(
        "{{\"filter_x_candidate\":{},\"matrix_candidate_first\":{},\"schema_version\":1}}\n",
        bool_json(&filter_assignments),
        bool_json(&matrix_assignments)
    );
    let mut role_json = String::new();
    for (index, (token, role)) in token_roles.iter().enumerate() {
        if index != 0 {
            role_json.push(',');
        }
        role_json.push_str(&format!("\"{token}\":\"{role}\""));
    }
    let key = format!(
        "{{\"filter_x_candidate\":{},\"matrix_candidate_first\":{},\"schema_version\":1,\"seed\":\"{}\",\"token_roles\":{{{}}}}}\n",
        bool_json(&filter_assignments),
        bool_json(&matrix_assignments),
        seed_value,
        role_json
    );
    write_new_mode(&private.join("assignment-key.json"), key.as_bytes(), 0o600)?;
    write_new_mode(
        &private.join("source-provenance.json"),
        &provenance_bytes,
        0o600,
    )?;

    let probe_hashes = {
        let mut hashes = probe_rendered
            .iter()
            .map(|role| sha256(&role.wave))
            .collect::<Vec<_>>();
        hashes.sort();
        hashes
    };
    let stimuli_json = stimuli
        .iter()
        .map(|(token, bytes, rms, peak, digest)| {
            format!(
                "{{\"bytes\":{bytes},\"frames\":{},\"peak\":{peak:.17e},\"rms\":{rms:.17e},\"sha256\":\"{digest}\",\"token\":\"{token}\"}}",
                source.left.len()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let probe_json = probe_hashes
        .iter()
        .map(|digest| format!("\"{digest}\""))
        .collect::<Vec<_>>()
        .join(",");
    let manifest = format!(
        "{{\"assignment_key_sha256\":\"{}\",\"probe_render_sha256\":[{}],\"schedule_sha256\":\"{}\",\"schema_version\":1,\"source_provenance_sha256\":\"{}\",\"source_sha256\":\"{}\",\"stimuli\":[{}]}}\n",
        sha256(key.as_bytes()),
        probe_json,
        sha256(schedule.as_bytes()),
        sha256(&provenance_bytes),
        sha256(&source_bytes),
        stimuli_json
    );
    write_new_mode(
        &public.join("render-manifest.json"),
        manifest.as_bytes(),
        0o444,
    )
}

#[derive(Clone, Copy)]
struct SplitMix64(u64);

impl SplitMix64 {
    const fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn bounded(&mut self, upper_exclusive: u64) -> u64 {
        let threshold = upper_exclusive.wrapping_neg() % upper_exclusive;
        loop {
            let value = self.next();
            if value >= threshold {
                return value % upper_exclusive;
            }
        }
    }
}

fn shuffle<T>(values: &mut [T], generator: &mut SplitMix64) {
    for upper in (1..values.len()).rev() {
        let index = generator.bounded(upper as u64 + 1) as usize;
        values.swap(upper, index);
    }
}

fn bool_json(values: &[bool]) -> String {
    let body = values
        .iter()
        .map(|value| if *value { "true" } else { "false" })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn parse_fixture(bytes: &[u8], max_frames: u64) -> Result<PcmFixtureV1, String> {
    PcmFixtureV1::parse(
        bytes,
        FixtureLimits {
            max_frames,
            max_channels: 2,
            max_payload_bytes: max_frames
                .checked_mul(8)
                .ok_or_else(|| "fixture size overflow".to_owned())?,
        },
    )
    .map_err(|error| format!("invalid mepcm fixture: {error:?}"))
}

fn source_stereo(fixture: &PcmFixtureV1) -> Result<Stereo, String> {
    if fixture.rate().0 != RATE || fixture.channels() != 2 || fixture.frames() != FRAMES as u64 {
        return Err("source must be stereo 48000 Hz with exactly 480000 frames".to_owned());
    }
    let stereo = fixture_stereo(fixture)?;
    if stereo
        .left
        .iter()
        .chain(&stereo.right)
        .any(|sample| !sample.is_finite() || sample.abs() > 0.5)
    {
        return Err("source samples must be finite with peak at most 0.5".to_owned());
    }
    for lane in [&stereo.left, &stereo.right] {
        if lane[..SILENCE_FRAMES]
            .iter()
            .chain(&lane[FRAMES - SILENCE_FRAMES..])
            .any(|sample| sample.to_bits() != 0)
        {
            return Err("source edge silence must be exact positive zero".to_owned());
        }
    }
    Ok(stereo)
}

fn probe_stereo(fixture: &PcmFixtureV1) -> Result<Stereo, String> {
    if fixture.rate().0 != RATE || fixture.channels() != 2 {
        return Err("probe must be stereo at 48000 Hz".to_owned());
    }
    let stereo = fixture_stereo(fixture)?;
    if stereo
        .left
        .iter()
        .chain(&stereo.right)
        .any(|sample| !sample.is_finite())
    {
        return Err("probe samples must be finite".to_owned());
    }
    Ok(stereo)
}

fn fixture_stereo(fixture: &PcmFixtureV1) -> Result<Stereo, String> {
    let frames = usize::try_from(fixture.frames()).map_err(|_| "frame count does not fit host")?;
    let samples = fixture.samples();
    if samples.len() != frames.checked_mul(2).ok_or("sample count overflow")? {
        return Err("fixture planar shape mismatch".to_owned());
    }
    Ok(Stereo {
        left: samples[..frames].to_vec(),
        right: samples[frames..].to_vec(),
    })
}

fn render_roles(source: &Stereo) -> Result<Vec<RoleOutput>, String> {
    if source.left.is_empty() || source.left.len() != source.right.len() {
        return Err("source stereo shape mismatch".to_owned());
    }
    let identity = source.clone();
    let filter = render_filter(source)?;
    let matrix_fixed = render_matrix(source, 0)?;
    let matrix_smoothed = render_matrix(source, 64)?;
    let (filter_comparator, filter_candidate) = match_pair(identity, filter)?;
    let (matrix_comparator, matrix_candidate) = match_pair(matrix_fixed, matrix_smoothed)?;
    [
        ("filter-comparator", filter_comparator),
        ("filter-candidate", filter_candidate),
        ("matrix-comparator", matrix_comparator),
        ("matrix-candidate", matrix_candidate),
    ]
    .into_iter()
    .map(|(role, stereo)| {
        let metrics = metrics(&stereo)?;
        if !metrics.rms.is_finite() || !metrics.peak.is_finite() || metrics.peak >= 1.0 {
            return Err("render metrics rejected".to_owned());
        }
        Ok(RoleOutput {
            role,
            wave: wave(&stereo)?,
            metrics,
        })
    })
    .collect()
}

fn render_filter(source: &Stereo) -> Result<Stereo, String> {
    let lane = ChannelParameters {
        hpf_hz: 100.0,
        lpf_hz: 1_000.0,
        ..ChannelParameters::default()
    };
    let mut chain = BuiltinChain::new(
        RATE,
        BuiltinParameters {
            left: lane,
            right: lane,
            ..BuiltinParameters::default()
        },
    )
    .map_err(|error| format!("filter preparation failed: {error:?}"))?;
    process_blocks(source.clone(), &mut chain, false)
}

fn render_matrix(source: &Stereo, smoothing_samples: u32) -> Result<Stereo, String> {
    let mut chain = BuiltinChain::new(
        RATE,
        BuiltinParameters {
            matrix: MATRIX_INITIAL,
            smoothing_samples,
            ..BuiltinParameters::default()
        },
    )
    .map_err(|error| format!("matrix preparation failed: {error:?}"))?;
    process_blocks(source.clone(), &mut chain, true)
}

fn process_blocks(
    mut output: Stereo,
    chain: &mut BuiltinChain,
    matrix_events: bool,
) -> Result<Stereo, String> {
    let frames = output.left.len();
    let mut first = 0;
    let mut event = 0;
    while first < frames {
        if matrix_events && event < EVENT_FRAMES.len() && first == EVENT_FRAMES[event] {
            chain
                .set_matrix_target(if event % 2 == 0 { MATRIX_A } else { MATRIX_B })
                .map_err(|error| format!("matrix target failed: {error:?}"))?;
            event += 1;
        }
        let next_event = if matrix_events && event < EVENT_FRAMES.len() {
            EVENT_FRAMES[event]
        } else {
            frames
        };
        let end = first.saturating_add(QUANTUM).min(frames).min(next_event);
        chain.process_dual_mono(
            DualMonoBlock::new(
                &mut output.left[first..end],
                &mut output.right[first..end],
                first as u64,
            )
            .map_err(|error| format!("block construction failed: {error:?}"))?,
        );
        first = end;
    }
    Ok(output)
}

fn match_pair(mut first: Stereo, mut second: Stereo) -> Result<(Stereo, Stereo), String> {
    if first.left.len() != second.left.len() || first.right.len() != second.right.len() {
        return Err("render length mismatch".to_owned());
    }
    let first_metrics = metrics(&first)?;
    let second_metrics = metrics(&second)?;
    if (first_metrics.rms == 0.0 || second_metrics.rms == 0.0)
        && first_metrics.rms != second_metrics.rms
    {
        return Err("cannot level-match a silent/non-silent pair".to_owned());
    }
    let quiet = first_metrics.rms.min(second_metrics.rms);
    let first_level = if first_metrics.rms > quiet {
        quiet / first_metrics.rms
    } else {
        1.0
    };
    let second_level = if second_metrics.rms > quiet {
        quiet / second_metrics.rms
    } else {
        1.0
    };
    let scaled_peak = (first_metrics.peak * first_level).max(second_metrics.peak * second_level);
    let cap = 10.0_f64.powf(-1.0 / 20.0);
    let common = if scaled_peak > cap {
        cap / scaled_peak
    } else {
        1.0
    };
    scale_once(&mut first, first_level * common)?;
    scale_once(&mut second, second_level * common)?;
    let first_after = metrics(&first)?;
    let second_after = metrics(&second)?;
    let rms_delta_db = 20.0 * (first_after.rms / second_after.rms).log10().abs();
    if !rms_delta_db.is_finite() || rms_delta_db > 0.1 {
        return Err("post-match RMS delta exceeds 0.1 dB".to_owned());
    }
    Ok((first, second))
}

fn scale_once(stereo: &mut Stereo, gain: f64) -> Result<(), String> {
    if !gain.is_finite() || !(0.0..=1.0).contains(&gain) {
        return Err("invalid attenuation gain".to_owned());
    }
    for sample in stereo.left.iter_mut().chain(&mut stereo.right) {
        *sample = (f64::from(*sample) * gain) as f32;
        if !sample.is_finite() {
            return Err("non-finite matched sample".to_owned());
        }
    }
    Ok(())
}

fn metrics(stereo: &Stereo) -> Result<Metrics, String> {
    let count = stereo
        .left
        .len()
        .checked_add(stereo.right.len())
        .ok_or("metric count overflow")?;
    if count == 0 || stereo.left.len() != stereo.right.len() {
        return Err("metric shape mismatch".to_owned());
    }
    let mut energy = 0.0_f64;
    let mut peak = 0.0_f64;
    for sample in stereo.left.iter().chain(&stereo.right) {
        if !sample.is_finite() {
            return Err("non-finite sample".to_owned());
        }
        let value = f64::from(*sample);
        energy += value * value;
        peak = peak.max(value.abs());
    }
    Ok(Metrics {
        rms: (energy / count as f64).sqrt(),
        peak,
    })
}

fn wave(stereo: &Stereo) -> Result<Vec<u8>, String> {
    if stereo.left.len() != stereo.right.len() {
        return Err("wave stereo shape mismatch".to_owned());
    }
    let data_bytes = stereo
        .left
        .len()
        .checked_mul(8)
        .ok_or("wave size overflow")?;
    let data_u32 = u32::try_from(data_bytes).map_err(|_| "wave data exceeds RIFF")?;
    let riff_size = 36_u32.checked_add(data_u32).ok_or("wave RIFF overflow")?;
    let mut bytes = Vec::with_capacity(WAVE_HEADER_BYTES + data_bytes);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&riff_size.to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&3_u16.to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&RATE.to_le_bytes());
    bytes.extend_from_slice(&(RATE * 8).to_le_bytes());
    bytes.extend_from_slice(&8_u16.to_le_bytes());
    bytes.extend_from_slice(&32_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_u32.to_le_bytes());
    for (&left, &right) in stereo.left.iter().zip(&stereo.right) {
        bytes.extend_from_slice(&left.to_bits().to_le_bytes());
        bytes.extend_from_slice(&right.to_bits().to_le_bytes());
    }
    Ok(bytes)
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(io_error)?;
    file.write_all(bytes).map_err(io_error)?;
    file.sync_all().map_err(io_error)
}

fn write_new_mode(path: &Path, bytes: &[u8], mode: u32) -> Result<(), String> {
    write_new(path, bytes)?;
    set_mode(path, mode)
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(io_error)
}

#[cfg(not(unix))]
fn set_mode(_path: &Path, _mode: u32) -> Result<(), String> {
    Err("Issue-033 preparation requires Unix permission semantics".to_owned())
}

fn sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_conformance::SampleRateHz;
    use std::os::unix::fs::{MetadataExt, PermissionsExt, symlink};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn stereo(frames: usize) -> Stereo {
        let mut left = vec![0.0; frames];
        let mut right = vec![0.0; frames];
        for index in SILENCE_FRAMES..frames.saturating_sub(SILENCE_FRAMES) {
            left[index] = ((index % 97) as f32 - 48.0) / 128.0;
            right[index] = ((index % 89) as f32 - 44.0) / 160.0;
        }
        Stereo { left, right }
    }

    fn fixture_bytes(value: &Stereo) -> Vec<u8> {
        let mut samples = value.left.clone();
        samples.extend_from_slice(&value.right);
        PcmFixtureV1::encode(SampleRateHz(RATE), 2, value.left.len() as u64, &samples)
            .expect("fixture")
    }

    fn fixture_bytes_at(value: &Stereo, rate: u32, channels: u16) -> Vec<u8> {
        let mut samples = value.left.clone();
        if channels == 2 {
            samples.extend_from_slice(&value.right);
        }
        PcmFixtureV1::encode(
            SampleRateHz(rate),
            channels,
            value.left.len() as u64,
            &samples,
        )
        .expect("fixture")
    }

    #[test]
    fn strict_source_shape_edges_peak_and_finite_reject() {
        let valid = fixture_bytes(&stereo(FRAMES));
        let fixture = parse_fixture(&valid, FRAMES as u64).expect("parse");
        assert!(source_stereo(&fixture).is_ok());
        for (index, value) in [(0, 0.1), (SILENCE_FRAMES, 0.51), (1_000, f32::NAN)] {
            let mut changed = stereo(FRAMES);
            changed.left[index] = value;
            let parsed = parse_fixture(&fixture_bytes(&changed), FRAMES as u64).expect("parse");
            assert!(source_stereo(&parsed).is_err());
        }
        let short = stereo(FRAMES - 1);
        let parsed = parse_fixture(&fixture_bytes(&short), FRAMES as u64).expect("parse");
        assert!(source_stereo(&parsed).is_err());
    }

    #[test]
    fn block_events_wave_and_repeat_are_exact() {
        let input = stereo(FRAMES);
        let first = render_roles(&input).expect("render");
        let second = render_roles(&input).expect("repeat");
        assert_eq!(first, second);
        assert_eq!(first.len(), 4);
        for role in &first {
            assert_eq!(&role.wave[..4], b"RIFF");
            assert_eq!(&role.wave[8..16], b"WAVEfmt ");
            assert_eq!(u16::from_le_bytes(role.wave[20..22].try_into().unwrap()), 3);
            assert_eq!(u16::from_le_bytes(role.wave[22..24].try_into().unwrap()), 2);
            assert_eq!(
                u32::from_le_bytes(role.wave[24..28].try_into().unwrap()),
                RATE
            );
            assert_eq!(&role.wave[36..40], b"data");
            assert_eq!(
                u32::from_le_bytes(role.wave[16..20].try_into().unwrap()),
                16
            );
            assert_eq!(
                u32::from_le_bytes(role.wave[28..32].try_into().unwrap()),
                RATE * 8
            );
            assert_eq!(u16::from_le_bytes(role.wave[32..34].try_into().unwrap()), 8);
            assert_eq!(
                u16::from_le_bytes(role.wave[34..36].try_into().unwrap()),
                32
            );
            assert_eq!(
                u32::from_le_bytes(role.wave[40..44].try_into().unwrap()) as usize,
                FRAMES * 8
            );
            assert_eq!(role.wave.len(), WAVE_HEADER_BYTES + FRAMES * 8);
            assert!(role.metrics.peak < 1.0);
        }
        assert_ne!(first[2].wave, first[3].wave);
        let filter_db = 20.0 * (first[0].metrics.rms / first[1].metrics.rms).log10().abs();
        let matrix_db = 20.0 * (first[2].metrics.rms / first[3].metrics.rms).log10().abs();
        assert!(filter_db <= 0.1 && matrix_db <= 0.1);
    }

    #[test]
    fn level_match_attenuates_only_and_caps_peak() {
        let first = Stereo {
            left: vec![1.0, -1.0],
            right: vec![0.5, -0.5],
        };
        let second = Stereo {
            left: vec![0.25, -0.25],
            right: vec![0.125, -0.125],
        };
        let (first, second) = match_pair(first, second).expect("match");
        let a = metrics(&first).unwrap();
        let b = metrics(&second).unwrap();
        assert!((a.rms - b.rms).abs() < 1e-7);
        assert!(a.peak <= 10.0_f64.powf(-1.0 / 20.0));
        assert!(b.peak <= 0.25);
        assert!(
            match_pair(
                Stereo {
                    left: vec![0.0],
                    right: vec![0.0]
                },
                Stereo {
                    left: vec![0.1],
                    right: vec![0.1]
                }
            )
            .is_err()
        );
        assert!(
            match_pair(
                Stereo {
                    left: vec![0.0],
                    right: vec![0.0]
                },
                Stereo {
                    left: vec![0.0],
                    right: vec![0.0]
                }
            )
            .is_err()
        );
    }

    #[test]
    fn probe_seed_and_cli_rejections_are_exact() {
        let mono = stereo(2_048);
        let wrong_rate = PcmFixtureV1::parse(
            &fixture_bytes_at(&mono, 44_100, 2),
            FixtureLimits {
                max_frames: 1_000_000,
                max_channels: 2,
                max_payload_bytes: 8_000_000,
            },
        )
        .unwrap();
        assert!(probe_stereo(&wrong_rate).is_err());
        let wrong_channels = PcmFixtureV1::parse(
            &fixture_bytes_at(&mono, RATE, 1),
            FixtureLimits {
                max_frames: 1_000_000,
                max_channels: 2,
                max_payload_bytes: 8_000_000,
            },
        )
        .unwrap();
        assert!(probe_stereo(&wrong_channels).is_err());
        let mut nonfinite = mono;
        nonfinite.left[1] = f32::INFINITY;
        let parsed = parse_fixture(&fixture_bytes(&nonfinite), 1_000_000).unwrap();
        assert!(probe_stereo(&parsed).is_err());
        assert!(run(Vec::new()).is_err());
        assert!(run(vec!["--wrong".to_owned(); 6]).is_err());

        let mut generator = SplitMix64::new(0);
        assert_eq!(generator.next(), 0xe220_a839_7b1d_cdaf);
        assert_eq!(generator.next(), 0x6e78_9e6a_a1b9_65f4);
        let mut balanced = [false; 20];
        balanced[..10].fill(true);
        shuffle(&mut balanced, &mut generator);
        assert_eq!(balanced.iter().filter(|value| **value).count(), 10);
    }

    #[test]
    fn output_is_closed_and_no_clobber() {
        let suffix = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = env::temp_dir().join(format!("miso-listening-{suffix}-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let source = root.join("source.mepcm");
        let probe = root.join("probe.mepcm");
        let provenance = root.join("provenance.json");
        let seed = root.join("seed.txt");
        let output = root.join("out");
        fs::create_dir(&output).unwrap();
        fs::write(&source, fixture_bytes(&stereo(FRAMES))).unwrap();
        fs::write(&probe, fixture_bytes(&stereo(2_048))).unwrap();
        fs::write(&provenance, b"{}\n").unwrap();
        fs::write(&seed, b"42\n").unwrap();
        render_files(&source, &probe, &provenance, &seed, &output).expect("render files");
        let mut names = fs::read_dir(output.join("public"))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(names.len(), 5);
        assert_eq!(
            names.iter().filter(|name| name.ends_with(".wav")).count(),
            4
        );
        assert!(names.contains(&"render-manifest.json".to_owned()));
        assert!(
            names
                .iter()
                .all(|name| !name.contains("candidate") && !name.contains("comparator"))
        );
        let key = fs::read_to_string(output.join("private/assignment-key.json")).unwrap();
        assert!(key.contains("filter-candidate") && key.contains("matrix-comparator"));
        assert_eq!(key.matches("true").count(), 20);
        assert_eq!(key.matches("false").count(), 20);
        for name in names.iter().filter(|name| name.ends_with(".wav")) {
            assert_eq!(name.len(), 36);
            assert!(name[..32].bytes().all(|byte| byte.is_ascii_hexdigit()));
            let metadata = fs::metadata(output.join("public").join(name)).unwrap();
            assert_eq!(metadata.permissions().mode() & 0o777, 0o444);
            assert_eq!(metadata.nlink(), 1);
        }
        let manifest = fs::read_to_string(output.join("public/render-manifest.json")).unwrap();
        assert!(!manifest.contains("candidate"));
        assert!(!manifest.contains("comparator"));
        assert!(!manifest.contains("\"seed\""));
        assert_eq!(
            fs::metadata(output.join("private/assignment-key.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        assert!(render_files(&source, &probe, &provenance, &seed, &output).is_err());

        let nonempty = root.join("nonempty");
        fs::create_dir(&nonempty).unwrap();
        fs::write(nonempty.join("existing"), b"sentinel").unwrap();
        assert!(render_files(&source, &probe, &provenance, &seed, &nonempty).is_err());
        let linked = root.join("linked-output");
        symlink(&nonempty, &linked).unwrap();
        assert!(render_files(&source, &probe, &provenance, &seed, &linked).is_err());

        let empty = root.join("empty");
        fs::create_dir(&empty).unwrap();
        fs::write(&seed, b"042\n").unwrap();
        assert!(render_files(&source, &probe, &provenance, &seed, &empty).is_err());
        fs::write(&seed, b"42\n").unwrap();
        fs::write(&provenance, b"{}").unwrap();
        assert!(render_files(&source, &probe, &provenance, &seed, &empty).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
