//! Native WAV/RF64 reference runner over the frozen Engine V2 C ABI.
//!
//! This crate is deliberately native-only tooling. Session compilation, source submission, and
//! rendering cross the public C ABI; only session declaration inspection and native file decoding
//! use Rust APIs directly.

#![allow(unsafe_code)]

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::{ffi::OsStrExt, fs::OpenOptionsExt, io::AsRawHandle};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    num::NonZeroUsize,
    path::{Component, Path, PathBuf},
    ptr,
};
#[cfg(any(target_os = "linux", target_os = "android", target_vendor = "apple"))]
use std::{
    ffi::CString,
    os::unix::{ffi::OsStrExt, fs::OpenOptionsExt},
};

use miso_engine_capi as capi;
use miso_engine_session::{SessionTomlV1, parse_session_toml};
use miso_engine_source::{
    NativeWaveDecoder, NativeWaveError, NativeWaveParseCaps, NativeWaveRegion, SourceFrame,
    parse_native_wave,
};
use sha2::{Digest, Sha256};

const DIAGNOSTIC_CAPACITY: usize = 65_536;
const MAX_SESSION_BYTES: u64 = 4 * 1024 * 1024;
const MAX_SOURCES: u64 = 4_096;
const MAX_FRAMES: u64 = u64::MAX / 8;

/// Stable runner failure phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailurePhase {
    /// Command-line parsing and scalar validation.
    Cli,
    /// Session and destination preflight.
    Preflight,
    /// Source locator, identity, or declaration validation.
    Resolve,
    /// Native WAVE/RF64 parsing or decoding.
    Decode,
    /// C ABI engine/session preparation.
    Compile,
    /// C ABI source submission.
    Submit,
    /// C ABI render.
    Render,
    /// Partial output creation, write, flush, sync, or verification.
    Output,
    /// Atomic no-replace publication.
    Publish,
}

impl FailurePhase {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::Preflight => "preflight",
            Self::Resolve => "resolve",
            Self::Decode => "decode",
            Self::Compile => "compile",
            Self::Submit => "submit",
            Self::Render => "render",
            Self::Output => "output",
            Self::Publish => "publish",
        }
    }
}

/// A stable typed runner error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunnerError {
    /// Frozen failure phase.
    pub phase: FailurePhase,
    /// Stable lowercase dotted code.
    pub code: &'static str,
}

impl RunnerError {
    const fn new(phase: FailurePhase, code: &'static str) -> Self {
        Self { phase, code }
    }
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "native-pcm-runner.v1\t{}\t{}",
            self.phase.as_str(),
            self.code
        )
    }
}

impl std::error::Error for RunnerError {}

/// Exact validated CLI arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunnerArgs {
    /// Strict V1 TOML session path.
    pub session: PathBuf,
    /// Canonical root containing native source files.
    pub source_root: PathBuf,
    /// Exact positive output frame count.
    pub frames: u64,
    /// Final block-planar `f32le` output path.
    pub output: PathBuf,
}

/// Parse the exact four-option CLI.
pub fn parse_cli(arguments: impl IntoIterator<Item = OsString>) -> Result<RunnerArgs, RunnerError> {
    let mut session = None;
    let mut source_root = None;
    let mut frames = None;
    let mut output = None;
    let mut arguments = arguments.into_iter();
    while let Some(option) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| RunnerError::new(FailurePhase::Cli, "missing.value"))?;
        match option.to_str() {
            Some("--session") if session.is_none() => session = Some(PathBuf::from(value)),
            Some("--source-root") if source_root.is_none() => {
                source_root = Some(PathBuf::from(value));
            }
            Some("--frames") if frames.is_none() => {
                let value = value
                    .to_str()
                    .ok_or_else(|| RunnerError::new(FailurePhase::Cli, "frames.utf8"))?;
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| RunnerError::new(FailurePhase::Cli, "frames.invalid"))?;
                if parsed == 0 {
                    return Err(RunnerError::new(FailurePhase::Cli, "frames.zero"));
                }
                frames = Some(parsed);
            }
            Some("--output") if output.is_none() => output = Some(PathBuf::from(value)),
            Some("--session" | "--source-root" | "--frames" | "--output") => {
                return Err(RunnerError::new(FailurePhase::Cli, "option.duplicate"));
            }
            Some(_) | None => return Err(RunnerError::new(FailurePhase::Cli, "option.unknown")),
        }
    }
    Ok(RunnerArgs {
        session: session.ok_or_else(|| RunnerError::new(FailurePhase::Cli, "session.missing"))?,
        source_root: source_root
            .ok_or_else(|| RunnerError::new(FailurePhase::Cli, "source_root.missing"))?,
        frames: frames.ok_or_else(|| RunnerError::new(FailurePhase::Cli, "frames.missing"))?,
        output: output.ok_or_else(|| RunnerError::new(FailurePhase::Cli, "output.missing"))?,
    })
}

/// Parse the CLI and execute the production runner.
pub fn run_cli(arguments: impl IntoIterator<Item = OsString>) -> Result<(), RunnerError> {
    let arguments = parse_cli(arguments)?;
    run(&arguments)
}

/// Execute the production runner over the frozen C ABI.
pub fn run(arguments: &RunnerArgs) -> Result<(), RunnerError> {
    run_with(arguments, &mut CAbi::default(), &RealOutput::default())
}

trait EngineBoundary {
    fn compile(
        &mut self,
        session: &[u8],
        rate: u32,
        quantum: u32,
        ring_frames: u32,
    ) -> Result<(), RunnerError>;
    fn submit(&mut self, chunk: SubmitChunk<'_>) -> Result<(), RunnerError>;
    fn render(
        &mut self,
        absolute: u64,
        quantum: u32,
        output: &mut [f32],
    ) -> Result<(), RunnerError>;
}

struct SubmitChunk<'a> {
    id: &'a [u8],
    rate: u32,
    start: u64,
    planar: &'a [f32],
    channel_count: usize,
    plane_stride: usize,
    frames: u32,
    end: bool,
}

trait OutputBoundary {
    fn begin(
        &self,
        final_path: &Path,
        expected_bytes: u64,
    ) -> Result<Box<dyn OutputSink>, RunnerError>;
}

trait OutputSink {
    fn write_block(&mut self, samples: &[f32]) -> Result<(), RunnerError>;
    fn finish(self: Box<Self>) -> Result<(), RunnerError>;
}

struct PreparedSource {
    id: Vec<u8>,
    rate: u32,
    region_start: u64,
    consumed: u64,
    decoder: NativeWaveDecoder<File>,
    planar: Vec<f32>,
    channel_count: usize,
}

fn run_with(
    arguments: &RunnerArgs,
    engine: &mut dyn EngineBoundary,
    output: &dyn OutputBoundary,
) -> Result<(), RunnerError> {
    run_with_platform(arguments, engine, output, platform_supported())
}

fn run_with_platform(
    arguments: &RunnerArgs,
    engine: &mut dyn EngineBoundary,
    output: &dyn OutputBoundary,
    supported_platform: bool,
) -> Result<(), RunnerError> {
    let session_bytes = read_bounded_session(&arguments.session)?;
    let session_text = std::str::from_utf8(&session_bytes)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "session.utf8"))?;
    let model = parse_session_toml(session_text)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "session.invalid"))?;
    validate_scalar_contract(arguments, &model)?;
    let ring_frames = u32::try_from(model.limits.pcm_ring_frames)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "ring.overflow"))?;
    if !supported_platform {
        return Err(RunnerError::new(
            FailurePhase::Preflight,
            "platform.unsupported",
        ));
    }
    preflight_output(&arguments.output)?;
    let mut sources = resolve_sources(&model, &arguments.source_root)?;
    engine.compile(
        &session_bytes,
        model.sample_rate_hz,
        model.quantum_frames,
        ring_frames,
    )?;

    let quantum = usize::try_from(model.quantum_frames)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "quantum.platform"))?;
    let blocks = arguments.frames / u64::from(model.quantum_frames);
    let mut rendered = vec![0.0_f32; quantum * 2];
    let expected_bytes = arguments
        .frames
        .checked_mul(8)
        .ok_or_else(|| RunnerError::new(FailurePhase::Preflight, "frames.overflow"))?;
    let mut sink = output.begin(&arguments.output, expected_bytes)?;
    for block in 0..blocks {
        feed_sources(engine, &mut sources, model.quantum_frames)?;
        let absolute = block
            .checked_mul(u64::from(model.quantum_frames))
            .ok_or_else(|| RunnerError::new(FailurePhase::Render, "absolute.overflow"))?;
        engine.render(absolute, model.quantum_frames, &mut rendered)?;
        sink.write_block(&rendered)?;
    }
    sink.finish()
}

const fn platform_supported() -> bool {
    cfg!(any(
        target_os = "linux",
        target_os = "android",
        target_vendor = "apple",
        windows
    ))
}

fn read_bounded_session(path: &Path) -> Result<Vec<u8>, RunnerError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "session.open"))?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(RunnerError::new(FailurePhase::Preflight, "session.type"));
    }
    if metadata.len() > MAX_SESSION_BYTES {
        return Err(RunnerError::new(FailurePhase::Preflight, "session.limit"));
    }
    let bytes =
        fs::read(path).map_err(|_| RunnerError::new(FailurePhase::Preflight, "session.read"))?;
    if u64::try_from(bytes.len()).ok() != Some(metadata.len()) {
        return Err(RunnerError::new(FailurePhase::Preflight, "session.changed"));
    }
    Ok(bytes)
}

