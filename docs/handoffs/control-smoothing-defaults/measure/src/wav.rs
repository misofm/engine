//! Minimal RIFF/WAVE I/O: writes stereo interleaved 32-bit float with the 44-byte header the
//! issue-033 listening packets use, and reads 16/24-bit PCM or 32-bit float for an owner-supplied
//! mix excerpt.

use std::fs;
use std::path::Path;

use crate::material::Stereo;

pub fn write_f32_stereo(path: &Path, rate: u32, audio: &Stereo) {
    let frames = audio.left.len();
    assert_eq!(frames, audio.right.len());
    let data_bytes = u32::try_from(frames * 8).expect("wav size");
    let mut bytes = Vec::with_capacity(44 + frames * 8);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_bytes).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&3_u16.to_le_bytes()); // WAVE_FORMAT_IEEE_FLOAT
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&rate.to_le_bytes());
    bytes.extend_from_slice(&(rate * 8).to_le_bytes());
    bytes.extend_from_slice(&8_u16.to_le_bytes());
    bytes.extend_from_slice(&32_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_bytes.to_le_bytes());
    for i in 0..frames {
        bytes.extend_from_slice(&audio.left[i].to_le_bytes());
        bytes.extend_from_slice(&audio.right[i].to_le_bytes());
    }
    fs::write(path, bytes).expect("write wav");
}

fn u16_at(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}

fn u32_at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

/// Reads a mono or stereo WAV (PCM 16/24-bit, or IEEE float 32-bit) and returns its rate and
/// planar samples. A mono file is duplicated into both lanes.
pub fn read(path: &Path) -> (u32, Stereo) {
    let b = fs::read(path).expect("read wav");
    assert!(&b[0..4] == b"RIFF" && &b[8..12] == b"WAVE", "not a RIFF/WAVE file");
    let mut i = 12;
    let mut format = None;
    let mut data = None;
    while i + 8 <= b.len() {
        let id = &b[i..i + 4];
        let size = u32_at(&b, i + 4) as usize;
        let body = i + 8;
        if id == b"fmt " {
            let mut tag = u16_at(&b, body);
            if tag == 0xFFFE {
                tag = u16_at(&b, body + 24); // WAVE_FORMAT_EXTENSIBLE sub-format
            }
            format = Some((
                tag,
                u16_at(&b, body + 2),
                u32_at(&b, body + 4),
                u16_at(&b, body + 14),
            ));
        } else if id == b"data" {
            data = Some((body, size.min(b.len() - body)));
        }
        i = body + size + (size & 1);
    }
    let (tag, channels, rate, bits) = format.expect("fmt chunk");
    let (start, size) = data.expect("data chunk");
    assert!(channels == 1 || channels == 2, "mono or stereo only");
    let width = usize::from(bits / 8);
    let sample = |k: usize| -> f32 {
        let o = start + k * width;
        match (tag, bits) {
            (1, 16) => f32::from(i16::from_le_bytes([b[o], b[o + 1]])) / 32_768.0,
            (1, 24) => {
                let v = i32::from_le_bytes([0, b[o], b[o + 1], b[o + 2]]) >> 8;
                v as f32 / 8_388_608.0
            }
            (3, 32) => f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]),
            _ => panic!("unsupported WAV encoding: format {tag}, {bits} bits"),
        }
    };
    let frames = size / (width * usize::from(channels));
    let (mut left, mut right) = (Vec::with_capacity(frames), Vec::with_capacity(frames));
    for f in 0..frames {
        let l = sample(f * usize::from(channels));
        let r = if channels == 2 { sample(f * 2 + 1) } else { l };
        left.push(l);
        right.push(r);
    }
    (rate, Stereo { left, right })
}
