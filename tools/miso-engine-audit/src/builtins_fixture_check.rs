//! Independent Issue-069 audit-evidence author and read-only checker.

use std::{
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use miso_engine_dsp_reference::{ReferenceRetainedTptF32, ReferenceTptOutput};
use sha2::{Digest, Sha256};

const BLOCKS: u64 = 1_000_000;
const QUANTUM: usize = 128;
const MANIFEST_HEADER: &str = "path\tlength\tsha256\n";
const PATHS: [&str; 4] = [
    "direct-result.json",
    "direct-schedule.pcm.f32le",
    "graph-meter-sets.jsonl",
    "prepared-chain-state-report.jsonl",
];
const TAPS: [&str; 7] = [
    "Input",
    "PostInputBuiltins",
    "PostSimd1",
    "PostDynamic",
    "PostSimd2PreFader",
    "PostFader",
    "PostMatrix",
];

pub(crate) fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [mode, root] if mode == "--write" => write_scratch(Path::new(root)),
        [mode, root] if mode == "--check" => check_read_only(Path::new(root)),
        _ => Err("usage: miso_engine_builtins_audit_fixture --write|--check DIRECTORY".to_owned()),
    }
}

#[derive(Clone, Copy, Default)]
struct Counts {
    sanitized_input: u64,
    sanitized_output: u64,
    recovered_left: u64,
    recovered_right: u64,
}

impl Counts {
    fn add(&mut self, other: Self) {
        self.sanitized_input += other.sanitized_input;
        self.sanitized_output += other.sanitized_output;
        self.recovered_left += other.recovered_left;
        self.recovered_right += other.recovered_right;
    }
}

struct ReferenceChain {
    left_hpf: ReferenceRetainedTptF32,
    left_lpf: ReferenceRetainedTptF32,
    right_hpf: ReferenceRetainedTptF32,
    right_lpf: ReferenceRetainedTptF32,
    matrix_current: [f32; 4],
    matrix_target: [f32; 4],
    matrix_step: [f32; 4],
    remaining: u32,
    lifetime: [u64; 2],
    inject_left_hpf: bool,
    inject_right_lpf: bool,
}

impl ReferenceChain {
    fn new() -> Self {
        let filter = |cutoff, output| {
            ReferenceRetainedTptF32::conditioned_butterworth(48_000, cutoff, output)
                .expect("frozen filter")
        };
        Self {
            left_hpf: filter(100.0, ReferenceTptOutput::HighPass),
            left_lpf: filter(1_000.0, ReferenceTptOutput::LowPass),
            right_hpf: filter(200.0, ReferenceTptOutput::HighPass),
            right_lpf: filter(2_000.0, ReferenceTptOutput::LowPass),
            matrix_current: [1.0, 0.0, 0.0, 1.0],
            matrix_target: [1.0, 0.0, 0.0, 1.0],
            matrix_step: [0.0; 4],
            remaining: 0,
            lifetime: [0, 0],
            inject_left_hpf: false,
            inject_right_lpf: false,
        }
    }

    fn set_target(&mut self, target: [f32; 4]) {
        const SAMPLES: u32 = 257;
        self.matrix_target = target;
        for ((step, target), current) in self
            .matrix_step
            .iter_mut()
            .zip(target)
            .zip(self.matrix_current)
        {
            *step = (target - current) / SAMPLES as f32;
        }
        self.remaining = SAMPLES;
    }

    fn reset(&mut self) {
        self.left_hpf.reset();
        self.left_lpf.reset();
        self.right_hpf.reset();
        self.right_lpf.reset();
        self.matrix_current = self.matrix_target;
        self.remaining = 0;
    }

    fn inject_recovery(&mut self) {
        self.inject_left_hpf = true;
        self.inject_right_lpf = true;
    }

    /// The D7 input stage of one channel: sanitise once, cascade the two sections, then check
    /// finiteness once for the whole block.
    ///
    /// A failing block is zeroed, both of that channel's sections are reset and one recovery is
    /// counted -- per block, not per sample. There is no output sanitisation any more: a finite
    /// block needs none, and a non-finite one is caught here.
    fn process_channel(
        samples: &mut [f32; QUANTUM],
        high: &mut ReferenceRetainedTptF32,
        low: &mut ReferenceRetainedTptF32,
        sanitized_input: &mut u64,
    ) -> u64 {
        for sample in samples.iter_mut() {
            let input = sanitize(*sample, sanitized_input);
            let high_output = f32::from_bits(high.process(input).output_bits);
            *sample = f32::from_bits(low.process(high_output).output_bits);
        }
        if samples.iter().all(|sample| sample.abs() < NONFINITE_LIMIT) {
            return 0;
        }
        samples.fill(0.0);
        high.reset();
        low.reset();
        1
    }

