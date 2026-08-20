# Console and DAW architecture patterns

## Scope and engineering question

Study documented control/routing patterns from exactly DiGiCo, SSL, Lawo, Avid, and Logic. Adopt typed fixed send taps, typed acyclic routes, exact fixed-latency PDC, and transport/audio separation; do not copy any product architecture.

## Algorithm and equations

N/A — this is an architecture/workflow comparison, not an audio algorithm. Its measurable artifacts are graph validity, route/PDC sample counts, and protocol behavior.

## Coefficients and update rules

N/A — console/DAW interface patterns do not define DSP coefficients. V2 effect coefficient rules live in the applicable DSP topic notes.

## Numerical and stability limits

Graph compilation rejects cycles, invalid typed ports, and incompatible fixed latencies. Route/PDC arithmetic uses exact bounded integer sample counts; no vendor workflow manual is used as numerical proof.

## Latency and tail

V2 adopts exact integer-sample PDC for all paths, sends, and sidechains, and bypass preserves prepared effect latency. Tail is processor metadata, not a route-level inference.

## Units, mappings, automation and smoothing

Control values use stable typed units and absolute sample-time automation. Matrix/pan changes declare smoothing; send-tap, route, and module placement are typed structural choices compiled transactionally.

## Definitions and assumptions

V2 tracks are dual-mono. Cross-channel processing requires declared detector link or smoothed 2x2 matrix. The fixed chain is input -> input builtins -> SIMD rack 1 -> dynamic rack -> SIMD rack 2 -> output builtins -> matrix/pan -> routes; taps are explicit stable enums.

## Adopted V2 decisions

Control mutations compile a replacement `PreparedRenderPlan` off the render thread and transfer it only at a block boundary. The callback owns prepared buffers/state and cannot allocate, block, I/O, log, or mutate graph structure.

## Denormal, signed-zero and NaN policy

N/A — this note defines control/routing patterns. V2 nevertheless requires every route/effect boundary to use the engine-wide finite audio/state policy, with counters for sanitized non-finite values.

## Primary and official sources

[DIGICO-SD] and [DIGICO-CORE2] document channel processing, aux choices, and remote scope. [SSL-REMOTE] and [SSL-TACO] distinguish control surfaces from audio transport. [LAWO-FLOW], [LAWO-BUS], [LAWO-INSERT], and [LAWO-AUTOMATION] document movable flow, pickup choices, inserts, buses, and automation. [AVID-PT-REF] documents Pro Tools channel, insert, send, bus, and automation workflows; [AVID-AAX] documents reported latency compensation. [LOGIC-ROUTING], [LOGIC-LATENCY], and [LOGIC-CONTROL] document buses, PDC, and control surfaces.

## Fixtures

Use typed graph fixtures for every stable send tap, route/submix, sidechain, invalid cycle, differing-latency reconvergence, bypass, matrix ramp, and dual-mono asymmetric input. Include canonical session/protocol snapshots.

## Objective tests and tolerances

Compile deterministically in stable node-ID order; reject cycles; verify every send/sidechain PDC by impulse sample count; assert bypass latency retention, matrix smoothing bounds, canonical snapshot round-trip, and protocol backpressure behavior.

## Rejected alternatives and tradeoffs

Reference graph/PDC tests construct expected path delays and reductions using test-local integer graph models. They do not invoke production graph compiler/scheduler code to compute expected answers.

## Known gaps and follow-up

SIMD cohorting is V2-specific: compatible program/routing signatures bank, incompatible tracks use another cohort or scalar fallback. Native may use deterministic prepared waves; browser remains single render-thread at launch.

## Benchmark plan

Benchmark a fixed graph with full bank/tail, fixed sends/submixes, and a declared PDC layout in exactly two clean native rounds; report median/p95/p99/p99.9 and complete machine/runtime JSON metadata.

## Listening protocol or evidence

Use blinded evidence for audible graph features only after exact PDC and route fixtures pass. Record fixture checksum, randomization, playback chain, gain matching, answers, confidence, and reveal log using `listening/TEMPLATE.md`.

## 17. Decision record

### Comparison matrix

All changing web sources were retrieved on 2026-08-20. A blank vendor capacity or behavior was not
inferred: the cell says when the bounded official source set did not document it.

