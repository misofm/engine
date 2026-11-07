# Web host: an instance can be both the control instance and a render-only one, and the render half's place is stored four ways

Child of #2258 (implicit state machines), item WEB-8. Found by the audit of 2026-11-06 and re-verified at base `69e5f69cf`.

## Mission
Give the web host one representation of where a host's render half is, and one of what an instance is. Make the illegal combinations unrepresentable where a type can do it, and refuse at the export the one combination that spans two thread-locals.

## Verified state (base `69e5f69cf`)

### Where a host's render half is: four fields, consistent today
Four fields in `hosts/host-web/src/lib.rs` say whether the render half is on this thread:
- `RenderHalf::released` (line 1190). The status shell sets it; the real half never does.
- `RenderHalf::ready` (line 1186). It is `None` on the shell and on a disposed half.
- `AudioWorkletEngineHost::spare_shell` (line 1292). It is `Some` while the half is here and `None` while it is away.
- `ControlHalf::render_here` (line 981). Only the blocking-rebuild count reads it (line 1126).

`render_here()` (line 2841) reads `!render.released`. Only `release_render` (2791), `adopt_render` (2813), `dispose` (2742) and the boot (1444-1467) write these fields. `release_render` writes three of them at once and `adopt_render` writes them back. The reachable states are:

| State | `render.released` | `render.ready` | `spare_shell` | `render_here` |
|---|---|---|---|---|
| booted, half here | false | Some | Some | true |
| half away | true | None | None | false |
| disposed, half here | false | None | Some | (no control half) |
| disposed, half away | true | None | None | (no control half) |

Every path the brief names keeps to that table:
- **Release.** `release_render` returns `None`, changing nothing, unless `ready` is `Some`. `ready` is `Some` only when the half is here and the session has not been disposed, so `spare_shell` and the control half are both there.
- **A release into a full registry.** It is adopted back at once (`ffi.rs` 1668-1679).
- **Adopt, and a failed adoption.** `adopt_render` checks everything before it writes anything (2814-2820). A refused adoption hands the half back untouched, and the registry deposits it again under its token (`handoff.rs` `take`).
- **Dispose with the half away.** `dispose` writes only the shell's status and drops the control half. A later `adopt_render` is refused because there is no control half. At the ABI, `miso_engine_web_v1_dispose` takes the whole `LiveHost` out of the slot, so a disposed host never stays live. The orphaned half then comes back as a render-only session (#1332 S8).
- **A trap mid-release.** A trap leaves the instance's `LIVE_HOST` `RefCell` borrowed for good, so every later export on that instance refuses the borrow. The four fields stay as the trap left them, and no export reads them again. The registry steps (`deposit`, `take`) are CAS sequences with no call that can trap between their two stores.

So no combination outside the table is reachable, and no red-on-base test can show one. The encoding is still the tracker's pattern 2: three hand-kept copies of one fact, written in the same order at two sites, and nothing but review stops a new site from writing two of them. The shell is a whole `RenderHalf`, allocated at boot (line 1467), that can never render. Its `ready: None` and its `released` flag together stand for "the session renders elsewhere". `RenderHalf::render_next` (3241) and `reject_output_quantum` (3213) each test `released` to tell that shell apart from a half with a broken invariant.

### What an instance is: reachable mismatches (red on base)
`LiveHost { handle, host: Option<Box<AudioWorkletEngineHost>>, render: Option<RenderHost> }` (`ffi.rs` 74-80) encodes the session an instance holds. `(Some, Some)` and `(None, None)` are never constructed. Only the boot and the adoption create a `LiveHost`, and `with_render_mut`/`with_render` (286-290, 310-325) keep an `(None, None) => invalid` arm for a state that cannot occur.

The instance's role is separate. `handoff::instance_bound()` (`handoff.rs` 314) is a thread-local set by `miso_engine_web_v1_instance_bind`. Its contract (line 311): a bound instance is "a render-only instance (the worklet), which never frees a render half"; "the instance that booted the host (the Worker) binds none". Dispose relies on that contract (`ffi.rs` 1607-1613). It frees an adopted half only on an unbound instance.