    fn process_block(
        &mut self,
        first_left: Option<f32>,
        first_right: Option<f32>,
    ) -> ([f32; QUANTUM], [f32; QUANTUM], Counts) {
        let mut left = [0.25; QUANTUM];
        let mut right = [-0.5; QUANTUM];
        if let Some(value) = first_left {
            left[0] = value;
        }
        if let Some(value) = first_right {
            right[0] = value;
        }
        let mut counts = Counts::default();

        if self.inject_left_hpf {
            self.inject_left_hpf = false;
            self.left_hpf.set_state_bits([f32::NAN.to_bits(), 0]);
        }
        if self.inject_right_lpf {
            self.inject_right_lpf = false;
            self.right_lpf.set_state_bits([f32::NAN.to_bits(), 0]);
        }

        counts.recovered_left = Self::process_channel(
            &mut left,
            &mut self.left_hpf,
            &mut self.left_lpf,
            &mut counts.sanitized_input,
        );
        self.lifetime[0] += counts.recovered_left;
        counts.recovered_right = Self::process_channel(
            &mut right,
            &mut self.right_hpf,
            &mut self.right_lpf,
            &mut counts.sanitized_input,
        );
        self.lifetime[1] += counts.recovered_right;

        for index in 0..QUANTUM {
            self.advance_matrix();
            let (in_left, in_right) = (left[index], right[index]);
            if self.matrix_current == [1.0, 0.0, 0.0, 1.0] && self.remaining == 0 {
                continue;
            }
            left[index] = self.matrix_current[0] * in_left + self.matrix_current[1] * in_right;
            right[index] = self.matrix_current[2] * in_left + self.matrix_current[3] * in_right;
        }
        (left, right, counts)
    }

    /// D11: one division at the event, iterated additions, an exact assignment at the end.
    fn advance_matrix(&mut self) {
        if self.remaining == 0 {
            return;
        }
        self.remaining -= 1;
        if self.remaining == 0 {
            self.matrix_current = self.matrix_target;
            return;
        }
        for (current, step) in self.matrix_current.iter_mut().zip(self.matrix_step) {
            *current += step;
        }
    }

    fn state_row(&self, call: u32, report: Counts) -> String {
        let filters = [
            self.left_hpf.state_bits()[0],
            self.left_hpf.state_bits()[1],
            self.left_lpf.state_bits()[0],
            self.left_lpf.state_bits()[1],
            self.right_hpf.state_bits()[0],
            self.right_hpf.state_bits()[1],
            self.right_lpf.state_bits()[0],
            self.right_lpf.state_bits()[1],
        ];
        format!(
            "{{\"call\":{call},\"filter\":{},\"current\":{},\"target\":{},\"remaining\":{},\"lifetime\":[\"{:016x}\",\"{:016x}\"],\"report\":[\"{:016x}\",\"{:016x}\",\"{:016x}\",\"{:016x}\"]}}",
            words(&filters),
            words(&self.matrix_current.map(f32::to_bits)),
            words(&self.matrix_target.map(f32::to_bits)),
            self.remaining,
            self.lifetime[0],
            self.lifetime[1],
            report.sanitized_input,
            report.sanitized_output,
            report.recovered_left,
            report.recovered_right,
        )
    }
}

/// Magnitude at or above which a sample is non-finite under D7. `!(|x| < 1e30)` covers NaN.
const NONFINITE_LIMIT: f32 = 1.0e30;

/// The D7 input sanitisation: exactly the kernel's one-compare form, written out here.
///
/// A subnormal input is no longer sanitised: it is a legal finite sample, and the only denormal
/// mechanism that remains is the in-kernel flush of the recursive state words.
fn sanitize(value: f32, count: &mut u64) -> f32 {
    if value.abs() < NONFINITE_LIMIT {
        value
    } else {
        *count += 1;
        0.0
    }
}

fn words(values: &[u32]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        write!(&mut output, "\"{value:08x}\"").expect("string");
    }
    output.push(']');
    output
}

fn prepare_call(chain: &mut ReferenceChain, call: u64, inject: bool) {
    match call {
        0 => chain.set_target([0.0, 1.0, 1.0, 0.0]),
        1 => chain.set_target([0.9, 0.1, -0.1, 0.9]),
        3 if inject => chain.inject_recovery(),
        4 | 5 => chain.reset(),
        _ => {}
    }
}