fn validate_scalar_contract(
    arguments: &RunnerArgs,
    model: &SessionTomlV1,
) -> Result<(), RunnerError> {
    if !matches!(model.sample_rate_hz, 44_100 | 48_000 | 88_200 | 96_000) {
        return Err(RunnerError::new(
            FailurePhase::Preflight,
            "rate.unsupported",
        ));
    }
    if arguments.frames > MAX_FRAMES || arguments.frames.checked_mul(8).is_none() {
        return Err(RunnerError::new(FailurePhase::Preflight, "frames.overflow"));
    }
    if model.quantum_frames == 0
        || !arguments
            .frames
            .is_multiple_of(u64::from(model.quantum_frames))
    {
        return Err(RunnerError::new(FailurePhase::Preflight, "frames.quantum"));
    }
    let quantum = usize::try_from(model.quantum_frames)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "quantum.platform"))?;
    quantum
        .checked_mul(2)
        .and_then(|samples| samples.checked_mul(size_of::<f32>()))
        .ok_or_else(|| RunnerError::new(FailurePhase::Preflight, "quantum.overflow"))?;
    if u64::try_from(model.sources.len()).unwrap_or(u64::MAX) > MAX_SOURCES {
        return Err(RunnerError::new(FailurePhase::Preflight, "sources.limit"));
    }
    Ok(())
}

fn partial_path(output: &Path) -> Result<PathBuf, RunnerError> {
    let file_name = output
        .file_name()
        .ok_or_else(|| RunnerError::new(FailurePhase::Preflight, "output.name"))?;
    let mut partial_name = file_name.to_os_string();
    partial_name.push(".issue073.partial");
    Ok(output.with_file_name(partial_name))
}

fn preflight_output(output: &Path) -> Result<(), RunnerError> {
    let partial = partial_path(output)?;
    for path in [output, partial.as_path()] {
        match fs::symlink_metadata(path) {
            Ok(_) => return Err(RunnerError::new(FailurePhase::Preflight, "output.exists")),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(RunnerError::new(FailurePhase::Preflight, "output.inspect")),
        }
    }
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    let metadata = fs::metadata(parent)
        .map_err(|_| RunnerError::new(FailurePhase::Preflight, "output.parent"))?;
    if !metadata.is_dir() {
        return Err(RunnerError::new(FailurePhase::Preflight, "output.parent"));
    }
    Ok(())
}

fn safe_locator(locator: &str) -> Result<&str, RunnerError> {
    let suffix = locator
        .strip_prefix("file:")
        .ok_or_else(|| RunnerError::new(FailurePhase::Resolve, "locator.scheme"))?;
    if suffix.is_empty() || suffix.contains(['\\', '\0']) {
        return Err(RunnerError::new(FailurePhase::Resolve, "locator.syntax"));
    }
    let path = Path::new(suffix);
    if path.is_absolute()
        || path.components().any(|component| {
            !matches!(component, Component::Normal(_)) || component.as_os_str().is_empty()
        })
        || suffix
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(RunnerError::new(FailurePhase::Resolve, "locator.syntax"));
    }
    Ok(suffix)
}

fn identity_digest(identity: &str) -> Result<[u8; 32], RunnerError> {
    let value = identity
        .strip_prefix("sha256:")
        .ok_or_else(|| RunnerError::new(FailurePhase::Resolve, "identity.scheme"))?;
    if value.len() != 64
        || value
            .bytes()
            .any(|byte| !byte.is_ascii_digit() && !(b'a'..=b'f').contains(&byte))
    {
        return Err(RunnerError::new(FailurePhase::Resolve, "identity.syntax"));
    }
    let mut digest = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        digest[index] = (hex_nibble(pair[0]) << 4) | hex_nibble(pair[1]);
    }
    Ok(digest)
}

const fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => 0,
    }
}

fn hash_reader(reader: &mut File) -> Result<[u8; 32], RunnerError> {
    use std::io::Seek;
    reader
        .rewind()
        .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.seek"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.read"))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    reader
        .rewind()
        .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.seek"))?;
    Ok(hasher.finalize().into())
}

fn resolve_sources(model: &SessionTomlV1, root: &Path) -> Result<Vec<PreparedSource>, RunnerError> {
    let root_metadata = fs::symlink_metadata(root)
        .map_err(|_| RunnerError::new(FailurePhase::Resolve, "root.open"))?;
    if !root_metadata.file_type().is_dir() || root_metadata.file_type().is_symlink() {
        return Err(RunnerError::new(FailurePhase::Resolve, "root.type"));
    }
    let root = fs::canonicalize(root)
        .map_err(|_| RunnerError::new(FailurePhase::Resolve, "root.canonical"))?;
    let quantum = usize::try_from(model.quantum_frames)
        .ok()
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| RunnerError::new(FailurePhase::Resolve, "quantum.invalid"))?;
    let mut identities = BTreeMap::<PathBuf, [u8; 32]>::new();
    let mut sources = Vec::new();
    sources
        .try_reserve_exact(model.sources.len())
        .map_err(|_| RunnerError::new(FailurePhase::Resolve, "sources.resource"))?;
    for declaration in &model.sources {
        let suffix = safe_locator(&declaration.content.locator)?;
        let requested = root.join(suffix);
        let metadata = fs::symlink_metadata(&requested)
            .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.missing"))?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(RunnerError::new(FailurePhase::Resolve, "source.type"));
        }
        let canonical = fs::canonicalize(&requested)
            .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.canonical"))?;
        if !canonical.starts_with(&root) {
            return Err(RunnerError::new(FailurePhase::Resolve, "source.escape"));
        }
        let expected = identity_digest(&declaration.content.identity)?;
        if let Some(prior) = identities.get(&canonical) {
            if prior != &expected {
                return Err(RunnerError::new(FailurePhase::Resolve, "source.alias"));
            }
        } else {
            identities.insert(canonical.clone(), expected);
        }
        let mut file = File::open(&canonical)
            .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.open"))?;
        if hash_reader(&mut file)? != expected {
            return Err(RunnerError::new(FailurePhase::Resolve, "identity.mismatch"));
        }
        let wave = parse_native_wave(
            &mut file,
            NativeWaveParseCaps {
                max_chunk_count: 4_096,
                max_skipped_metadata_bytes: 16 * 1024 * 1024,
            },
        )
        .map_err(map_wave_prepare)?;
        if wave.sample_rate_hz.0 != declaration.sample_rate_hz
            || declaration.sample_rate_hz != model.sample_rate_hz
        {
            return Err(RunnerError::new(FailurePhase::Resolve, "source.rate"));
        }
        if wave.channel_count != u16::from(declaration.mapping.channel_count) {
            return Err(RunnerError::new(FailurePhase::Resolve, "source.channels"));
        }
        let region = NativeWaveRegion {
            start_frame: SourceFrame(declaration.mapping.region.start_sample),
            length_frames: declaration.mapping.region.length_samples,
        };
        let decoder =
            NativeWaveDecoder::prepare(file, wave, region, quantum).map_err(map_wave_prepare)?;
        let channels = usize::from(wave.channel_count);
        let samples = channels
            .checked_mul(quantum.get())
            .ok_or_else(|| RunnerError::new(FailurePhase::Resolve, "source.overflow"))?;
        let mut planar = Vec::new();
        planar
            .try_reserve_exact(samples)
            .map_err(|_| RunnerError::new(FailurePhase::Resolve, "source.resource"))?;
        planar.resize(samples, 0.0);
        sources.push(PreparedSource {
            id: declaration.id.as_str().as_bytes().to_vec(),
            rate: declaration.sample_rate_hz,
            region_start: region.start_frame.0,
            consumed: 0,
            decoder,
            planar,
            channel_count: channels,
        });
    }
    sources.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    Ok(sources)
}

fn map_wave_prepare(error: NativeWaveError) -> RunnerError {
    let code = match error {
        NativeWaveError::Io(_) => "wave.io",
        NativeWaveError::ContainerInvalid => "wave.container",
        NativeWaveError::FormatUnsupported => "wave.format",
        NativeWaveError::ArithmeticOverflow => "wave.overflow",
        NativeWaveError::ResourceLimit => "wave.resource",
        NativeWaveError::RegionOutOfBounds => "wave.region",
        NativeWaveError::OutputShape => "wave.shape",
    };
    RunnerError::new(FailurePhase::Decode, code)
}

fn feed_sources(
    engine: &mut dyn EngineBoundary,
    sources: &mut [PreparedSource],
    quantum_frames: u32,
) -> Result<(), RunnerError> {
    let quantum = usize::try_from(quantum_frames)
        .map_err(|_| RunnerError::new(FailurePhase::Decode, "quantum.platform"))?;
    for source in sources {
        if source.consumed == source.decoder.region().length_frames {
            continue;
        }
        source.planar.fill(0.0);
        let mut plane_storage: [std::mem::MaybeUninit<&mut [f32]>; 255] =
            [const { std::mem::MaybeUninit::uninit() }; 255];
        for (channel, destination) in plane_storage[..source.channel_count].iter_mut().enumerate() {
            // SAFETY: Each channel selects one disjoint `quantum` range in the preallocated planar
            // block. The resulting borrows live only through this decoder call.
            let plane = unsafe {
                std::slice::from_raw_parts_mut(
                    source.planar.as_mut_ptr().add(channel * quantum),
                    quantum,
                )
            };
            destination.write(plane);
        }
        // SAFETY: Exactly `channel_count` entries were initialized above and are consumed only for
        // the decoder call; the backing samples stay live and disjoint.
        let planes = unsafe {
            std::slice::from_raw_parts_mut(
                plane_storage.as_mut_ptr().cast::<&mut [f32]>(),
                source.channel_count,
            )
        };
        let report = source
            .decoder
            .decode_into(planes)
            .map_err(map_wave_prepare)?;
        let start = source
            .region_start
            .checked_add(source.consumed)
            .ok_or_else(|| RunnerError::new(FailurePhase::Decode, "frame.overflow"))?;
        engine.submit(SubmitChunk {
            id: &source.id,
            rate: source.rate,
            start,
            planar: &source.planar,
            channel_count: source.channel_count,
            plane_stride: quantum,
            frames: report.decoded_frames,
            end: report.end_of_region,
        })?;
        source.consumed = source
            .consumed
            .checked_add(u64::from(report.decoded_frames))
            .ok_or_else(|| RunnerError::new(FailurePhase::Decode, "frame.overflow"))?;
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Clone, Copy, Default)]
enum RealOutputFault {
    #[default]
    None,
    #[cfg(test)]
    ShortWrite,
    #[cfg(test)]
    Flush,
    #[cfg(test)]
    Sync,
    #[cfg(test)]
    Digest,
    #[cfg(test)]
    Publish,
    #[cfg(test)]
    PartialUnlink,
    #[cfg(test)]
    FinalVerify,
}

