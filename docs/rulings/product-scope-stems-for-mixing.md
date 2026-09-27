# Product scope: imported stems for mixing, no live audio input

**Owner ruling, 2026-09-27.** Miso is built around importing edited stems that are ready to be
mixed. Live audio input is out of scope: no recording, no input monitoring, no tracking mode, and
no audio-interface capture path in the engine, the hosts or the SDK.

## Why

The owner judged live input too risky for the product now. Its hardest constraints sit outside the
engine's control: how an audio interface reaches the browser, the browser's input latency and
permissions, and how that audio would reach the engine. None of the engine, the hosts or the SDK
implements live input today (no capture, microphone, record-arm or input-monitoring code), so this
ruling removes no code; it keeps future briefs from designing toward it.

## What this means for design

- Everything the engine renders is playback of known, pre-recorded audio, so rendering ahead of the
  audio callback is always allowed. The only realtime deadline is keeping the output fed.
- A few milliseconds of latency between a control change (a fader move, a mute) and hearing it is
  acceptable. Realtime mix controls ("the live console": faders, pan, solo, mute, meters, spectrum,
  automation) stay in scope; they are how a mix is made.
- Effects are never restricted for latency's sake; plugin-delay compensation absorbs them.
- Worker sharding with render-ahead ring buffers needs no low-latency mode.
- The render callback's own rules are unchanged: allocation-, lock- and syscall-free, never
  blocking.

Reopening live input needs a new owner ruling and its own research issue.
