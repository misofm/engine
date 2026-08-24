<!--
Provenance: copied from misofm/engine-v2-old docs/research/00-launch-contract.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Launch contract

V0.1 is a mountable resident-stem mixer, not a partial console. Every session requires a finite `sample_rate_hz`, validated at mount and immutable thereafter; it accepts exact finite mono/stereo/dual-mono track counts and processes two f64 planes per track through:

`input built-ins → static Rack A → mount-fixed dynamic rack → static Rack B → output built-ins/meters → deterministic buses`.

Stock effects are EQ, compressor, and true-peak limiter. Static rack order/topology is global per compatible template; values and bypass remain per-track. Dynamic rack order and plugin selection are fixed at mount. An attempted live topology edit returns `requires_remount`, never a best-effort mutation.

The core derives coefficient values, sample-time conversions, latency/tail, and automation from session rate. The contract succeeds only if admission produces a private immutable plan with checked byte, CPU, and event budgets. It refuses overflow, unsupported/unsafe rate, wasm32 address-space pressure, deadline-cost excess, unsupported layout, invalid IDs, and unavailable capabilities using stable machine-readable codes. There is no `MAX_TRACKS` product setting. The first web adapter may choose 48 kHz; it does not hard-code the engine.

Launch ABI reserves tagged plugin capability/descriptor vocabulary but advertises plugins as `capability: false`; plugin ABI, certified hosting, and Walrus execution are immediate post-launch work. A separate plugin-memory boundary, when enabled, is a counted copy. True untrusted browser deadline safety awaits isolation/watchdog work. WIT/component metadata is discovery/control only; the RT ABI is flat numeric data in preallocated memory.

The first client is React + AudioWorklet; processing never relies on the main thread. The adapter requests a context at session `sample_rate_hz` and refuses start with actionable `web_audio_sample_rate_mismatch` when the actual context/worklet rate differs; launch has no SRC. WebAudio’s default render quantum is 128 frames but exposes quantum configuration, so code uses the actual supplied buffer length; 128 is the launch certification point. Host callback APIs establish the general rule: callbacks may occur in high-priority contexts and must return promptly ([PortAudio](https://portaudio.com/docs/v19-doxydocs/portaudio_8h.html), [JACK](https://jackaudio.org/api/group__ClientCallbacks.html)).