Nothing ties the two together, and two orders reach a mismatch through the exports:
1. **Bind after a boot.** `miso_engine_web_v1_instance_bind` (`ffi.rs` 1785) never looks at `LIVE_HOST`. An instance holding a booted host can bind a reservation, and it is then bound while it holds a host. That host's dispose frees the plan, the compiled session and the rings on an instance whose role says it never frees. An orphan that this instance later adopted could never be freed: its dispose is refused `RESULT_WRONG_STATE`.
2. **Boot after a bind.** `boot_staged` (`ffi.rs` 700) checks only whether a session is live. A bound render-only instance boots a host, which compiles and allocates on the render thread's instance. Its dispose then frees there.

The shipped JavaScript does neither. The worker-mode worklet binds and adopts, and never boots. The single-mode worklet and the control Worker boot, and never bind. Both orders are still reachable through the module's exports, and each breaks the bound role's contract.

**Severity L** (the tracker's). The four-field location is consistent today. The role mismatch needs export calls that the shipped pages never make.

## Decision / model
- **`enum RenderSlot` replaces `render`, `spare_shell`, `RenderHalf::released` and `ControlHalf::render_here`.** It has two variants:
  - `Here(Box<RenderHalf>)`: the half is on this thread.
  - `Away(WebStatus)`: the half has been released. This is the status shell, inline in the host, and it is what the status pointer names while the half is away.

  The shell's status is the half's status, copied at the release. From then on, `record` writes the lifecycle state and the control half's results into it, as before. A `RenderHalf` is always a real half, so it loses its `released` field and the `shell` constructor. `render_here()` is `matches!(Here)`.
- **One discriminant for every reader.** The render-side host calls (`render_next`, `poll_meters`, `set_meter_lease`, `reject_output_quantum`) match the slot. `Here` runs the half's call, as before. `Away` refuses exactly as the shell did:
  - `render_next` returns `RESULT_WRONG_STATE` and records it in the shell's status;
  - the lease and the shape check return `RESULT_WRONG_STATE`;
  - `poll_meters` returns `0`.
- **The blocking-rebuild count.** It takes "the half is here" as an argument from the host's slot, not from a copy. `run_command` uses `plan.is_some()`, since the plan is lent exactly when the half is here.
- **Release and adoption still allocate and free nothing.** The release moves the half's box out of the slot with `mem::replace`, putting in `Away(status)`, after a check that the slot is `Here` with a session. The adoption puts `Here` back after a check that the slot is `Away`. The displaced value is the `WebStatus`, which is `Copy`, and it is forgotten, so no drop glue joins either call.
- **`impl RenderSlot` sits below `impl RenderReady` and `impl RenderHalf`.** The call-graph gate pins `call_indirect` sites by mangled name, and those names carry the impl block's index (`Ms2`, `Ms3`). Placed above them, the new impl renumbers both. The sites and counts stay the same, but the pin then needs an edit. The shell is no longer a second boxed `RenderHalf` allocated at boot. The boot's resource projection counts one render half, and the host now carries the inline status. `bridgeMetadataBytes` is a ceiling row in `expected.json`, so it is not re-pinned.
- **`enum LiveHost` replaces the two options.** Its variants:
  - `Booted { handle, host }`;
  - `Adopted { handle, render }`.

  The unreachable `(None, None)` arms go. Release, adoption and dispose match the variant.
- **An instance is the control instance or a render-only one, never both.** The role lives in two thread-locals, `LIVE_HOST` and `handoff`'s `BOUND_CANARY`. The render export reads the canary while it holds the `LIVE_HOST` borrow, so one type over both would mean restructuring that export. Instead, the two exports that set each one refuse the other's state:
  - `miso_engine_web_v1_instance_bind` refuses an instance that holds a session with `RESULT_WRONG_STATE`, and binds nothing. Its native test twin shares the one body.
  - `miso_engine_web_v1_boot` on a bound instance is refused `RESULT_REFUSED_LIFECYCLE`. That is the same code and staging record as a boot on an instance whose session is live.

  So a bound instance only ever holds `Adopted`. An unbound one holds `Booted`, or `Adopted` for an orphan it will free.
- **Cross-instance state is unchanged.** The handoff registry, the lifecycle word and the shared words are not touched. `RenderSlot` and `LiveHost` are per instance. So no loom model is added. The registry's CAS protocol is the same code, and the lifecycle word keeps `lifecycle_loom` (#2298).
- **Realtime.** The render-path change is control flow only. The host's render calls branch on the slot's discriminant instead of on `ready`/`released`. No arithmetic, buffer or plan code moves. A one-time no-bit-moved comparison against base is PR evidence.

## Non-goals
- **The JavaScript under `hosts/host-web/web/`.** #2317 is parked on it. One finding, for the tracker: the control Worker caches the status pointer before its release (`miso-engine-v1-control-worker.js` 368, 374, 439). So its `readStatus` reads the released half's own status, which the worklet writes, and not the shell that `status_ptr` names after the release.
- **`RenderHalf::ready` stays an `Option`.** A disposed half that is still here has no session. A third slot variant could carry that, but it would change the render half's own checks. It is a separate item if it is wanted.
- **Freeing a leaked orphan.** An orphan released after its Worker has booted a new host stays in the registry. That is not a representation problem.
- **Other tracker items.**

## Gates
- A regression test, `an_instance_is_the_control_instance_or_a_render_only_one_never_both` in `hosts/host-web/tests/worker_handoff.rs`, red on base.
- A one-time no-bit-moved comparison of the rendered output against base, across release, adoption, the failure paths and dispose. It is PR evidence, not a committed test.
- `cargo test --locked -p host-web` with and without `--all-features`.
- `cargo clippy --locked -p host-web --all-targets -- -D warnings`, with and without `--all-features`.
- `cargo fmt --check` and `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-web`.
- `bash scripts/check-workspace-policy.sh` and `cargo run --locked --release -p realtime-policy`.
- `bash scripts/test-web-audioworklet.sh`. Also the wasm artifact build and its gates (the call graph and the expected resources), if the pinned nightly and wabt are available.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `69e5f69cf`.

**Red on base.** The regression test was added to base with no other change. Its first half failed:

```
thread '<unnamed>' panicked at hosts/host-web/tests/worker_handoff.rs:769:17:
assertion `left == right` failed: an instance that holds a host stays the control instance
  left: 0
 right: 3
```

A probe then skipped that assertion: it bound a fresh reservation and printed the result, `0` (`RESULT_OK`). With the probe in place, the second half failed on base too:

```
assertion `left == right` failed: a bound instance boots no host
  left: 1
 right: 0
```

**The four-field location: proof, no red test.** See "Verified state". No reachable path leaves the table, so the change makes the illegal combinations unrepresentable. There is no regression test for it.

**Mutations of the fix.** Each was applied, run and restored.

| Mutation | Result |
|---|---|
| boot without the `handoff::instance_bound()` refusal | red: "a bound instance boots no host" (`1` vs `0`) |
| bind without the live-session refusal | red: "an instance that holds a host stays the control instance" (`0` vs `3`) |
| bind that calls `handoff::bind` before the refusal (a refused bind still binds) | red: "the refused bind left the reservation unbound" (`1` vs `0`) |
| `run_command` counts every published rebuild as blocking (`render_here = true`) | red: the round-trip test's "a rebuild published while render is elsewhere blocks nothing" (`Some(1)` vs `Some(0)`) |
| the `Away` arm of `render_next` answers `RESULT_OK` | red: the round-trip test's "the released host's render refuses" (`0` vs `3`) |
| `record` stops copying the lifecycle state into the `Away` shell | **green**: no test reads the shell's `state` after a failure. This gap was already on base, and no shipped page reads it (see the JS finding in Non-goals). It is listed as a follow-up, not tested here. |

**Test changes.**
- New: `an_instance_is_the_control_instance_or_a_render_only_one_never_both`.
- The round-trip test lost one assertion: "a second instance cannot bind a stack another instance runs on". Its Worker holds a host, so its bind is now refused before the registry's one-instance-per-stack CAS. The assertion moved to the new test, as its third instance, which holds no session and so reaches the CAS. The doc comment of `worker_handoff.rs` now counts three reservations in that binary.
- `render_failure_retains_ownership_and_silences` and two observation helpers in `src/tests.rs` read the half through `RenderSlot`.

**No bit moved.** A temporary probe, not committed, used the Rust API on the nine-track fixture (`fixtures/session/v1/parametric-eq-nine-track.json`, 48 kHz, 128-frame blocks, a fed ramp source). It ran eight scenarios: single-instance; release, render away and adopt back; and then, at block 4 of the released half, each of the following:
- `fail_session`;
- the half's `reject_output_quantum(64)`;
- the half's canary `fail`;
- dispose with the half away;
- `fail_session`, then the half's `fail`.

The eighth scenario was dispose with the half here. On the released host, every scenario also called `render_next`, `poll_meters`, `set_meter_lease`, `reject_output_quantum`, `output_pcm` and `meters_attached`.

The probe wrote each block's output-bit hash and each call's return code. It also wrote the full status after each step: state, last result, blocking rebuilds, clock, quanta and watermark. Base and fix produced byte-identical transcripts: SHA-256 `632fc6ce9c2af72fedbe88db05898344e6930b5b471555717672b8f8f4f3361d` (320 lines, 73 block hashes).

The wasm artifact's direct oracle, run by `check-browser-expected-resources.py --artifacts`, also matched `expected.json`'s PCM digests.

**Gates run (all green):**
- `cargo test --locked -p host-web --all-features` (198 tests) and `cargo test --locked -p host-web` (123);
- `cargo clippy --locked -p host-web --all-targets -- -D warnings`, with and without `--all-features`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-web`, with and without `--all-features`, and `cargo fmt --check`;
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`;
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (295 marked regions in 46 files)`;
- `bash scripts/test-web-audioworklet.sh`, exit 0.

The wasm artifact was built with `bash scripts/build-web-audioworklet.sh --named-twin` on the pinned `nightly-2026-08-20`. wabt 1.0.34 came from the noble deb, fetched with `apt download` and unpacked locally. The artifact gates then ran:
- `scripts/check-web-audioworklet.sh --without-metadata-regeneration` exited 0. The call graph's indirect sites match their pins, and `host_release` and `host_adopt` have closures of 2 and 1 with no allocator, free or drop glue.
- `scripts/check-browser-expected-resources.py --artifacts` exited 0. `bridgeMetadataBytes` is now 806,431 of its 893,120 ceiling, after losing the second boxed `RenderHalf`.
- `scripts/check-scalar-oracle-absent.py` exited 0.

One gate went red on the way. The first build placed `impl RenderSlot` above `impl RenderReady`. The call graph then reported `RenderReady::adopt_shape` and `RenderHalf::render_next` under `Ms3`/`Ms4` instead of the pinned `Ms2`/`Ms3`, with the same site counts. Moving the impl below them, which is the decision above, cleared it without a pin edit.

**Loom.** None added. No cross-instance state changed (see Decision).

**Test value.** `an_instance_is_the_control_instance_or_a_render_only_one_never_both` is red if an instance that holds a booted host can bind a reservation. That host's dispose would then free on a bound, render-only instance, and an orphan adopted there could never be freed. It is also red if a bound instance can boot a host: the preparation would allocate on the render thread's instance, and the dispose would free there. No existing test binds an instance that holds a session, or boots one that is bound.

**Follow-ups (not filed):**
1. The control Worker's `readStatus` reads the released half's status through a pointer cached before the release, not the shell (`miso-engine-v1-control-worker.js` 368-374, 439). That is JS, which #2317 owns.
2. No test reads the released host's shell `state` after a control failure (the green mutation row). Take it with item 1, which decides whether the shell's mirror of the lifecycle word has a reader at all.
3. `RenderHalf::ready: Option` still encodes "disposed while here". A `Disposed` variant would remove the last `ready: None` branches.