fn generated() -> Vec<(String, Vec<u8>)> {
    let mut public_chain = ReferenceChain::new();
    let mut pcm = Vec::with_capacity(6 * QUANTUM * 2 * 4);
    for call in 0..6 {
        prepare_call(&mut public_chain, call, false);
        let special = (call == 2).then_some((f32::NAN, f32::INFINITY));
        let (left, right, _) =
            public_chain.process_block(special.map(|row| row.0), special.map(|row| row.1));
        for sample in left.into_iter().chain(right) {
            pcm.extend_from_slice(&sample.to_le_bytes());
        }
    }

    let mut private_chain = ReferenceChain::new();
    let mut states = String::new();
    for call in 0..6 {
        prepare_call(&mut private_chain, call, true);
        let special = (call == 2).then_some((f32::NAN, f32::INFINITY));
        let (_, _, report) =
            private_chain.process_block(special.map(|row| row.0), special.map(|row| row.1));
        writeln!(
            &mut states,
            "{}",
            private_chain.state_row(call as u32 + 1, report)
        )
        .expect("string");
    }

    let mut graph = String::new();
    for outcome in ["success", "saturation"] {
        for (index, tap) in TAPS.into_iter().enumerate() {
            let post_drop = u64::from(outcome == "saturation");
            writeln!(
                &mut graph,
                "{{\"outcome\":\"{outcome}\",\"tap\":\"{tap}\",\"handle\":\"{:016x}\",\"first_dropped\":\"0000000000000000\",\"post_dropped\":\"{post_drop:016x}\",\"pdc_samples\":9,\"first_early_route_frame\":9}}",
                index + 1,
            )
            .expect("string");
        }
    }

    let mut full = ReferenceChain::new();
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    let mut total = Counts::default();
    for call in 0..BLOCKS {
        prepare_call(&mut full, call, false);
        let special = (call == 2).then_some((f32::NAN, f32::INFINITY));
        let (left, right, report) =
            full.process_block(special.map(|row| row.0), special.map(|row| row.1));
        total.add(report);
        for sample in left.into_iter().chain(right) {
            digest ^= u64::from(sample.to_bits());
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    let result = format!(
        "{{\"schema_version\":1,\"calls\":1000000,\"sample_rate_hz\":48000,\"quantum_frames\":128,\"pcm_digest\":\"{digest:016x}\",\"sanitized_input\":\"{:016x}\",\"sanitized_output\":\"{:016x}\",\"recovered_left\":\"{:016x}\",\"recovered_right\":\"{:016x}\"}}\n",
        total.sanitized_input, total.sanitized_output, total.recovered_left, total.recovered_right,
    );
    vec![
        ("direct-result.json".to_owned(), result.into_bytes()),
        ("direct-schedule.pcm.f32le".to_owned(), pcm),
        ("graph-meter-sets.jsonl".to_owned(), graph.into_bytes()),
        (
            "prepared-chain-state-report.jsonl".to_owned(),
            states.into_bytes(),
        ),
    ]
}

fn manifest(files: &[(String, Vec<u8>)]) -> String {
    let mut output = String::from(MANIFEST_HEADER);
    for (path, bytes) in files {
        writeln!(&mut output, "{path}\t{}\t{}", bytes.len(), sha256(bytes)).expect("string");
    }
    output
}

fn write_scratch(root: &Path) -> Result<(), String> {
    let accepted = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/builtins-audit-v1");
    if canonical_candidate(root) == canonical_candidate(&accepted) {
        return Err("author refuses the accepted fixture root".to_owned());
    }
    if root.exists()
        && root
            .read_dir()
            .map_err(|error| error.to_string())?
            .next()
            .is_some()
    {
        return Err("author refuses a nonempty destination".to_owned());
    }
    fs::create_dir_all(root).map_err(|error| format!("create scratch: {error}"))?;
    let files = generated();
    for (path, bytes) in &files {
        fs::write(root.join(path), bytes).map_err(|error| format!("write {path}: {error}"))?;
    }
    fs::write(root.join("MANIFEST.tsv"), manifest(&files))
        .map_err(|error| format!("write manifest: {error}"))?;
    check_read_only(root)
}

fn canonical_candidate(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn check_read_only(root: &Path) -> Result<(), String> {
    let before = tree_hash(root)?;
    check(root)?;
    let after = tree_hash(root)?;
    if before != after {
        return Err("checker mutated fixture root".to_owned());
    }
    Ok(())
}

fn check(root: &Path) -> Result<(), String> {
    let manifest_bytes = read_file(&root.join("MANIFEST.tsv"))?;
    let manifest_text =
        std::str::from_utf8(&manifest_bytes).map_err(|_| "manifest is not UTF-8".to_owned())?;
    let mut lines = manifest_text.lines();
    if lines.next() != Some(MANIFEST_HEADER.trim_end()) {
        return Err("manifest header".to_owned());
    }
    for expected_path in PATHS {
        let line = lines
            .next()
            .ok_or_else(|| "manifest row missing".to_owned())?;
        let mut fields = line.split('\t');
        let path = fields.next().unwrap_or_default();
        let length = fields.next().unwrap_or_default();
        let hash = fields.next().unwrap_or_default();
        if path != expected_path || fields.next().is_some() {
            return Err(format!("manifest path/order: {path}"));
        }
        let bytes = read_file(&root.join(path))?;
        if length != bytes.len().to_string() || hash != sha256(&bytes) {
            return Err(format!("manifest identity: {path}"));
        }
    }
    if lines.next().is_some() {
        return Err("manifest extra row".to_owned());
    }
    if fs::read_dir(root)
        .map_err(|error| error.to_string())?
        .filter_map(Result::ok)
        .count()
        != PATHS.len() + 1
    {
        return Err("fixture path coverage".to_owned());
    }
    let pcm = read_file(&root.join("direct-schedule.pcm.f32le"))?;
    if pcm.len() != 6 * QUANTUM * 2 * 4
        || !pcm.chunks_exact(4).all(|word| {
            f32::from_bits(u32::from_le_bytes(word.try_into().expect("word"))).is_finite()
        })
    {
        return Err("direct PCM semantics".to_owned());
    }
    let states = read_text(root, "prepared-chain-state-report.jsonl")?;
    if states.lines().count() != 6
        || !states.lines().enumerate().all(|(index, line)| {
            line.starts_with(&format!("{{\"call\":{},", index + 1))
                && line.contains("\"filter\":[")
                && line.contains("\"report\":[")
        })
    {
        return Err("prepared state/report semantics".to_owned());
    }
    let graph = read_text(root, "graph-meter-sets.jsonl")?;
    if graph.lines().count() != 14
        || graph
            .lines()
            .filter(|line| line.contains("\"outcome\":\"success\""))
            .count()
            != 7
        || graph
            .lines()
            .filter(|line| line.contains("\"outcome\":\"saturation\""))
            .count()
            != 7
    {
        return Err("graph meter-set semantics".to_owned());
    }
    let result = read_text(root, "direct-result.json")?;
    if result.lines().count() != 1
        || !result.starts_with("{\"schema_version\":1,\"calls\":1000000,")
        || !result.ends_with("}\n")
    {
        return Err("direct result semantics".to_owned());
    }
    Ok(())
}

fn read_text(root: &Path, path: &str) -> Result<String, String> {
    String::from_utf8(read_file(&root.join(path))?).map_err(|_| format!("{path} is not UTF-8"))
}

fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| format!("read metadata: {error}"))?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err("fixture entry is not a regular file".to_owned());
    }
    fs::read(path).map_err(|error| format!("read fixture: {error}"))
}