| Family / official version | Channel structure | Inserts/order/bypass | Send taps | Buses/submixes | Latency/PDC | Automation/timebase | Remote control and PCM | Resource limits in source |
|---|---|---|---|---|---|---|---|---|
| DiGiCo SD/Quantum, current official attachment | Filters, EQ, dynamics, and insert positions [DIGICO-SD]. | Named/positioned insert facilities [DIGICO-SD]. | Aux workflows expose pre/post choices [DIGICO-SD]. | Aux, group, matrix, and master are distinct [DIGICO-SD]. | No exact public PDC contract found in the bounded source. | Snapshots/macros are documented workflow controls [DIGICO-SD]. | Core2 exposes processing/routing/snapshot control [DIGICO-CORE2]; PCM transport was not documented. | Hardware/product capacities are documented but are not V2 limits. |
| SSL Live Help, current online help | Remote material exposes channel-oriented mixer state [SSL-REMOTE]. | Inserted rack effects are remotely controlled [SSL-TACO]. | TaCo exposes aux/stem assignment and level [SSL-TACO]. | Aux/stem routing is explicit [SSL-TACO]. | No exact public PDC contract found in the bounded source. | Shared processing/automation changes are controlled in parallel [SSL-REMOTE]. | Remote control explicitly excludes audio transport [SSL-REMOTE]. | Surface/controller counts are product limits, not V2 limits. |
| Lawo mc²56/mc²96 online manuals | DSP channels include filters, EQ, dynamics, delay, and selectable flow [LAWO-FLOW]. | Insert/direct-out pickup is configurable [LAWO-INSERT]. | Aux pickup includes documented pre/post choices [LAWO-BUS]. | Groups, Auxes, and Sums are distinct [LAWO-BUS]. | External insert latency is handled manually [LAWO-INSERT]. | Timecode automation covers fader, mute, sends, EQ, routing, and flow [LAWO-AUTOMATION]. | Official remote GUI is documented; PCM behavior was not found in the bounded source. | Configured DSP capacities are hardware/product choices, not V2 limits. |
| Avid Pro Tools 2024.6 | Tracks provide inserts, sidechains, sends, and aux paths [AVID-PT-REF]. | Insert/plugin workflow and bypass are explicit [AVID-PT-REF]. | Track sends feed declared paths [AVID-PT-REF]. | Internal buses and aux tracks are first-class [AVID-PT-REF]. | AAX processors report latency for host compensation [AVID-AAX]. | Recorded/editable automation and timebase are documented [AVID-PT-REF]. | Control-surface workflow is documented; no PCM-over-control claim is made [AVID-PT-REF]. | Track capacity is resource/product dependent and not adopted as a V2 constant. |
| Apple Logic Pro, current online guide | Channel strips host plug-ins and sends [LOGIC-ROUTING]. | Plug-ins occupy channel-strip inserts [LOGIC-ROUTING]. | Sends split parallel paths to aux strips [LOGIC-ROUTING]. | Aux and output routing are documented [LOGIC-ROUTING]. | PDC covers channels, aux/output paths, sidechains, and automation [LOGIC-LATENCY]. | Mixer/plugin automation can be controlled and recorded [LOGIC-CONTROL]. | Bidirectional control-surface profiles map mixer/automation state [LOGIC-CONTROL]; PCM transport was not documented. | Product capacities were not adopted as V2 limits. |

### Common patterns

All five families organize processing around channels, provide send/bus routing, expose some remote control, and make latency visible or consequential. This supports V2's broad semantic control model while keeping PCM outside the control protocol.

### Disagreements

Lawo and DiGiCo expose flexible pickup/module placement, whereas the consulted DAW material emphasizes automated compensation across a fixed session graph. Console sources do not supply a uniform public automatic-PDC contract; Avid and Logic explicitly document latency compensation. SSL explicitly separates remote control from audio transport.

### V2 adoptions and measurable reasons

| V2 adoption | Source-backed observation | Measurable reason |
|---|---|---|
| Stable typed send-tap enum | Console/DAW workflows expose multiple pickup points. | Every enum value is covered by impulse/PDC fixtures and canonical session serialization. |
| Typed acyclic graph rather than arbitrary live reordering | Flexible flow exists in console workflows; DAW compensation depends on known paths. | Compiler rejects cycles and produces deterministic topological order. |
| Exact fixed-latency PDC, including bypass | Avid/Logic document latency compensation [AVID-AAX] [LOGIC-LATENCY]. | Reconvergent send/sidechain impulses align at exact integer samples. |
| Transport-neutral control, no PCM in protocol | SSL distinguishes remote control from audio transport [SSL-REMOTE]. | Protocol fixtures contain no PCM payload and queue saturation yields typed backpressure. |
| Explicit dual-mono/link/matrix model | Channel workflow alone does not establish implicit stereo semantics. | Asymmetric L/R and matrix-ramp fixtures prove no undeclared cross-channel coupling. |

Open question: future feedback routing requires a separate issue with a positive-latency edge and dedicated safety/PDC tests.
