<!--
Provenance: copied from misofm/engine-v2-old docs/research/06-web-audio-runtime.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Web Audio runtime

The web package adapts, rather than defines, the core. An AudioWorklet owns resident stems and calls the same mounted executor used offline/native. WebAudio’s model and render-quantum behavior are specified in the [Web Audio 1.1 specification draft](https://www.w3.org/TR/webaudio-1.1/). The adapter creates/requests its context at the immutable session `sample_rate_hz`, then compares actual `AudioContext`/AudioWorklet `sampleRate` before start. An exact mismatch returns `web_audio_sample_rate_mismatch` with an action to recreate the context or remount; launch has no sample-rate conversion. Core certification at an arbitrary rate does not promise a hardware context accepts it; `OfflineAudioContext` may exercise awkward rates.

f32 ingress widens once into persistent f64 TrackBank planes off the DSP walk; final WebAudio f32 writes are named/counted conversion/copy boundaries. Control communication uses bounded preallocated queues. The [Chrome AudioWorklet WASM ring-buffer pattern](https://googlechromelabs.github.io/web-audio-samples/audio-worklet/design-pattern/wasm-ring-buffer/) is useful evidence for a non-blocking transport, not a substitute for engine event budgets.

SharedArrayBuffer deployment is conditional on cross-origin isolation; capability negotiation reports this honestly and provides a structured fallback/refusal rather than assuming it ([MDN](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/crossOriginIsolated)). Launch executes no plugins and reports plugin capability false. Immediate post-launch hosting is certified/allowlisted only; browser timing does not make untrusted WASM deadline-safe, so watchdog/isolation remains deferred.

Web and native gates render identical session fixtures at 44,100, 47,999, 48,000, 88,200, and 96,000 Hz. Performance goals are separately measured at 48 kHz/128 frames.