#[derive(Default)]
struct RealOutput {
    fault: RealOutputFault,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

impl FileIdentity {
    #[cfg(unix)]
    fn from_file(file: &File) -> Option<Self> {
        let metadata = file.metadata().ok()?;
        Some(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }

    #[cfg(windows)]
    fn from_file(file: &File) -> Option<Self> {
        use std::ffi::c_void;

        #[repr(C)]
        #[derive(Clone, Copy, Default)]
        struct FileTime {
            low: u32,
            high: u32,
        }
        #[repr(C)]
        #[derive(Clone, Copy, Default)]
        struct ByHandleFileInformation {
            attributes: u32,
            creation: FileTime,
            access: FileTime,
            write: FileTime,
            volume_serial: u32,
            size_high: u32,
            size_low: u32,
            link_count: u32,
            index_high: u32,
            index_low: u32,
        }
        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetFileInformationByHandle(
                file: *mut c_void,
                information: *mut ByHandleFileInformation,
            ) -> i32;
        }
        let mut information = ByHandleFileInformation::default();
        // SAFETY: The retained std File supplies a live Windows handle and `information` is exact
        // writable fixed storage for this call.
        if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &raw mut information) } == 0 {
            return None;
        }
        Some(Self {
            device: u64::from(information.volume_serial),
            inode: (u64::from(information.index_high) << 32) | u64::from(information.index_low),
        })
    }

    #[cfg(not(any(unix, windows)))]
    fn from_file(_file: &File) -> Option<Self> {
        None
    }
}

struct RealOutputSink {
    final_path: PathBuf,
    partial_path: PathBuf,
    file: Option<File>,
    expected_bytes: u64,
    written_bytes: u64,
    digest: Sha256,
    identity: FileIdentity,
    #[allow(dead_code)]
    fault: RealOutputFault,
}

impl RealOutputSink {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn open_without_follow(path: &Path) -> Option<File> {
        const O_NOFOLLOW: i32 = 0x2_0000;
        OpenOptions::new()
            .read(true)
            .custom_flags(O_NOFOLLOW)
            .open(path)
            .ok()
    }

    #[cfg(target_vendor = "apple")]
    fn open_without_follow(path: &Path) -> Option<File> {
        const O_NOFOLLOW: i32 = 0x100;
        OpenOptions::new()
            .read(true)
            .custom_flags(O_NOFOLLOW)
            .open(path)
            .ok()
    }

    #[cfg(windows)]
    fn open_without_follow(path: &Path) -> Option<File> {
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x20_0000;
        OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .ok()
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_vendor = "apple",
        windows
    )))]
    fn open_without_follow(_path: &Path) -> Option<File> {
        None
    }

    fn path_is_owned(&self, path: &Path) -> bool {
        let Some(metadata) = fs::symlink_metadata(path).ok() else {
            return false;
        };
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return false;
        }
        let Some(file) = Self::open_without_follow(path) else {
            return false;
        };
        file.metadata()
            .ok()
            .is_some_and(|metadata| metadata.file_type().is_file())
            && FileIdentity::from_file(&file) == Some(self.identity)
    }

    fn remove_owned_partial(&self) {
        if self.path_is_owned(&self.partial_path) {
            let _ = fs::remove_file(&self.partial_path);
        }
    }

    fn remove_owned_final(&self) {
        if self.path_is_owned(&self.final_path) {
            let _ = fs::remove_file(&self.final_path);
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn publish_owned_inode(&self, _file: &File) -> Result<(), RunnerError> {
        const AT_FDCWD: i32 = -100;
        const RENAME_NOREPLACE: u32 = 1;
        unsafe extern "C" {
            fn renameat2(
                old_directory: i32,
                old_path: *const std::ffi::c_char,
                new_directory: i32,
                new_path: *const std::ffi::c_char,
                flags: u32,
            ) -> i32;
        }
        if !self.path_is_owned(&self.partial_path) {
            return Err(RunnerError::new(FailurePhase::Publish, "path.replaced"));
        }
        let partial_path = CString::new(self.partial_path.as_os_str().as_bytes())
            .map_err(|_| RunnerError::new(FailurePhase::Publish, "partial.path"))?;
        let final_path = CString::new(self.final_path.as_os_str().as_bytes())
            .map_err(|_| RunnerError::new(FailurePhase::Publish, "final.path"))?;
        // SAFETY: Exclusive ownership keeps the checked partial stable until this single atomic
        // operation. RENAME_NOREPLACE rejects every existing final entry.
        let result = unsafe {
            renameat2(
                AT_FDCWD,
                partial_path.as_ptr(),
                AT_FDCWD,
                final_path.as_ptr(),
                RENAME_NOREPLACE,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(RunnerError::new(
                FailurePhase::Publish,
                "final.exists_or_publish",
            ))
        }
    }

    #[cfg(target_vendor = "apple")]
    fn publish_owned_inode(&self, _file: &File) -> Result<(), RunnerError> {
        const AT_FDCWD: i32 = -2;
        const RENAME_EXCL: u32 = 0x4;
        unsafe extern "C" {
            fn renameatx_np(
                old_directory: i32,
                old_path: *const std::ffi::c_char,
                new_directory: i32,
                new_path: *const std::ffi::c_char,
                flags: u32,
            ) -> i32;
        }
        if !self.path_is_owned(&self.partial_path) {
            return Err(RunnerError::new(FailurePhase::Publish, "path.replaced"));
        }
        let partial_path = CString::new(self.partial_path.as_os_str().as_bytes())
            .map_err(|_| RunnerError::new(FailurePhase::Publish, "partial.path"))?;
        let final_path = CString::new(self.final_path.as_os_str().as_bytes())
            .map_err(|_| RunnerError::new(FailurePhase::Publish, "final.path"))?;
        // SAFETY: Exclusive ownership keeps the checked partial stable until this atomic call.
        // Darwin RENAME_EXCL rejects every existing final entry.
        let result = unsafe {
            renameatx_np(
                AT_FDCWD,
                partial_path.as_ptr(),
                AT_FDCWD,
                final_path.as_ptr(),
                RENAME_EXCL,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(RunnerError::new(
                FailurePhase::Publish,
                "final.exists_or_publish",
            ))
        }
    }

    #[cfg(windows)]
    fn publish_owned_inode(&self, file: &File) -> Result<(), RunnerError> {
        use std::{ffi::c_void, mem::size_of, ptr::null_mut};

        #[repr(C)]
        struct FileRenameInfo {
            replace_if_exists: u8,
            root_directory: *mut c_void,
            file_name_length: u32,
            file_name: [u16; 1],
        }
        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn SetFileInformationByHandle(
                file: *mut c_void,
                information_class: u32,
                information: *const c_void,
                bytes: u32,
            ) -> i32;
        }
        const FILE_RENAME_INFO_CLASS: u32 = 3;
        let final_name: Vec<u16> = self.final_path.as_os_str().encode_wide().collect();
        let name_bytes = final_name
            .len()
            .checked_mul(size_of::<u16>())
            .and_then(|bytes| u32::try_from(bytes).ok())
            .ok_or_else(|| RunnerError::new(FailurePhase::Publish, "final.path"))?;
        let total = size_of::<FileRenameInfo>()
            .checked_add(final_name.len().saturating_sub(1) * size_of::<u16>())
            .ok_or_else(|| RunnerError::new(FailurePhase::Publish, "final.path"))?;
        let mut storage = vec![0_u64; total.div_ceil(size_of::<u64>())];
        let info = storage.as_mut_ptr().cast::<FileRenameInfo>();
        // SAFETY: `storage` is aligned and large enough for the fixed header plus exact UTF-16
        // name. The handle is the retained create-new file; ReplaceIfExists is false, so Windows
        // atomically moves that exact handle and rejects every existing final name.
        let result = unsafe {
            (*info).replace_if_exists = 0;
            (*info).root_directory = null_mut();
            (*info).file_name_length = name_bytes;
            std::ptr::copy_nonoverlapping(
                final_name.as_ptr(),
                (*info).file_name.as_mut_ptr(),
                final_name.len(),
            );
            SetFileInformationByHandle(
                file.as_raw_handle(),
                FILE_RENAME_INFO_CLASS,
                info.cast(),
                u32::try_from(total)
                    .map_err(|_| RunnerError::new(FailurePhase::Publish, "final.path"))?,
            )
        };
        if result != 0 {
            Ok(())
        } else {
            Err(RunnerError::new(
                FailurePhase::Publish,
                "final.exists_or_publish",
            ))
        }
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_vendor = "apple",
        windows
    )))]
    fn publish_owned_inode(&self, _file: &File) -> Result<(), RunnerError> {
        Err(RunnerError::new(
            FailurePhase::Preflight,
            "platform.unsupported",
        ))
    }
}

trait PublicationAdapter {
    fn partial_is_owned(&mut self) -> bool;
    fn publish_held(&mut self) -> Result<(), RunnerError>;
    fn partial_is_absent(&mut self) -> bool;
    fn final_is_owned(&mut self) -> bool;
    fn remove_owned_final(&mut self);
}

// Adapter calls are phase boundaries. Tests may substitute entries between calls, but deliberately
// do not claim safety for a mutation inside an OS pathname operation: the caller's exclusive
// output-directory ownership is the precondition that excludes that interstitial race.
fn complete_publication(adapter: &mut impl PublicationAdapter) -> Result<(), RunnerError> {
    if !adapter.partial_is_owned() {
        return Err(RunnerError::new(FailurePhase::Output, "partial.replaced"));
    }
    adapter.publish_held()?;
    if !adapter.partial_is_absent() || !adapter.final_is_owned() {
        adapter.remove_owned_final();
        return Err(RunnerError::new(FailurePhase::Publish, "path.replaced"));
    }
    if !adapter.final_is_owned() {
        adapter.remove_owned_final();
        return Err(RunnerError::new(FailurePhase::Publish, "final.shape"));
    }
    Ok(())
}

