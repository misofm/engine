#ifndef MISO_ENGINE_V1_H
#define MISO_ENGINE_V1_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Thread ownership (frozen for ABI 1.0; unchanged by any minor version).
 *
 *   any thread   miso_engine_v1_abi_version and miso_engine_v1_query_capabilities take no handle
 *                and may be called from any thread at any time.
 *   engine       Control thread. At most one thread at a time calls any function taking the engine
 *                (miso_engine_v1_engine_create, miso_engine_v1_compile_session, and
 *                miso_engine_v1_last_error on the engine).
 *   session      Control thread. At most one thread at a time calls
 *                miso_engine_v1_source_submit_planar_f32, miso_engine_v1_source_seek,
 *                miso_engine_v1_source_seek_at, miso_engine_v1_source_seek_report,
 *                miso_engine_v1_submit_command, miso_engine_v1_submit_document_command,
 *                miso_engine_v1_dequeue_event,
 *                miso_engine_v1_service, miso_engine_v1_declare_discontinuity,
 *                miso_engine_v1_session_seek, miso_engine_v1_session_seek_at,
 *                miso_engine_v1_session_seek_at_once, or
 *                miso_engine_v1_last_error on the session; they are serialized with each other.
 *   plan         Split ownership.
 *                miso_engine_v1_render_f32_planar: render thread only, never concurrently with
 *                itself, and the exclusive owner of the plan's render state.
 *                miso_engine_v1_plan_resources, miso_engine_v1_plan_watermark and
 *                miso_engine_v1_last_error on a plan: any thread, at any time while the plan is
 *                live, including concurrently with a render call. They are pure with respect to
 *                the plan handle: the report is copied from the plan's frozen resource
 *                accounting, the watermark from the record render publishes, and the diagnostic
 *                is one atomic error word. None of them writes plan render state, allocates, or
 *                blocks the render thread.
 *   *_destroy    miso_engine_v1_engine_destroy, miso_engine_v1_session_destroy and
 *                miso_engine_v1_plan_destroy require quiescence: no other call on that handle is
 *                in flight or will start. A session and its plan may be destroyed in either order.
 *
 * Live edits. A committed SESSION_TRANSACTION_APPLY whose changes are all live is applied to the
 * running plan: no plan is prepared, and the source rings, the effect state and the render
 * position continue, so the host goes on submitting with no seek. The live changes are a track's
 * fader levels, mutes, and pan or matrix values; a changed value of a track effect parameter
 * whose automation rate is Block, on a console-slot entry or an insert, the parametric EQ's live
 * parameters included (applied through prepared targets); a track effect's bypass; and the
 * link mode of an insert or of a console entry's override of the compressor, gate/expander,
 * transient shaper, true-peak limiter, de-esser or dynamic EQ (#1371): the lane's detector glides
 * to the new mode over the edit's own ramp_samples, else the session's fader_ms. A console slot's
 * declared link mode, and the link mode of any other effect, rebuild. A
 * transaction that changes only the
 * session ID, a profile's id or a stored automation entry on a row that does not render yet
 * commits with no replacement and no live value. A stored fader automation edit (an entry on a
 * strip's fader_db added, removed, retargeted or given other segments) rebuilds the plan, and
 * the successor carries every owner; a static fader value of a lane the session automates is that
 * lane's fallback and gives no live value, and a SET_CONTROL of that lane's fader_db writes the
 * entry's hand offset instead, a live value heard on top of the curve (#1650); the other rows stay
 * model-only until they render.
 * Any other change takes the replacement path exactly as before: every submix strip
 * value (its fader, its effects), any edit while the session declares a VCA, the mute of a track
 * that a follows_mute send reads, a fader outside its domain or effect parameters that
 * preparation refuses (refused as MISO_ENGINE_V1_COMPILE_REJECTED with the same diagnostic as
 * before), and every other field. A committed live value (fader, mute, pan or matrix, effect
 * parameter, effect bypass, effect link mode or parametric EQ) reaches render no later than the first block whose
 * render call begins after miso_engine_v1_submit_command returns or, while a replacement plan is
 * pending, at that plan's adoption, which the watermark reports. Every such value of one
 * transaction takes effect in the same block, and the watermark names that block's first sample;
 * it is heard up to latency_samples later. These values are held in latest-target cells, not
 * queues: a host sends any number of fader, mute, pan, effect parameter, effect bypass, effect
 * link mode or parametric EQ edits between two render calls, a paused host included, and none is refused for
 * room. A later value for the same cell committed before that block replaces the earlier one.
 * The counter LIVE_VALUES_SUPERSEDED (protocol counter 16, read with COUNTERS_GET) counts, per
 * cell, each write to a plan that render adopts which the block covering it does not apply
 * because a later write to the same cell exists. An edit that moves both channels of a per-lane
 * parameter or of a parametric EQ target writes two cells and counts in each; an edit of a shared
 * parameter writes its one cell. A write to a candidate that is withdrawn, and a value on a lane
 * that a swap restarts, are not counted; no value is lost either way. The restart itself is
 * counted: STRIPS_RESTARTED (protocol counter 18, read with COUNTERS_GET) adds, for each
 * replacement plan that render adopts, the strips present in both plans of which the swap starts a
 * part at rest: a fader, mute or matrix stage, a console slot, an insert or a strip delay line
 * that cannot carry its state, or an effect chain that changed. Each such strip counts once per
 * adopted plan, however many of its parts restart. A candidate that is withdrawn adds nothing, and
 * an added strip is not a restart. COUNTERS_GET reports it once the render call of the adopting
 * block has returned. An effect's bypass, its link mode (one cell, for the effects above), its
 * parameters (one cell per channel for a per-lane parameter, one for a shared one) and a
 * parametric EQ's prepared targets are cells of that effect's lane; a block applies the bypass
 * first, then the link mode, then the parameters in parameter order, then the targets, all under
 * the one snapshot that covers the fader values. A replacement plan that lands while a link mode
 * glides continues the glide to its target. An acknowledged live edit
 * is never dropped. A live edit commits
 * the same reliable events as a replacement, so the host drains the reliable event lane after
 * each edit either way. The success response of every SESSION_TRANSACTION_APPLY names how the
 * engine applied the transaction in its required field 2, edit-path (a U8), known when the
 * command returns and in the same frame as the new revision: 1 = live (records went to the
 * running plan's live lanes, no plan was prepared), 2 = model_only (committed, and the delta
 * holds no record for render: any transaction whose effective values do not change), 3 = rebuild
 * (a successor plan was prepared and published, whatever else the transaction held), 4 = monitor
 * (see "Solo" below). Zero and every other value are refused by the decoder. An exact replay of
 * the request returns the same bytes, the path included. A fallback to a rebuild after submit
 * and a supersession are not in the response; the watermark's outcome flags report them. After
 * a live edit or a replacement the host goes on submitting every source the transaction left
 * unchanged, with no seek; after a replacement it acts only on a source the transaction added,
 * changed or removed (see "Sources across a structural transaction" and the seek paragraphs from
 * "The source-read clock" to "Where a seek landed" below). (miso_engine_v1_plan_resources
 * describes whichever plan is active, so a replacement may change it, but an unchanged report
 * does not mean that no replacement happened; the edit path is the signal.)
 *
 * Latency across a replacement (issue 1285). No node's latency, the output's included, drops
 * across a structural transaction: the successor plan is compiled with every surviving node's
 * input arrival in the plan it displaces as a lower bound, so removing a latent effect, or the
 * track that holds it, moves no other path in time, every compensation line keeps its length, and
 * latency_samples does not fall. A path whose own upstream latency dropped gets a new
 * compensation line, which starts at rest. An added latency still raises latency_samples. The
 * lower latency returns only with a fresh miso_engine_v1_compile_session, or at a host-declared
 * discontinuity (below); there is no permanent latency reserve.
 *
 * Declared discontinuity (issue 1323). miso_engine_v1_declare_discontinuity(session) says that
 * the next block the host renders need not continue the previous output, because the host has
 * stopped or is about to seek every source. The host's duty: call it at a stop, and before a seek
 * of every source made with per-source seeks. A plain session seek is a declared discontinuity:
 * the engine declares it in the same call (see "Session seek"), so the host does not call this
 * before it; an anchored session seek keeps continuity. If the newest plan (the waiting
 * replacement, else the plan that renders) runs behind its natural timing because a floor above
 * raised some node's arrival, the call prepares
 * the committed session again with no floors and publishes it for the next block: from that
 * block latency_samples is the session's natural latency. That plan carries only the sources
 * (each source's ring, generation and read position), so no frame is repeated or skipped; every
 * effect, line and meter starts at rest. A waiting replacement is superseded, as a structural
 * transaction supersedes it, and its revisions complete when the new plan is adopted with
 * MISO_ENGINE_V1_OUTCOME_EXACT (MISO_ENGINE_V1_OUTCOME_SUPERSEDED for the revisions it had folded
 * in), never MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK. A structural transaction that supersedes
 * the new plan before render adopts it is prepared the same way. With nothing to reset, or at
 * revision 0, the call returns MISO_ENGINE_V1_OK and publishes nothing. The model and the
 * revision never change. It returns MISO_ENGINE_V1_OK; a null session or another handle as every
 * session call; and MISO_ENGINE_V1_COMPILE_REJECTED, MISO_ENGINE_V1_BACKPRESSURE or
 * MISO_ENGINE_V1_INTERNAL with the session diagnostic as miso_engine_v1_submit_command returns
 * them for a rebuild; a refusal changes nothing. A host checks
 * MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY before calling the symbol.
 *
 * Solo (issue 1559). Solo is a SET_CONTROL like every other value: the address is the strip's
 * own section, control 2 (named "solo", a toggle, value exactly 0 or 1, lane both), on a track.
 * A submix has no solo row (a submix is solo-safe), so a solo on a submix is the typed refusal
 * control.address.unknown. A solo is host-local: it lives in the engine's monitor overlay, it is
 * never saved, it is never in a SESSION_SNAPSHOT_GET document and it does not change the control
 * list's structure epoch; CONTROL_LIST_GET and CONTROL_STATE_GET report the overlay's value on
 * the solo row. A transaction of solo writes only commits no model: its response has edit-path
 * 4 (monitor) at the committed revision, no SESSION_COMMITTED event follows, and an exact replay
 * returns the same bytes and applies nothing again. The other tracks mute and the sends that
 * follow them close through the same effective values that a plan's preparation reads, ramped
 * over the write's ramp_samples (absent: the session's mute_ms; 0: a step); a structural
 * transaction keeps the solo (a strip it removes loses its entry, a strip it adds while another
 * is soloed is muted), and a transaction that mixes solo writes with model edits is all or
 * nothing and advances the revision once. The records of a solo are published on the running
 * plan's revision gate under the committed revision with a monitor publication number (issue
 * 1603): such a solo spends no revision number, so committed revisions stay dense and the next
 * commit after any number of solos is the committed revision plus one. The applied watermark
 * reports revisions only, so it never reads a number above the committed revision, and it is
 * never early.
 *
 * Observation subscriptions (issue 1584). An OBSERVE edit (opcode 0x0801) in a
 * SESSION_TRANSACTION_APPLY arms or disarms one observation tap of an effect, such as a
 * compressor's gain reduction: the address is the strip and its console slot or insert, with the
 * effect-local tap ID, armed 1 or 0, and window_blocks (0 is the plan's meter window, at most
 * 4096). No C symbol or struct is added. A subscription is monitor state, as a solo is: never
 * saved, never in a SESSION_SNAPSHOT_GET document, no revision; a transaction of OBSERVE and solo
 * edits only replies edit-path 4 (monitor) at the committed revision with no SESSION_COMMITTED.
 * Subscribers share one arm per tap and window: a second arm with the same window and a disarm
 * that leaves a subscriber queue nothing, an arm with another window is
 * observation.window_conflict, and a disarm of a tap not armed with that window is
 * observation.not_subscribed. Every refusal (also observation.address.unknown,
 * observation.tap.unknown, observation.tap.unsupported, observation.window) is
 * VALIDATION_FAILED at the edit's index, with nothing changed. The tap's one-slot queue drains at
 * the next rendered block; an OBSERVE that needs a record in a queue that has not drained yet
 * (an arm and a disarm before any render) is MISO_ENGINE_V1_RESULT_BACKPRESSURE with nothing
 * recorded, and the same request succeeds after one render. A subscription survives every plan
 * rebuild while its effect exists; a rebuild that removes the effect ends it with one
 * OBSERVATION_ENDED event. Every plan binds an observation queue for every declared tap.
 *
 * Response preview (issue 1573). RESPONSE_GET (message ID 0x000e, capability flag bit 14)
 * through miso_engine_v1_submit_command asks for the frequency response of a native effect or of
 * the input filters, for parameters the caller chooses, and answers the curves in the reply. No C
 * symbol or struct is added. The request names one configuration (the effect, its quality and
 * link mode, or the input filters), the axis (linear or logarithmic, 2 to 4096 points, between
 * two frequencies up to the session's Nyquist frequency), the sides, whether the per-section
 * curves are wanted, and any overrides: a control ID, a lane (left, right or both) and a value.
 * Control 0 of an effect is its bypass (lane both, value exactly 0 or 1); both on a per-lane
 * parameter sets both lanes. The reply is computed at the session's sample rate and quantum from
 * the same core function the browser uses, so the request must not carry its own rate or quantum
 * (response.rate.session). It is a read: it changes no session, revision or catalog. A refusal is
 * a typed non-OK response whose one error diagnostic carries the code (for example
 * response.points, response.grid, response.effect.unknown, response.override.control,
 * response.limit, response.target.unavailable); a reply that would not fit the control frame
 * limit, or the request's own maximum_reply_bytes, is refused whole as
 * response.reply_too_large before any curve is computed. An exact replay of the request returns
 * the same bytes and computes nothing.
 *
 * Track response (issue 1578). RESPONSE_GET with a strip target (a track or submix stable ID)
 * through miso_engine_v1_submit_command answers a request tag at once; the curve of the strip's
 * input filters and EQs, from the values its plan renders, with the list of the owners that make
 * it, arrives later as one reliable RESPONSE_READY event (message ID 0x8040, listed with 0x000e
 * under capability flag bit 14) through miso_engine_v1_dequeue_event, carrying the same tag. No
 * C symbol or struct is added. The result comes from the first successful render block of the
 * newest plan after the request, so a host that does not render and service the session does not
 * get it. One capture is in flight per session: a strip request while one is in flight or its
 * event is unread is BACKPRESSURE response.capture.pending. Every request answered with a tag
 * gets exactly one RESPONSE_READY: OK with the curves, or a refusal (response.strip.removed when
 * the strip left the session first, response.effect.unsupported, response.limit,
 * response.capture, and LIMIT_EXCEEDED response.reply_too_large when a rebuild after the request
 * grew the strip's event past the control frame limit); a full event queue delays it and never
 * drops it. An unknown strip is
 * NOT_FOUND control.address.unknown, and a request whose event could exceed the control frame
 * limit or maximum_reply_bytes is response.reply_too_large; neither arms anything. A replayed
 * request returns the same tag and arms nothing.
 *
 * Spectrum taps (issue 1594). SPECTRUM_CONFIGURE (message ID 0x0010) through
 * miso_engine_v1_submit_command sets, changes or removes the session's spectrum taps while it
 * plays. No C symbol or struct is added; the capability report lists it with the spectrum reads
 * and capability flag bit 16 (issue 1595). The taps are monitor state: the command changes no
 * revision and no saved state and emits no SESSION_COMMITTED. The same configuration again
 * changes nothing; a change of only the hop or the byte budget prepares no plan; any other change
 * prepares a replacement plan at the unchanged committed revision, in which taps that stay keep
 * their window and history, and the plan watermark does not move when it is adopted. The reply
 * carries the committed revision, the configured and adopted configuration epochs, whether a plan
 * was prepared and, if one was, its plan epoch; the configuration is in effect when a later
 * reply's adopted epoch equals its configured epoch. A refusal is a typed non-OK response whose
 * one error diagnostic carries the code (spectrum.target.unknown, spectrum.duplicate,
 * spectrum.budget.retained, spectrum.budget, spectrum.target.id) and changes nothing. An exact
 * replay of the request returns the same bytes and prepares nothing.
 *
 * Spectrum reads (issue 1595). SPECTRUM_CAPTURE (message ID 0x0011) and SPECTRUM_STREAM (0x0012)
 * through miso_engine_v1_submit_command read the one-shot window and the continuous stream. No C
 * symbol or struct is added. The capability report lists 0x0010, 0x0011 and 0x0012 together and
 * sets capability flag bit 16. A capture's arm, cancel and select answer done; a read answers
 * pending until 2048 frames have rendered since the arm, then ready with the window analyzed into
 * 1025 dB bins (and, when asked, the raw 2048-frame planes). A stream's start, select, reset and
 * stop answer the stream's metadata; a read answers its status (inactive, warming, pending, gap
 * with the count of dropped windows, failed, stopped, none of them a refusal) or ready with the
 * held window analyzed with the stream's smoothing. The analysis runs on the thread that calls
 * miso_engine_v1_submit_command, never in render. A read never consumes what it cannot deliver:
 * one whose ready reply exceeds the session's maximum_control_frame_bytes is refused
 * LIMIT_EXCEEDED (spectrum.frame) and leaves the window, and a stream's gap, queued; a stream
 * read's size is fixed by the selected entry's channels. SPECTRUM_CONFIGURE refuses a tap whose
 * one-plane read the frame limit cannot carry (8576 bytes for an identity of up to 8 bytes; 12688
 * to stream or read both planes; 29008 for both raw planes). A refusal is a typed non-OK response
 * whose one error diagnostic carries the service's code (spectrum.unprepared,
 * spectrum.entry.unknown, spectrum.busy, spectrum.capture.dropped, spectrum.not_armed,
 * spectrum.channels, spectrum.select.single, spectrum.stream.active, spectrum.stream.inactive and
 * others) and changes nothing. An exact replay of a read returns the same bytes and pops nothing;
 * a host gives each read a new request ID.
 *
 * Observation reads (issue 1586). OBSERVATION_READ (message ID 0x000f, capability flag bit 15,
 * which also means the OBSERVE edit is served) through miso_engine_v1_submit_command reads the
 * latest window of 1 to 256 observation taps, each named by its strip, its console slot or insert
 * and its tap ID. No C symbol or struct is added. The read consumes nothing and arms nothing. Each
 * row says pending, unarmed or ready, the arm state render applied (armed, arm sample, arm tag;
 * none until render applies the tap's last record), and, when ready, the window (first and end
 * sample, sequence, blocks) and the asked-for lanes in the tap's native unit. A tap is ready once
 * a window that opened at or after its arm has closed; a disarm reads unarmed when it is accepted.
 * The batch is checked whole first: a reply wider than maximum_control_frame_bytes (64 + 192 bytes
 * per row; 21 rows fit 4096 bytes) is LIMIT_EXCEEDED (observation.frame), and the first bad
 * selection is refused at selection[i] (observation.address.unknown, observation.tap.unknown,
 * observation.tap.unsupported, observation.unbound, observation.selection.duplicate), with no
 * row. A tap whose effect a rebuild carried keeps its reader, window and arm across the swap: the
 * session pairs the carried readers from the swap block's record when it promotes the new plan
 * (issue 1674's observation half, issue 1587).
 *
 * Completion (issue 1314). miso_engine_v1_submit_command is synchronous only for what can fail:
 * it validates and classifies the transaction, prepares a replacement plan and reserves its
 * publication and retirement credit, commits, and returns; it never waits for a render call or a
 * plan swap. Every committed revision is pending until the plan watermark covers it.
 * miso_engine_v1_plan_watermark copies (revision, first_sample, outcome_flags) and per-outcome
 * counters: revision is the highest committed revision in effect together with every revision
 * before it, and first_sample the absolute render sample at the start of the block that reported
 * it. The watermark is never early: a revision is in effect no later than the block it names.
 * first_sample is the start of the first block that begins after the commit's last write, so at
 * most one block after the submit returns (while a replacement plan is pending: the block that
 * adopts it). Every live value of that revision takes effect together, in the block that
 * first_sample names. A replacement plan renders from first_sample. A host that
 * renders nothing completes nothing: a paused host's revisions stay pending until render
 * resumes. The watermark is a level, not a queue: render overwrites it, so a host that polls it
 * late still reads the newest revision, and the counters still count every revision.
 * outcome_flags is the OR
 * over the revisions the last advance covered of MISO_ENGINE_V1_OUTCOME_EXACT,
 * MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK and MISO_ENGINE_V1_OUTCOME_SUPERSEDED.
 * MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK reports a warm successor's fallback to its
 * transition, or a rebuild that carries a send following a stored mute into one that follows
 * none (conservative: it can be set when no send moves at the swap; #1713 removes it). It also
 * reports any rebuild that carries an automated following send while a live send move may still
 * ramp (conservative; #1718 removes it). The host
 * polls the watermark to learn that an edit is audible, never the event lane. The caller zeroes
 * reserved0 and reserved[] and sets struct_size, or the call returns
 * MISO_ENGINE_V1_INVALID_ARGUMENT; a copy that keeps landing inside render's publication
 * returns MISO_ENGINE_V1_BACKPRESSURE with out untouched, and the host retries. A host checks
 * MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK before calling the symbol.
 *
 * Meters (issues 1583, 1587, 1690). TELEMETRY_CONFIGURE takes a meter table: entry i (meter
 * handle i + 1) names the output, a track or submix by its stable ID, or the master, and a metric,
 * so a structural commit never moves a handle. Its reply carries the session's meter window in blocks
 * (meter_window_blocks, about 32 ms). A nonempty table turns the meters on from the next rendered
 * block, an empty one off; render applies the change before a block, on the strip meters' window
 * grid, with no allocation, lock or syscall. Each METER_BATCH reports meter_period_blocks /
 * meter_window_blocks consecutive windows: first_sample to observed_sample (exclusive),
 * window_sequence from 1 after each accepted configure, and, in table order, a left and a right
 * record per peak entry: for the output, its two planes' maxima over the batch, and for a strip,
 * that strip's post-matrix maxima. A record is VALID, and CLIPPED above 1.0; an entry whose strip
 * the rendering plan does not have (removed, not yet added) has flags 0 and value 0.0, and is
 * VALID again under the same handle once a commit brings the strip back. A strip's gain-reduction
 * entry gives one AGGREGATE record in non-negative dB: the strip's reading from the batch's last
 * window that has one, VALID when any window has one, else flags 0 and value 0.0. A strip's
 * reading is the deepest reduction among its observation taps that an Observe edit armed, each
 * from its newest window that opened at or after the tap's arm; a tap not armed, or disarmed,
 * gives none. Readings continue across a plan swap that carries the strip, and a tap's reading
 * and arm continue across a swap that carries its effect; a swap that carries no strip meter
 * restarts the window grid at its first block. A swap that does not carry the effect (one whose
 * state cannot continue) restarts its taps: an armed tap stays armed, and the strip's gain
 * reduction is absent until the tap's first window in the new plan closes. The master entry
 * (gain reduction only; output peak only; any other pair is INVALID_FIELD) gives the same record
 * for the strip the master row names when the batch is staged, which follows every SET_CONTROL of
 * that row and every adopted plan, with no reconfigure; with no master, or a master that has no
 * reading, it has flags 0 and value 0.0, and it stays in the table. A configure is all or nothing: an
 * unknown strip is NOT_FOUND (meter.strip.unknown), a period that is not a whole number of windows
 * INVALID_FIELD (meter.period), and a table of more than 128 entries INVALID_FIELD (meter.records,
 * at its 129th entry), so a batch never needs more than its 256 records. Each
 * meter queues eight windows; a window that reaches no batch (dropped while the host did not call
 * into the session, or in a partial batch a gap or a swap discarded) is counted by
 * METER_WINDOWS_LOST (protocol counter 22), and the sequence never skips.
 *
 * Control service (issue 1348). The engine owns no thread, so the host drives the control work
 * between edits. miso_engine_v1_service(session) runs one bounded step of it: it reclaims plans
 * render has retired, brings the session's view of the running plan up to render, refreshes the
 * session counters, and stages render telemetry: the counter snapshot, the diagnostics and one
 * METER_BATCH per completed batch of meter windows. Every other session call (source submit, seek,
 * seek_at, seek_report, session_seek, session_seek_at, submit_command, dequeue_event) that passes
 * its argument checks runs the same step first; a call refused for its arguments returns before the
 * step. So a host that makes those calls anyway need not also call service. The step never waits
 * for render, never commits or acknowledges anything, and never takes or drops a pending edit; its
 * cost does not grow with the time since the last call. It returns MISO_ENGINE_V1_OK; a null
 * session as MISO_ENGINE_V1_INVALID_ARGUMENT and another handle as MISO_ENGINE_V1_WRONG_HANDLE;
 * and, should the session's plan epochs fail to synchronize, MISO_ENGINE_V1_INTERNAL with the
 * session diagnostic "source.epoch". The host's duty: a pending edit progresses only while the
 * host makes session calls, the same duty as draining events. A host that renders but never calls
 * keeps retired plans allocated, and its next structural edit waits for that reclaim; a warm
 * successor that misses its deadline falls back only at the host's next session call. The plan
 * watermark never covers an edit before it is in effect. While edits are pending, call
 * miso_engine_v1_service (or another session call) at least once per render-buffer period. A host
 * checks MISO_ENGINE_V1_FEATURE_SERVICE before calling the symbol.
 *
 * Engine memory (issue 1602). The engine owns no control-thread memory outside the compile
 * limits beyond its own handle; miso_engine_v1_engine_destroy frees that handle. Everything a
 * compile retains is owned by its session and plan and is charged to the fields of
 * miso_engine_v1_compile_limits.
 *
 * Borrowed pointers (session JSON, source IDs, request frames, chunk planes, output samples, and
 * every out pointer) are read or written only for the duration of the call and are never retained.
 * Every float plane pointer (chunk planes and output samples) must be 4-byte aligned, and the chunk
 * plane array must be pointer-aligned; a null or misaligned pointer returns
 * MISO_ENGINE_V1_INVALID_ARGUMENT before any access. No borrowed byte or sample region may exceed
 * PTRDIFF_MAX bytes; a larger declared length also returns MISO_ENGINE_V1_INVALID_ARGUMENT before
 * any access.
 *
 * miso_engine_v1_last_error on a plan returns a fixed diagnostic selected by the most recent render
 * call, one string per rule -- "render.output.unaligned", "render.output.platform",
 * "render.output.layout", "render.output.shape", "render.time.discontinuity",
 * "render.time.overflow", "render.plan.rejected" -- and is empty after a successful render; on a
 * session or engine it returns the most recent control-thread diagnostic. The counter
 * LIVE_BLOCK_ERRORS (protocol counter 19, read with COUNTERS_GET) counts the render calls that
 * returned MISO_ENGINE_V1_RENDER_REJECTED with "render.plan.rejected": blocks the plan refused
 * after the block took its live snapshot. Such a block may have applied live values, its output
 * is not a rendered block, and the watermark does not advance on it. No valid plan reaches this
 * path, so a nonzero value is an engine defect. The counter SOURCE_UNDERRUN_BLOCKS (protocol
 * counter 20, read with COUNTERS_GET) counts the rendered blocks in which a source played zeros
 * because the host had not delivered PCM it owed, or a source had no stream (graph-wide, not per source); a held block and a
 * block past the end of a region are not counted, and it never resets. The counter
 * SOURCE_FRAMES_DISCARDED (protocol counter 21) counts, in frames, the accepted source PCM that
 * render never read because the caller's own transaction removed or redeclared the source: the
 * chunks still queued in the discarded ring (a stale-generation chunk included) when a later call
 * reclaims the displaced plan. A failed carry (the new plan could not take the old plan's rings)
 * loses the frames of every ring it did not move, and those frames are counted too. Frames render
 * played, a ring discarded by session destroy and the
 * rings of a withdrawn replacement (never fed: such a transaction is refused) are not counted; it
 * never resets. A rejected source
 * submission or seek likewise names the rule it broke ("source.region.outside",
 * "source.generation.stale", "source.channels.mismatch", ...). A null, empty, non-UTF-8 or
 * over-127-byte source ID on submit, seek, seek_at or seek_report is
 * MISO_ENGINE_V1_INVALID_ARGUMENT with "source.id.invalid".
 * MISO_ENGINE_V1_UNSUPPORTED is returned by exactly one entry point: miso_engine_v1_engine_create
 * refuses to create an engine on a CPU that cannot execute the instruction set this library was
 * built for (issue 083, master plan D4 -- the engine dispatches nothing at runtime, so the check
 * happens once at boot instead of inside a render callback). No other entry point returns it. An
 * embedder that receives it must not retry; the library and the CPU do not match.
 *
 * Sources across a structural transaction (issue 1273). A SESSION_TRANSACTION_APPLY that replaces
 * the plan keeps every source it leaves unchanged playing: the source keeps its ring, its
 * generation and its read position, so the host neither seeks nor refills it, and must not reseek
 * it. From the moment the command returns OK, source submissions and seeks address the newest
 * committed session, whether or not its plan has been swapped in yet, with one exception: a source
 * the transaction removed (issue 1325).
 *
 * Removing a strip in two phases (issue 1325). A transaction that removes a track or a submix does
 * not cut it at the swap. Phase 1: before the command returns OK, the strip's fader is muted on
 * both channels on the running plan with the session's mute_ms ramp N, and so is each of its routes
 * whose tap precedes the fader. Phase 2: the replacement plan is adopted no earlier than
 * S = ceil_q(p + q + N + C), where p is the render clock after the mutes were written, q the render
 * quantum and C the longest compensation delay of a route out of a removed strip. The watermark
 * reports the transaction's revision first at S. Until then a source the transaction removed still
 * feeds the running plan: its submissions and seeks are accepted and its PCM plays the fade, so the
 * host keeps feeding it. From the adoption on it is refused as "source.id.unknown", and PCM
 * accepted for it that render has not read is discarded with the running plan. A later transaction
 * before S writes no second fade for a strip already fading, and its replacement is adopted no
 * earlier than the same S. A transaction before S that restores a removed strip commits too, and
 * its replacement is adopted no earlier than the same S: the strip fades back in from S over the
 * session's mute_ms ramp, and so does each of its routes whose tap precedes the fader. A restored
 * source declared as before keeps its ring: the PCM accepted for it in phase 1 plays, and the host
 * keeps feeding it without a seek. A source the transaction added starts at generation 1, frame 0,
 * until the host seeks it. A source whose declaration the transaction changed restarts like an
 * added one: in a new ring at generation 1, frame 0, with the PCM accepted for it before the
 * command discarded with its plan; the host restarts its feed.
 *
 * Removing or re-pointing a route (issue 1953). A transaction that removes a route between two
 * strips it keeps, or gives it another source, tap or destination, and whose route goes into a
 * submix or leaves its source before the fader, ramps it out the same way: the route is muted on
 * the running plan over N, C counts its compensation delay, and the replacement is adopted no
 * earlier than S. A re-pointed route is a removal plus an addition: from S it fades in at its new
 * destination as an added route does, and its source strip plays on. A post_fader or post_pan
 * route into an output has no live lane, so it cannot ramp before the swap: the replacement fades
 * a copy of it out over N from the adoption, through the route's own compensation delay, and a
 * route re-pointed from one fades in at its new destination from the adoption.
 *
 * Duck-swapping a strip whose state cannot continue (issue 1324). A transaction whose
 * replacement restarts a strip it keeps -- an insert added, removed or reordered, delay_samples, a
 * value the plan keeps prepared (such as the true-peak limiter's lookahead), any edit after which
 * one of the strip's owners cannot carry its state -- ducks that strip as a removal does: its fader
 * and each of its routes whose tap precedes the fader are muted on the running plan over N, and the
 * replacement is adopted no earlier than S, with C the longest compensation delay of a route out of
 * a ducked strip or of a sidechain line the replacement can shorten. A strip whose insert is keyed
 * from a restarted strip's tap behind a restarted owner (post_input, insert_send, insert_return,
 * pre_fader, or input when its delay line restarts) is restarted and ducks with it. The
 * replacement starts every owner of the strip before its fader, and its fader, at rest, then fades
 * the strip and those routes back in over N from the first block at or after S + D, where D is the
 * strip's own latency before its fader. The dip is N, the wait to S (from q + C up to 2q + C), D,
 * then N. Strips the edit does not restart play on, bit for bit, and a re-pointed route never
 * ducks its source strip. The watermark reports the revision first at S as EXACT. A transaction
 * before S that reverts the edit commits and is adopted at the same S; the strip fades back in from
 * S. One limit remains: a route before the fader that follows its source's automated mute is not
 * faded back in: it restarts at its prepared value at S.
 *
 * A transaction behind an unadopted replacement (issue 1310). A SESSION_TRANSACTION_APPLY that
 * needs a new plan while an earlier transaction's replacement plan has not been adopted by render
 * (a paused host never adopts one) commits as any other: it is not refused as backpressure because
 * a replacement waits, with the one exception below. The engine takes the waiting replacement back,
 * prepares one plan that carries both transactions against the plan that renders, and publishes it;
 * nothing of the earlier transaction is lost. A source the new plan keeps from the waiting
 * replacement or from the plan that renders keeps playing: the PCM accepted for it and a seek or
 * seek_at queued on it move into the new plan, so the host neither refeeds nor reseeks it. An owner
 * (a strip, an effect, a meter) that the earlier transaction restarted or removed and the new one
 * puts back in its form in the plan that renders carries from the plan that renders, as one
 * transaction of both would carry it (issue 1698). A source does not: its declaration changed
 * against the newest committed session, so the host restarted its feed, and a source the earlier
 * transaction added, redeclared or removed and the new one does not keep as the waiting replacement
 * declares it starts fresh in the new plan (generation 1, frame 0, as an added one), also when the
 * new one puts it back in its form in the plan that renders. No accepted chunk on a ring the
 * waiting replacement created is dropped across a supersession: when the waiting replacement
 * created a source's ring (the earlier transaction added the source, or redeclared it) and the
 * session has accepted PCM, a seek or an end of region for that source since, a new transaction
 * that does not keep the source as the waiting replacement declares it (it declares it as the plan
 * that renders does, redeclares it another way, or removes it) is refused with
 * MISO_ENGINE_V1_BACKPRESSURE before it commits: nothing changes, the waiting replacement stays
 * published, no event is queued and the revision does not move. Once render adopts the waiting
 * replacement, a retry commits. A host that does not render keeps getting
 * MISO_ENGINE_V1_BACKPRESSURE for such a transaction until it renders. A source the waiting
 * replacement carries unchanged from the plan that renders is not refused this way. A source the
 * caller's own transaction removes discards its queued audio, whether the plan that renders or the
 * waiting replacement (carrying it unchanged) holds it: the caller asked for the removal, so this
 * is not an unintended drop. The counter SOURCE_FRAMES_DISCARDED (protocol counter 21, read with
 * COUNTERS_GET; issue 1699) counts those frames, and the frames a redeclared source's old ring
 * discards (above). A resource cap counts every plan the session still holds and the new
 * one: the running plan, the waiting replacement, and a plan render has displaced until a later
 * call reclaims it. A refusal is MISO_ENGINE_V1_BACKPRESSURE when the new plan fit against the
 * newest held plan alone, so a retry after a render normally commits; the split is approximate, and
 * such a retry may still be refused within a small band (a plan prepared on the adopted replacement
 * differs by a few hundred bytes from one prepared across it). A refusal that does not fit even
 * then is MISO_ENGINE_V1_COMPILE_REJECTED with the cap's diagnostic. When render adopts the new
 * plan, the watermark advances to its revision with MISO_ENGINE_V1_OUTCOME_SUPERSEDED set, and
 * superseded_count grows by the revisions the earlier replacements carried.
 *
 * The source-read clock (issues 1275, 1316). A seek that names a block names it on the plan's
 * source-read clock: the absolute sample of source time the block reads. It is equal to the
 * render clock until a plan that reads its sources ahead of it is adopted (a warm successor, issue
 * 1355); no call publishes one in this version (issue 1403). A plan's clock is 0 at compile.
 * Each miso_engine_v1_render_f32_planar call must pass the clock's value as absolute_sample,
 * renders one quantum and advances the clock by one quantum; a call at another sample renders
 * nothing, moves nothing and returns MISO_ENGINE_V1_RENDER_REJECTED with
 * "render.time.discontinuity". A plan that a structural transaction swaps in continues the clock
 * of the plan it replaces, so an anchor stays valid across a swap.
 *
 * Seek. miso_engine_v1_source_seek(id, g, F) makes generation g play from source frame F. The
 * call switches the source to g at once: from then on the host submits generation g in order from
 * F, and a chunk of any other generation is refused as "source.generation.stale". A plain seek
 * lands when its PCM is ready (issue 2205): the PCM of the playing generation that is still queued
 * plays on until the seek's runway of R blocks of g is queued, then the seek lands in one cut and
 * g plays from F, its first frame; the earlier generations' queued PCM is discarded, not played.
 * R is min(ceil(50 ms / quantum), ring blocks / 2), fixed when the plan is prepared: 19 blocks at
 * 48 kHz and 128 frames with the default ring. The seek also lands as soon as a block of g that
 * ends the region is queued, at once when F is the region's end, and at once when the source had
 * received no PCM before the call (an empty boot or a stem still loading waits for nothing). It
 * lands at the latest R blocks after the first block that observed it, which is the first block
 * that begins after the call: R quanta are 50 ms rounded up to a whole quantum, so the old
 * position plays for less than 50 ms plus one quantum after that block, plus the part of a block
 * between the call and it. Blocks of g with no PCM queued then render silence and count as
 * underrun frames. Old audio that runs out before
 * the seek lands renders silence counted as held frames, not underrun. So a host submits g from
 * F at once after the call; the seek report tells where it landed. The refusals,
 * all MISO_ENGINE_V1_INVALID_ARGUMENT: g not above the source's generation
 * ("source.generation.stale"), g = 0 ("source.generation.zero"), F outside the source's region
 * ("source.region.outside"), an unknown source ("source.id.unknown"). Where the seek landed is
 * the seek report (below). Like every session call, a seek runs the service step first (see
 * "Control service").
 *
 * Anchored seek. miso_engine_v1_source_seek_at(id, g, F, A) is the seek with an anchor: F enters
 * the graph in the block that starts at sample A of the source-read clock, and the output hears
 * it the plan's latency later. A must be a multiple of the plan's quantum, or the call returns
 * MISO_ENGINE_V1_INVALID_ARGUMENT with "source.seek.anchor_unaligned". Its other checks and
 * results are the seek's, and it too switches the source to g at once, so the host submits g from
 * F before the block at A renders. A host checks MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT before
 * calling it.
 *
 * Session seek (issue 1642). The timeline is the session's playhead: it starts at 0 and advances
 * one quantum per render, and stored automation follows it. miso_engine_v1_session_seek(session,
 * g, T) moves it and every source of the newest committed session in one step, all or nothing:
 * the timeline goes to T and each source to min(T, its frames); every one of them takes the seek
 * in the same render block, or the call changes nothing and returns a refusal. The host then
 * submits generation g of each source from its frame. A plain session seek lands as a plain seek
 * does, for every source and the timeline in one block (issue 2205): the first block at which
 * every source that waits has its runway queued, or the deadline R blocks after the block that
 * observed it; until then every source plays its old generation on, and the timeline runs on.
 * That is a live seek. miso_engine_v1_session_seek_at_once(session, g, T) is the same seek for a
 * host whose transport is stopped (issue 2205): same arguments, checks, results and declaration,
 * but every source and the timeline land at the first block rendered after the call, without
 * waiting for a runway, so the stop position never plays after the host restarts. Use it while
 * stopped. PCM of the old generation still queued in a source's ring is dropped at that block, so
 * a host that filled its rings before the stop and cannot drain them while stopped underruns at
 * restart until it has submitted generation g from each source's frame. A host checks
 * MISO_ENGINE_V1_FEATURE_SESSION_SEEK_AT_ONCE before calling it. g must be above every generation the host
 * has used on the session, on any source or session seek; one above the largest it has used is
 * always valid. The refusals, all MISO_ENGINE_V1_INVALID_ARGUMENT unless named: g = 0
 * ("source.generation.zero"), g not above every generation the session has used
 * ("source.generation.stale"), T over the timeline limit ("source.seek.frame_out_of_range"), a
 * source whose producer the newest session does not hold yet ("source.ring.vacated"), and
 * MISO_ENGINE_V1_BACKPRESSURE with "source.seek.backpressure" while any source or the timeline
 * still holds an unpopped seek; MISO_ENGINE_V1_INTERNAL with "source.epoch" when the
 * session's plan epochs cannot be synchronized, or with "source.internal" for an engine invariant.
 * miso_engine_v1_session_seek_at(session, g, T, A) is the anchored form, with the anchor of
 * miso_engine_v1_source_seek_at: every consumer changes at the block at A, or, reached late, at
 * its frame plus the lateness; an A that is not a multiple of the quantum is
 * MISO_ENGINE_V1_INVALID_ARGUMENT with "source.seek.anchor_unaligned". A per-source seek leaves
 * the timeline where it is. A plain session seek is a declared discontinuity; the engine declares
 * it in the same call, and a refused seek declares nothing: every check runs first, then the
 * declaration (miso_engine_v1_declare_discontinuity's, all or nothing), then the seek. A refused
 * declaration changes nothing and returns its own result and diagnostic, as
 * miso_engine_v1_declare_discontinuity returns them (MISO_ENGINE_V1_COMPILE_REJECTED,
 * MISO_ENGINE_V1_BACKPRESSURE or MISO_ENGINE_V1_INTERNAL). An anchored session seek keeps
 * continuity: it declares nothing, so its effect tails and latency floors stay (it is the loop
 * and exact-start form); a host that wants the floors reset declares at a stop. A seek of every
 * source made with per-source seeks is still the host's own call, and the host declares before
 * it. A host checks MISO_ENGINE_V1_FEATURE_SESSION_SEEK before calling either symbol.
 *
 * Session replace (#1386). miso_engine_v1_submit_document_command is
 * miso_engine_v1_submit_command plus a session document borrowed beside the frame for this call
 * only, under miso_engine_v1_compile_session's pointer-plus-length rule: document may be null
 * only when document_bytes is 0, else MISO_ENGINE_V1_INVALID_ARGUMENT. The library keeps nothing
 * of it. A SESSION_REPLACE (0x0013) frame names the document by its exact length and its
 * SHA-256; the engine checks both, bounds the document by the compile limits'
 * maximum_document_bytes (not by the control frame), parses and validates it as a boot would,
 * diffs it against the committed session and applies the diff as the one transaction it is: the
 * same revision, edit path, SESSION_COMMITTED event and rendered blocks as that
 * SESSION_TRANSACTION_APPLY. A document equal to the committed session replies edit path 5,
 * unchanged, and commits nothing. A document refusal is a non-OK reply frame
 * (session.replace.document_length, session.replace.document_too_large,
 * session.replace.document_digest, session.replace.document_utf8, or VALIDATION_FAILED with the
 * session's diagnostics) and changes nothing; a SESSION_REPLACE through
 * miso_engine_v1_submit_command is the document_length refusal. Every other frame behaves as it
 * does through miso_engine_v1_submit_command. A replayed request answers its cached reply without
 * reading the document. A host checks MISO_ENGINE_V1_FEATURE_SESSION_REPLACE before calling the
 * symbol.
 *
 * Choosing the anchor. To start a stem that a transaction added, after the command returns OK:
 * pick A, a quantum multiple past the last rendered block by enough quanta to cover the time the
 * host needs to decode generation g from F and submit it (a few quanta); call
 * miso_engine_v1_source_seek_at(id, g, F, A), where g is above the source's generation (2 for a
 * just-added source) and F is the source frame the playing stems read at A; then submit g from F.
 * Until A the stem renders silence; from A it plays F on. A track the transaction added fades in
 * from the first block its stem plays (A here) over the session mute ramp (control_smoothing
 * mute_ms), D samples later when latent effects or delay compensation sit between the track's
 * input and its fader, so it never enters as a step (#1288); a muted channel stays muted, a mute
 * the host sends before the fade cancels it, and an unmute (the track is unmuted already) leaves
 * the fade waiting. A route the transaction adds between two strips that keep playing, and a
 * send of an added track from a tap before its fader, fade in over the same ramp (#1363), at
 * adoption or with that track's fader; a removed or re-pointed route ramps out first (issue 1953),
 * or, when it is a post_fader or post_pan route into an output, from the adoption. If g's
 * PCM is not queued when the block
 * at A renders, the seek applies there anyway: the blocks without PCM count as underrun frames,
 * and PCM that falls behind the read position is discarded. A playing stem's seek report gives
 * that frame: a stem whose report reads (generation, s, f) reads source frame f + (A - s) at A,
 * until its next seek and inside its region.
 *
 * Held. From the block that pops an anchored seek until the block that applies it, the seek is
 * held. Queued PCM of the playing generation plays meanwhile (the host cannot add to it, because
 * the call switched the producer to g); when that runs out the source renders silence, and the
 * frames count in the report's cumulative_held_frames, not in cumulative_underrun_frames. A
 * planned wait is not starvation. A block without PCM while no seek is held is an underrun.
 *
 * Replacement. A newer seek, plain or anchored, replaces a held one: the held seek never applies,
 * and the PCM already submitted for its generation is discarded. A plain seek holds in its place
 * until it is ready, as above; an anchored one holds in its place with its own anchor.
 *
 * One seek in flight. A source's command queue holds one seek. A seek call that arrives before
 * render has popped the previous one, which happens when render begins a block, returns
 * MISO_ENGINE_V1_BACKPRESSURE with "source.seek.backpressure" and changes nothing; the host
 * retries after a block has begun, and a host that renders nothing keeps getting it. A held seek
 * has been popped, so it does not block the next one.
 *
 * Seeks across a structural transaction (issues 1273, 1310). A source that a replacement plan
 * keeps unchanged keeps its seek state with its ring: a seek queued and not yet popped, and a
 * held seek, move to the new plan, and a held anchor stays on the continuing clock, so it applies
 * in the anchor block of the new plan or, if that block has passed, as a late seek (below). The
 * host does not repeat the seek. The same holds when a transaction supersedes an unadopted
 * replacement. A source that the transaction added or changed restarts in a new ring at
 * generation 1, frame 0, with its seek report at generation 0: a seek held or queued on the old
 * ring is discarded with it, and the host seeks the new one after the command returns OK. A seek
 * on a source that the transaction removed is accepted until the replacement is adopted and
 * refused as "source.id.unknown" from then on (issue 1325).
 *
 * Far past anchors. An anchor that render has already passed is not refused. The seek applies no
 * later than the first block that begins after the call returns, at source frame F plus the
 * lateness, the number of samples from A to that block's first sample, so the stem stays in time;
 * a replacement plan adopted only after A does the same in its first block. If F plus the
 * lateness lies past the end of a region that the host closed with end_of_region, the block
 * renders silence as end of region: neither an underrun nor held.
 *
 * Rate. The lateness is added to F in source frames, one for one with samples of the source-read
 * clock. That is exact because every chunk must be at the session's sample rate (refused as
 * "source.rate.mismatch" otherwise); the engine converts no rates, so a host that decodes at
 * another rate resamples before it submits.
 *
 * Where a seek landed (issue 1316). miso_engine_v1_source_seek_report(session, id, out) copies
 * what render last published for the source of the newest committed session into out, a
 * miso_engine_v1_seek_report (MISO_ENGINE_V1_SOURCE_SEEK_REPORT_SIZE bytes; the caller sets
 * struct_size and zeroes reserved0 and reserved[]). generation is the generation in effect, 0
 * until the source's ring has rendered its first block; first_sample is the source-read sample of
 * the first block in which that generation was in effect, and source_frame the source frame that
 * block started at, for an anchored seek that render reached late F plus the lateness; so the
 * source plays frame source_frame + (t - first_sample) in the block at t, until its next seek and
 * inside its region.
 * cumulative_underrun_frames and cumulative_held_frames count the source frames that blocks
 * without PCM covered since the ring was made, split as under "Held"; a seek does not reset them.
 * A plain seek lands in the block that applies it, and the report is how a host learns which: it
 * keeps the previous generation until that block has rendered. The report is a level that render
 * overwrites, not a queue. The ring owns it, so a swap that carries the source keeps it, and a
 * source that a transaction added or changed starts at generation 0. The call returns
 * MISO_ENGINE_V1_OK; MISO_ENGINE_V1_INVALID_ARGUMENT for a wrong struct_size, a nonzero reserved
 * word, a malformed source ID ("source.id.invalid"), or an undeclared source
 * ("source.id.unknown"); and
 * MISO_ENGINE_V1_BACKPRESSURE with "source.report.busy", out untouched, when every bounded read
 * (64) landed inside render's publication, so the host reads again. A host checks
 * MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_REPORT before calling it.
 */

/*
 * ABI growth. MISO_ENGINE_V1_ABI_VERSION changes only on a break. Everything else grows in place:
 *
 *   - Each added exported symbol, and each former reserved struct word that gets a name, gets its
 *     own MISO_ENGINE_V1_FEATURE_* bit in miso_engine_v1_capabilities.feature_mask: the next free
 *     one when it merges, never a reused one.
 *   - A new value of an existing enum or result code, and a field added in place to a V1 protocol
 *     message, takes no bit. So a host tolerates an enum value it does not know, and treats a
 *     result code it does not know as a failure.
 *   - A feature added as a protocol message is announced by its protocol capability flag in the
 *     CAPABILITIES reply, not by a feature bit. The control protocol registry, not this rule,
 *     governs protocol flags.
 *
 * A host tests each bit it needs with &, tolerates bits it does not know (a newer library reports
 * more bits than the header the host was built with), and never compares feature_mask with ==.
 * MISO_ENGINE_V1_FEATURE_MASK is every bit this header defines; it is not a value to compare the
 * report with. A feature bit protects only a host that resolves the symbol at run time: a host
 * that links a symbol directly fails to load against a library that lacks it, before it can test
 * any bit.
 */

#define MISO_ENGINE_V1_ABI_VERSION UINT32_C(0x00010000)

#define MISO_ENGINE_V1_OK UINT32_C(0)
#define MISO_ENGINE_V1_INVALID_ARGUMENT UINT32_C(1)
#define MISO_ENGINE_V1_ABI_MISMATCH UINT32_C(2)
#define MISO_ENGINE_V1_WRONG_HANDLE UINT32_C(3)
#define MISO_ENGINE_V1_BUFFER_TOO_SMALL UINT32_C(4)
#define MISO_ENGINE_V1_COMPILE_REJECTED UINT32_C(5)
#define MISO_ENGINE_V1_BACKPRESSURE UINT32_C(6)
#define MISO_ENGINE_V1_UNSUPPORTED UINT32_C(7)
#define MISO_ENGINE_V1_RENDER_REJECTED UINT32_C(8)
#define MISO_ENGINE_V1_INTERNAL UINT32_C(255)

#define MISO_ENGINE_V1_EVENT_LANE_RELIABLE UINT32_C(0)
#define MISO_ENGINE_V1_EVENT_LANE_LOSSY UINT32_C(1)

#define MISO_ENGINE_V1_TAIL_FINITE UINT64_C(0)
#define MISO_ENGINE_V1_TAIL_INFINITE UINT64_C(1)

#define MISO_ENGINE_V1_RATE_44100 UINT64_C(1)
#define MISO_ENGINE_V1_RATE_48000 UINT64_C(2)
#define MISO_ENGINE_V1_RATE_88200 UINT64_C(4)
#define MISO_ENGINE_V1_RATE_96000 UINT64_C(8)
#define MISO_ENGINE_V1_EXACT_LAUNCH_RATE_MASK UINT64_C(15)

#define MISO_ENGINE_V1_FEATURE_IMMUTABLE_SESSION UINT64_C(1)
#define MISO_ENGINE_V1_FEATURE_HOST_PLANAR_SOURCE UINT64_C(2)
#define MISO_ENGINE_V1_FEATURE_SOURCE_SEEK UINT64_C(4)
#define MISO_ENGINE_V1_FEATURE_PLANAR_STEREO_RENDER UINT64_C(8)
#define MISO_ENGINE_V1_FEATURE_CAPABILITY_COMMAND UINT64_C(16)
#define MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT UINT64_C(32)
#define MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK UINT64_C(64)
#define MISO_ENGINE_V1_FEATURE_SERVICE UINT64_C(128)
#define MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY UINT64_C(256)
#define MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_REPORT UINT64_C(512)
#define MISO_ENGINE_V1_FEATURE_SESSION_SEEK UINT64_C(1024)
#define MISO_ENGINE_V1_FEATURE_SESSION_REPLACE UINT64_C(2048)
#define MISO_ENGINE_V1_FEATURE_SESSION_SEEK_AT_ONCE UINT64_C(4096)
#define MISO_ENGINE_V1_FEATURE_MASK UINT64_C(8191)

#define MISO_ENGINE_V1_OUTCOME_EXACT UINT64_C(1)
#define MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK UINT64_C(2)
#define MISO_ENGINE_V1_OUTCOME_SUPERSEDED UINT64_C(4)

#define MISO_ENGINE_V1_ENGINE_CONFIG_SIZE UINT32_C(40)
#define MISO_ENGINE_V1_COMPILE_LIMITS_SIZE UINT32_C(208)
#define MISO_ENGINE_V1_BYTES_OUT_SIZE UINT32_C(32)
#define MISO_ENGINE_V1_SOURCE_CHUNK_SIZE UINT32_C(48)
#define MISO_ENGINE_V1_SUBMIT_REPORT_SIZE UINT32_C(32)
#define MISO_ENGINE_V1_PLANAR_OUTPUT_SIZE UINT32_C(48)
#define MISO_ENGINE_V1_CAPABILITIES_SIZE UINT32_C(56)
#define MISO_ENGINE_V1_PLAN_RESOURCE_REPORT_SIZE UINT32_C(240)
#define MISO_ENGINE_V1_WATERMARK_SIZE UINT32_C(96)
#define MISO_ENGINE_V1_SOURCE_SEEK_REPORT_SIZE UINT32_C(64)

typedef struct miso_engine_v1_engine miso_engine_v1_engine;
typedef struct miso_engine_v1_session miso_engine_v1_session;
typedef struct miso_engine_v1_plan miso_engine_v1_plan;

typedef struct miso_engine_v1_engine_config {
    uint32_t struct_size;
    uint32_t abi_version;
    uint64_t reserved[4];
} miso_engine_v1_engine_config;

typedef struct miso_engine_v1_compile_limits {
    uint32_t struct_size;
    uint32_t source_ring_frames;
    /* Formerly maximum_automation_spans_per_block; must be zero in ABI V1;
       the layout is unchanged. */
    uint32_t reserved1;
    uint32_t reserved0;
    uint64_t maximum_document_bytes;
    uint64_t maximum_diagnostic_bytes;
    uint64_t maximum_tracks;
    uint64_t maximum_sources;
    uint64_t maximum_routes;
    uint64_t maximum_effects;
    /* Sizing rule for the resource caps from here to maximum_capi_retained_bytes (#1398). A
       structural transaction is admitted against every plan the session still holds plus its
       own: the running plan, and at most a withdrawn candidate and its successor, so up to three
       plans live at once (a plan render has just displaced also counts until a later call
       reclaims it; a refusal only it causes is MISO_ENGINE_V1_BACKPRESSURE). Set each of these
       caps to at least three times one plan's row (the
       matching field of miso_engine_v1_plan_resource_report, for the largest session the host
       will compile, read while a successor plan of it runs: such a plan carries its sources and
       owns their rings), and maximum_graph_session_plus_plan_bytes to that plus three compiled
       models. A cap below that refuses a transaction before it commits, and nothing is
       acknowledged: with MISO_ENGINE_V1_BACKPRESSURE when the transaction fit against the
       newest held plan alone, so a retry after a render normally commits (the split is
       approximate: such a retry may still be refused within a small band), else with
       MISO_ENGINE_V1_COMPILE_REJECTED and a resource diagnostic. */
    uint64_t maximum_graph_session_plus_plan_bytes;
    uint64_t maximum_source_total_bytes;
    uint64_t maximum_source_overhead_bytes;
    uint64_t maximum_effect_state_bytes;
    uint64_t maximum_effect_scratch_bytes;
    uint64_t maximum_builtin_retained_bytes;
    uint64_t maximum_capi_retained_bytes;
    uint64_t maximum_named_allocation_bytes;
    /* Every plan carries the engine's meters (#1581): one post-matrix sample-peak meter stream
       per strip (tracks and submixes) and, from #1581's second slice, one for the output. These
       three caps bound them like any other meter, so they must admit one meter stream per strip
       plus the output, at the default window of about 32 ms; a cap below that refuses the
       compile with MISO_ENGINE_V1_COMPILE_REJECTED and the diagnostic builtin.resource.limit
       $.meter_requests. */
    uint64_t maximum_meter_streams;
    uint64_t maximum_meter_items;
    uint64_t maximum_meter_bytes;
    /* Largest control frame, in bytes. It also bounds the observation taps a
       session can keep armed: one commit that ends them all reports them in one OBSERVATION_ENDED
       event, which must fit this frame, so a subscription that would outgrow it is refused (#1685).
       That event is 64 bytes plus, per armed tap, 64 bytes + pad8(strip ID bytes) + pad8(effect ID
       bytes), where pad8 rounds up to a multiple of 8. Example: 64 insert meters whose strip and
       effect IDs are 8 bytes or shorter need 64 + 64 * 80 = 5,184 bytes (about 5.1 KiB); with
       16-byte IDs, 6,208 bytes. The browser uses 16 KiB. */
    uint64_t maximum_control_frame_bytes;
    uint64_t maximum_replay_bytes;
    uint64_t maximum_replay_entries;
    /* Maximum session submix strips. Zero means "use maximum_tracks"; it never means
       "no submixes" or "unbounded". Formerly reserved[0]; the layout is unchanged. */
    uint64_t maximum_submixes;
    /* Maximum session VCA groups. Zero means "use maximum_tracks"; it never means
       "no VCAs" or "unbounded". Formerly reserved[1] of the original reserved[4]
       (offset 184); the layout is unchanged. */
    uint64_t maximum_vcas;
    uint64_t reserved[2]; /* Must be zero in ABI V1. */
} miso_engine_v1_compile_limits;

typedef struct miso_engine_v1_bytes_out {
    uint32_t struct_size;
    uint32_t reserved0;
    uint8_t *data;
    uint64_t capacity_bytes;
    uint64_t required_bytes;
} miso_engine_v1_bytes_out;

typedef struct miso_engine_v1_source_chunk {
    uint32_t struct_size;
    uint32_t sample_rate_hz;
    uint64_t generation;
    uint64_t start_frame;
    const float *const *planes;
    uint32_t plane_count;
    uint32_t frames;
    uint32_t end_of_region;
    uint32_t reserved0;
} miso_engine_v1_source_chunk;

typedef struct miso_engine_v1_submit_report {
    uint32_t struct_size;
    uint32_t reserved0;
    uint64_t accepted_frames;
    uint64_t cumulative_written_frames;
    uint64_t active_generation;
} miso_engine_v1_submit_report;

typedef struct miso_engine_v1_planar_output {
    uint32_t struct_size;
    uint32_t channels;
    float *samples;
    uint64_t sample_capacity;
    uint32_t frames;
    uint32_t plane_stride_samples;
    uint64_t reserved[2];
} miso_engine_v1_planar_output;

typedef struct miso_engine_v1_capabilities {
    uint32_t struct_size;
    uint32_t abi_version;
    uint64_t exact_launch_rate_mask;
    uint64_t feature_mask;
    uint64_t reserved[4];
} miso_engine_v1_capabilities;

typedef struct miso_engine_v1_plan_resource_report {
    uint32_t struct_size;
    uint32_t abi_version;
    uint32_t sample_rate_hz;
    uint32_t quantum_frames;
    uint64_t source_count;
    uint64_t track_count;
    uint64_t latency_samples;
    uint64_t tail_kind;
    uint64_t tail_samples;
    uint64_t graph_session_plus_plan_bytes;
    uint64_t graph_incremental_plan_bytes;
    uint64_t graph_metadata_bytes;
    uint64_t graph_delay_bytes;
    uint64_t effect_bank_scratch_bytes;
    uint64_t effect_bank_runtime_buffer_bytes;
    uint64_t effect_bank_metadata_bytes;
    uint64_t builtin_bank_bytes;
    uint64_t builtin_bank_scratch_bytes;
    uint64_t source_pcm_payload_bytes;
    uint64_t source_overhead_bytes;
    uint64_t source_total_bytes;
    uint64_t effect_scalar_state_bytes;
    uint64_t effect_scalar_scratch_bytes;
    uint64_t builtin_processor_payload_bytes;
    uint64_t builtin_meter_payload_bytes;
    uint64_t builtin_retained_payload_bytes;
    uint64_t capi_retained_bytes;
    uint64_t largest_named_allocation_bytes;
    uint64_t reserved[4];
} miso_engine_v1_plan_resource_report;

typedef struct miso_engine_v1_watermark {
    uint32_t struct_size;
    uint32_t reserved0; /* Must be zero in ABI V1. */
    uint64_t revision;
    uint64_t first_sample;
    uint64_t outcome_flags;
    uint64_t exact_count;
    uint64_t transition_fallback_count;
    uint64_t superseded_count;
    uint64_t reserved[5]; /* Must be zero in ABI V1. */
} miso_engine_v1_watermark;

typedef struct miso_engine_v1_seek_report {
    uint32_t struct_size;
    uint32_t reserved0; /* Must be zero in ABI V1. */
    uint64_t generation;
    uint64_t first_sample;
    uint64_t source_frame;
    uint64_t cumulative_underrun_frames;
    uint64_t cumulative_held_frames;
    uint64_t reserved[2]; /* Must be zero in ABI V1. */
} miso_engine_v1_seek_report;

uint32_t miso_engine_v1_abi_version(void);
uint32_t miso_engine_v1_query_capabilities(miso_engine_v1_capabilities *out);
uint32_t miso_engine_v1_engine_create(const miso_engine_v1_engine_config *config,
                                      miso_engine_v1_engine **out_engine);
void miso_engine_v1_engine_destroy(miso_engine_v1_engine *engine);
uint32_t miso_engine_v1_compile_session(miso_engine_v1_engine *engine,
                                        const uint8_t *document,
                                        uint64_t document_bytes,
                                        const miso_engine_v1_compile_limits *limits,
                                        miso_engine_v1_bytes_out *diagnostics,
                                        miso_engine_v1_session **out_session,
                                        miso_engine_v1_plan **out_plan);
uint32_t miso_engine_v1_source_submit_planar_f32(miso_engine_v1_session *session,
                                                 const uint8_t *source_id,
                                                 uint64_t source_id_bytes,
                                                 const miso_engine_v1_source_chunk *chunk,
                                                 miso_engine_v1_submit_report *out_report);
uint32_t miso_engine_v1_source_seek(miso_engine_v1_session *session,
                                    const uint8_t *source_id,
                                    uint64_t source_id_bytes,
                                    uint64_t generation,
                                    uint64_t source_frame);
uint32_t miso_engine_v1_source_seek_at(miso_engine_v1_session *session,
                                       const uint8_t *source_id,
                                       uint64_t source_id_bytes,
                                       uint64_t generation,
                                       uint64_t source_frame,
                                       uint64_t anchor_sample);
/* The reserved source_id "$timeline" (9 bytes) reads the session timeline's seek report: its
 * source_frame is the timeline sample that block started at, and its underrun and held counts are
 * 0. Only this call takes it; submit, seek and seek_at refuse it as an unknown source. */
uint32_t miso_engine_v1_source_seek_report(miso_engine_v1_session *session,
                                           const uint8_t *source_id,
                                           uint64_t source_id_bytes,
                                           miso_engine_v1_seek_report *out);
uint32_t miso_engine_v1_session_seek(miso_engine_v1_session *session,
                                     uint64_t generation,
                                     uint64_t timeline_sample);
uint32_t miso_engine_v1_session_seek_at(miso_engine_v1_session *session,
                                        uint64_t generation,
                                        uint64_t timeline_sample,
                                        uint64_t anchor_sample);
uint32_t miso_engine_v1_session_seek_at_once(miso_engine_v1_session *session,
                                             uint64_t generation,
                                             uint64_t timeline_sample);
uint32_t miso_engine_v1_submit_command(miso_engine_v1_session *session,
                                       const uint8_t *request,
                                       uint64_t request_bytes,
                                       miso_engine_v1_bytes_out *response);
uint32_t miso_engine_v1_submit_document_command(miso_engine_v1_session *session,
                                                const uint8_t *request,
                                                uint64_t request_bytes,
                                                const uint8_t *document,
                                                uint64_t document_bytes,
                                                miso_engine_v1_bytes_out *response);
uint32_t miso_engine_v1_dequeue_event(miso_engine_v1_session *session,
                                      uint32_t lane,
                                      miso_engine_v1_bytes_out *event);
uint32_t miso_engine_v1_service(miso_engine_v1_session *session);
uint32_t miso_engine_v1_declare_discontinuity(miso_engine_v1_session *session);
uint32_t miso_engine_v1_render_f32_planar(miso_engine_v1_plan *plan,
                                          uint64_t absolute_sample,
                                          const miso_engine_v1_planar_output *output);
uint32_t miso_engine_v1_plan_resources(const miso_engine_v1_plan *plan,
                                       miso_engine_v1_plan_resource_report *out);
uint32_t miso_engine_v1_plan_watermark(const miso_engine_v1_plan *plan,
                                       miso_engine_v1_watermark *out);
uint32_t miso_engine_v1_last_error(const void *live_handle, miso_engine_v1_bytes_out *out);
void miso_engine_v1_session_destroy(miso_engine_v1_session *session);
void miso_engine_v1_plan_destroy(miso_engine_v1_plan *plan);

#ifdef __cplusplus
}
#endif

#endif
