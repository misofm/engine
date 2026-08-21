//! Linux-only Issue-041 actual-duration source allocation-layout and RSS audit.

use std::{
    fs::{File, OpenOptions, remove_file},
    io::Write,
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use miso_engine_core::{QuantumFrames, SampleRateHz, realtime::RenderEnvelope};
use miso_engine_graph::{GraphNodeId, StableGraphId, TrackStage};
use miso_engine_source::{
    NativeResolvedAsset, NativeSourceAllocationLayoutEntry, NativeSourcePrepareCaps,
    NativeSourcePrepareRequest, NativeSourceResolver, NativeSourceResolverError,
    NativeWaveParseCaps, NativeWaveRegion, PcmSourceRingConfig, SourceFrame, SourceGeneration,
    SourceGraphTrackMapping, native_source_allocation_layout, prepare_graph_source_set,
    prepare_native_source,
};

const RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
const ONE_MINUTE_FRAMES: u64 = RATE as u64 * 60;
const MULTI_HOUR_FRAMES: u64 = RATE as u64 * 60 * 60 * 3;

fn main() {
    if let Err(error) = run() {
        eprintln!("source duration audit failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    if !cfg!(target_os = "linux") {
        return Err("Issue-041 duration/RSS audit requires Linux".to_owned());
    }
    let base =
        std::env::temp_dir().join(format!("miso-engine-source-audit-{}", std::process::id()));
    let minute_path = base.with_extension("minute.wav");
    let multi_hour_path = base.with_extension("multi-hour-sparse.wav");
    write_wave(&minute_path, ONE_MINUTE_FRAMES, false)?;
    write_wave(&multi_hour_path, MULTI_HOUR_FRAMES, true)?;
    let minute = capture(&minute_path, ONE_MINUTE_FRAMES)?;
    let multi_hour = capture(&multi_hour_path, MULTI_HOUR_FRAMES)?;
    remove_file(&minute_path).map_err(|error| format!("remove minute: {error}"))?;
    remove_file(&multi_hour_path).map_err(|error| format!("remove multi-hour: {error}"))?;

    if minute.layout != multi_hour.layout {
        return Err("duration changed exact engine allocation-layout multiset".to_owned());
    }
    if minute.source_report != multi_hour.source_report {
        return Err("duration changed exact native source resource report".to_owned());
    }
    if minute.graph_report != multi_hour.graph_report {
        return Err("duration changed exact graph source-set resource report".to_owned());
    }
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue041_source_duration_layout\",",
            "\"minute_frames\":{},\"multi_hour_frames\":{},",
            "\"minute_file_bytes\":{},\"multi_hour_file_bytes\":{},",
            "\"layout_entries\":{},\"layout_total_bytes\":{},",
            "\"layout_equal\":true,\"source_report_equal\":true,\"graph_report_equal\":true,",
            "\"minute_rss_bytes\":{},\"multi_hour_rss_bytes\":{},",
            "\"os\":\"linux\",\"arch\":\"{}\",\"rust\":\"{}\",",
            "\"timed_benchmark_invocations\":0}}"
        ),
        minute.frames,
        multi_hour.frames,
        minute.file_bytes,
        multi_hour.file_bytes,
        minute.layout.len(),
        layout_total(&minute.layout)?,
        minute.rss_bytes,
        multi_hour.rss_bytes,
        std::env::consts::ARCH,
        rust_version().replace('"', ""),
    );
    Ok(())
}

struct Capture {
    frames: u64,
    layout: Vec<NativeSourceAllocationLayoutEntry>,
    source_report: miso_engine_source::NativeSourceResourceReport,
    graph_report: miso_engine_graph::GraphSourceSetResourceReport,
    rss_bytes: u64,
    file_bytes: u64,
}

