# C ABI V1 native qualification

#1033 (owner ruling R2, 2026-09-28) removed the native PCM reference runner, its fixtures and its
checks; the C ABI stays as the mobile interface. The runner rows below are historical.

Issue 114 qualifies the joined, accepted Issue-116 native PCM runner and Issue-121 C ABI product.
It does not change or reseal either product. The accepted C header, CAPI/protocol implementation,
runner, runner contract, portability checks, Cargo lock, session fixture, and native runner corpus
were pinned by `fixtures/capi-qualification/v1/AUTHORITIES.sha256`. #1029 deleted that ledger,
which no code read and which named checkers #319 had already deleted; the files named below are at
<https://github.com/misofm/engine/tree/5379e46ca3b349b9d277d642c008bb7a9643fb76/fixtures/capi-qualification/v1>.
The C ABI's live gates are `scripts/check-capi-abi.sh`, the `capi` tests and `audit capi`.

The shipped mobile libraries are the `aarch64-apple-ios` and `aarch64-linux-android` `capi`
staticlibs, built by `scripts/build-capi-release.sh <target> staticlib <out-dir>` (#1499). It runs
`cargo rustc --locked --release -p capi --lib --crate-type staticlib --target <target>`, because
`cargo build -p capi` builds no LTO while `crates/capi/Cargo.toml` lists `rlib` (cargo passes
`-C lto` only when every crate type can take it). The script copies the archive to `<out-dir>`
and refuses it unless its Rust objects are one `capi` module plus `compiler_builtins`, the
fat-LTO shape. `scripts/check-capi-abi.sh` links the x86_64 cdylib and staticlib it builds the
same way; they are test forms, not shipped. Linking with the NDK or Xcode and packaging are
successors.

The source authority is clean-main commit `feb039765271ca62b0c905004689b88ad92df65b`, tree
`e3e11c343c6f6a5b5b380abe03c0431c6fe81579`. Issue 116 is bound to commit/tree
`45f8f5af8bdd578b5ccb27fdb787f7a663c39818` /
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5`; Issue 121 is bound to commit/tree
`a9a975d8f679707701cc60ad102c817eb54c3082` /
`16728c5ea434dde1a75bdd4500568db8c283a2ca`. The joined header SHA-256 is
`83880c2fd7b5bc835425a5a64cae19c8a0bba17f49b4802b4033a8e7dfeac37c`, and the joined lock
SHA-256 is `c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52`.

## Frozen preflight

Tool availability was frozen before the qualification build. The Linux x86_64 host has Rust and
Cargo 1.97.1 with LLVM 22.1.6, GCC/G++ 13.3.0, GNU binutils 2.42, Python 3.12.3, Bash 5.2.21,
strace 6.8, and jq 1.7. Exact paths and installed Rust targets are in `TOOLCHAINS.tsv`.

Cross-target outcomes are candid `UNAVAILABLE`, not compile failures:

- Windows GNU has an installed Rust standard library but no MinGW C/C++ compiler, linker, or
  object inspector.
- Windows MSVC has neither its Rust target nor `cl`, MSVC `link`/`lib`, or `dumpbin`.
  `/usr/bin/link` is GNU coreutils and was explicitly rejected as an MSVC tool.
- macOS x86_64/AArch64 and iOS AArch64 device Rust targets are installed, but `xcrun`, Apple SDK
  linkers, `otool`, and `lipo` are absent.
- The iOS AArch64 simulator Rust target and Apple SDK tools are absent.
- Android AArch64 has its Rust standard library but no Android NDK Clang/linker or LLVM object
  inspectors.

No cross row was executed or relabeled after the preflight. Only the Linux runtime was run.

## Linux artifact and consumer boundary

Fresh `target/capi-qualification/v1` staging was required to be absent. One locked release Cargo
command produced the static and shared libraries there. The accepted header and both libraries
were copied into qualification-owned `installed/` staging and hashed before any consumer linked
them. Existing artifacts elsewhere under `target/` were never inputs.

The same warning-denied source compiled as strict C11 and C++17. Each language linked and ran once
against each frozen library form. The consumer verifies version/layout constants, reserved-zero
rejection, engine/session/plan construction, source generation 1 submission, generation 2 seek and
submission, two render blocks, resource rows, malformed event lane, one-short command canary,
exact command replay bytes, empty reliable egress, and both plan/session destruction orders.
The accepted Rust exported-C regressions supply the complete 11-command, six-event, transactional
replacement, retirement/reclaim, source-preserving/source-changing, failure, replay, and lifecycle
matrix without copying protocol semantics into the qualification consumer.

The compile-limit word at offset 8 is `reserved1` and must be zero (#1668).
It formerly held `maximum_automation_spans_per_block`: #1306 derives each effect's
capacity from its descriptor, and #1627–#1629 retired the protocol automation queue.
The 208-byte layout, exported symbols, feature mask and ABI version are unchanged.
A nonzero reserved word returns `MISO_ENGINE_V1_INVALID_ARGUMENT` before the document
or diagnostics is read; both output handle locations are cleared to null under
the existing failure-publication rule and diagnostics remains untouched.
Prelaunch apps and library move together: an older library refuses a zero word
with `MISO_ENGINE_V1_INVALID_ARGUMENT`; an older caller's nonzero value is refused
by this library. Rebuilding against the renamed header requires setting `reserved1`
to zero. No new feature bit is added because this removes a role.

A structural transaction keeps every unchanged source playing (#1273, slice 4 of #1269). The
replacement plan is prepared from the newest plan's state inventory and committed session model,
with no ring for a source whose declaration and ring configuration are unchanged; at its swap block
it takes that source's ring from the plan it displaces, with the queued PCM, the generation and the
read position. The source's producer moves to the replacement's source set right after the protocol
commit succeeds, which is the last fallible step, so a refused transaction moves nothing. From the
commit on, `miso_engine_v1_source_submit_planar_f32` and `miso_engine_v1_source_seek` address the
newest committed session: a host keeps feeding an unchanged source contiguously and must not reseek
it. A removed source keeps feeding the running plan until the replacement is adopted (#1325, below)
and is refused as `source.id.unknown` from then on; PCM accepted for it that render has not read
is discarded with the running plan. An added source starts at generation 1, frame 0, until the host seeks it. A source
whose declaration the transaction changed restarts the same way, in a new ring at generation 1,
frame 0; PCM accepted for it before the commit is discarded with its plan, so the host restarts
its feed. For a session whose unchanged paths hold no DSP state, the output across the swap is
bit-identical to the post-edit session compiled fresh and fed the same PCM from frame 0
(`runtime::tests`' `a_c_abi_structural_transaction_keeps_the_source_playing`). The replacement's
resource report counts the rings it carries and its carry program, as the plan will own them once
active; the double-live admission counts a carried ring once, with the plan it displaces; and
capi's epoch row charges the state inventory each epoch keeps.

A transaction that removes a track or a submix removes it in two phases (#1325, decision 15
D15-9). Phase 1, before the command returns OK and after every fallible step: the strip's fader is
muted on both channels on the running plan with the session's `mute_ms` ramp `N`, and so is each
route out of it whose tap precedes the fader; the writes are the records a value-only transaction
making those mutes would write, under a gate publication that spends no revision. Phase 2: the
replacement is published to adopt no earlier than `S = ceil_q(p + q + N + C)`, where `p` is the
render clock read after the writes, `q` the quantum and `C` the longest compensation line, in the
running plan, of a route out of a removed strip. The watermark reports the revision first at `S`,
as `EXACT`. Until the adoption a removed source's submissions and seeks go to the running plan's
producer, so the fade plays the PCM the host feeds; from the adoption on they are refused as
`source.id.unknown`. A later structural transaction before `S` writes no second mute for a strip or
route already ducked and inherits `S`. A transaction before `S` that restores a removed strip
commits and inherits `S`: its replacement is prepared against the running plan's base with the
ducks laid over it, so the strip and its pre-fader routes carry from their ducked state and ramp
back in over `N` from `S`, and a restored source declared as before carries the running plan's ring
and producer, with every frame accepted for it in phase 1 (#1325 D6).
`crates/capi/tests/strip_transitions.rs` holds the gates.

A transaction whose replacement restarts a strip it keeps is a duck-swap (#1324, decision 15
D15-7, D15-9): the strips are the replacement's `restarted_strips()`, every strip with an owner the
carry joins cannot continue (an insert added, removed or reordered, `delay_samples`, a value the
plan keeps prepared, such as the true-peak limiter's `lookahead_ms`). Phase 1 ducks them as it
ducks a removed strip, with their routes whose tap precedes the fader, in the same transaction and
under one `S`; `C` covers every route out of a ducked strip. The replacement restarts each such
strip's owners before its fader (its input section, its effects, the compensation lines on its
chain and on those routes, and their live lanes) and its fader, keeps its matrix and its
post-fader routes carrying the ducked tail, and arms the fader and those routes (#1288, #1363 D1
(c)) to fade in over `N` from the first block at or after `S + D`, `D` the strip's latency before
its fader. A strip whose insert is keyed from a restarted strip's `post_input`, `insert_send`,
`insert_return` or `pre_fader` tap, or from its `input` tap when its delay line restarts, is
restarted and duck-swapped with it, repeated until no strip joins; a track's other `input` tap, a
`post_fader` or `post_pan` tap, and a submix's `input` tap whose sidechain line keeps its length or
whose every incoming line comes from a ducked or an added strip add nothing. `C` also counts each sidechain line a
replacement can shorten: from a ducked strip's `post_fader` or `post_pan` tap into a carried strip,
and from a ducked submix's `input` tap kept by the last rule, plus the longest route line into that
submix. A planned duck-swap completes as `EXACT`, never as the transition fallback. A revert before `S` inherits `S` and fades the strip
back in from it. A pre-fader route that follows its source's automated mute is armed too, and its
fire composes with the curve it hears then, so a column the curve silences stays silent (#2011).

Submix strips are editable through the same `SESSION_TRANSACTION_APPLY` transactions as track
strips: opcodes `0203`-`0211` take a submix ID as their strip ID (#1204). Every such transaction is
structural, compiling and swapping a replacement plan, except the live values of a strip (#1554,
#1556) and of a send into a submix (#1557, section "Live sends" below).

A caller bounds submix strips through `miso_engine_v1_compile_limits.maximum_submixes` (#1206), an
in-place V1 amendment: the word is the former `reserved[0]`, at offset 176, followed by
`maximum_vcas` at 184 (#1243, below) and two reserved words at 192..207, so the struct stays 208
bytes and no symbol, size or `ABI_VERSION` changes. Zero means "use `maximum_tracks`" -- not "no
submixes" and not "unbounded" -- which is the value every caller written before the word was named
already passes. A nonzero value is the bound itself; a session over it refuses with
`host.resource.count`. The two remaining reserved words must still be zero. For a caller written
before #1206, zero newly bounds submixes by `maximum_tracks`: between K1 (#1199-#1205) and #1206
they were uncounted, so such a caller now gets that typed refusal for a session with more submixes
than `maximum_tracks`. A caller cannot probe whether a library honours the word; a library older
than #1206 refuses a nonzero word with `RESULT_INVALID_ARGUMENT`, the safe direction.

A caller bounds VCA groups through `miso_engine_v1_compile_limits.maximum_vcas` (#1243), the same
kind of in-place V1 amendment: the word is the next former reserved word, at offset 184, and
`reserved` shrinks to two words at 192..207, so the struct is still 208 bytes and no symbol, size
or `ABI_VERSION` changes. Zero means "use `maximum_tracks`" -- not "no VCAs" and not "unbounded" --
which every caller written before the word was named already passes. A nonzero value is the bound
itself, independent of `maximum_tracks` and `maximum_submixes`; a session over it refuses with
`host.resource.count`. No released library accepted a VCA before this word existed (VCA groups
arrive in the same batch, #1240), so zero changes no previously accepted session; a library older
than #1243 refuses a nonzero word with `RESULT_INVALID_ARGUMENT`.

A host starts a stem a structural transaction added in exact time through
`miso_engine_v1_source_seek_at` (#1275, slice 6 of #1269), an in-place V1 amendment: one new
exported symbol, announced by `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (32) in the capability
report, whose feature mask becomes 63; no struct, size or `ABI_VERSION` changes. It is
`miso_engine_v1_source_seek` with an anchor: the seek's source frame enters the graph in the
block that starts at sample `anchor_sample` of the plan's source-read clock (equal to the render
clock until a plan that reads its sources ahead of it, a warm successor, is adopted, which no call
publishes in this version; #1316 D1, #1355 D1, #1403), which must be a multiple of the quantum or the call returns
`RESULT_INVALID_ARGUMENT` with `source.seek.anchor_unaligned`; its
thread rule, argument checks and every other result are the plain seek's. A host that resolves
the symbol at run time checks the bit before calling it; a host that links the symbol directly
fails to load against a library older than #1275 before it can check anything (see "ABI growth"
below). After the
transaction returns, the host picks `A` a few quanta past the last rendered block, calls
`seek_at(id, g, F, A)` with `F` the frame the playing stems read at `A`, then submits generation
`g` from `F`; the stem renders silence until `A` and plays `F` from `A` on, bit-identical to the
post-edit session compiled fresh and fed that PCM from frame 0 (`runtime::tests`'
`an_added_c_abi_source_starts_at_its_anchored_render_sample`), except that the added track fades
in (#1288): its fader holds an exact `+0.0` mute until the first block at or after its first
played block plus `D`, the samples of latent nodes and compensation lines between its input and
its fader (zero for a plain track), then unmutes over the session mute ramp, bit-identical to the
fresh session with the track muted and given a live unmute at that block
(`host-core`'s `an_added_track_fades_in_from_its_first_played_block_on_every_width`). A channel
the session mutes is not armed, a host mute before the fire cancels it (an unmute is a no-op that
keeps it waiting), and a declared
discontinuity arms nothing. Strips the transaction did not add are untouched. A later structural
transaction adopted before the fade has run keeps it (#1392): while the strip's fader lane carries,
a waiting fade keeps waiting, a scheduled one keeps its render sample, and a fired one never runs
again (`host-core`'s `a_pending_fade_survives_a_second_swap_on_every_width`). A route the
transaction adds fades in the same way (#1363): a route between two strips that keep playing (a
send, or a strip's path to the output) holds an exact `+0.0` until the first block at or after
adoption plus the samples of any compensation line before its own op (none for a plain route;
its destination-edge line sits after the op, so the fade enters it and is heard as the line
delivers), and a send of an added strip from a tap before its fader fires with that fader. Both
unmute over the session mute ramp, bit-identical to the fresh session with the route muted and
given a live unmute at that block (`an_added_route_fades_in_after_its_line_fills_on_every_width`,
`an_added_strips_pre_fader_send_fades_with_it_on_every_width`). An added `post_fader` or `post_pan`
route into the output, which has no lane, binds live for that plan only so it can ramp, and loses
the fold there. A muted route, a route whose destination strip is added, a following route with
an automated source mute and a declared discontinuity arm nothing. A route the transaction
removes or re-points (another source, tap or destination) between two strips it keeps ramps out
first when it has a lane, that is when it goes into a submix or leaves before the fader (#1953a,
D15-9): phase 1 mutes it on the running plan over the session mute ramp, `C` counts its lines, and
the successor is adopted at `S`; a re-pointed route is a removal plus an addition and fades in at
its new destination from `S`, while its source strip plays on
(`capi`'s `a_removed_or_repointed_send_ramps_out_then_in_at_s`, `host-core`'s
`a_repointed_send_fades_in_at_its_new_destination_on_every_width`). A removed or re-pointed
`post_fader` or `post_pan` route into the output keeps the fold, so phase 1 cannot ramp it
(#1953b): the successor compiles a fading copy of the old route, a transition route at the
predecessor's coefficients that fades out from the adoption block over the session mute ramp
through the old route's own compensation line, and later successors keep the copy, carrying its
ramp and its line, until its fade has settled; a route re-pointed from one fades in at its new
destination as an added route does (`capi`'s `a_removed_output_route_fades_out_through_its_line`,
`host-core`'s `a_removed_or_repointed_lane_less_route_fades_out_through_its_line_on_every_width`). A later structural transaction adopted before an added route's fade has run
keeps it as #1392 keeps a strip's (#1877): the successor re-arms the route with its own target and
ramp, a waiting fade keeps waiting, a scheduled one keeps its render sample, and a fired one never
runs again (`a_pending_route_fade_survives_a_second_swap_on_every_width`). The frozen exported set is now 15
`miso_engine_v1_*` definitions (`scripts/check-capi-abi.sh`, whose self-test refuses a library
where the new symbol is only an undefined reference: its `nm` wrapper drops the symbol from a
defined-only listing and shows it as `U` otherwise, so the case also fails if the checker stops
asking for defined symbols alone).

A host learns which committed revision is audible through `miso_engine_v1_plan_watermark`
(#1314, decision 15 D15-17), an in-place V1 amendment: one new exported symbol and one new
96-byte struct, `miso_engine_v1_watermark` (`MISO_ENGINE_V1_WATERMARK_SIZE`), announced by
`MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK` in the capability report, whose feature mask grows to
include it; no other struct, size or `ABI_VERSION` changes. Like `miso_engine_v1_plan_resources`
it runs on any thread, concurrently with render, and is pure: it writes nothing through the plan
handle, its diagnostic word included. It copies `(revision, first_sample, outcome_flags)` and
saturating `exact`, `transition_fallback` and `superseded` counts: the highest committed revision
in effect together with every revision before it, the render sample of the block that reported it (never early: it is the start of the first block that begins after the commit's last write, so at most one block after the submit returns (while a replacement plan is pending: the block that adopts it); every live value of that revision takes effect together, in that block), and the OR
of `MISO_ENGINE_V1_OUTCOME_*` over the revisions the last advance covered. Submit stays
synchronous only for what can fail (validate, classify, prepare a rebuild and reserve its
credits, commit); every committed revision is pending until the watermark covers it, and a paused
host's revisions stay pending until render resumes. Each revision travels with the plan that
carries it (a revision gate in each plan, #1502), the control plane publishes each commit's
revision on the gate of the newest pending candidate, else the running plan, as the commit's last
write, and render takes one snapshot of the gate of the plan it runs per block, after the adoption
decision and before the executor renders. The
watermark is a level, not a queue: render overwrites it, never waits and drops no command, so an
acknowledged edit cannot be dropped behind it. The revision ceiling is `2^63 - 1`, the largest
revision a plan's gate can hold (#1503): a document above it is refused at compile as any invalid
document is, and a transaction on a session at the ceiling returns the revision-exhausted status
before it pushes a record, so the watermark never reads a revision the gate cannot hold. A wrong
`struct_size` or a nonzero reserved word is `RESULT_INVALID_ARGUMENT`; a copy that keeps landing
inside render's publication (bounded at 64 attempts) is `RESULT_BACKPRESSURE` with `out`
untouched, and the host retries. In this slice every revision completes as `exact`; the
`superseded` and `transition_fallback` producers are #1310, #1397 and #1358. `transition_fallback`
also reports a rebuild that carries a send following a stored mute into one that follows none
(conservative: it can be set when no send moves at the swap; #1713 removes it; #1652 U2a fold 2).
It also reports any rebuild that carries an automated following send while a live send move may
still ramp (conservative; #1718 removes it; #1654, root rulings B19-R7 and B19-R8).
The frozen exported
set is now 16 `miso_engine_v1_*` definitions;
`abi_smoke.c` checks the bit against the queried mask and queries a compiled plan with a valid
and a wrong-size struct, `header_smoke.cpp` pins the size and every field offset, and
`./target/release/audit capi` stays at 0 allocations, 0 deallocations, 0 syscalls and
`"total_violations":0` with watermark publication on (its live edits and its carrying swap
advance the watermark during the audited calls).

The engine owns no thread, so the host drives the control work between edits through
`miso_engine_v1_service(session)` (#1348, decision 15 D15-17), an in-place V1 amendment: one new
exported symbol, announced by `MISO_ENGINE_V1_FEATURE_SERVICE` (the next free bit, 128) in the
capability report, whose feature mask grows to include it; no struct, size or `ABI_VERSION`
changes. It is a session call, serialized with the others. One call runs one bounded step: it
reclaims the plans render has retired and brings the provider epochs up to render, refreshes the
provider's counters (the controller's telemetry counters today), stages the render observation,
and refreshes the counters again so a record that staging coalesced or dropped is counted in the
same call. Every other session control call (`source_submit_planar_f32`, `source_seek`,
`source_seek_at`, `session_seek` and `session_seek_at` (#1642), `session_seek_at_once` (#2205),
`submit_command`, `dequeue_event`)
that passes its argument checks runs the same
step first, in place of the parts each ran before; a call refused for its arguments (a null
`source_id`, a bad struct size, an invalid lane, an empty request) returns before the step. Before
#1348 only `submit_command` refreshed the counters and the source
calls staged no telemetry. The step reclaims at most the retirement queue's capacity of plans,
because each retired plan holds one of that many retirement credits and only a publication
reserves one; it stages at most one render observation and at most one meter batch per closed
meter window the output meter's queue holds (its depth, eight, and one more; #1583), never waits
for render and never loops on its progress, so its cost does not grow with the time since the last
call. It commits and
acknowledges nothing, adds no queue, and never takes or drops a pending candidate: a scheduled
candidate stays in its mailbox cell until render adopts it, and its provider is promoted only
after. So an ack cannot precede a drop through it. Results: `OK`; a null session
`INVALID_ARGUMENT` and another handle `WRONG_HANDLE`, as every session call; an epoch
synchronization failure `INTERNAL` with the session diagnostic `source.epoch`, the one the
source calls already set for it. The host's duty, in the header: a pending edit progresses only
while the host makes session calls, the same duty as draining events. A host that renders but
never calls keeps retired plans allocated, and its next structural edit waits for that reclaim; a
warm successor that misses its deadline falls back only at the host's next session call (#1358,
#1360); the watermark never covers an edit before it is in effect. While edits are pending, the host calls
`service` (or another session call) at least once per render-buffer period. The frozen exported
set is now 17 `miso_engine_v1_*` definitions; `abi_smoke.c` checks the bit against the queried
mask and calls `service(NULL)` for `INVALID_ARGUMENT`, `header_smoke.cpp` pins the bit inside the
mask and the signature, and the capi runtime tests cover reclaim by `service` alone, the step
inside a source submit, the counter refresh, a scheduled candidate kept across 1,000 calls, and
idle calls that change nothing.

A host learns where a seek landed through `miso_engine_v1_source_seek_report` (#1316, decision 15
D15-12), an in-place V1 amendment: one new exported symbol and one new 64-byte struct,
`miso_engine_v1_seek_report` (`MISO_ENGINE_V1_SOURCE_SEEK_REPORT_SIZE`; not D6's
`miso_engine_v1_source_seek_report`, which is the function's name, and C gives a typedef and a
function one name space), announced by `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_REPORT` (bit 9, 512;
#1323 took bit 8 first) in the capability report, whose feature mask grows to include it; no other
struct, size or `ABI_VERSION` changes. It is a session call, serialized with the others, and runs
the service step first. It copies the source ring's `(generation, first_sample, source_frame)` and
its cumulative underrun and held frames: the generation in effect, the source-read sample (the
render clock in this version) of the first block it played in, and the source frame that block
started at, a late anchored seek's frame including its lateness. Render publishes the record from
the source consumer at the end of a block's acquire step, only when a new generation took effect or
a cumulative counter moved, through the engine's one single-writer seqlock
(`crates/engine/src/realtime/seqlock.rs`), so it is a level, not a queue: no ack can precede a drop.
Before the ring's first block it reads generation 0. The record belongs to the ring (one shared
record charged as `seek_report_bytes` in the source's overhead), so a plan swap that carries the
source keeps it and an added or redeclared source starts at 0. A host that plain-seeks one stem and
reads `(g, s, f)` then starts another with `seek_at(Y, g', f + (A - s), A)` and the two align
exactly. Results: `OK`; `INVALID_ARGUMENT` for a wrong `struct_size`, a nonzero reserved word or a
malformed source ID, and with `source.id.unknown` for an undeclared source; `BACKPRESSURE` with
`source.report.busy` when every bounded read (64 attempts) landed inside render's publication, `out`
untouched. The frozen exported set is now 19 `miso_engine_v1_*` definitions (with #1323's);
`abi_smoke.c` checks the bit against the queried mask and reads a compiled session's source
(generation 0), an unknown source and a wrong size, `header_smoke.cpp` pins the size, every field
offset and the signature, and the capi runtime tests cover a plain seek, a late anchored seek, a
scrub-then-align across a transaction, a carried and an added ring, and the two counters.

A host moves the playhead with one call, `miso_engine_v1_session_seek(session, g, T)` and its
anchored form `miso_engine_v1_session_seek_at(session, g, T, A)` (#1642), an in-place V1
amendment: two new exported symbols, announced by `MISO_ENGINE_V1_FEATURE_SESSION_SEEK` (bit 10,
1024) in the capability report, whose feature mask grows to 2047; no struct, size or `ABI_VERSION`
changes. The timeline is the session's playhead: it starts at 0, advances one quantum per render
and stored automation follows it (#1058). The call moves it and every source of the newest
committed session (a pending candidate's set while a swap waits, so a source the candidate adds is
sought too) in one step, all or nothing: the timeline to `T`, each source to `min(T, its frames)`;
the host then submits generation `g` from each source's frame. `g` must be above every generation
the session has used (one above the largest the host has used is always valid). It checks every
consumer's slot before it pushes to any, pushes every command unpublished and then publishes `g`
once to the session's published-generation word, which render loads once per block entry and
at or below which alone a consumer takes a command; so every consumer takes the seek at the same block
entry or none does, and a held-back command keeps its slot, so no ack precedes a drop. Results:
`OK`; `INVALID_ARGUMENT` for `g = 0` (`source.generation.zero`), a `g` not above the session's
published generation or a consumer's (`source.generation.stale`), an unaligned anchor
(`source.seek.anchor_unaligned`) or a `T` over the timeline limit
(`source.seek.frame_out_of_range`); `BACKPRESSURE` with `source.seek.backpressure` while any slot
holds a seek; the source seek's mapping otherwise (`INVALID_ARGUMENT` with `source.ring.vacated`,
`INTERNAL` with `source.epoch` or `source.internal`). The anchored form takes the anchor of
`miso_engine_v1_source_seek_at`. A per-source seek leaves the timeline where it is.

A plain session seek is live (#2205): every source plays its old generation on until each source
that waits has its runway queued, or the deadline, then all land in one block.
`miso_engine_v1_session_seek_at_once(session, g, T)` (#2205) is the same seek for a host whose
transport is stopped, an in-place V1 amendment: one new exported symbol, announced by
`MISO_ENGINE_V1_FEATURE_SESSION_SEEK_AT_ONCE` (bit 12, 4096) in the capability report, whose
feature mask grows to 8191; no struct, size or `ABI_VERSION` changes. Same arguments, checks,
results and declaration as the plain form, through `control_plane::SessionState::session_seek_at_once`,
but every source and the timeline land at the first block rendered after the call, without waiting
for a runway, so the stop position never plays after the host restarts. Old-generation PCM still
queued in a ring is dropped at that block; a host that filled its rings before the stop and cannot
drain them while stopped underruns at restart until it submits generation `g`, unless it prepares
the plan first (#2213, below). `capi` test
`session_seek::an_at_once_session_seek_plays_no_old_block_after_a_stop` holds it.

`miso_engine_v1_plan_prepare_seek(plan)` (#2213) is that refill step, an in-place V1 amendment:
one new exported symbol, announced by `MISO_ENGINE_V1_FEATURE_PLAN_PREPARE_SEEK` (bit 13, 8192) in
the capability report, whose feature mask grows to 16383; the frozen exported set grows from 22 to
23 symbols; no struct, size or `ABI_VERSION` changes (`Plan` is opaque, so its new atomic word is
not a layout change). It takes the plan, so it sits on the render side by construction: the host
calls it on the render thread between blocks, or on any thread while its render callback is
stopped, never concurrently with `render_f32_planar`. Every source of the active plan applies its
admitted plain seek under the consumer's hold clock (`source::PcmSourceConsumer` through
`GraphPreparedSourceSetDriver::prepare_admitted_seeks`) and discards and recycles the old
generation's queued blocks, so the producer accepts the new generation before the restart; the
timeline lands at the next block through the at-once word. An anchored seek stays held. It returns
`RESULT_OK` with nothing to prepare too. Misuse is refused, not raced: both render-side calls claim
a `render_busy` word with one compare-exchange on entry and release it with one `Release` store on
every exit; a call that finds it claimed touches no state and returns `RESULT_RENDER_REJECTED` with
`render.busy`. The claim never waits, spins or makes a syscall. `capi` tests
`session_seek::a_stopped_host_refills_a_full_ring_before_restart` (the issue's gate) and
`session_seek::a_render_side_call_during_render_is_refused_not_raced` hold it.

A plain session seek is a declared discontinuity; the engine declares it in the same call, and a
refused seek declares nothing (root ruling B16-R9). `control_plane::SessionState::session_seek`,
the one function behind both hosts, runs the service step, checks every consumer
(`SourceControlSet::check_session_seek`), declares the discontinuity through the core path of
`miso_engine_v1_declare_discontinuity` (#1323, all or nothing), then pushes and publishes. A refused
check declares nothing and pushes nothing; a refused declaration pushes nothing and returns its own
result and diagnostic, as `miso_engine_v1_declare_discontinuity` returns them (`COMPILE_REJECTED`,
`BACKPRESSURE` or `INTERNAL`). A declaration that supersedes a waiting replacement moves every
producer the check ran on into the successor, so the push reaches the producers it checked; a
refusal of that push is an engine invariant (`INTERNAL`, `source.internal`). An anchored session
seek keeps continuity: it declares nothing, because it is the loop and exact-start form, and a
declaration publishes its successor at the next block, before the anchor, which would put the
effects at rest and drop the compensation lines while the old pass still plays. A host that wants
the floors reset declares at a stop. A seek of every source made with per-source seeks is still the
host's own call, so `miso_engine_v1_declare_discontinuity` keeps its duty for that case and for a
stop. No host or SDK calls it before a session seek (this retires B15-R7's order). The frozen
exported set is now 21 `miso_engine_v1_*` definitions; `abi_smoke.c` checks the bit against the
queried mask and calls both symbols with a null session for `INVALID_ARGUMENT`, `header_smoke.cpp`
pins the bit inside the mask and both signatures, and `audit capi` runs a session seek (which
declares the discontinuity itself; on the audit session it publishes nothing) outside its render
scope with 0 violations. The gates: `capi` `runtime::tests::session_seek_declares` (a refused plain
seek publishes nothing; an accepted one publishes exactly one successor and applies, also across a
waiting replacement; an anchored one publishes none) and the parity row
`control_parity::a_plain_session_seek_declares_and_an_anchored_one_does_not_alike_on_both_hosts`.

A host replaces the whole session with a document while it plays through
`miso_engine_v1_submit_document_command(session, request, request_bytes, document,
document_bytes, response)` (#1386), an in-place V1 amendment: one new exported symbol, announced by
`MISO_ENGINE_V1_FEATURE_SESSION_REPLACE` (bit 11, 2048) in the capability report, whose feature
mask grows to 4095; no struct, size or `ABI_VERSION` changes. It is `miso_engine_v1_submit_command`'s
contract plus a document borrowed for the call only, under `compile_session`'s pointer-plus-length
rule (a null `document` with a nonzero `document_bytes` is `INVALID_ARGUMENT`); both exports run one
body into `control_plane::SessionState::command_with_document`, the one core path both hosts call,
and `submit_command` passes no document. The `SESSION_REPLACE` frame (`0013`) names the document by
its length and SHA-256; the control plane checks both, bounds it by the compile limits'
`maximum_document_bytes` (never by the control frame), parses and compiles it as a boot would, and
commits its diff as the one transaction it is (`docs/CONTROL_PROTOCOL_SEMANTICS.md`, "Session
replace"). A document refusal is a non-OK reply frame with `RESULT_OK`. The library keeps nothing of
the document. The frozen exported set is now 22 `miso_engine_v1_*` definitions; `abi_smoke.c`
checks the bit against the queried mask and calls the symbol with a null session for
`INVALID_ARGUMENT`, `header_smoke.cpp` pins the bit inside the mask and the signature, and
`audit protocol` encodes one `SESSION_REPLACE` frame caller-buffer. The gates: `capi`
`tests/session_replace.rs` (the export renders the blocks, reply and event of the equal
`SESSION_TRANSACTION_APPLY` on a twin, for a rebuild, a live change and an identical document; a
null document is refused and a retry through `submit_command` answers the cached reply; a document
more than four times the 16 KiB frame commits, and a large live replace leaves exactly the live
bytes the equal transaction leaves on a twin, so 0 bytes of the document stay) and the parity row
`control_parity::a_session_replace_without_its_document_is_refused_alike_on_both_hosts`.

The seek contract in the header (#1317, decision 15 D15-12) states, from the code, the source-read
clock and its origin (0 at compile, one quantum per render call, continued by a swapped-in plan),
the plain and the anchored seek, how to choose the anchor, held behaviour (queued PCM of the
playing generation plays, then silence counted as held frames, never as underrun frames),
replacement of a held seek, the one-slot refusal (`BACKPRESSURE`, `source.seek.backpressure`),
displacement by a structural transaction, far-past anchors and the region end, the source-rate
assumption (`source.rate.mismatch`), and how `miso_engine_v1_source_seek_report` reports a seek's
landing. It promises nothing the code does not do; a source-read clock that leads the render clock
is a later change and amends the "equal to the render clock in this version" sentence.

ABI growth (#1317, decision 15 D15-3). `MISO_ENGINE_V1_ABI_VERSION` changes only on a break. Each
added exported symbol, and each former reserved struct word given a name, gets its own
`MISO_ENGINE_V1_FEATURE_*` bit, the next free one when it merges. A new value of an existing enum or
result code, and a field added in place to a V1 protocol message, takes no bit, so a host tolerates
an enum value it does not know and treats a result code it does not know as a failure. A feature
added as a protocol message is announced by its protocol capability flag in the `CAPABILITIES`
reply, not by a feature bit; the control protocol registry, not this rule, governs protocol flags. A
host tests each bit it needs with `&`, tolerates bits it does not know and never compares
`feature_mask` with `==`. A feature bit protects only a host that resolves the symbol at run time: a
host that links a symbol directly fails to load against a library that lacks it, before it can test
anything. The library's own Rust tests compare the reported mask with the library's `FEATURE_MASK`
(`version_and_capabilities_are_exact` and the constant pins in `abi.rs`); the C smoke test does not
compare the reported mask. `abi_smoke.c` tests one `&` per `MISO_ENGINE_V1_FEATURE_*` symbol (it
fails when the library stops reporting a defined bit and passes when it reports an extra unknown
one) and keeps its compile-time pins of the header's constants.

The first C11-static launch found one qualification-fixture error: it attempted generation-1 seek
before the initial generation-1 submission and exited 13. No product byte or staged library was
changed or rebuilt. The new consumer was corrected to submit generation 1 first, then seek and
submit generation 2; all four consumer rows passed against the same once-built libraries. This is
recorded as one consumer-fixture correction in `QUALIFICATION.tsv`.

GNU `nm` found exactly the 14 frozen `miso_engine_v1_*` definitions (15 since #1275) in both
library forms. The object parser classifies undefined references separately; a synthetic mutation
replacing a definition with an identically named undefined reference is rejected.

## Runner and realtime evidence

The frozen runner package test command was invoked exactly once. Its 18 tests passed, including
the single test that executes all four RIFF launch rates and representative RF64, compares every
8,192-byte output SHA-256 to the independent manifest, and covers atomic success/failure cleanup
and no-clobber behavior. The runner, its fixtures, and its exclusive-output-directory contract were
not modified, retried, or described as a new Issue-116 seal.

The exported C render audit completed 100,000 calls with stable caller storage and zero allocation,
deallocation, lock, feature-detection, log, file/network I/O, syscall, unwind, or render errors.
Since #1273 the audit applies one structural transaction (a muted track and its route) after its
first call, outside the render scopes, so its second audited call is a swap block that carries the
source's ring; the tool fails unless that transaction's plan is the only one the render thread
swapped in (so every periodic live edit was applied live, #1258 D3) and its swap carried. A
separate functional one-million-block render/swap audit observed two accepted swaps, one retirement
deferral, zero forbidden-operation counters, and zero syscalls between the explicit realtime trace
markers. Neither audit selected a benchmark mode or recorded durations.

The exact matrix was `fixtures/capi-qualification/v1/MATRIX.tsv`. `ARTIFACTS.tsv`, `SYMBOLS.tsv`,
`AUDITS.jsonl`, `QUALIFICATION.tsv`, `CONSUMER_RESULTS.tsv`, `RAW_EVIDENCE.tsv`, `GATES.tsv`, and
`TOOLCHAINS.tsv` contained its independent evidence, and `EVIDENCE.sha256` bound those files. The
semantic checker independently pins every artifact size/hash, symbol set, audit field, result
counter, consumer exit/binary/library binding, raw-log hash, and strict gate; updating the checksum
manifest cannot bless correlated fabricated data. Preserved-stage mode additionally checks the raw
manifest, logs, audit JSON, binaries, libraries, symbols, and armed syscall trace in place. The
checker mutations cover each evidence family as well as authority drift, target omissions,
undefined-reference false positives, fabricated tool availability, timing surfaces, stale staging,
and generated source-tree artifacts.

Final prohibited counters are: benchmark 0, timing 0, playback 0, listening 0, browser 0, and
device 0.

## Issue 369 control-provider refresh

IO-4 replaces the C ABI's conformance-only `MockProvider` with the opt-in
`host_core::SessionControlProvider`. Since #1552 the provider serves the control list
(`CONTROL_LIST_GET`, `CONTROL_STATE_GET`, capability flag bit 7; decision 17, C4) from the core, and since #1573 the
response preview (`RESPONSE_GET`, capability flag bit 14; decision 17, C13) from the core's `prepare_response_query`: its
catalog is `host_core::ControlCatalog`, built from the compiled session's `controls::list` with an
owner header before each owner's rows and a section header before each section's, each entry at its
handle (from 1, in list order) and its committed value beside it. A request with `headers_only` (#2206) rides the same `CONTROL_LIST_GET` frame, with no new symbol, and its pages hold the owner and section headers alone. The catalog holds owners, sections and the distinct control descriptors once each in three shared tables (descriptors compared by bits) and one 16-byte record per entry, and a page builds its wire entries from them (#1599). The list therefore covers every
control of the session (strips, builtins, console slots, inserts, routes, VCAs and the session's
own rows), and can never disagree with what `controls` validates. Every commit publishes the
catalog of the committed model. A rebuild builds the whole catalog in preparation. A live commit
stages its catalog before its first write (`ControlCatalog::stage_live`, fallible, #1600): it reads
again only the rows of the owners whose values changed, from `controls::owner_rows_with`, and
every derived row from one effective pass, and it stages the whole catalog instead when a setting
(`control_smoothing`) changed, the owners differ (a VCA added or removed) or a changed owner lists
other addresses or descriptors. The publish after the commit writes the staged values or swaps the
staged catalog, with no allocation, so a derived row follows its inputs and the list equals what a
full build gives. `structure_epoch` starts at 1 and
advances by one on every commit, live or rebuild, after which the address sequence, a header or any
control row's descriptor (every field except `value`) differs from the previous catalog (a live
`matrix_mode` switch flips `active` and so advances it; a fader move does not); a
request that names another epoch is `REVISION_CONFLICT` with `control.list.stale`. The page
`revision` is the committed session revision. A page holds at most `limit` rows and fewer when its
response would exceed the endpoint's response bound (the page then is not the end of the list).
`AUTOMATION_ENQUEUE` (`0x0006`) is retired (#1627): the C ABI refuses a frame under that ID at decode (`INVALID_ARGUMENT`, nothing written and no state moved), and `CAPABILITIES_GET` lists no command `0x0006`, clears flag bit 5 and has no capability fields 9, 12, 19 and 20 (23 fields). Sample-timed changes are stored automation edits. Current/effective sample reads the active
plan's existing release/acquire next-sample publication, while transport remains endpoint-local.
Protocol telemetry counters and the existing bounded CAPI render-diagnostic slots feed the
provider's counter and diagnostic pages. The nine provider-owned counters (the two telemetry
counters `TELEMETRY_COALESCED` 5 and `TELEMETRY_DROPPED` 6, `LIVE_VALUES_SUPERSEDED` 16, #1312 D8,
`TRANSITION_REPREPARE_REFUSALS` 17, #1351 D6, `STRIPS_RESTARTED` 18, #1768,
`LIVE_BLOCK_ERRORS` 19, #1521, `SOURCE_UNDERRUN_BLOCKS` 20, #1537, `SOURCE_FRAMES_DISCARDED`
21, #1699, and `METER_WINDOWS_LOST` 22, #1583) retain an independent nine-slot minimum even when
the frame-derived telemetry configuration capacity is smaller. Canceled automation (counter 12) retired with the automation
queue (#1629).

**Meter batches (#1583, #1587a, #1587b, #1690b, decision 17 C1/C9/C13).** `TELEMETRY_CONFIGURE` takes a meter
table of stable targets: entry `i` (handle `i + 1`) is the output, a strip by its ID or the master
(#1690b), with a metric (peak or gain reduction; the output takes peak only and the master gain
reduction only, any other pair `INVALID_FIELD` at the codec). A handle is the table's, so a structural commit never moves it.
The C ABI session owns one meter delivery from boot over the session's output meter and the
running plan's strip meters, one post-matrix meter per strip whose queue is as deep as the output
meter's (eight), and its echo carries the meter window in blocks (`meter_window_blocks`,
host-core's default of about 32 ms: 12 blocks at 48 kHz and 128 frames). A nonempty table turns
the meter level on from the next block and an empty one off: the control plane writes a requested
gate word, and the plan's relay applies it to the output meter's gate on the render thread before
a block, on the strip meters' window grid at or after that block (a request the control thread
computed from an older render clock, or on a grid render has since restarted, is moved onto it),
so the gate moves only between blocks and the render call stays allocation-, lock- and
syscall-free. The configure is all or nothing: an unknown strip is `NOT_FOUND`
`meter.strip.unknown` (`meters[i]`); a period that is not a positive multiple of the window is
`INVALID_FIELD` `meter.period`; and a table of more than 128 entries is `INVALID_FIELD`
`meter.records` at its 129th entry, so a batch never needs more than its 256 records. Each `METER_BATCH` is
`meter_period_blocks / meter_window_blocks` consecutive windows: `first_sample` is the first
window's start, `observed_sample` the last one's end, `window_sequence` counts batches from 1
after each accepted configure, and each peak entry, in table order, gives a left and a right
record: the output's planes' maxima over the batch, or its strip's post-matrix maxima. A record is
`VALID`, and `CLIPPED` above 1.0; an entry whose strip the plan that rendered the batch does not
have has flags 0 and value 0.0, and is `VALID` again under its handle once a commit brings the
strip back. A strip's gain-reduction entry gives one `AGGREGATE` record in non-negative decibels
(#1587b): the strip's reading from the batch's last window that has one, `VALID` when any window
has one, else flags 0 and value 0.0. A strip's reading is the deepest reduction among its
observation taps that an `Observe` edit armed (#1584), each from its newest window that opened at
or after the tap's arm sample, which render publishes in the tap's slot (#1585); a tap never armed,
or disarmed, gives none, and a window from before a re-arm is none. Entries resolve to strips at each configure and each adoption, against the plan whose
frames the delivery folds, so a pending rebuild never relabels a record. Readings continue across
a swap that carries the strip meters; a swap that carries none restarts the grid at its adoption
block, where render restarts the gate and the control thread's adoption asks for the same value.
A tap's reading and arm continue across a swap that carries its effect: the swap block copies its
record's observation entries into the plan's attachment, and the promotion pairs the readers
from them (#1674's observation half, #1587b), never from a list made at preparation. A swap that
does not carry the effect restarts its taps: an armed tap is prepared armed (#1584), and the
strip's gain reduction is absent until the tap's first window in the new plan closes.
The master's gain-reduction entry (#1690b) is the same record for the strip the core's `master`
row names (`SessionState::master_strip`), resolved by ID in the one meter staging both hosts share:
at each adopted plan, and again after a live or monitor commit that moves or clears the row, so
it follows the row with no reconfigure (a rebuild's successor resolves it at its own first
frame, and the plan still rendering keeps its master until then, as the browser does). With no
master, or a master without a reading, the record has flags 0 and value 0.0; the entry stays in
the table, and a configure with no master is accepted. The browser's `masterGrDb` reads the same
strip's reading through `MeterFrame::master_gain_reduction` (#1690a), and the master parity row
holds the two equal bit for bit, and absent together.
The control thread adopts each swap when it reclaims the displaced plan, whose clock gives the
adoption sample. The lossy meter lane holds one batch whole (256 slots; the counter-snapshot lane keeps
two). `METER_WINDOWS_LOST` counts every closed window that reached no batch: the windows render
dropped at a full meter queue (eight), the windows of a partial batch a gap discarded or a later
configure replaced, and the windows of a batch the lane refused to stage; the sequence never skips.
A coalesced batch is counted by `TELEMETRY_COALESCED`, as any lossy record. The per-block render
peak (#1570a) is gone (#1583 D6). The strip-ID text of the stored tables is charged to
`capi_retained_bytes` at the 128 entries an accepted table can hold, 127 bytes each, twice.

**Served counters and the counter snapshot (#1351).** The provider serves exactly those nine
counters, each at 0 from construction, so `COUNTERS_GET` with `all` returns all nine and a read of
any of them is never `NOT_FOUND`. `STRIPS_RESTARTED` adds, for each replacement plan render
adopted, the strips that plan started at rest, each once; the control plane adds it when it
promotes the adopted plan, so a withdrawn candidate adds nothing (#1768). `LIVE_BLOCK_ERRORS`
counts the blocks render refused after the block took its live snapshot; the service step copies
the render-side mirror in, and it stays 0 in a correct build (#1521). `SOURCE_UNDERRUN_BLOCKS`
counts the rendered blocks in which a source played zeros because the host had not delivered PCM
it owed (graph-wide; a held block and a block past the end of a region are not counted); the
service step copies the render-side mirror in, on both hosts (#1537). `SOURCE_FRAMES_DISCARDED`
counts the accepted source frames render never read because the caller's transaction removed or
redeclared the source, or a failed carry could not move; the service step adds them when it
reclaims the displaced plan (#1699). `TELEMETRY_CONFIGURE` naming any other counter ID (for example
`MALFORMED_FRAMES`) is `NOT_FOUND`, and the stored configuration does not change. `TRANSITION_REPREPARE_REFUSALS`
is protocol counter 17; nothing writes a nonzero value until #1360. The periodic `COUNTER_SNAPSHOT`
event (`0x8021`) reports, for each configured ID in order, the value after the service step's first
refresh. A counter that render changed since the previous call therefore reaches the snapshot that
the next call stages, whichever control call that is. A snapshot is staged once render has completed at least `counter_period_blocks` blocks
since the last one (a configuration counts its period from the call that accepted it), with the render
sample at that point; blocks rendered with no control call between them yield one snapshot at the
next call, and skipped periods are not replayed. The lane is lossy: the lossy queue may keep only part
of a snapshot.
Candidate catalogs allocate before structural commit and are included in double-live resource
admission. Host-core's default feature graph remains protocol-free; only capi enables the optional
`control-provider` edge.

`resource_lifecycle` checks these charges against the allocator (#1060). Its counting allocator
observes a C ABI compile and a replay of its host-core half owner by owner, and nothing is taken
from the accounting it checks. What capi allocates itself, plus the observed effect live-control
producers (#1263), strip live-control producers (#1256), source producers, parameter catalog and
plan state inventory (#1273), must equal `capi_retained_bytes` to the byte.
The launch native-effect registry is built once per process (#1469) and its bytes are
process-level: no plan or engine is charged for them. A plan is charged only what it retains, so a
factory `Arc` the registry owns is charged to no plan (the plan's clone allocates nothing); on the
nine-track EQ reference session this removes the EQ factory's 16-byte allocation from
`capi_retained_bytes`.
The session store must fit its compiled-model estimate, and the prepared plan its engine rows (a
bound; see the test). The canonical JSON is charged once, with the compiled model in the graph cap:
capi's epoch row no longer charges it a second time. The double-live admission is derived from the two live reports
and the owning crates' resource reports rather than from a hand-maintained layout mirror.
The C response vectors now pin session-derived control-list and control-state pages and registered
telemetry counter rows. `MockProvider` and `MockProviderConfig` are absent from a normal protocol library
build and available only to unit tests or consumers explicitly selecting `protocol/test-support`.
The exact AudioWorklet rebuild remains protocol-free but changes crate identity because host-core's
declared feature surface changed. The subsequent #371 marker-only integration was rebuilt
and reproducibly qualified as `a89c9606bfa72d69ced42b606cc4b7000d1b53f2b419b12ec63649a385b3eaf1`.
The RT-1 (#399) artifact from source candidate
`e46bc0d1a7917de8c65204cdee931877aea671d8` has SHA-256
`60c23ee23e7f16c1f71c503baa07a462a8ce94c5287bec4580060e27a4651503`; its reproducibility and browser evidence are recorded in #399.
The RT-2 (#419/#422) artifact from source candidate
`0a0e39e42e4ae2585d5f5ee507a4cb9aaf7b741a` has SHA-256
`518b5aa864c0a825cd324112b24270a7e0714fc63db6bd1029779f21066ea9de`.
The independent rebuild, static/resource checks and three-browser matrix passed;
retained workspace and descriptive measurement delivery are recorded in #419.
The RT-3 (#420) artifact from source candidate
`51e2aed211b30523076e0e8dd07973b13b57dc11` has SHA-256
`24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe`.
Independent builds, static/resource/mutation checks and all three browser engines passed;
workspace evidence belongs to #420; its uninvoked descriptive measurement is tracked by #436.
The RT-4 public full-chain (#429) artifact from source candidate
`e4bcaa2feae13c9f016bb7b2e1eaff8bd7314547` has SHA-256
`10b0581f72d921b520e4066b82dc32cb7bea90b757c20ccca3dfc52cf7b9e098`.
Independent builds, static/resource/mutation checks and all three browser engines passed;
workspace and supported-Wasm evidence were retained in `artifacts/issue429-qualification`,
which #1030 removed in `df8cebb3`; git keeps it
(`git show df8cebb3^:artifacts/issue429-qualification/<file>`).
Live integration and descriptive full-chain measurement remain in #430 and #431.
The current RT-14 lease cleanup (#435) artifact from source candidate
`69fd0bfb0504075db4d302df08ff480faab4102e` has SHA-256
`766848a4688b2ec34c96e81c243286216a7d7e647b6b42f842c0f85a654fc326`.
Independent builds, static/resource/hermetic checks and all three browser engines passed;
workspace and supported-Wasm evidence were retained in `artifacts/issue435-qualification`,
which #1030 removed in `df8cebb3`; git keeps it
(`git show df8cebb3^:artifacts/issue435-qualification/<file>`).
This is an intentional public Rust lease API retirement; wire/C ABI identities are unchanged.

Refresh gates: `cargo test -p capi`; `cargo test -p capi --test resource_lifecycle`;
`scripts/check-capi-abi.sh`; `scripts/check-abi-layout-v1.py`; and `cargo test --workspace` against
both the issue worktree and `origin/main`. Exact outcomes and the worktree comparison are attached
to issue #369's implementation pull request.

## Live fader/matrix qualification candidate (#430/#459)

The fresh SIMD AudioWorklet digest build from immutable source candidate
`7951736605fa64870bc1d91342d00d5fdb6417c5` produced SHA-256
`a08a868cf1b62bb466a8fa5b826b214fa708265669fc730398706c869c9e43bd`. Independent rebuild, static/resource gates, hermetic worklet tests, and
Chromium/Firefox/WebKit qualification with matrix checks passed. The resource
expectations include one additional eight-byte graph owner on Wasm; PCM digests are unchanged. The initial
builder invocation refused a missing output directory before compilation (exit 2);
the corrected invocation with an existing empty directory completed successfully.
Historical artifact and qualification records above retain their original identities.

## Live fader and matrix lanes on every plan (#1256)

Every C ABI plan, at compile and at every structural replacement, carries two live lanes per
strip, tracks and submixes alike: a fader/mute lane and a matrix/pan lane (#1053 D4). #1256
attached each as a 16-record ring, charged in `builtin_retained_payload_bytes`; since #1312 each
is a set of latest-target cells (below), and the ring figures in this section are #1256's. capi
keeps the strips' producers with the plan's provider epoch, and that producer table (one producer
per strip plus its strip ID bytes) is charged in `capi_retained_bytes`. On the nine-track EQ
reference session the
two rows move 17,451 -> 28,521 and 256,812 -> 258,135 bytes. A caller whose
`maximum_builtin_retained_bytes` and `maximum_capi_retained_bytes` were exact before this change
must raise both. #1256 attached no input, effect or route lane, so rendering, `latency_samples`,
`tail_kind` and `tail_samples` did not change (since #1379 the fader and matrix lanes do move
`tail_samples`: a live gain states its domain maximum in the tail composition, section "Tail on
every strip"); the effect lanes follow (#1263), and the input lane
(#1556, section "Live strip input" below) and the route lane (#1557, section "Live sends" below)
are attached later. #1257
delivers fader, mute and pan edits through them (next section).

## Live strip input: trim, polarity and the input filters (#1556)

The strip input section's `trim_db`, `polarity_invert`, `hpf_hz` and `lpf_hz` are live rows of the
control list (decision 17, S5). `miso_engine_v1_submit_command` commits a change of them as a live
revision, on tracks and submixes, per lane and on both, through `SET_CONTROL`; no plan is prepared and the plan epoch does not move. `delay_samples` stays rebuild-only (it
needs new memory). The core chooses the lanes from the rows (`controls::live_lanes`), so **every C
ABI plan carries an input lane per strip**: the trim, polarity and filter cells of
`TrackControlProducer::input`, charged in `builtin_retained_payload_bytes` (31,424 -> 36,153 on the
nine-track EQ reference session) and, for the producers' table, in `capi_retained_bytes`.

- **Records.** Per strip, in write order: `TrimDb` (one per lane whose gain changes, or one `Both`
  when both lanes change to the same `trim_db`; `0.0` to `-0.0` is no record), `PolarityInvert`
  (the same rule), then the filter targets that host-core's `InputFilterPreparer` designs on the
  control thread from the strip's pre-commit values: a one-sided edit per lane, one atomic pair
  edit when both of a lane's cutoffs change (so a pair that two one-sided edits would refuse, an
  HPF passing the old LPF, is live), one `Both` edit when both lanes change the same way. A cutoff
  the filter domain refuses (above the rate-keyed maximum, non-finite, an HPF at or above the LPF,
  `-0.0`) is the same refusal a rebuild gives. The designed targets are validated before the first
  cell write, and a write cannot fail, so no acked edit is dropped.
- **Ramps.** A trim ramps over `fader_ms`, a polarity flip over twice `mute_ms` (the coefficient
  passes through zero on the trim ramp, so a flip is a linear ramp from `+g` to `-g`), both
  replaceable by the edit's own `ramp_samples`; a filter target is applied over a fixed 64
  coefficient updates, and its edit takes no length.
- **Rendering, latency and tail.** An input lane adds no latency. A strip with the lane reports the
  rate's live input bound (finite for every history of live trim and filter targets) in place of
  its prepared design's bound, so a plan with no infinite-tail effect reports `TAIL_FINITE`
  (checked on the one-, ten- and nine-track parity sessions at the four launch rates). After the
  ramp, a live edit renders the bits of a plan prepared from the committed session (the filters
  from a common silence, `-0.0` samples included); gated at 44.1 and 96 kHz, with zero render
  allocations and frees.
- **Tail on every strip.** Every C ABI plan carries an input lane per strip, so every strip reports
  the rate's live input bound even with both filters off (before #1556 such a strip reported its
  prepared design's bound, zero without a filter). The bound is a function of the rate alone
  (`builtins::input_section_live_bound_table`); before #1379 `tail_samples` of a plan with no
  infinite-tail effect was that figure, measured so on the bare parity sessions at one and ten
  tracks:

  | Rate | `tail_samples` (`T`) | Seconds | Exact rest, peak up to +24 dBFS | Exact rest, any sanitized input |
  | --- | --- | --- | --- | --- |
  | 44.1 kHz | 704,010 | 15.97 | 1,067,207 | 2,384,997 |
  | 48 kHz | 699,952 | 14.58 | 1,061,497 | 2,372,008 |
  | 88.2 kHz | 704,018 | 7.98 | 1,071,057 | 2,388,800 |
  | 96 kHz | 699,960 | 7.29 | 1,065,688 | 2,376,147 |

  **Since #1379 (decision 15 D15-4(b) as amended by #1379 H10)** `tail_samples` is the graph's
  `output_tail_every_peak` (`graph-compiler/src/pdc.rs`, `compose`): the strips' figures compose
  through every gain between the nodes, each live gain at its domain maximum (every C ABI plan's
  fader and matrix lanes are live, +24 dB and a row sum of 2; a live send +24 dB with ones), so a
  node's decay part grows by one `D` per 20 dB of gain on its paths; and below the graph's flush
  floor `P*_graph` the value is the exact-rest extent (the exact-rest figures above, added along
  the path), from which the output is exactly `+-0.0`. A live graph's floor can sit near or above
  full scale, so the reported value is usually the rest branch: C6's one-track 48 kHz session
  (`crates/host-core/tests/tail_composition.rs`, the 1 kHz input LPF, every gain at 0 dB) reports
  1,061,497, the 48 kHz `peak_plus_24_dbfs` rest; a plan without a control queue states its fixed
  gains and reports less. The graph tail cap is not limited by host preparation (`u64::MAX`), so
  no session is refused for it. **Host impact:** a host that waits
  `tail_samples` after its last source ends before it stops rendering, to let the output ring
  out, now waits that long on every plan, many seconds, where a plan whose filters were
  all off waited none. The bound is an upper bound: `tail_samples` is the time after which the
  output is below -144 dB of the input peak (exactly zero below the floor) for every history of
  live values, not a prediction for the session at
  hand, so a host that wants to stop sooner ends on its own silence test. The reason is browser
  parity (decision 17, C6): the core attaches the live lanes to every plan on both hosts, so the
  browser and the C ABI report the same tail for the same session, and the live bound is the one
  that holds for every history of live trim and filter targets. No render path reads these
  values: silence skipping inside the effects is driven by each lane's own count of zero input
  samples (`lane::silence_frames`, 3,764 to 8,192 samples), not by the tail or the exact-rest
  bounds, and `PreparedBuiltinsSession::input_bounds` (the exact-rest figures above) has no reader
  outside tests, so a larger bound costs no render work. The tail feeds only the PDC composition
  and the reported `tail_samples`.

## Live sends: gain, mute and matrix (#1557)

A route into a submix, and since #1391 a route into the output whose source tap precedes its
strip's fader (`input`, `post_input`, `insert_send`, `insert_return`, `pre_fader`), has `gain_db`,
`mute` and the four `matrix_*` coefficients as live rows of the control list (decision 17, S6;
`controls::route_rows_live` is the one rule). `miso_engine_v1_submit_command` commits a change of
them as a live revision through `SET_CONTROL`; no plan is prepared and the plan epoch does not
move. A `post_fader` or `post_pan` route into the output is unchanged: its gain, mute and matrix
stay rebuilds for decision 14's fold reason. `follows_mute` never applies to a route into the
output (#1218), at any tap. **A live route carries a lane and does not fold**: the core chooses
the lanes from the rows (`controls::live_lanes`), so a C ABI plan with a live route attaches one
latest-target cell per such route (`graph::route_control_resources`, charged in
`capi_retained_bytes` and admitted against the graph cap) and the route's constants are no longer
folded into the epilogue. A plan with no live route attaches none and renders as before; a
`post_fader` or `post_pan` route into the output keeps its fold. With no record, a live route
renders the bits of its prepared constant.

- **Record.** One per changed route, in canonical route-ID order: the four coefficients
  `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)` returns, and
  whether the route is silenced. These are the constants preparation folds into a route, so a
  settled send renders the bits of a plan prepared from the committed session. A record is emitted
  only when the folded target or the gate changes: a gain or matrix edit on a muted send gives
  none, and the later unmute carries the new values. A gain or matrix outside the route domain, or
  an edit ramp above 4,194,304 samples, is the rebuild's refusal.
- **Ramps.** If the gate changes, the record ramps over `mute_ms` (or the edit's own length for the
  route's mute). Otherwise it ramps over the longest of the changed rows' lengths: `gain_db` over
  `fader_ms`, the matrix over `pan_ms`, each replaceable by the edit's own `ramp_samples` (an
  explicit `0` is a step). A transaction that changes the gain with length 240 and the matrix with
  length 480 gives one record of 480. A gate that its source strip's mute moved, and not an edit of
  the route, takes the source's `Mute` length instead, and that length governs the whole record,
  also when the gain or the matrix moved with it: see the ramps of "Live VCAs and follows" (#1558)
  below.
- **Resources.** The route cells, lanes and render-side owners are charged in
  `capi_retained_bytes` and in the replacement peak as `route_control_resources.total_bytes`, an
  upper bound; the producer table and route IDs are observed exactly by `resource_lifecycle`.
- **Rendering.** The route lane adds no latency or tail, and no render block allocates or frees
  (gated by `send_values_are_live_and_render_the_prepared_plan_bits`).


## Live VCAs and follows (#1558)

A VCA's offset and mute, a VCA's members, a VCA added or removed, a mute that a send follows, and a
send's `follows_mute` are value-only (decision 17, S7a). `miso_engine_v1_submit_command` commits a
change of them as a live revision through `SET_CONTROL` or the structural edits (`UpsertVca`,
`RemoveVca`, ...); no plan is prepared and the plan epoch does not move. A VCA has
no render cell: a change reaches render as its members' effective fader and mute records, and a
follow as the route's record, because the classifier diffs the effective values of the committed
and the prospective model (`controls::effective`, the function preparation reads), never the raw
fader, mute or VCA fields. A strip muted by its VCA whose own mute then toggles moves nothing, and a
VCA move that leaves every member clamped at +24 dB writes nothing.

- **Ramps.** A level record of a lane the strip's own value moved takes the strip's own edit and
  row (`fader_db`, `mute`); else the first reaching VCA, in ascending VCA-ID order, whose offset or
  mute moved on the lane, with that VCA's edit length; else the `VcaOffset` or `VcaMute` row's
  session length (a membership change, a VCA added or removed). A send whose gate moved takes the
  send's own `mute` or `follows_mute` edit when one of them moved, else the length of its source
  strip's `Mute` record, so a strip and the sends that follow it ramp together.
- **Row set.** A VCA added or removed adds or removes its four rows (`offset_db` and `mute`, left
  and right), so the commit advances the control list's `structure_epoch`; a member change, an
  offset or a mute keeps it.
- **Counts.** A live edit over `maximum_vcas` (or any other count cap) is refused like a compile:
  `COMPILE_REJECTED` with `host.resource.count`, and the model, the revision and every cell are
  unchanged.
- **Successors.** A rebuild that also changes a VCA or a followed mute carries each strip's fader
  and mute stage and retargets it to its effective value (#1277 D5). The sends it prepares start at
  their gate, so a following send can step at the swap while its source strip ramps; carrying a
  send's ramp across a plan swap is #1284.
- **Rendering.** After the ramp, a live edit renders the bits of a plan prepared from the
  committed session; gated at four rates in `live_tests` and under 20 races of
  `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`, which add VCA
  offsets, mutes, membership, VCAs added and removed and `follows_mute` toggles to its live edits,
  with zero render allocations and frees.

## Value-only track edits on the running plan (#1257)

`miso_engine_v1_submit_command` classifies each `SESSION_TRANSACTION_APPLY` by the committed
model's delta (`host_core::classify_live_delta`, #1053 D1), never by its opcodes.

- **Live.** The committed models before and after differ only in track `fader` (`left_db`,
  `right_db`, `left_mute`, `right_mute`) and `pan`/`matrix` values. The engine pushes the records
  that change a rendered value to the newest plan -- the pending replacement if there is one, else
  the current plan -- and commits. No plan is prepared; source rings, effect state and the render
  position continue, and the host keeps submitting with no seek. Fader, mute, pan and matrix
  changes ramp over the session's `control_smoothing` (#1054): its `fader_ms`, `mute_ms` and
  `pan_ms`, or the researched default table (20, 10 and 20 ms) when the session says `default`,
  rounded at the session's sample rate; an explicit 0 is a step. A pan or matrix record whose
  committed model carries its own non-zero `smoothing_samples` uses that window instead. A live
  edit may carry its own length (#1394): the optional `ramp_samples` of `SET_CONTROL` on a
  fader, mute, pan or matrix row overrides the session's length for the records that edit writes -- the
  fader and mute records, or the matrix record, which it also puts ahead of a model window --
  and an explicit 0 steps that edit only. An edit with no length uses the session's. A length
  above one second at the session rate refuses the whole transaction with
  `session.edit.ramp_out_of_range` before anything commits. The length is not model state: it is
  never in canonical JSON, and the next transaction without one ramps over the session's again.
  A transaction that also rebuilds the plan keeps these lengths: each strip the new plan carries
  retargets its fader, mute and matrix over the same ones (#1277 D5, #1394 D6).
  Editing `control_smoothing` itself is a model-only delta (#1365): see the next item.
- **Model-only (#1260, #1365).** A delta that changes only fields no prepared plan reads -- the
  session ID, the render profile's `id`, the output profile's `id`, the stored `automation`
  entries of rows that do not render yet, and `control_smoothing` (`SET_CONTROL` of the session rows `fader_ms`, `mute_ms` and `pan_ms`;
  only a document replace writes the default table back, #1561) -- is live with no records: it commits through the same admission and
  protocol predicates, emits the same response and `SESSION_COMMITTED`, and the running plan, its
  source rings and its effect state continue untouched. It may ride with fader, mute and pan
  edits, and the records of the same transaction ramp over the new `control_smoothing` lengths; a
  ramp already in flight keeps the length it started with. Every other profile field stays
  structural.
- **Stored fader automation (#1648).** An edit of a stored automation entry on a row that renders
  (today a strip's `fader_db`, `controls::automation_row_renders`) -- an entry added, removed,
  retargeted or given other segments -- rebuilds the plan (`LiveRebuild::Automation`, reported
  `rebuild`), and the successor carries every owner and seeds its jump-length cell from its own
  model. An entry's hand offset (#1650) is not part of that comparison. A static fader value of a
  lane the session automates is that lane's fallback: it gives no `FaderDb` record, whether an own
  edit or a VCA move reaches it, and its mute records are unchanged. Every other row stays
  model-only until its rendering slice adds it.
- **A hand move on an automated fader (#1650b).** On a lane a rendered `fader_db` entry drives,
  `SET_CONTROL fader_db` writes the entry's hand offset, not the static fader: it is live (in place
  when it is the transaction's only kind of edit), writes the lane's offsets cell with
  `S = offset + the reaching VCA offsets` over the `Fader` row's length, and the list reports the
  offset as the row's value with `automated` (field 35). An offset-only `UpsertAutomation` is the
  same live write. A change of an automated lane's static fader is structure (a rebuild).
- **Live effect parameters (#1264, #1265).** A track's console-slot entry or insert whose
  `params` change only in live parameters, parametric EQ values included, is live too; see
  "Value-only effect parameter edits" and "Value-only parametric EQ edits" below. So is a
  bypass change on one; see "Value-only effect bypass edits" below.
- **Rebuild.** Everything else replaces the plan exactly as before: a fader dB outside
  `[-144, 24]` or a pan/matrix the setter refuses (reported as `COMPILE_REJECTED` with the
  preparation diagnostic a rebuild gives), and every other field.
- **Timing (#1053 D2, #1312).** A committed fader, mute or pan/matrix value reaches render no
  later than the first block whose render call begins after the submit returns or, while a
  replacement plan is pending, at that plan's adoption, which the watermark reports. Every such
  value of one transaction takes effect in the same block, and the watermark names that block's
  first sample (D15-2). It is heard up to `latency_samples` later. This holds for the effect
  parameter, bypass and EQ-target values too (#1345): they are cells read under the block's one
  snapshot beside the fader values, so a transaction's values never land in different blocks.
- **Latest-target cells, no backpressure (#1312).** Each strip's fader, mute and matrix/pan values
  are held in latest-target cells: four on the fader lane (fader left and right, mute left and
  right) and one on the matrix lane. A write cannot fail and never waits, so a host that is not
  rendering can send any number of fader, mute or pan edits and none returns `BACKPRESSURE`. A
  later value for a cell committed before the block that reads it replaces the earlier one; the
  block applies the newest value whose revision its snapshot covers. `LIVE_VALUES_SUPERSEDED`
  (protocol counter 16, reported by `COUNTERS_GET` after any control call) counts, per cell, each
  write to a plan that render adopts which the block that covers it does not apply because a later
  write to the same cell exists. A `Both` record writes two cells and counts in each: forty `Both`
  fader edits before one block add 78. A write to a candidate that is withdrawn, and a value on a
  lane a swap restarts, are not counted. The effect lanes are cells too (#1345): an effect's bypass,
  its parameters (two cells for a per-lane parameter, one for a shared one) and an EQ's prepared
  targets (one cell per target slot and channel) are written under the commit's stamp, so no live
  edit is refused for room and none returns `BACKPRESSURE` with `control.live.backpressure`,
  which no longer exists. A full plan exchange is `INTERNAL` since #1398 D3 (the exchange holds
  every candidate a session can meet), never `BACKPRESSURE`.
- **No ack before a drop (#1053 D6, #1312 D7, D11).** Classification, the live admission
  (#1053 D8), the producer lookup and preflight of every effect record and the protocol's commit
  predicate all run before the commit's stamp, and the stamp is the last fallible step; nothing
  changes until the last of them passes. The cell writes and the commit then cannot
  fail, and render applies every value it reads, because the classifier refuses every value the
  setters would refuse. A refused transaction writes no cell and uses no revision.
- **Same events.** A live edit emits one `SESSION_COMMITTED` and nothing else (`AUTOMATION_CANCELED`
  is retired, #1628), exactly as a replacement does. The reliable event lane holds two events, so a
  host drains it after each edit either way.
- **Edit path in the response (#1313).** The `TransactionApplied` response carries a required
  field `2:edit-path U8`: `live` 1 (records on the running plan's live lanes, no plan prepared),
  `model_only` 2 (committed; the classifier's delta holds no record of any kind) and `rebuild` 3
  (a successor plan was prepared and published, whatever live records the transaction also
  held) and `monitor` 4 (a transaction of solo writes only, #1559, below). The path is known at submit and written by the commit into the response frame and its
  replay entry, so an exact replay of the request returns the same bytes. Zero and every other
  value are refused on decode. This reverses #1053 Q3 ("not now"). A fallback after submit and a
  supersession are reported by the watermark's outcome flags (#1314). After a live edit or a
  replacement alike a host goes on submitting every source the transaction left unchanged, with
  no seek; after a replacement it acts only on a source the transaction added, changed or removed
  (see the #1273 paragraph under "Linux artifact and consumer boundary", and
  `miso_engine_v1_source_seek_at` (#1275) to start an added one in time).
  `miso_engine_v1_plan_resources` describes whichever plan is active, which a replacement may
  leave unchanged. No new symbol, opcode, result code or event is added; the field travels inside
  the response bytes.
- **Solo (#1559, decision 17 C8).** Solo is a `SET_CONTROL` on a track's own row 2 (`solo`, a
  toggle, `both`); a submix has no solo row (`control.address.unknown`, a submix is solo-safe). It
  is host-local: it lives in the session state's monitor overlay (empty at boot), is never saved,
  is never in a `SESSION_SNAPSHOT_GET` document and changes no structure epoch; the list's and the
  state's `solo` value is the overlay's. A transaction of solo writes only keeps the revision and
  the model, replies `monitor` (4) at the committed revision, emits no `SESSION_COMMITTED`, and
  is recorded in the replay table like any response. The overlay composes through
  `controls::effective` for the live path and for preparation alike: the other tracks' mutes and
  the sends that follow them are records of the running plan (or, while a replacement is pending,
  of the newest plan's cells), and a rebuild prepares its successor under the overlay and carries
  the difference as retargets. A solo's records ramp over `LiveRampRow::Solo` (`mute_ms`), the
  write's `ramp_samples` when it gives one; a record that a model edit of the same transaction
  also produced takes the model edit's length. The overlay is written only after every fallible
  step of the transaction passed. The records ride the plan's revision gate as a monitor publication under the committed
  revision (#1603): the gate orders every publication, and the applied watermark reports the
  revision a publication carries, so a solo spends no revision number, committed revisions stay
  dense (the next model commit after any number of solos is the committed revision plus one), and
  the watermark never reads a number above the committed revision, and never an early one. No symbol, opcode, result code or event is added.
- **Live admission.** The prospective compiled model lives beside the current one until the
  commit, so a live edit is admitted only if both plans' graph bytes plus both compiled models fit
  `maximum_graph_session_plus_plan_bytes`; the newest plan's `capi_retained_bytes`, plus the
  current epoch's rows while a candidate is pending, plus the prepared-protocol rows, fit
  `maximum_capi_retained_bytes` (the last term counts a catalog the live arm never builds: a
  documented overcount); and the largest allocation fits `maximum_named_allocation_bytes`.
- **Resource movement.** Each provider epoch now keeps its own capi resource figures (32 bytes)
  for the live admission, so every session's `capi_retained_bytes` grows by 96 bytes: the inline
  current epoch in the session plus its two reserved epoch slots. On the nine-track EQ reference
  session the row moves 258,135 -> 258,231 bytes. A caller whose `maximum_capi_retained_bytes` was
  exact must raise it.

## Live effect lanes on every plan (#1263)

Every C ABI plan, at compile and at every structural replacement, also carries one live control
lane per prepared effect instance, console slots and inserts alike, on tracks and submixes. Each
lane holds the effect's latest-target cells (#1345): one for the bypass, two per per-lane and one
per shared `Block`-rate automatable parameter, and for a parametric EQ one per target slot and
channel in place of the parameters; the render-side staging window is raised to hold every
parameter cell. capi keeps the producers with the plan's provider epoch;
each EQ's producer owns its prepared-target owner. #1264 writes live effect parameter edits to
their cells, and #1265 publishes live EQ targets through the owners (below). The input lane is
#1556's and the route lane is #1557's (both below).

- **Rendering, latency and tail do not change.** A lane is seeded from the session's bypass, so
  the plan renders bit-identically to a lanes-free plan, and `latency_samples`, `tail_kind` and
  `tail_samples` equal those of the same plan without effect lanes. Both are checked on the
  nine-track EQ reference session and on the one- and ten-track parity sessions (the ten-track
  one has a bypassed limiter insert) at the four launch rates.
- **Charges.** The cells, the staging windows and the banked live-control owner the lanes make the
  plan build are graph rows, charged in `graph_session_plus_plan_bytes`,
  `graph_incremental_plan_bytes` and `graph_metadata_bytes`. The producer table, its strip and
  effect IDs and the EQ owners are reported once, in `capi_retained_bytes`. Both enter the
  replacement and live admissions through those rows. The producer payload is admitted twice, a
  conservative double admission: at preparation host-core also admits it against
  `maximum_graph_session_plus_plan_bytes`, so the initial compile needs the graph row plus the
  compiled model plus the payload `capi_retained_bytes` charges for the producers (15,361 bytes on
  the reference session below). A replacement's two-plan peak and the live admission dominate it.
- **Resource movement on the nine-track EQ reference session** at the reference `limits()`
  (a 128-frame quantum; x86-64, eight lanes). These
  are that session's figures, not general ones:

  | row | before | after |
  |---|---|---|
  | `graph_session_plus_plan_bytes`, `graph_incremental_plan_bytes` | 253,934 | 382,918 |
  | `graph_metadata_bytes` | 56,137 | 185,121 |
  | `capi_retained_bytes` | 258,231 | 273,640 |
  | `largest_named_allocation_bytes` | 90,720 | 90,720 |

- **The historical #1263 formula.** Before cell-sized windows (#1611/#1621) and
  descriptor-derived preparation capacity (#1306), the graph rows grew by a per-member
  term plus a per-bank term:

  - each parametric EQ member 2,104 bytes, each other effect member 1,208 (no 896-byte target
    staging);
  - at a 128-frame quantum, each eight-lane effect bank 8,944 + 360 * S bytes, plus 64 * L for a
    prepared latency of L samples, and each four-lane bank 4,560 + 200 * S. S is the caller's
    `maximum_automation_spans_per_block`, because each lane's packed window equals the automation
    capacity, so the bank term scales with it: at S = 4,096 a four-lane bank is about 824 KB.

  On the reference session that is nine EQ members and two eight-lane banks at S = 128 and L = 0:
  9 * 2,104 + 2 * 55,024 = +128,984. `capi_retained_bytes` grows by the producer table and its
  payload (15,361 bytes there: nine 104-byte producers, their IDs and the nine EQ owners) and 16
  bytes in each of the three provider-epoch slots. Every other row is unchanged. A session without
  effects moves only `capi_retained_bytes`, by 48 bytes. A caller whose
  `maximum_graph_session_plus_plan_bytes` or `maximum_capi_retained_bytes` was exact before this
  change must raise it; a structural replacement charges both plans' graph rows, so its peak grows
  by twice the graph move.

- **Resource movement of the cells (#1345).** The lane's 16-record ring and target staging became
  the latest-target cells and their dirty words; the producer's payload now holds the writer's
  cell arrays. The figures above are #1263's; on the nine-track EQ reference session at the same
  `limits()` (x86-64, eight lanes) the rows move, measured with
  `cargo test --locked -p capi --test resource_lifecycle reference_session_retained -- --nocapture`
  on the parent (`710f77df7`) and on this change:

  | row | parent | cells |
  |---|---|---|
  | `graph_session_plus_plan_bytes` | 393,718 | 405,798 |
  | `graph_incremental_plan_bytes` | 393,694 | 405,774 |
  | `graph_metadata_bytes` | 185,521 | 197,601 |
  | `capi_retained_bytes` | 277,481 | 281,513 |
  | `largest_named_allocation_bytes` | 90,720 | 90,720 |

  Every other reference row is unchanged. The graph estimate charges what a lane retains
  (`EffectControlLane::retained_resources`) and, in a bank, one `(lane index, queue)` table entry
  for each lane that has an observation queue; since #1584 the C ABI binds one for every effect
  that declares a tap, and the reference session has none, so its rows charge none. `tests/bypass_resources.rs` holds the lane term and the queue table to the audited
  allocator, exactly. The member and bank terms above no longer
  hold: a member's term is its cell arrays, which grow with the effect's live parameter cells and,
  for the parametric EQ, its target slots. Since #1611/#1621 the staging window follows
  the live parameter cells. #1306 also makes each prepared effect capacity
  `max(1, descriptor live parameter cells)`, independent of caller S; the independent
  `EffectCompileCaps` value is a ceiling and rejects an insufficient value. Host
  preparation uses `u32::MAX` for that ceiling and charges the resulting graph bytes. A caller whose `maximum_graph_session_plus_plan_bytes` or
  `maximum_capi_retained_bytes` was exact before this change must raise it.

## Value-only effect parameter edits on the running plan (#1264)

A transaction that changes, adds or removes `params` of a track's native effect -- a console
slot's entry or an insert -- is live when every value it changes is live. The classifier resolves
the pre- and post-commit `params` through `effect_compiler::resolve_initial_values`, the one
function preparation uses (unit, channel policy, defaults, `-0.0` to `+0.0`, domain, unknown IDs),
and compares the resolved values bit by bit.

- **Live.** A changed value of a parameter whose descriptor is automatable with
  `automation_rate` `Block`. The engine writes one value per changed `(parameter, lane)` -- the
  one cell of a shared parameter, a `Left` and a `Right` cell apart for a per-lane one -- to that
  instance's lane in the newest plan, under the commit's stamp, and the effect ramps it with the
  descriptor's own smoothing. A removed entry returns to the default. Rewriting a value in another
  representation (a `both` value as equal `left` and `right` values) writes nothing. It may ride
  with fader, mute, pan and model-only edits in one transaction.
- **Prepared, so a rebuild.** A changed value of a parameter that is not automatable or whose
  `automation_rate` is `None` (for example the gate/expander's attack, hold and release, and a
  parametric EQ band's `enabled` and `kind`). The parametric EQ's live parameters ride prepared
  targets; see "Value-only parametric EQ edits" below.
- **Live link mode (#1371).** A changed link mode of an insert, or of a console entry's override,
  of an effect on `controls::LIVE_LINK_EFFECTS` (the compressor, gate/expander, transient
  shaper, true-peak limiter, de-esser and dynamic EQ): one write to the instance's link cell,
  which the stage that owns the processor hands to `retarget_link` at the block boundary, so the
  lane's detector glides to the new mode over the `SET_CONTROL`'s own ramp, else the session's
  `fader_ms` (`LiveRamps::link_samples`). A replacement plan that lands during the glide
  continues it to its target. A console slot's declared mode stays a structural edit (decision
  17 C5).
- **Rebuild for everything else about an effect**: identity, quality, the link mode of any other
  effect (the multiband compressor's until #1367), a prepared bypass
  (#1266, below), sidechain, the insert order and the console slot set, any effect on a submix
  strip, and `params` that preparation refuses (reported as `COMPILE_REJECTED` with the
  preparation diagnostic).
- **No ack before a drop.** Every record passes its producer's preflight before the first cell
  write.
- **The readback follows.** After the commit, the control state (`CONTROL_STATE_GET`) and the
  control list (`CONTROL_LIST_GET`) report each live value, exactly as a rebuild's catalog would
  (#1552, #1600: the catalog's values are refreshed from the committed model, reading again only the
  changed owners' rows and the derived rows). The catalog's handles and
  `structure_epoch` do not change.
- **Effects keep their state.** A live parameter edit is not bit-identical to a fresh plan of the
  edited session, because the effect's history differs; it is bit-identical to the browser's live
  lane given the same records at the same block, at the four launch rates on one and ten tracks.
- **No resource movement.** No row moves: the records ride the lanes #1263 attached.

## Value-only parametric EQ edits on the running plan (#1265)

A parametric EQ instance (a console slot's entry or an insert of a track) takes its live values as
prepared targets through its owner, never as bare parameter records.

- **Live.** Each band's frequency, gain, Q and shelf slope, and the HPF's and the LPF's `enabled`,
  frequency and Q (every EQ parameter whose `automation_rate` is `Block`).
- **Prepared, so a rebuild.** A band's `enabled` and `kind` (decision 14, F2), and any EQ value
  preparation refuses (`COMPILE_REJECTED`, as for any effect).
- **Targets are designed on the control thread.** The classifier seeds host-core's
  `EqTargetPreparer` with the instance's pre-commit values, resolved by the one function
  preparation uses, passes the changed `(parameter, lane)` values as edits, and designs the
  targets at the session's sample rate. Render only applies them, from the EQ's prepared-target
  cells.
- **All or nothing.** For each EQ instance, in the newest plan: `begin_owner` at the owner's
  committed revision, `edit_owner` for each edit, and `preflight_candidate_targets` (revision,
  and validation), before the preflights of #1264 and the protocol predicate. A refusal
  there is `INTERNAL`. A refusal at any of these steps
  discards every owner begun and changes nothing. Then the records are written, each owner's targets
  published, the protocol committed and each owner committed; none of them can fail.
- **The readback follows** every changed EQ value, as for #1264.
- **Effects keep their state.** A live EQ edit renders bit-identically to the browser's lane given
  targets that `EqTargetPreparer` designs from the owner's own committed values, published through
  the owner at the same block, on the nine-track EQ fixture at the four launch rates.
- **No resource movement.** The targets ride the EQ lanes and owners #1263 attached.

## Value-only effect bypass edits on the running plan (#1266)

A transaction that switches the bypass of a track's console-slot entry or insert is live through
the rack's latency-preserving bypass shunt (decision 14).

- **Live.** The new bypass is written to the instance's bypass cell in the newest plan. It rides
  with the instance's parameter values or, for a parametric EQ, its targets, in the one block that
  applies the transaction; the block applies the bypass first.
  The shunt selects the latency-matched dry signal for the lane while the wet path keeps running,
  so the effect's state continues either way and prepared latency does not change. The switch is a
  linear crossfade over the session's `mute_ms` (#1341): the record carries `mute_ms` in samples
  (the one live row with no per-edit length, so an edit's own `ramp_samples` does not apply) and
  the lane mixes `dry + (wet - dry) * m(k)` on the indexed ramp law, `m` from the mix the lane is at
  (`1` wet, `0` dry) to its target, over the frames from the block that applies the record. The
  dry side stays latency-matched throughout. From the first frame after the ramp the output is
  the settled select, bit for bit; a session whose `mute_ms` is 0 steps at the record's block. A
  record for the target the lane is already heading for changes nothing, and a reversal continues
  from the current mix. It may ride with every other live edit.
  The crossfade runs in the per-node render path and in an effect bank (a console slot, or a
  cohort of inserts) alike: a bank mixes each lane along its own ramp and renders exactly the bits
  the per-node path renders for that lane (#1341a2), so where the session places the effect never
  moves a rendered bit. The browser edits through the same core, so it crossfades alike
  (`hosts/host-web/tests/control_parity.rs`); #1393 is superseded.
- **Every effect's bypass is lane state.** No session effect keeps a prepared bypass: the delay's
  is live since #1339 and the multiband compressor's since #1340, whose bank recovers per lane. A
  submix strip's bypass stays a rebuild, as every submix value does.
- **No ack before a drop.** The bypass record passes its producer's preflight before the first
  cell write, and nothing after it can fail.
- **Equal to a rebuild, on the gate fixture.** The wet path keeps running, so the switched
  instance's own state is the same either way. An effect downstream of it keeps the live history,
  as after any live edit, so the from-sample-0 oracle holds only while that downstream memory fits
  the window. On the gate fixture (a compressor insert, then a soft-clip console slot), from
  E + ceil((latency + `mute_ms` in samples) / quantum) + 1 on, a live bypass or lift renders
  bit-identically to a plan
  compiled from the committed snapshot and fed the same source from sample 0, on one and ten
  tracks at the four launch rates. A second stateful effect after the switched one (a compressor
  after a compressor) does not converge within that window; its difference decays.
- **No resource movement.** The record rides the lanes #1263 attached; no readback row reads it.

## Strip meters on every plan (#1581)

Every C ABI plan carries the engine's meters: one post-matrix sample-peak meter stream per strip,
the tracks then the submixes, at the default window (`default_meter_window_blocks`, about 32 ms:
12 blocks at 48 kHz and 128 frames), and, from #1581's second slice, one output meter. The
caller's `maximum_meter_streams`, `maximum_meter_items` and `maximum_meter_bytes` bound them like
any other meter, so they must admit one meter stream per strip plus the output. A cap below that
refuses the compile with `MISO_ENGINE_V1_COMPILE_REJECTED` and
`builtin.resource.limit $.meter_requests`. On the nine-strip reference session the meters add
10,458 bytes to `builtin_meter_payload_bytes` (it was 0); `builtin_retained_payload_bytes` is
46,917 bytes on x86-64. No symbol, result code or event is added; nothing reads the meters through
the C ABI until #1587.

## Input design bounds (#1602)

The engine keeps no cache of input-section design bounds. Every C ABI strip carries the live input
lane, which reports the rate's live bound and designs none, so a compile and a rebuild bound no
input design and walk no frame (`input_design_bounds`'s `a_c_abi_plan_bounds_no_input_design`).
Before #1602 the engine also owned a control-thread cache of designs (#1457, #1471) and each
provider epoch a seed of the designs its plan used; both were always empty on the C ABI after #1556,
and #1602 removed them with their ceilings (4,300,720 bytes of cache and 947,248 bytes of open
journal outside every compile limit) and their epoch row. The engine's memory outside the compile
limits is its handle alone; the C header states it in its "Engine memory" paragraph. A host that
needs fast bounded designs again re-earns a cache in a new issue.

## Plan capacities for a superseding candidate (#1398)

A structural transaction is never refused because render has swapped plans but not yet published
the new epoch: the refusal that waited for the epoch atomic is gone, and a transaction submitted in
that window is admitted against the plan that renders.

- **Capacities.** The plan-resource report table holds 4 rows (a lagging epoch's, the running
  plan's, a withdrawn candidate's and its successor's), the pending and retired provider lists 2
  each, and the plan exchange 3 retirement credits (a displaced plan not yet reclaimed, a withdrawn
  candidate's credit and its successor's reservation). A full exchange, report table or pending
  list while a candidate is reserved is now `INTERNAL`, never `BACKPRESSURE`; an incompatible
  envelope, an exhausted epoch and a destroyed plan stay `BACKPRESSURE`.
- **Admission.** A structural candidate is admitted against every plan the session holds
  unreclaimed (the running plan and every pending or withdrawn candidate) plus itself, and every
  compiled model it holds plus its own, in every cap the replacement peak checks; the live
  admission sums the same held plans. Since supersession (#1310) a transaction submitted while a
  candidate is pending commits, and that admission holds three plans. A source ring is counted
  once, in the plan it was carried from: the candidate's carried rings leave its row, and so do
  those of every held candidate after the running plan (batch 15a review fold).
- **Sizing rule.** The header states it beside `maximum_graph_session_plus_plan_bytes`: each
  resource cap must be at least three times one plan's row, and the graph cap must also cover three
  compiled models. The repository's reference limits (`tools/audit/src/capi.rs`,
  `crates/capi/tests/plan_swap_race.rs`, `crates/capi/tests/resource_lifecycle.rs`) admit three
  plans of the nine-track EQ reference session with room to spare (graph 316,350 bytes against
  100,000,000; capi 167,754 against 10,000,000), so none was raised. The rule is measured from the
  row of a running successor plan, which carries its sources and owns their rings. The browser's
  caps are its one memory budget, which charges the same tables through the same control-plane code; its
  replacement-peak term must cover the three-plan peak when a warm candidate is admitted (#1403).
- **Resource movement.** The larger tables add fixed bytes to every session's
  `capi_retained_bytes` (+1,904 bytes, #1398 D1's tables). On the nine-track EQ reference session
  the row measures 165,842 bytes at the batch 15a head, 165,922 after its review fold, 168,810
  after fold 2, 167,514 after #1698 and 167,754 after #1285's floors (+240: the `input_floors` and
  `raised_arrivals` tables; x86-64; the ceiling in `crates/capi/tests/resource_lifecycle.rs` is
  175,296). A caller whose `maximum_capi_retained_bytes` was exact must raise it. No symbol, result code or event is added.

## Supersession of an unadopted candidate (#1310)

- **Behavior.** A structural transaction submitted while an earlier replacement is unadopted
  commits (`RESULT_OK`); `RESULT_BACKPRESSURE` for a pending candidate is gone. The session takes
  the candidate back, prepares one successor of the running plan that carries both transactions
  (from the candidate's carried base and the running plan's kept model, monitor overlay and edit
  ramps), moves the rings of the sources the candidate added into it, and publishes it. A refusal
  after the withdrawal republishes the candidate unchanged. The watermark completes the displaced
  revisions with `MISO_ENGINE_V1_OUTCOME_SUPERSEDED` when the successor is adopted. A transaction
  that finds the candidate already taken by render builds on it, as before; the session first
  treats every pending candidate as adopted, so it never holds two taken candidates beside a third.
- **Refusal at the three-plan peak.** A successor that does not fit beside the running plan, the
  withdrawn candidate and their models is refused. The refusal is `RESULT_BACKPRESSURE` when the
  successor fit against the newest held plan alone and the committed model, so a retry after a
  render normally commits; it is `RESULT_COMPILE_REJECTED` with the cap's diagnostic when it does
  not fit even then. The split is approximate: a successor prepared on the adopted candidate
  differs from one prepared across the withdrawn candidate (about +64 graph and +158 capi bytes
  on gate 4's session), and the second check counts a donated ring twice, so a retry may still be
  refused within that small band. A host that is not rendering keeps getting
  `RESULT_BACKPRESSURE` until it renders.
- **A fed ring the candidate created is backpressure (batch 15a folds 2 and 3, root rulings of
  2026-10-16).** When the candidate created a source's ring (it added the source or redeclared
  it), the host has fed that ring (an accepted chunk, seek or end of region), and the new
  transaction does not keep the source as the candidate declares it (it declares it as the running
  plan does, redeclares it another way, or removes it), the successor would not take that ring and
  the accepted PCM would drop with the candidate. The core (`host_core::drops_acked_fresh_source`,
  checked right after the donation check, before the commit) refuses the transaction with
  `RESULT_BACKPRESSURE`: nothing is committed, the candidate is republished, no event is queued.
  After render adopts the candidate, the retry commits on it; a host that does not render keeps
  getting `RESULT_BACKPRESSURE` until it renders. A source the candidate carries unchanged from the
  running plan is not refused. Both hosts answer alike
  (`control_parity::a_supersession_that_would_drop_acked_pcm_is_backpressure_alike_on_both_
  hosts`, cases reverted, redeclared a third way and removed). #1698 keeps the reverted case
  refused too: the host restarted its feed of the source against the candidate, so the successor
  starts it fresh and cannot play the candidate's chunks in the running plan's source.
- **Displaced plans are charged until reclaimed (batch 15a fold 2).** When a transaction finds the
  candidate taken by render and builds on it, the plan render displaced may not yet be reclaimed
  (a render preempted between its claim and its retirement). The session keeps that plan's
  resource row and charges it in both admissions until a later call reclaims it; a refusal it
  causes is `RESULT_BACKPRESSURE`, which the next service call clears. The session parks one such
  epoch per retirement credit (three), and every call reclaims what render handed back before it
  promotes from the published epoch, so two taken candidates and a published adoption never leave
  the session answering `RESULT_INTERNAL`.
- **Admission.** While a successor of the running plan is pending, the running plan's model is
  kept, so the replacement and the live admissions count it: three plans and three models when a
  candidate is superseded, three models when a live edit lands while one waits.
- **Resource movement.** Each candidate epoch kept its carried base (#1344), charged in its capi
  epoch row (`carried_base_retained_bytes`), until #1698 removed it (below); the boot plan had
  none. The provider epoch itself grew
  (its kept base and carried base, and the batch 15a review fold's carried source bytes), and the
  pending and retired provider lists charge that size, so the reference session's boot
  `capi_retained_bytes` moves: 165,842 bytes at the batch 15a head, 165,922 after its review fold,
  168,810 after fold 2 (x86-64; a third retired slot and a 248-byte parked row in each of the
  five slots, +2,888), within the 175,296 ceiling. No symbol, result code or event is added.
- **An owner the successor reverts carries from the running plan (#1698).** The successor is
  prepared from the running plan's own inventory, with only the source rows the candidate carried
  from it, so a strip, effect or other owner it keeps unchanged from the running plan carries as
  a direct swap would carry it, also when the candidate restarted or removed it. A source the
  candidate added, redeclared or removed, and the successor does not keep as the candidate
  declares it (that one moves by the donation), starts fresh in the successor (generation 1,
  frame 0, as the host contract for a declaration change against the newest committed session
  says). The candidate epoch no longer keeps a carried base, and `carried_base_retained_bytes` is gone: the
  reference session's `capi_retained_bytes` moves 168,810 -> 167,514 (x86-64, -1,296: 216 bytes
  in each of the six provider epochs). #1285's floors (`input_floors`, `raised_arrivals`) then add
  240 bytes: 167,754 (ceiling unchanged).

## Latency floors across a structural transaction (#1285)

- **Behavior.** No node's latency, the output's included, drops across a structural transaction.
  The core (`host_core`, for the C ABI and the browser alike) compiles a successor with every node's
  input arrival in the plan it displaces as a lower bound (an input floor), and PDC compensates
  every incoming edge of the node to it. Removing a latent effect, or the track that holds it,
  therefore moves no other path in time: every surviving compensation line keeps its length and
  carries (#1283), at the output and inside a submix. `latency_samples` reports the floored output
  arrival, the one the plan renders. A path whose own upstream latency dropped gets a new
  compensation line, which starts at rest; its strip lost a latent effect, so it is in the restart
  set, and it starts at rest with no duck until #1324 adds the duck-swap. An added latency still
  grows the node (#1287 owns growth).
- **How long.** The inventory records the floored arrivals, so a chain of successors keeps the
  first plan's timing. A successor prepared across a withdrawn candidate takes the running plan's
  floors (#1698's base is the running plan's inventory). A fresh `miso_engine_v1_compile_session`
  and a browser boot compile with no floors; the lower latency returns there, or at a
  host-declared discontinuity (#1323, below). There is no permanent latency reserve.
- **Evidence.** `host-core` `tests/successor_swap.rs::removing_a_limited_track_keeps_the_output_timing_at_every_rate_and_width`
  (`RemoveTrack` of a limited track: an unchanged `latency_samples` after the swap, and every
  block equal to the floored reference), with `host-web`
  `tests/control_parity.rs::removing_a_limited_track_keeps_the_plan_latency_alike_on_both_hosts` as
  the C ABI parity row. With no floors neither the program nor its canonical bytes
  change: the graph fixture corpus, `scripts/check-graph-determinism.sh` and `console-workload`
  keep their digests.

## A host-declared discontinuity resets the floors (#1323)

- **Surface.** `miso_engine_v1_declare_discontinuity(session)`, an in-place V1 amendment: one new
  exported symbol, announced by `MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY` (the next free bit,
  256) in the capability report, whose feature mask grows to include it; no struct, size or
  `ABI_VERSION` changes. The mask is the OR of every feature symbol and is never compared with a
  literal, except the C smoke test's header-constant pins (`capi`
  `abi::tests::masks_and_result_codes_are_frozen` folds the symbols; `abi_smoke.c` and
  `header_smoke.cpp` pin the bit inside the mask and the signature, and `abi_smoke.c` calls the
  symbol with a null session for `INVALID_ARGUMENT`). The frozen exported set is now 18
  `miso_engine_v1_*` definitions. It is a session call, serialized with the others; it runs the
  service step first. The browser's `miso_engine_web_v1_declare_discontinuity(handle)` forwards to
  the same control-plane function, `control_plane::SessionState::declare_discontinuity` (root ruling
  B15-R7); the browser SDK surface is #1296's.
- **Duty.** A host calls it at a stop, and before a seek of every source made with per-source
  seeks: the next block it renders need not continue the previous output. A plain session seek
  (#1642) declares it in the same call, so a host does not call it before one; an anchored session
  seek keeps continuity (root ruling B16-R9).
- **Behavior.** If the newest plan (the waiting replacement, else the running plan) runs behind its
  natural timing (a floor raised some node's input arrival above its natural one; the inventory
  records each raised node's natural arrival beside its floored one), the committed model is
  prepared again as a successor at a discontinuity (`SuccessorBase::discontinuity`): no floors, and
  only the source consumers (ring, generation, read position) carry, so no frame is repeated or
  skipped and every other owner starts at rest. It is published `Next` with the committed
  revision and its retirement credit reserved. A waiting replacement, a `Primed` warm successor
  included (it always runs behind its natural timing), is superseded through #1310: withdrawn, the
  donor of the rings it created, its persisting producers returned to the running epoch, then
  dropped. Its revisions complete when the successor is adopted: `EXACT`, with `SUPERSEDED` for
  the revisions it had folded in, never `TRANSITION_FALLBACK`. With nothing to reset, or at
  revision 0 (root ruling B15-R6), the call returns `OK` and publishes nothing. The model and the
  revision never change; a refusal (a preparation or cap rejection, backpressure) changes nothing,
  and a withdrawn replacement is published again whole.
- **Supersession of the successor.** A structural transaction that supersedes the discontinuity
  successor before render adopts it is prepared at the same discontinuity: render never played the
  successor, so the pair is one transaction "declare, then edit" against the running plan, whose
  combined twin has no floors.
- **Evidence.** `capi` `runtime::tests::` gates
  `a_declared_discontinuity_reports_the_natural_latency_on_the_c_abi`,
  `a_declaration_with_nothing_to_reset_publishes_nothing`, `a_refused_declaration_changes_nothing`,
  `a_declared_stop_supersedes_a_pending_warm_successor_alone_and_folded` and
  `an_edit_that_supersedes_a_discontinuity_successor_is_prepared_at_it`; `host-core`
  `successor_swap::a_declared_discontinuity_resets_the_floors_at_every_rate_and_width`; `host-web`
  `tests::structural_apply::a_declared_discontinuity_resets_the_floors_on_the_browser` and the
  parity row `control_parity::a_declared_discontinuity_resets_the_floors_alike_on_both_hosts` (deleted by TA4).

## Issue 1594 spectrum configure

`SPECTRUM_CONFIGURE` (`0010`) crosses `miso_engine_v1_submit_command` to the one core path
(`SessionState::command`), which executes it as monitor state: no revision, no saved state, no
`SESSION_COMMITTED`, no new C symbol. A form or entry change prepares a replacement plan at the
unchanged committed revision; a hop or budget change prepares none. The capability report does not
list `0010` until #1595 sets bit 16. A plan prepared with spectrum captures charges the session's
spectrum service and analysis (`SpectrumService::RETAINED_BYTES`) to its capi retained bytes. The
gate is `runtime::spectrum_tests::spectrum_commands_cross_the_c_abi_to_the_session`; the core
claims are `crates/control-plane/tests/spectrum_monitor.rs`.

## Issue 1595 spectrum capture (slice a)

`SPECTRUM_CAPTURE` (`0011`) crosses `miso_engine_v1_submit_command` to the same core path, with no
new C symbol or struct: arm, read, cancel and select on the session's spectrum service, and a ready
read analyzed into dB bins on the calling control thread. The second slice lists it, with
bit 16. A read the session's frame limit cannot carry is refused
`LIMIT_EXCEEDED` (`spectrum.frame`) before it pops the window, and a configure refuses a tap no
one-plane read can carry, so a C ABI host that reads spectrum compiles with
`maximum_control_frame_bytes` of at least 8,496 bytes (12,608 for stereo reads, 29,008 with the
window). The session's capi retained bytes also charge the one analysis result a read writes
(`SpectrumAnalysisResult`, beside the service and analysis). The gate is the same
`runtime::spectrum_tests::spectrum_commands_cross_the_c_abi_to_the_session` at a 65,536-byte frame
limit; the core claims are `crates/control-plane/tests/spectrum_monitor.rs`.

## Issue 1595 spectrum stream (slice b)

`SPECTRUM_STREAM` (`0012`) crosses `miso_engine_v1_submit_command` to the same core path, with no
new C symbol or struct: start, select, read, reset and stop on the session's spectrum service and
analysis, and a ready read analyzed with the stream's smoothing on the calling control thread. The
C ABI's capability report now lists `0010`, `0011` and `0012` and sets flag bit 16 (the adapter's
`ControlAdapter::SPECTRUM`); the browser lists none until #1596. A configure (and a boot) refuses
a tap whose stream read of its own channels cannot fit the frame limit (`spectrum.frame`), so a C
ABI host that reads spectrum compiles with `maximum_control_frame_bytes` of at least 8,576 bytes
for a one-plane tap (12,688 for a stereo tap, 29,008 for a one-shot read of both sides with the
window); a stream read also measures its reply before it reads, in depth. The capability vectors pinned in
`crates/capi/src/runtime/tests.rs` re-pin for the three IDs and bit 16. The gate is
`runtime::spectrum_tests::spectrum_commands_cross_the_c_abi_to_the_session` (the capability report
and one ready stream read); the core claims are `crates/control-plane/tests/spectrum_monitor.rs`.

## Issue 1584 observation subscriptions

The `Observe` edit (`0801`) crosses `miso_engine_v1_submit_command` in a `SESSION_TRANSACTION_APPLY`
to the one core path, which checks it against the session's binding table at preparation, shares
one arm among subscribers of the same tap and window, and queues at most one record per tap after
the commit; no C symbol or struct changes. Every C ABI plan now binds an observation queue slot for
every declared tap (`host_core::launch_maximum_observation_taps()`, with the effect lanes it rides),
and the session keeps each plan's observation readers with its epoch. `capi_retained_bytes` charges
those readers' heap per epoch exactly (the slice, the ID text, the reader boxes); the readers' slots
and the plan's observation lanes are the plan's `observation_retained_bytes`. The reference
nine-track session has no tapped effect, so its rows are unchanged. The parity row is
`runtime::live_tests::an_observe_edit_arms_a_tap_through_the_c_abi`; the race of
`live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` arms and disarms a
bypassed compressor's tap every sixteenth edit with zero render-thread allocations; the core claims
are `crates/control-plane/tests/observation_bindings.rs`. A carried tap keeps its publisher across a
swap (#1327 D2), and the promotion of the new epoch pairs its reader from the swap block's record
(`pair_carried_observations`, #1674's observation half, #1587b; see "Meter batches"), so the read
command (#1586), which reads the running epoch's readers, follows the carried tap.

## Issue 1586 observation read (slice b)

`OBSERVATION_READ` (`000f`) crosses `miso_engine_v1_submit_command` to the one core path, which
executes it on the session (`PreparedCommandFrame::Session`), with no new C symbol or struct. The
C ABI's capability report lists `000f` and sets flag bit 15 (`ProviderFeatures::observation`,
from the adapter's `ControlAdapter::OBSERVE_EDIT`); the browser lists neither until #1589. Each
epoch keeps its plan's `ObservationIndex` (one entry per effect instance by stable key, with the
position of its reader set), built in `prepare_runtime` and promoted with the epoch; the session
keeps a 256-row reply table from boot. `capi_retained_bytes` charges the index per epoch (its
entries and their ID copies) and the row table as a fixed row. The read runs
`host_core::observation::read_with` on the running epoch's readers, checks the whole batch first
and never acknowledges. The read allocates nothing on the submit path: the controller prepares it
without boxing or copying the request (`PreparedCommandFrame::ObservationRead`), and the selections
are decoded onto the stack, borrowed from the request bytes (`ObservationSelectionRef`), by the
same address codec as the `Observe` edit's. The capability vectors pinned in `crates/capi/src/runtime/tests.rs`
re-pin for `000f` and bit 15. The gates are `runtime::live_tests::an_observation_read_follows_the_arm_through_the_c_abi`
(the arm's status, tag and window, and zero allocations through `miso_engine_v1_submit_command`
for a 1-row and a 256-row read), the race of
`live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` with a read
after every edit and zero render-thread allocations, `runtime::live_tests::an_observation_read_refuses_a_bad_batch_whole`
and the parity row `protocol_cross_host::both_hosts_read_one_observation_tap_alike`. After a
rebuild that carries a tapped effect the read follows the carried tap: the promotion pairs the
carried readers from the swap block's record (#1674's observation half, #1587b).

## Issue 1578 track response

A strip `RESPONSE_GET` (`000e`, target 3) crosses `miso_engine_v1_submit_command` to the session
provider, which checks it (the core's checks, `response.rate.session`, the strip, the bound of its
event at the strip's owner count against the control frame limit and `maximum_reply_bytes`), arms
the session's one response capture (#1577) and answers its tag; the capture's result arrives as
one reliable `RESPONSE_READY` (`8040`) through `miso_engine_v1_dequeue_event`. No C symbol or
struct changes; the capability report lists `8040` with `000e` under flag bit 14, and the
capability vectors pinned in `crates/capi/src/runtime/tests.rs` re-pin for it. `SessionState::service`
turns the delivered result into the event (`host_core::response_ready_event`, #1578 D5) and moves
it into the protocol controller's one cell; a full reliable queue, a held cell or an outstanding
structural transaction keeps it for the next `service`, an event the frame limit cannot carry
becomes its `LIMIT_EXCEEDED` `response.reply_too_large` refusal, and the capture is released once
its event's frame has reached the host. The response capture's charge
(`host_core::response_capture_ceiling`) adds the event's member table, which is allocated while
the result's is still held. The gates are `crates/capi/tests/response_strip.rs`:
`end_to_end_then_removed_strip` (no event before a render; one event after one render whose curves
and owners are bit-equal to `compose_track_response` over a `PreparedHost` of the same document;
one event across a rebuild; `NOT_FOUND` `response.strip.removed` for a strip a superseding removal
took), `one_in_flight_none_lost_and_replay` (`BACKPRESSURE` `response.capture.pending` with queue
kind 8 while one is in flight; held, not lost, behind a full reliable queue; a replay returns the
same tag and arms nothing; released after delivery) and
`a_capture_grown_past_the_frame_limit_is_answered_by_its_refusal`.