struct OsPublication<'a> {
    sink: &'a RealOutputSink,
    file: &'a File,
    final_checks: u8,
}

impl PublicationAdapter for OsPublication<'_> {
    fn partial_is_owned(&mut self) -> bool {
        self.sink.path_is_owned(&self.sink.partial_path)
    }

    fn publish_held(&mut self) -> Result<(), RunnerError> {
        #[cfg(test)]
        if matches!(self.sink.fault, RealOutputFault::Publish) {
            return Err(RunnerError::new(FailurePhase::Publish, "injected.publish"));
        }
        #[cfg(test)]
        if matches!(self.sink.fault, RealOutputFault::PartialUnlink) {
            return Err(RunnerError::new(FailurePhase::Publish, "partial.remove"));
        }
        self.sink.publish_owned_inode(self.file)
    }

    fn final_is_owned(&mut self) -> bool {
        self.final_checks = self.final_checks.saturating_add(1);
        #[cfg(test)]
        if matches!(self.sink.fault, RealOutputFault::FinalVerify) && self.final_checks == 2 {
            return false;
        }
        self.sink.path_is_owned(&self.sink.final_path)
    }

    fn partial_is_absent(&mut self) -> bool {
        !self.sink.partial_path.exists() && !self.sink.partial_path.is_symlink()
    }

    fn remove_owned_final(&mut self) {
        self.sink.remove_owned_final();
    }
}

impl Drop for RealOutputSink {
    fn drop(&mut self) {
        self.file.take();
        self.remove_owned_partial();
    }
}

impl OutputBoundary for RealOutput {
    fn begin(
        &self,
        final_path: &Path,
        expected_bytes: u64,
    ) -> Result<Box<dyn OutputSink>, RunnerError> {
        preflight_output(final_path)?;
        let partial_path = partial_path(final_path)?;
        #[cfg(not(windows))]
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&partial_path)
            .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.create"))?;
        #[cfg(windows)]
        let file = {
            const DELETE: u32 = 0x1_0000;
            const GENERIC_READ: u32 = 0x8000_0000;
            const GENERIC_WRITE: u32 = 0x4000_0000;
            const FILE_SHARE_READ: u32 = 1;
            const FILE_SHARE_WRITE: u32 = 2;
            const FILE_SHARE_DELETE: u32 = 4;
            OpenOptions::new()
                .read(true)
                .write(true)
                .access_mode(GENERIC_READ | GENERIC_WRITE | DELETE)
                .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
                .create_new(true)
                .open(&partial_path)
                .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.create"))?
        };
        let identity = FileIdentity::from_file(&file)
            .ok_or_else(|| RunnerError::new(FailurePhase::Preflight, "platform.identity"))?;
        Ok(Box::new(RealOutputSink {
            final_path: final_path.to_path_buf(),
            partial_path,
            file: Some(file),
            expected_bytes,
            written_bytes: 0,
            digest: Sha256::new(),
            identity,
            fault: self.fault,
        }))
    }
}

impl OutputSink for RealOutputSink {
    fn write_block(&mut self, samples: &[f32]) -> Result<(), RunnerError> {
        #[cfg(test)]
        if matches!(self.fault, RealOutputFault::ShortWrite) {
            if let Some(sample) = samples.first() {
                let bytes = sample.to_bits().to_le_bytes();
                self.file
                    .as_mut()
                    .ok_or_else(|| RunnerError::new(FailurePhase::Output, "partial.closed"))?
                    .write_all(&bytes)
                    .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.write"))?;
                self.written_bytes = 4;
            }
            return Err(RunnerError::new(
                FailurePhase::Output,
                "partial.short_write",
            ));
        }
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| RunnerError::new(FailurePhase::Output, "partial.closed"))?;
        for sample in samples {
            let bytes = sample.to_bits().to_le_bytes();
            file.write_all(&bytes)
                .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.write"))?;
            self.digest.update(bytes);
            self.written_bytes = self
                .written_bytes
                .checked_add(4)
                .ok_or_else(|| RunnerError::new(FailurePhase::Output, "partial.overflow"))?;
        }
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), RunnerError> {
        let mut file = self
            .file
            .take()
            .ok_or_else(|| RunnerError::new(FailurePhase::Output, "partial.closed"))?;
        #[cfg(test)]
        if matches!(self.fault, RealOutputFault::Flush) {
            return Err(RunnerError::new(FailurePhase::Output, "partial.flush"));
        }
        file.flush()
            .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.flush"))?;
        #[cfg(test)]
        if matches!(self.fault, RealOutputFault::Sync) {
            return Err(RunnerError::new(FailurePhase::Output, "partial.sync"));
        }
        file.sync_all()
            .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.sync"))?;
        if self.written_bytes != self.expected_bytes {
            return Err(RunnerError::new(FailurePhase::Output, "partial.length"));
        }
        if !self.path_is_owned(&self.partial_path) {
            return Err(RunnerError::new(FailurePhase::Output, "partial.replaced"));
        }
        let expected_digest: [u8; 32] = self.digest.clone().finalize().into();
        file.seek(SeekFrom::Start(0))
            .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.verify"))?;
        let mut verifier = Sha256::new();
        let mut buffer = [0_u8; 16 * 1024];
        loop {
            let count = file
                .read(&mut buffer)
                .map_err(|_| RunnerError::new(FailurePhase::Output, "partial.verify"))?;
            if count == 0 {
                break;
            }
            verifier.update(&buffer[..count]);
        }
        #[cfg(test)]
        if matches!(self.fault, RealOutputFault::Digest) {
            return Err(RunnerError::new(FailurePhase::Output, "partial.digest"));
        }
        if <[u8; 32]>::from(verifier.finalize()) != expected_digest {
            return Err(RunnerError::new(FailurePhase::Output, "partial.digest"));
        }
        complete_publication(&mut OsPublication {
            sink: &self,
            file: &file,
            final_checks: 0,
        })
    }
}

#[derive(Default)]
struct CAbi {
    engine: *mut capi::Engine,
    session: *mut capi::Session,
    plan: *mut capi::Plan,
}

impl Drop for CAbi {
    fn drop(&mut self) {
        // SAFETY: These are the unique live handles returned by the matching frozen C ABI calls.
        unsafe {
            capi::miso_engine_v2_plan_destroy(self.plan);
            capi::miso_engine_v2_session_destroy(self.session);
            capi::miso_engine_v2_engine_destroy(self.engine);
        }
        self.plan = ptr::null_mut();
        self.session = ptr::null_mut();
        self.engine = ptr::null_mut();
    }
}