fn tree_hash(root: &Path) -> Result<[u8; 32], String> {
    let mut digest = Sha256::new();
    for path in PATHS.into_iter().chain(["MANIFEST.tsv"]) {
        let bytes = read_file(&root.join(path))?;
        digest.update(path.as_bytes());
        digest.update([0]);
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }
    Ok(digest.finalize().into())
}

fn sha256(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(&mut output, "{byte:02x}").expect("string");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue069_checker_is_read_only_and_rejects_payload_mutation() {
        let accepted = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/builtins-audit-v1");
        check_read_only(&accepted).expect("accepted audit fixtures");
        let before = tree_hash(&accepted).expect("before");
        check_read_only(&accepted).expect("repeat checker");
        assert_eq!(tree_hash(&accepted).expect("after"), before);

        let scratch =
            env::temp_dir().join(format!("miso-engine-issue069-check-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("scratch");
        for path in PATHS.into_iter().chain(["MANIFEST.tsv"]) {
            fs::copy(accepted.join(path), scratch.join(path)).expect("copy fixture");
        }
        let path = scratch.join("direct-result.json");
        let mut bytes = fs::read(&path).expect("result");
        bytes[0] ^= 1;
        fs::write(path, bytes).expect("mutation");
        assert!(check_read_only(&scratch).is_err());
        fs::remove_dir_all(scratch).expect("cleanup");
    }

    #[test]
    fn issue069_author_is_not_reachable_from_audit_mains() {
        for source in [
            include_str!("builtins.rs"),
            include_str!("builtins_graph.rs"),
        ] {
            assert!(!source.contains("--write"));
            assert!(!source.contains("write_scratch"));
        }
        let author = include_str!("builtins_fixture_check.rs");
        assert!(!author.contains(&("fixtures/builtins".to_owned() + "/v1")));
        assert!(!author.contains(&("benchmark".to_owned() + "/")));
    }
}
