//! Raw Wasm32 staging and block-pump boundary for the ingest Worker.

#![allow(unsafe_code)]

use std::cell::RefCell;

use crate::{
    ABI_VERSION, FlacBlockDecoder, RESULT_END, RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK,
};

struct LiveDecoder {
    handle: u32,
    maximum_canonical_bytes: u64,
    staging: Option<Vec<u8>>,
    decoder: Option<FlacBlockDecoder>,
}

thread_local! {
    static LIVE_DECODER: RefCell<Option<LiveDecoder>> = const { RefCell::new(None) };
}

fn with_live<R>(handle: u32, invalid: R, operation: impl FnOnce(&LiveDecoder) -> R) -> R {
    LIVE_DECODER.with(|slot| {
        let Ok(slot) = slot.try_borrow() else {
            return invalid;
        };
        let Some(live) = slot
            .as_ref()
            .filter(|live| live.handle == handle && handle != 0)
        else {
            return invalid;
        };
        operation(live)
    })
}

fn with_live_mut<R>(handle: u32, invalid: R, operation: impl FnOnce(&mut LiveDecoder) -> R) -> R {
    LIVE_DECODER.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return invalid;
        };
        let Some(live) = slot
            .as_mut()
            .filter(|live| live.handle == handle && handle != 0)
        else {
            return invalid;
        };
        operation(live)
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_create(
    input_bytes: u32,
    maximum_canonical_bytes: u32,
) -> u32 {
    if input_bytes == 0 || maximum_canonical_bytes == 0 {
        return 0;
    }
    LIVE_DECODER.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return 0;
        };
        if slot.is_some() {
            return 0;
        }
        let Ok(input_len) = usize::try_from(input_bytes) else {
            return 0;
        };
        let mut staging = Vec::new();
        if staging.try_reserve_exact(input_len).is_err() {
            return 0;
        }
        staging.resize(input_len, 0);
        *slot = Some(LiveDecoder {
            handle: 1,
            maximum_canonical_bytes: u64::from(maximum_canonical_bytes),
            staging: Some(staging),
            decoder: None,
        });
        1
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_input_pointer(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.staging
            .as_ref()
            .map_or(0, |bytes| bytes.as_ptr() as usize as u32)
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_begin(handle: u32) -> u32 {
    with_live_mut(handle, RESULT_INVALID_ARGUMENT, |live| {
        if live.decoder.is_some() {
            return RESULT_INVALID_ARGUMENT;
        }
        let Some(input) = live.staging.take() else {
            return RESULT_INVALID_ARGUMENT;
        };
        match FlacBlockDecoder::new(input, live.maximum_canonical_bytes) {
            Ok(decoder) => {
                live.decoder = Some(decoder);
                RESULT_OK
            }
            Err(error) => error.result(),
        }
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_decode_next(handle: u32) -> u32 {
    with_live_mut(handle, RESULT_INVALID_ARGUMENT, |live| {
        let Some(decoder) = live.decoder.as_mut() else {
            return RESULT_INVALID_ARGUMENT;
        };
        match decoder.decode_next_block() {
            Ok(true) => RESULT_OK,
            Ok(false) => match decoder.finish_report() {
                Ok(_) => RESULT_END,
                Err(error) => error.result(),
            },
            Err(error) => error.result(),
        }
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_pcm_pointer(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder.as_ref().map_or(0, |decoder| {
            decoder.canonical_block().as_ptr() as usize as u32
        })
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_pcm_length(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder.as_ref().map_or(0, |decoder| {
            u32::try_from(decoder.canonical_block().len()).unwrap_or(0)
        })
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_sample_rate_hz(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder
            .as_ref()
            .map_or(0, |decoder| decoder.stream_info().sample_rate_hz)
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_channels(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder
            .as_ref()
            .map_or(0, |decoder| u32::from(decoder.stream_info().channels))
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_bit_depth(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder.as_ref().map_or(0, |decoder| {
            u32::from(decoder.stream_info().bit_depth.bits())
        })
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_frames_low(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder
            .as_ref()
            .map_or(0, |decoder| decoder.stream_info().frames as u32)
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_frames_high(handle: u32) -> u32 {
    with_live(handle, 0, |live| {
        live.decoder
            .as_ref()
            .map_or(0, |decoder| (decoder.stream_info().frames >> 32) as u32)
    })
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn miso_flac_decoder_v1_dispose(handle: u32) -> u32 {
    LIVE_DECODER.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return RESULT_INTERNAL;
        };
        if slot
            .as_ref()
            .is_some_and(|live| live.handle == handle && handle != 0)
        {
            *slot = None;
            RESULT_OK
        } else {
            RESULT_INVALID_ARGUMENT
        }
    })
}