fn capture(path: &Path, frames: u64) -> Result<Capture, String> {
    let config = PcmSourceRingConfig {
        channel_count: 1,
        quantum_frames: QuantumFrames(QUANTUM),
        frame_capacity: u64::from(QUANTUM) * 5,
        initial_generation: SourceGeneration(1),
    };
    let caps = NativeSourcePrepareCaps {
        parser: NativeWaveParseCaps {
            max_chunk_count: 4,
            max_skipped_metadata_bytes: 0,
        },
        max_worker_read_scratch_bytes: u64::from(QUANTUM) * 4,
        max_total_engine_owned_bytes: u64::MAX,
        max_largest_allocation_bytes: u64::MAX,
        control_queue_items: NonZeroUsize::new(2).expect("two commands"),
    };
    let mut resolver = FileResolver {
        path: path.to_owned(),
        used: false,
    };
    let prepared = prepare_native_source(
        &mut resolver,
        NativeSourcePrepareRequest {
            locator: "audit:actual-wave".to_owned(),
            declared_identity: b"issue041-actual-wave".to_vec(),
            declared_sample_rate_hz: SampleRateHz(RATE),
            engine_sample_rate_hz: SampleRateHz(RATE),
            declared_channel_count: 1,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: frames,
            },
            ring_config: config,
        },
        caps,
    )
    .map_err(|error| format!("prepare {}: {error:?}", path.display()))?;
    let source_report = prepared.resource_report();
    let layout = native_source_allocation_layout(config, caps, source_report)
        .map_err(|error| format!("layout {}: {error:?}", path.display()))?;
    if layout_total(&layout)? != source_report.total_engine_owned_bytes {
        return Err(format!(
            "layout/report total mismatch for {}",
            path.display()
        ));
    }
    let (controller, source) = prepared.into_graph_source();
    let envelope = RenderEnvelope {
        sample_rate: SampleRateHz(RATE),
        quantum: QuantumFrames(QUANTUM),
        input_channels: None,
        output_channels: NonZeroUsize::new(2).expect("stereo"),
    };
    let source_set = prepare_graph_source_set(
        envelope,
        vec![source],
        vec![SourceGraphTrackMapping {
            node: GraphNodeId::TrackStage {
                track_id: StableGraphId::parse("audit.duration").expect("stable id"),
                stage: TrackStage::Input,
            },
            source_index: 0,
            left_channel: 0,
            right_channel: 0,
        }],
    )
    .map_err(|error| format!("source-set {}: {error:?}", path.display()))?;
    let graph_report = source_set.resource_report();
    let rss_bytes = linux_rss_bytes()?;
    let file_bytes = std::fs::metadata(path)
        .map_err(|error| format!("metadata {}: {error}", path.display()))?
        .len();
    drop(source_set);
    drop(controller);
    Ok(Capture {
        frames,
        layout,
        source_report,
        graph_report,
        rss_bytes,
        file_bytes,
    })
}

struct FileResolver {
    path: PathBuf,
    used: bool,
}
impl NativeSourceResolver for FileResolver {
    type Asset = File;
    fn resolve(
        &mut self,
        locator: &str,
    ) -> Result<NativeResolvedAsset<File>, NativeSourceResolverError> {
        if locator != "audit:actual-wave" || self.used {
            return Err(NativeSourceResolverError::Unresolved);
        }
        self.used = true;
        let reader = File::open(&self.path).map_err(|_| NativeSourceResolverError::Unresolved)?;
        Ok(NativeResolvedAsset {
            observed_identity: b"issue041-actual-wave".to_vec(),
            reader,
        })
    }
}

fn write_wave(path: &Path, frames: u64, sparse: bool) -> Result<(), String> {
    let data_bytes = frames
        .checked_mul(4)
        .ok_or_else(|| "data byte overflow".to_owned())?;
    let data_u32 =
        u32::try_from(data_bytes).map_err(|_| "RIFF data exceeds 32-bit audit scope".to_owned())?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open(path)
        .map_err(|error| format!("create {}: {error}", path.display()))?;
    file.write_all(&wave_header(data_u32))
        .map_err(|error| format!("header {}: {error}", path.display()))?;
    if sparse {
        file.set_len(44 + data_bytes)
            .map_err(|error| format!("sparse length {}: {error}", path.display()))?;
    } else {
        let samples = [0.25_f32.to_le_bytes(); 16_384];
        let bytes: Vec<u8> = samples.into_iter().flatten().collect();
        let mut remaining = data_bytes;
        while remaining != 0 {
            let count = usize::try_from(remaining.min(bytes.len() as u64)).expect("bounded write");
            file.write_all(&bytes[..count])
                .map_err(|error| format!("PCM {}: {error}", path.display()))?;
            remaining -= count as u64;
        }
    }
    file.flush()
        .map_err(|error| format!("flush {}: {error}", path.display()))
}