impl EngineBoundary for CAbi {
    fn compile(
        &mut self,
        session: &[u8],
        rate: u32,
        quantum: u32,
        ring_frames: u32,
    ) -> Result<(), RunnerError> {
        let config = capi::EngineConfig {
            struct_size: capi::ENGINE_CONFIG_SIZE,
            abi_version: capi::ABI_VERSION,
            reserved: [0; 4],
        };
        // SAFETY: All pointers refer to live caller-owned fixed ABI storage for this call.
        let result =
            unsafe { capi::miso_engine_v2_engine_create(&raw const config, &raw mut self.engine) };
        if result != capi::RESULT_OK {
            return Err(RunnerError::new(FailurePhase::Compile, "engine.create"));
        }
        let limits = capi::CompileLimits {
            struct_size: capi::COMPILE_LIMITS_SIZE,
            source_ring_frames: ring_frames,
            maximum_automation_spans_per_block: 4_096,
            reserved0: 0,
            maximum_toml_bytes: MAX_SESSION_BYTES,
            maximum_diagnostic_bytes: DIAGNOSTIC_CAPACITY as u64,
            maximum_tracks: MAX_SOURCES,
            maximum_sources: MAX_SOURCES,
            maximum_routes: 16_384,
            maximum_effects: 16_384,
            maximum_graph_session_plus_plan_bytes: 512 * 1024 * 1024,
            maximum_source_total_bytes: 512 * 1024 * 1024,
            maximum_source_overhead_bytes: 128 * 1024 * 1024,
            maximum_effect_state_bytes: 512 * 1024 * 1024,
            maximum_effect_scratch_bytes: 512 * 1024 * 1024,
            maximum_builtin_retained_bytes: 512 * 1024 * 1024,
            maximum_capi_retained_bytes: 128 * 1024 * 1024,
            maximum_named_allocation_bytes: 512 * 1024 * 1024,
            maximum_meter_streams: 16_384,
            maximum_meter_items: 1_000_000,
            maximum_meter_bytes: 128 * 1024 * 1024,
            maximum_control_frame_bytes: 1024 * 1024,
            maximum_replay_bytes: 16 * 1024 * 1024,
            maximum_replay_entries: 65_536,
            reserved: [0; 4],
        };
        let mut diagnostics = vec![0_u8; DIAGNOSTIC_CAPACITY];
        let mut output = capi::BytesOut {
            struct_size: capi::BYTES_OUT_SIZE,
            reserved0: 0,
            data: diagnostics.as_mut_ptr(),
            capacity_bytes: diagnostics.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: Handles and borrowed buffers satisfy their fixed ABI contracts for this call.
        let result = unsafe {
            capi::miso_engine_v2_compile_session(
                self.engine,
                session.as_ptr(),
                session.len() as u64,
                &raw const limits,
                &raw mut output,
                &raw mut self.session,
                &raw mut self.plan,
            )
        };
        if result != capi::RESULT_OK {
            return Err(RunnerError::new(FailurePhase::Compile, "session.rejected"));
        }
        let mut resources = capi::PlanResourceReport {
            struct_size: capi::PLAN_RESOURCE_REPORT_SIZE,
            abi_version: capi::ABI_VERSION,
            sample_rate_hz: 0,
            quantum_frames: 0,
            source_count: 0,
            track_count: 0,
            latency_samples: 0,
            tail_kind: 0,
            tail_samples: 0,
            graph_session_plus_plan_bytes: 0,
            graph_incremental_plan_bytes: 0,
            graph_metadata_bytes: 0,
            graph_delay_bytes: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            source_pcm_payload_bytes: 0,
            source_overhead_bytes: 0,
            source_total_bytes: 0,
            effect_scalar_state_bytes: 0,
            effect_scalar_scratch_bytes: 0,
            builtin_processor_payload_bytes: 0,
            builtin_meter_payload_bytes: 0,
            builtin_retained_payload_bytes: 0,
            capi_retained_bytes: 0,
            largest_named_allocation_bytes: 0,
            reserved: [0; 4],
        };
        // SAFETY: `self.plan` is live and `resources` is writable fixed ABI storage.
        let result = unsafe { capi::miso_engine_v2_plan_resources(self.plan, &raw mut resources) };
        if result != capi::RESULT_OK
            || resources.quantum_frames != quantum
            || resources.sample_rate_hz != rate
        {
            return Err(RunnerError::new(
                FailurePhase::Compile,
                "resources.mismatch",
            ));
        }
        Ok(())
    }

    fn submit(&mut self, submitted: SubmitChunk<'_>) -> Result<(), RunnerError> {
        let mut plane_pointers = [ptr::null(); 255];
        for (channel, pointer) in plane_pointers[..submitted.channel_count]
            .iter_mut()
            .enumerate()
        {
            *pointer = submitted.planar[channel * submitted.plane_stride..].as_ptr();
        }
        let chunk = capi::SourceChunk {
            struct_size: capi::SOURCE_CHUNK_SIZE,
            sample_rate_hz: submitted.rate,
            generation: 1,
            start_frame: submitted.start,
            planes: plane_pointers.as_ptr(),
            plane_count: submitted.channel_count as u32,
            frames: submitted.frames,
            end_of_region: u32::from(submitted.end),
            reserved0: 0,
        };
        let mut report = capi::SubmitReport {
            struct_size: capi::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        // SAFETY: The session handle is live and all borrowed source storage remains live for call.
        let result = unsafe {
            capi::miso_engine_v2_source_submit_planar_f32(
                self.session,
                submitted.id.as_ptr(),
                submitted.id.len() as u64,
                &raw const chunk,
                &raw mut report,
            )
        };
        if result == capi::RESULT_BACKPRESSURE {
            return Err(RunnerError::new(FailurePhase::Submit, "backpressure"));
        }
        if result != capi::RESULT_OK || report.accepted_frames != u64::from(submitted.frames) {
            return Err(RunnerError::new(FailurePhase::Submit, "rejected"));
        }
        Ok(())
    }

    fn render(
        &mut self,
        absolute: u64,
        quantum: u32,
        output: &mut [f32],
    ) -> Result<(), RunnerError> {
        let descriptor = capi::PlanarOutput {
            struct_size: capi::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: output.as_mut_ptr(),
            sample_capacity: output.len() as u64,
            frames: quantum,
            plane_stride_samples: quantum,
            reserved: [0; 2],
        };
        // SAFETY: The plan is live and the output descriptor borrows the complete mutable slice.
        let result = unsafe {
            capi::miso_engine_v2_render_f32_planar(self.plan, absolute, &raw const descriptor)
        };
        if result != capi::RESULT_OK {
            return Err(RunnerError::new(FailurePhase::Render, "rejected"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    };

    const FIXTURES: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/native-pcm-runner/v1"
    );
    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_dir(label: &str) -> PathBuf {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "miso-native-pcm-runner-{}-{label}-{id}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create temp directory");
        path
    }

    fn fixture_args(name: &str, output: PathBuf) -> RunnerArgs {
        RunnerArgs {
            session: Path::new(FIXTURES).join(format!("{name}.toml")),
            source_root: PathBuf::from(FIXTURES),
            frames: 1_024,
            output,
        }
    }

    #[test]
    fn cli_is_closed_and_exact() {
        let valid = [
            "--session",
            "a",
            "--source-root",
            "b",
            "--frames",
            "8",
            "--output",
            "c",
        ];
        assert_eq!(
            parse_cli(valid.map(OsString::from)).expect("valid").frames,
            8
        );
        for arguments in [
            vec!["--session", "a"],
            vec![
                "--frames",
                "0",
                "--session",
                "a",
                "--source-root",
                "b",
                "--output",
                "c",
            ],
            vec![
                "--frames",
                "x",
                "--session",
                "a",
                "--source-root",
                "b",
                "--output",
                "c",
            ],
            vec![
                "--other",
                "x",
                "--session",
                "a",
                "--source-root",
                "b",
                "--frames",
                "8",
                "--output",
                "c",
            ],
            vec![
                "--session",
                "a",
                "--session",
                "b",
                "--source-root",
                "c",
                "--frames",
                "8",
                "--output",
                "d",
            ],
            vec!["positional", "x"],
        ] {
            assert!(parse_cli(arguments.into_iter().map(OsString::from)).is_err());
        }
    }

    #[test]
    fn locator_and_identity_grammars_are_closed() {
        assert_eq!(safe_locator("file:a/b.wav"), Ok("a/b.wav"));
        for value in [
            "host:a",
            "file:",
            "file:/a",
            "file:a//b",
            "file:a/./b",
            "file:a/../b",
            "file:a\\b",
        ] {
            assert_eq!(
                safe_locator(value).expect_err(value).phase,
                FailurePhase::Resolve
            );
        }
        assert!(identity_digest(&format!("sha256:{}", "a".repeat(64))).is_ok());
        for value in [
            format!("sha256:{}", "A".repeat(64)),
            format!("sha256:{}", "a".repeat(63)),
            format!("sha512:{}", "a".repeat(64)),
        ] {
            assert_eq!(
                identity_digest(&value).expect_err(&value).code,
                if value.starts_with("sha256:") {
                    "identity.syntax"
                } else {
                    "identity.scheme"
                }
            );
        }
    }

    #[test]
    fn real_c_abi_riff_and_rf64_render_exact_block_planar_outputs() {
        for name in [
            "riff-44100",
            "riff-48000",
            "riff-88200",
            "riff-96000",
            "rf64-48000",
        ] {
            let temp = temp_dir(name);
            let output = temp.join("output.f32le");
            run(&fixture_args(name, output.clone())).expect(name);
            let bytes = fs::read(&output).expect("read accepted output");
            assert_eq!(bytes.len(), 8_192);
            assert_eq!(bytes.len() % 4, 0);
            let bits: Vec<u32> = bytes
                .chunks_exact(4)
                .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("word")))
                .collect();
            assert_eq!(bits.len(), 2_048);
            assert!(bits.iter().all(|bits| f32::from_bits(*bits).is_finite()));
            if name.starts_with("riff") {
                assert!(bits.iter().any(|bits| *bits != 0));
            }
            let digest: [u8; 32] = Sha256::digest(&bytes).into();
            assert_eq!(hex_digest(digest), expected_output_digest(name));
            assert!(!partial_path(&output).expect("partial").exists());
            fs::remove_dir_all(temp).expect("remove temp");
        }
    }

    fn expected_output_digest(name: &str) -> &'static str {
        match name {
            "riff-44100" => "49663d8451e470a7a05511e68388ebff7b4d844db42d38e9632473f897a0b91d",
            "riff-48000" => "1e856978bbd412daebd2ac9dd81f554e4c3512244ce36b7437bb65cc5f43c99e",
            "riff-88200" => "bc8aa669d31090d7cc9a0abf740e6c63cf719db47cf5dc071fc724e19dfe6fff",
            "riff-96000" => "5645de29f441710a3a7b67f2e4a24e93086c9baa34426d8963e3f278ceb9d516",
            "rf64-48000" => "43fa3c4ed46228d1ee13050b118f379f82a021e85f5dfff6f72593912e298ad0",
            _ => panic!("unknown fixture"),
        }
    }