fn wave_header(data_bytes: u32) -> [u8; 44] {
    let riff_size = 36_u32.checked_add(data_bytes).expect("bounded RIFF size");
    let mut header = [0_u8; 44];
    header[..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&riff_size.to_le_bytes());
    header[8..12].copy_from_slice(b"WAVE");
    header[12..16].copy_from_slice(b"fmt ");
    header[16..20].copy_from_slice(&16_u32.to_le_bytes());
    header[20..22].copy_from_slice(&3_u16.to_le_bytes());
    header[22..24].copy_from_slice(&1_u16.to_le_bytes());
    header[24..28].copy_from_slice(&RATE.to_le_bytes());
    header[28..32].copy_from_slice(&(RATE * 4).to_le_bytes());
    header[32..34].copy_from_slice(&4_u16.to_le_bytes());
    header[34..36].copy_from_slice(&32_u16.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&data_bytes.to_le_bytes());
    header
}

fn linux_rss_bytes() -> Result<u64, String> {
    let text = std::fs::read_to_string("/proc/self/status")
        .map_err(|error| format!("read RSS: {error}"))?;
    let line = text
        .lines()
        .find(|line| line.starts_with("VmRSS:"))
        .ok_or_else(|| "VmRSS unavailable".to_owned())?;
    let kib = line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "VmRSS malformed".to_owned())?
        .parse::<u64>()
        .map_err(|error| format!("VmRSS parse: {error}"))?;
    kib.checked_mul(1024)
        .ok_or_else(|| "VmRSS overflow".to_owned())
}

fn rust_version() -> String {
    std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unavailable".to_owned())
        .trim()
        .to_owned()
}

fn layout_total(entries: &[NativeSourceAllocationLayoutEntry]) -> Result<u64, String> {
    entries.iter().try_fold(0_u64, |total, entry| {
        entry
            .requested_size_bytes
            .checked_mul(entry.count)
            .and_then(|bytes| total.checked_add(bytes))
            .ok_or_else(|| "layout total overflow".to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_accounting(capture: &Capture) -> String {
        let mut text = String::new();
        for entry in &capture.layout {
            text.push_str(entry.category);
            text.push('=');
            text.push_str(&entry.requested_size_bytes.to_string());
            text.push('/');
            text.push_str(&entry.alignment_bytes.to_string());
            text.push('/');
            text.push_str(&entry.count.to_string());
            text.push(';');
        }
        text.push_str(&format!(
            "source={:?};graph={:?}",
            capture.source_report, capture.graph_report
        ));
        text
    }

    fn fnv1a64(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
    }

    #[test]
    fn exact_duration_independent_accounting_serialization_is_canonical() {
        let path = std::env::temp_dir().join(format!(
            "miso-engine-source-audit-unit-{}.wav",
            std::process::id()
        ));
        write_wave(&path, QUANTUM.into(), false).expect("write bounded unit WAVE");
        let capture = capture(&path, QUANTUM.into()).expect("capture bounded unit accounting");
        remove_file(&path).expect("remove bounded unit WAVE");

        assert_eq!(capture.layout.len(), 16);
        assert_eq!(layout_total(&capture.layout).expect("layout total"), 4_504);
        let canonical = canonical_accounting(&capture);
        assert_eq!(fnv1a64(canonical.as_bytes()), 0xbc5d_f020_c1e8_ea1a);
        println!(
            "issue041 accounting fnv1a64={:016x} bytes={} minute_identity=wave-f32le-mono-48000-2880000-11520044-materialized multi_hour_identity=wave-f32le-mono-48000-518400000-2073600044-sparse accounting={canonical}",
            fnv1a64(canonical.as_bytes()),
            canonical.len()
        );
    }
}