    fn hex_digest(digest: [u8; 32]) -> String {
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[derive(Default)]
    struct MockEngine {
        fail_compile: bool,
        fail_submit: bool,
        fail_render: bool,
        compile_calls: usize,
        ring_frames: Option<u32>,
        submissions: Vec<(Vec<u8>, u64, u32, bool)>,
        renders: Vec<u64>,
    }

    impl EngineBoundary for MockEngine {
        fn compile(
            &mut self,
            _: &[u8],
            _: u32,
            _: u32,
            ring_frames: u32,
        ) -> Result<(), RunnerError> {
            self.compile_calls += 1;
            self.ring_frames = Some(ring_frames);
            if self.fail_compile {
                Err(RunnerError::new(FailurePhase::Compile, "mock"))
            } else {
                Ok(())
            }
        }

        fn submit(&mut self, chunk: SubmitChunk<'_>) -> Result<(), RunnerError> {
            self.submissions
                .push((chunk.id.to_vec(), chunk.start, chunk.frames, chunk.end));
            if self.fail_submit {
                Err(RunnerError::new(FailurePhase::Submit, "backpressure"))
            } else {
                Ok(())
            }
        }

        fn render(
            &mut self,
            absolute: u64,
            quantum: u32,
            output: &mut [f32],
        ) -> Result<(), RunnerError> {
            self.renders.push(absolute);
            if self.fail_render {
                return Err(RunnerError::new(FailurePhase::Render, "mock"));
            }
            for (index, sample) in output.iter_mut().enumerate() {
                *sample = absolute as f32 + index as f32 + quantum as f32;
            }
            Ok(())
        }
    }

    #[derive(Default)]
    struct MemoryOutput {
        bytes: Arc<Mutex<Vec<u8>>>,
        begin_calls: Arc<AtomicU64>,
    }
    struct MemorySink {
        bytes: Arc<Mutex<Vec<u8>>>,
    }

    #[derive(Clone, Copy)]
    enum OutputFailure {
        Create,
        Write,
        Sync,
        Publish,
    }
    struct FailingOutput {
        failure: OutputFailure,
    }
    struct FailingSink {
        failure: OutputFailure,
    }
    impl OutputBoundary for FailingOutput {
        fn begin(&self, _: &Path, _: u64) -> Result<Box<dyn OutputSink>, RunnerError> {
            if matches!(self.failure, OutputFailure::Create) {
                return Err(RunnerError::new(FailurePhase::Output, "injected.create"));
            }
            Ok(Box::new(FailingSink {
                failure: self.failure,
            }))
        }
    }
    impl OutputSink for FailingSink {
        fn write_block(&mut self, _: &[f32]) -> Result<(), RunnerError> {
            if matches!(self.failure, OutputFailure::Write) {
                Err(RunnerError::new(FailurePhase::Output, "injected.write"))
            } else {
                Ok(())
            }
        }
        fn finish(self: Box<Self>) -> Result<(), RunnerError> {
            match self.failure {
                OutputFailure::Sync => Err(RunnerError::new(FailurePhase::Output, "injected.sync")),
                OutputFailure::Publish => {
                    Err(RunnerError::new(FailurePhase::Publish, "injected.publish"))
                }
                OutputFailure::Create | OutputFailure::Write => unreachable!(),
            }
        }
    }
    impl OutputBoundary for MemoryOutput {
        fn begin(&self, _: &Path, _: u64) -> Result<Box<dyn OutputSink>, RunnerError> {
            self.begin_calls.fetch_add(1, Ordering::Relaxed);
            Ok(Box::new(MemorySink {
                bytes: Arc::clone(&self.bytes),
            }))
        }
    }
    impl OutputSink for MemorySink {
        fn write_block(&mut self, samples: &[f32]) -> Result<(), RunnerError> {
            let mut bytes = self.bytes.lock().expect("memory output");
            for sample in samples {
                bytes.extend_from_slice(&sample.to_bits().to_le_bytes());
            }
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<(), RunnerError> {
            Ok(())
        }
    }

    #[test]
    fn shared_runner_orders_short_final_submission_and_terminal_failures() {
        let temp = temp_dir("mock");
        let arguments = fixture_args("rf64-48000", temp.join("out"));
        let output = MemoryOutput::default();
        let mut engine = MockEngine::default();
        run_with(&arguments, &mut engine, &output).expect("mock run");
        assert_eq!(engine.compile_calls, 1);
        assert_eq!(engine.ring_frames, Some(1_024));
        assert_eq!(engine.renders, [0, 128, 256, 384, 512, 640, 768, 896]);
        assert_eq!(
            engine.submissions,
            [
                (b"fixture-source".to_vec(), 1, 128, false),
                (b"fixture-source".to_vec(), 129, 128, false),
                (b"fixture-source".to_vec(), 257, 128, false),
                (b"fixture-source".to_vec(), 385, 128, false),
                (b"fixture-source".to_vec(), 513, 2, true),
            ]
        );
        assert_eq!(output.bytes.lock().expect("bytes").len(), 8_192);

        for (compile, submit, render, phase) in [
            (true, false, false, FailurePhase::Compile),
            (false, true, false, FailurePhase::Submit),
            (false, false, true, FailurePhase::Render),
        ] {
            let mut engine = MockEngine {
                fail_compile: compile,
                fail_submit: submit,
                fail_render: render,
                ..MockEngine::default()
            };
            let output = MemoryOutput::default();
            assert_eq!(
                run_with(&arguments, &mut engine, &output)
                    .expect_err("failure")
                    .phase,
                phase
            );
        }
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn preflight_and_resolution_fail_before_compile_or_output() {
        let temp = temp_dir("negative");
        let output_path = temp.join("out");
        fs::write(&output_path, b"sentinel").expect("sentinel");
        let mut engine = MockEngine::default();
        let output = MemoryOutput::default();
        let error = run_with(
            &fixture_args("riff-48000", output_path.clone()),
            &mut engine,
            &output,
        )
        .expect_err("existing output");
        assert_eq!(
            error,
            RunnerError::new(FailurePhase::Preflight, "output.exists")
        );
        assert_eq!(fs::read(&output_path).expect("preserved"), b"sentinel");
        assert_eq!(engine.compile_calls, 0);
        assert_eq!(output.begin_calls.load(Ordering::Relaxed), 0);

        let mut invalid = fixture_args("riff-48000", temp.join("other"));
        invalid.frames = 7;
        assert_eq!(
            run_with(&invalid, &mut engine, &output)
                .expect_err("quantum")
                .code,
            "frames.quantum"
        );
        assert_eq!(engine.compile_calls, 0);
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn unsupported_platform_stops_before_output_source_engine_or_publication() {
        let temp = temp_dir("unsupported-platform");
        let mut arguments = fixture_args("riff-48000", temp.join("out"));
        arguments.source_root = temp.join("missing-source-root");
        let mut engine = MockEngine::default();
        let output = MemoryOutput::default();
        let error = run_with_platform(&arguments, &mut engine, &output, false)
            .expect_err("unsupported platform");
        assert_eq!(
            (error.phase, error.code),
            (FailurePhase::Preflight, "platform.unsupported")
        );
        assert_eq!(engine.compile_calls, 0);
        assert_eq!(output.begin_calls.load(Ordering::Relaxed), 0);
        assert!(!arguments.output.exists());
        assert!(!partial_path(&arguments.output).expect("partial").exists());
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn publication_refuses_every_preexisting_final_and_partial_kind() {
        for (kind, at_partial) in [
            ("regular", false),
            ("symlink", false),
            ("hardlink", false),
            ("directory", false),
            ("regular", true),
            ("symlink", true),
            ("hardlink", true),
            ("directory", true),
        ] {
            let temp = temp_dir(&format!("{kind}-{at_partial}"));
            let sentinel = temp.join("sentinel");
            fs::write(&sentinel, b"sentinel").expect("sentinel");
            let output = temp.join("out");
            let collision = if at_partial {
                partial_path(&output).expect("partial")
            } else {
                output.clone()
            };
            match kind {
                "regular" => fs::write(&collision, b"sentinel").expect("regular"),
                "symlink" => std::os::unix::fs::symlink(&sentinel, &collision).expect("symlink"),
                "hardlink" => fs::hard_link(&sentinel, &collision).expect("hardlink"),
                "directory" => fs::create_dir(&collision).expect("directory"),
                _ => unreachable!(),
            }
            let before = fs::read(&sentinel).expect("before");
            assert_eq!(
                run(&fixture_args("riff-48000", output))
                    .expect_err(kind)
                    .phase,
                FailurePhase::Preflight
            );
            assert_eq!(fs::read(&sentinel).expect("after"), before);
            fs::remove_dir_all(temp).expect("remove temp");
        }
    }

    #[test]
    fn resolver_rejects_identity_shape_file_shape_and_declaration_mismatches_precompile() {
        for (label, replace_from, replace_to, expected) in [
            ("uppercase", "sha256:", "sha256:A", "identity.syntax"),
            (
                "wrong-rate",
                "sample_rate_hz = 48000",
                "sample_rate_hz = 44100",
                "source.rate",
            ),
            (
                "wrong-channels",
                "channel_count = 2",
                "channel_count = 1",
                "source.channels",
            ),
            (
                "wrong-region",
                "length_samples = 514",
                "length_samples = 9999",
                "wave.region",
            ),
        ] {
            let temp = temp_dir(label);
            fs::copy(
                Path::new(FIXTURES).join("rf64-48000.wav"),
                temp.join("rf64-48000.wav"),
            )
            .expect("copy source");
            let original =
                fs::read_to_string(Path::new(FIXTURES).join("rf64-48000.toml")).expect("session");
            fs::write(
                temp.join("session.toml"),
                original.replacen(replace_from, replace_to, 1),
            )
            .expect("mutated session");
            let arguments = RunnerArgs {
                session: temp.join("session.toml"),
                source_root: temp.clone(),
                frames: 1_024,
                output: temp.join("out"),
            };
            let mut engine = MockEngine::default();
            let output = MemoryOutput::default();
            assert_eq!(
                run_with(&arguments, &mut engine, &output)
                    .expect_err(label)
                    .code,
                expected
            );
            assert_eq!(engine.compile_calls, 0);
            assert_eq!(output.begin_calls.load(Ordering::Relaxed), 0);
            fs::remove_dir_all(temp).expect("remove temp");
        }

        let temp = temp_dir("truncated");
        fs::write(temp.join("rf64-48000.wav"), b"RF64").expect("truncated");
        let session =
            fs::read_to_string(Path::new(FIXTURES).join("rf64-48000.toml")).expect("session");
        let actual = hex_digest(Sha256::digest(b"RF64").into());
        let start = session.find("sha256:").expect("identity") + 7;
        let mut changed = session;
        changed.replace_range(start..start + 64, &actual);
        fs::write(temp.join("session.toml"), changed).expect("session");
        let arguments = RunnerArgs {
            session: temp.join("session.toml"),
            source_root: temp.clone(),
            frames: 1_024,
            output: temp.join("out"),
        };
        let mut engine = MockEngine::default();
        let output = MemoryOutput::default();
        assert_eq!(
            run_with(&arguments, &mut engine, &output)
                .expect_err("truncated")
                .code,
            "wave.container"
        );
        assert_eq!(engine.compile_calls, 0);
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn source_symlink_is_rejected_and_sentinel_preserved() {
        let temp = temp_dir("source-symlink");
        let target = Path::new(FIXTURES).join("riff-48000.wav");
        std::os::unix::fs::symlink(&target, temp.join("riff-48000.wav")).expect("symlink");
        fs::copy(
            Path::new(FIXTURES).join("riff-48000.toml"),
            temp.join("session.toml"),
        )
        .expect("session");
        let arguments = RunnerArgs {
            session: temp.join("session.toml"),
            source_root: temp.clone(),
            frames: 1_024,
            output: temp.join("out"),
        };
        let mut engine = MockEngine::default();
        let output = MemoryOutput::default();
        assert_eq!(
            run_with(&arguments, &mut engine, &output)
                .expect_err("symlink")
                .code,
            "source.type"
        );
        assert_eq!(engine.compile_calls, 0);
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn encoder_preserves_signed_zero_and_publication_is_no_clobber() {
        let temp = temp_dir("signed-zero");
        let final_path = temp.join("out");
        let output = RealOutput::default();
        let mut sink = output.begin(&final_path, 8).expect("begin");
        sink.write_block(&[0.0, -0.0]).expect("write");
        sink.finish().expect("publish");
        assert_eq!(
            fs::read(&final_path).expect("output"),
            [0, 0, 0, 0, 0, 0, 0, 128]
        );
        assert_eq!(
            fs::symlink_metadata(&final_path)
                .expect("accepted metadata")
                .nlink(),
            1
        );
        assert!(!partial_path(&final_path).expect("partial").exists());
        let error = match RealOutput::default().begin(&final_path, 8) {
            Ok(_) => panic!("second publication must not overwrite"),
            Err(error) => error,
        };
        assert_eq!(
            (error.phase, error.code),
            (FailurePhase::Preflight, "output.exists")
        );
        assert_eq!(
            fs::read(&final_path).expect("preserved output"),
            [0, 0, 0, 0, 0, 0, 0, 128]
        );
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn injected_create_write_and_publish_failures_are_terminal() {
        let temp = temp_dir("injected-output");
        let arguments = fixture_args("rf64-48000", temp.join("accepted"));
        for (injection, expected_phase) in [
            (OutputFailure::Create, FailurePhase::Output),
            (OutputFailure::Write, FailurePhase::Output),
            (OutputFailure::Sync, FailurePhase::Output),
            (OutputFailure::Publish, FailurePhase::Publish),
        ] {
            let mut engine = MockEngine::default();
            let output = FailingOutput { failure: injection };
            assert_eq!(
                run_with(&arguments, &mut engine, &output)
                    .expect_err("injected output failure")
                    .phase,
                expected_phase
            );
            assert!(!arguments.output.exists());
        }
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn scalar_caps_precede_resolution_and_cover_overflow_and_unsupported_rate() {
        let temp = temp_dir("scalar-caps");
        let base =
            fs::read_to_string(Path::new(FIXTURES).join("riff-48000.toml")).expect("base session");
        let ring_overflow = base.replace("pcm_ring_frames = 1024", "pcm_ring_frames = 4294967296");
        fs::write(temp.join("ring.toml"), ring_overflow).expect("ring session");
        let mut engine = MockEngine::default();
        let output = MemoryOutput::default();
        let arguments = RunnerArgs {
            session: temp.join("ring.toml"),
            source_root: temp.join("missing-root"),
            frames: 1_024,
            output: temp.join("out"),
        };
        assert_eq!(
            run_with(&arguments, &mut engine, &output)
                .expect_err("ring overflow")
                .code,
            "ring.overflow"
        );
        assert_eq!(engine.compile_calls, 0);

        let mut overflow = fixture_args("riff-48000", temp.join("overflow"));
        overflow.frames = MAX_FRAMES + 1;
        assert_eq!(
            run_with(&overflow, &mut engine, &output)
                .expect_err("frame overflow")
                .code,
            "frames.overflow"
        );

        let unsupported = base.replace("sample_rate_hz = 48000", "sample_rate_hz = 192000");
        fs::write(temp.join("unsupported.toml"), unsupported).expect("unsupported session");
        let unsupported = RunnerArgs {
            session: temp.join("unsupported.toml"),
            source_root: PathBuf::from(FIXTURES),
            frames: 1_024,
            output: temp.join("unsupported"),
        };
        assert_eq!(
            run_with(&unsupported, &mut engine, &output)
                .expect_err("unsupported rate")
                .code,
            "session.invalid"
        );
        assert_eq!(engine.compile_calls, 0);
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn missing_mismatched_and_truncated_riff_sources_are_exact_precompile_failures() {
        let base =
            fs::read_to_string(Path::new(FIXTURES).join("riff-48000.toml")).expect("base session");
        for (label, source, session, code) in [
            ("missing", None, base.clone(), "source.missing"),
            (
                "identity",
                Some(fs::read(Path::new(FIXTURES).join("riff-48000.wav")).expect("wave")),
                base.replacen(
                    "sha256:01113503536aed2ab96afd970f8f0bf2ddeb3a7115ae8c3663fd6ff9a345d085",
                    &format!("sha256:{}", "0".repeat(64)),
                    1,
                ),
                "identity.mismatch",
            ),
            (
                "truncated-riff",
                Some(b"RIFF".to_vec()),
                {
                    let digest = hex_digest(Sha256::digest(b"RIFF").into());
                    base.replacen(
                        "sha256:01113503536aed2ab96afd970f8f0bf2ddeb3a7115ae8c3663fd6ff9a345d085",
                        &format!("sha256:{digest}"),
                        1,
                    )
                },
                "wave.container",
            ),
        ] {
            let temp = temp_dir(label);
            if let Some(source) = source {
                fs::write(temp.join("riff-48000.wav"), source).expect("source");
            }
            fs::write(temp.join("session.toml"), session).expect("session");
            let arguments = RunnerArgs {
                session: temp.join("session.toml"),
                source_root: temp.clone(),
                frames: 1_024,
                output: temp.join("out"),
            };
            let mut engine = MockEngine::default();
            assert_eq!(
                run_with(&arguments, &mut engine, &MemoryOutput::default())
                    .expect_err(label)
                    .code,
                code
            );
            assert_eq!(engine.compile_calls, 0);
            assert!(!arguments.output.exists());
            fs::remove_dir_all(temp).expect("remove temp");
        }
    }

    #[test]
    fn reversed_source_declarations_submit_in_canonical_id_order() {
        let temp = temp_dir("source-order");
        fs::copy(
            Path::new(FIXTURES).join("rf64-48000.wav"),
            temp.join("rf64-48000.wav"),
        )
        .expect("source");
        let base = fs::read_to_string(Path::new(FIXTURES).join("rf64-48000.toml"))
            .expect("session")
            .replace("fixture-source", "z-source");
        let declaration = base
            .lines()
            .find(|line| line.contains("{ id = \"z-source\""))
            .expect("source declaration");
        let second = declaration.replace("z-source", "a-source");
        let session = base.replacen(declaration, &format!("{declaration}\n{second}"), 1);
        fs::write(temp.join("session.toml"), session).expect("session");
        let arguments = RunnerArgs {
            session: temp.join("session.toml"),
            source_root: temp.clone(),
            frames: 128,
            output: temp.join("out"),
        };
        let mut engine = MockEngine::default();
        run_with(&arguments, &mut engine, &MemoryOutput::default()).expect("ordered run");
        assert_eq!(engine.submissions.len(), 2);
        assert_eq!(engine.submissions[0].0, b"a-source");
        assert_eq!(engine.submissions[1].0, b"z-source");
        assert_eq!(engine.ring_frames, Some(1_024));
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[test]
    fn real_output_faults_remove_only_the_owned_partial() {
        for (label, fault, phase, code) in [
            (
                "short-write",
                RealOutputFault::ShortWrite,
                FailurePhase::Output,
                "partial.short_write",
            ),
            (
                "flush",
                RealOutputFault::Flush,
                FailurePhase::Output,
                "partial.flush",
            ),
            (
                "sync",
                RealOutputFault::Sync,
                FailurePhase::Output,
                "partial.sync",
            ),
            (
                "digest",
                RealOutputFault::Digest,
                FailurePhase::Output,
                "partial.digest",
            ),
            (
                "publish",
                RealOutputFault::Publish,
                FailurePhase::Publish,
                "injected.publish",
            ),
            (
                "partial-unlink",
                RealOutputFault::PartialUnlink,
                FailurePhase::Publish,
                "partial.remove",
            ),
            (
                "final-verify",
                RealOutputFault::FinalVerify,
                FailurePhase::Publish,
                "final.shape",
            ),
        ] {
            let temp = temp_dir(label);
            let output_path = temp.join("out");
            let arguments = fixture_args("rf64-48000", output_path.clone());
            let mut engine = MockEngine::default();
            let output = RealOutput { fault };
            let error = run_with(&arguments, &mut engine, &output).expect_err(label);
            assert_eq!((error.phase, error.code), (phase, code));
            assert!(!output_path.exists());
            assert!(!partial_path(&output_path).expect("partial").exists());
            fs::remove_dir_all(temp).expect("remove temp");
        }

        let temp = temp_dir("length");
        let final_path = temp.join("out");
        let mut sink = RealOutput::default()
            .begin(&final_path, 12)
            .expect("length begin");
        sink.write_block(&[0.0, -0.0]).expect("length write");
        let error = sink.finish().expect_err("length mismatch");
        assert_eq!(
            (error.phase, error.code),
            (FailurePhase::Output, "partial.length")
        );
        assert!(!final_path.exists());
        assert!(!partial_path(&final_path).expect("partial").exists());
        fs::remove_dir_all(temp).expect("remove temp");
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum FakeEntry {
        Owned,
        WrongPublished,
        Sentinel(&'static str),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum FakeMutation {
        None,
        ReplacePartialBeforeInitial(&'static str),
        ReplacePartialBeforePublish(&'static str),
        ReplacePartialAfterPublish(&'static str),
        ReplaceFinalAfterPublish(&'static str),
        FinalCollision(&'static str),
        WrongIdentityPublication,
        PublishFailure,
        PartialUnlinkFailure,
        FinalVerificationFailure,
    }

    struct FakePublication {
        partial: Option<FakeEntry>,
        final_entry: Option<FakeEntry>,
        mutation: FakeMutation,
        partial_checks: u8,
        final_checks: u8,
        publish_calls: u8,
    }

    impl FakePublication {
        fn new(mutation: FakeMutation) -> Self {
            Self {
                partial: Some(FakeEntry::Owned),
                final_entry: None,
                mutation,
                partial_checks: 0,
                final_checks: 0,
                publish_calls: 0,
            }
        }

        fn replace_partial(&mut self, shape: &'static str) {
            self.partial = Some(FakeEntry::Sentinel(shape));
        }

        fn drop_owned_partial(&mut self) {
            if self.partial == Some(FakeEntry::Owned) {
                self.partial = None;
            }
        }
    }

    impl PublicationAdapter for FakePublication {
        fn partial_is_owned(&mut self) -> bool {
            self.partial_checks += 1;
            if let (FakeMutation::ReplacePartialBeforeInitial(shape), 1) =
                (self.mutation, self.partial_checks)
            {
                self.replace_partial(shape);
            }
            self.partial == Some(FakeEntry::Owned)
        }

        fn publish_held(&mut self) -> Result<(), RunnerError> {
            self.publish_calls += 1;
            match self.mutation {
                FakeMutation::ReplacePartialBeforePublish(shape) => {
                    self.replace_partial(shape);
                    return Err(RunnerError::new(FailurePhase::Publish, "path.replaced"));
                }
                FakeMutation::FinalCollision(shape) => {
                    self.final_entry = Some(FakeEntry::Sentinel(shape));
                }
                FakeMutation::PublishFailure => {
                    return Err(RunnerError::new(
                        FailurePhase::Publish,
                        "final.exists_or_publish",
                    ));
                }
                FakeMutation::PartialUnlinkFailure => {
                    return Err(RunnerError::new(FailurePhase::Publish, "partial.remove"));
                }
                _ => {}
            }
            if self.final_entry.is_some() {
                return Err(RunnerError::new(
                    FailurePhase::Publish,
                    "final.exists_or_publish",
                ));
            }
            self.final_entry = Some(if self.mutation == FakeMutation::WrongIdentityPublication {
                FakeEntry::WrongPublished
            } else {
                FakeEntry::Owned
            });
            self.partial = None;
            Ok(())
        }

        fn partial_is_absent(&mut self) -> bool {
            if let FakeMutation::ReplacePartialAfterPublish(shape) = self.mutation {
                self.replace_partial(shape);
            }
            self.partial.is_none()
        }

        fn final_is_owned(&mut self) -> bool {
            self.final_checks += 1;
            if let (FakeMutation::ReplaceFinalAfterPublish(shape), 1) =
                (self.mutation, self.final_checks)
            {
                self.final_entry = Some(FakeEntry::Sentinel(shape));
            }
            if self.mutation == FakeMutation::FinalVerificationFailure && self.final_checks == 2 {
                return false;
            }
            self.final_entry == Some(FakeEntry::Owned)
        }

        fn remove_owned_final(&mut self) {
            if self.final_entry == Some(FakeEntry::Owned) {
                self.final_entry = None;
            }
        }
    }

    #[test]
    fn portable_publication_state_machine_freezes_every_race_and_failure() {
        for shape in ["regular", "symlink", "hardlink", "rename"] {
            for (mutation, phase, code) in [
                (
                    FakeMutation::ReplacePartialBeforeInitial(shape),
                    FailurePhase::Output,
                    "partial.replaced",
                ),
                (
                    FakeMutation::ReplacePartialBeforePublish(shape),
                    FailurePhase::Publish,
                    "path.replaced",
                ),
                (
                    FakeMutation::ReplacePartialAfterPublish(shape),
                    FailurePhase::Publish,
                    "path.replaced",
                ),
            ] {
                let mut adapter = FakePublication::new(mutation);
                let error = complete_publication(&mut adapter).expect_err("partial replacement");
                adapter.drop_owned_partial();
                assert_eq!((error.phase, error.code), (phase, code));
                assert_eq!(adapter.partial, Some(FakeEntry::Sentinel(shape)));
                assert_eq!(adapter.final_entry, None);
            }

            let mut adapter = FakePublication::new(FakeMutation::ReplaceFinalAfterPublish(shape));
            let error = complete_publication(&mut adapter).expect_err("final replacement");
            assert_eq!(
                (error.phase, error.code),
                (FailurePhase::Publish, "path.replaced")
            );
            assert_eq!(adapter.partial, None);
            assert_eq!(adapter.final_entry, Some(FakeEntry::Sentinel(shape)));

            let mut adapter = FakePublication::new(FakeMutation::FinalCollision(shape));
            let error = complete_publication(&mut adapter).expect_err("final collision");
            adapter.drop_owned_partial();
            assert_eq!(
                (error.phase, error.code),
                (FailurePhase::Publish, "final.exists_or_publish")
            );
            assert_eq!(adapter.final_entry, Some(FakeEntry::Sentinel(shape)));
            assert_eq!(adapter.publish_calls, 1);
        }

        let mut wrong_identity = FakePublication::new(FakeMutation::WrongIdentityPublication);
        let error =
            complete_publication(&mut wrong_identity).expect_err("wrong publication identity");
        assert_eq!(
            (error.phase, error.code),
            (FailurePhase::Publish, "path.replaced")
        );
        assert_eq!(wrong_identity.partial, None);
        assert_eq!(
            wrong_identity.final_entry,
            Some(FakeEntry::WrongPublished),
            "known-unowned final is preserved rather than deleted"
        );

        for (mutation, code) in [
            (FakeMutation::PublishFailure, "final.exists_or_publish"),
            (FakeMutation::PartialUnlinkFailure, "partial.remove"),
            (FakeMutation::FinalVerificationFailure, "final.shape"),
        ] {
            let mut adapter = FakePublication::new(mutation);
            let error = complete_publication(&mut adapter).expect_err("publication failure");
            adapter.drop_owned_partial();
            assert_eq!((error.phase, error.code), (FailurePhase::Publish, code));
            assert_eq!(adapter.partial, None);
            assert_eq!(adapter.final_entry, None);
        }

        let mut accepted = FakePublication::new(FakeMutation::None);
        complete_publication(&mut accepted).expect("first publication");
        assert_eq!(accepted.partial, None);
        assert_eq!(accepted.final_entry, Some(FakeEntry::Owned));
        assert_eq!(accepted.publish_calls, 1);
        let mut second = FakePublication::new(FakeMutation::None);
        second.final_entry = accepted.final_entry;
        let error = complete_publication(&mut second).expect_err("second publication");
        second.drop_owned_partial();
        assert_eq!(
            (error.phase, error.code),
            (FailurePhase::Publish, "final.exists_or_publish")
        );
        assert_eq!(second.final_entry, Some(FakeEntry::Owned));
    }

    #[test]
    fn post_create_partial_replacements_are_preserved_and_never_published() {
        for kind in ["regular", "symlink", "hardlink", "rename"] {
            let temp = temp_dir(&format!("race-{kind}"));
            let final_path = temp.join("out");
            let partial = partial_path(&final_path).expect("partial");
            let owned_away = temp.join("owned-away");
            let sentinel = temp.join("sentinel");
            fs::write(&sentinel, b"sentinel").expect("sentinel");
            let mut sink = RealOutput::default().begin(&final_path, 8).expect("begin");
            sink.write_block(&[0.0, -0.0]).expect("write");
            fs::rename(&partial, &owned_away).expect("move owned inode");
            match kind {
                "regular" => fs::write(&partial, b"replacement").expect("regular"),
                "symlink" => std::os::unix::fs::symlink(&sentinel, &partial).expect("symlink"),
                "hardlink" => fs::hard_link(&sentinel, &partial).expect("hardlink"),
                "rename" => {
                    let replacement = temp.join("replacement");
                    fs::write(&replacement, b"renamed").expect("replacement");
                    fs::rename(replacement, &partial).expect("renamed replacement");
                }
                _ => unreachable!(),
            }
            let error = sink.finish().expect_err(kind);
            assert_eq!(
                (error.phase, error.code),
                (FailurePhase::Output, "partial.replaced")
            );
            assert!(!final_path.exists());
            assert!(partial.exists() || partial.is_symlink());
            assert_eq!(fs::read(&sentinel).expect("sentinel"), b"sentinel");
            fs::remove_dir_all(temp).expect("remove temp");
        }
    }

    #[test]
    fn final_collisions_inserted_immediately_before_publication_are_preserved() {
        for kind in ["regular", "symlink", "hardlink", "directory", "rename"] {
            let temp = temp_dir(&format!("late-final-{kind}"));
            let final_path = temp.join("out");
            let sentinel = temp.join("sentinel");
            fs::write(&sentinel, b"sentinel").expect("sentinel");
            let mut sink = RealOutput::default().begin(&final_path, 8).expect("begin");
            sink.write_block(&[0.0, -0.0]).expect("write");
            match kind {
                "regular" => fs::write(&final_path, b"regular").expect("regular"),
                "symlink" => {
                    std::os::unix::fs::symlink(&sentinel, &final_path).expect("symlink");
                }
                "hardlink" => fs::hard_link(&sentinel, &final_path).expect("hardlink"),
                "directory" => fs::create_dir(&final_path).expect("directory"),
                "rename" => {
                    let replacement = temp.join("replacement");
                    fs::write(&replacement, b"renamed").expect("replacement");
                    fs::rename(replacement, &final_path).expect("rename");
                }
                _ => unreachable!(),
            }
            let error = sink.finish().expect_err(kind);
            assert_eq!(
                (error.phase, error.code),
                (FailurePhase::Publish, "final.exists_or_publish")
            );
            assert!(!partial_path(&final_path).expect("partial").exists());
            let metadata = fs::symlink_metadata(&final_path).expect("preserved final");
            match kind {
                "regular" | "rename" => assert!(metadata.file_type().is_file()),
                "symlink" => assert!(metadata.file_type().is_symlink()),
                "hardlink" => {
                    assert!(metadata.file_type().is_file());
                    assert_eq!(
                        fs::symlink_metadata(&sentinel).expect("sentinel").nlink(),
                        2
                    );
                }
                "directory" => assert!(metadata.file_type().is_dir()),
                _ => unreachable!(),
            }
            assert_eq!(fs::read(&sentinel).expect("sentinel bytes"), b"sentinel");
            fs::remove_dir_all(temp).expect("remove temp");
        }
    }
}
