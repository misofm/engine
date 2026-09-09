# Execute retained mapping evidence and deliver the artifact

Parent: #560 (lane B, CP8)

Predecessors: #669, #670, exhausted #672, #678, and #679

Coordination: #559

## Problem and smallest closable slice

#679 exhausted because its final executor swapped two words in the verifier
pathname and Python returned status 2 before loading verifier code. Its earlier
two production attempts stopped on verifier-only literal assumptions after each
had emitted the complete 77-file SDK inventory. The accepted product source,
six-file candidate artifact, eight successful retained gates and postchecks,
three-browser results, and preserved target were never invalidated or mutated.

This successor starts a fresh three-attempt workflow. Its smallest closable slice
is one correct execution of the already static-reviewed content-addressed
verifier, a self-excluding evidence manifest, Astra LOW evidence review, and a
separately reviewed promotion/delivery. It does not repair or rerun a product,
build, SDK, package, browser, resource, PCM, Cargo, npm, Node, or qualification
gate.

## Ownership and immutable inputs

Sol HIGH coordinates checkpoints, artifact qualification/pinning, GitHub
synchronization, PR, merge, and cleanup. Luna HIGH
`/root/issue583_luna_impl` is the sole attempt-1 executor. Astra LOW performs
scope, evidence, promotion, exact-head/current-main, and delivery review. #680
is lane B's sole active child; #559 owns coordination only.

Preserve all predecessor worktrees, branches, commits, evidence, artifacts,
exports, dependency roots, targets, and #679 attempt records byte-for-byte. Do
not clean, reinstall, rebuild, recapture, regenerate, edit, or repair them.
Historical compiled-output hashes are stability anchors, not reconstructed
contemporaneous provenance. No `.ll`, `.s`, compiler stream, binary, target,
browser payload, generated SDK file, or raw evidence enters Git. Git receives
only issue decisions and, after a separate scope PASS, the three approved
pin/lineage edits.

Authority source commit is
`8708c9b998a484d49ccb17a803e79540ca13fcd6`. Accepted source hashes are:

- `crates/effect-contract/src/lib.rs`:
  `be709c2293b108feccfe14b0049c08e32d09ce61188a865dca59fa6cee185f98`;
- `crates/effect-package/src/wire.rs`:
  `9a4e833512ab8f70bf4804fc149bfe21e2cb568eb6f53a707c212529e7e66818`.

The candidate artifact root is `/tmp/issue672-attempt3-candidate-artifact` and
must contain exactly these six files and hashes:

- ABI JSON: `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`;
- host declaration: `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`;
- host JavaScript: `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`;
- worklet JavaScript: `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`;
- Wasm: `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`;
- parameter metadata: `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`.

The preserved target streamed digest is
`420c7c4db6427802163e3d08deb8022327722fadbc6bb4178f13212506257151`.
The retained gate root is `/tmp/issue678-attempt3-evidence`; all eight gate and
postcheck statuses previously returned 0, including Chromium 151.0.7922.34,
Firefox 153.0, WebKit 26.5, matrix/mutation checks, and SDK packaging.

## Executor lease reconciliation

Checkpoint `a7e573aa` transferred the attempt-1 lease after incorrectly observing
that Luna HIGH `/root/issue583_luna_impl` was absent. That executor was active and
its read-only preflight correctly stopped at status 1 because the concurrent
checkpoint advanced the authorization head from `b3dc8bd4`; stdout was empty and
the exact failed assertion was not captured. No preflight, control, evidence, or
manifest path was created, and neither the self-test nor production verifier ran,
so no attempt was consumed. The stale transfer is withdrawn and the sole lease
returns to `/root/issue583_luna_impl`. Verifier bytes, invocations, gates, and
stop conditions remain unchanged. Fresh Astra LOW exact-head scope review is
mandatory before the executor acts.

After that reconciliation, the superseded executor began its already-issued
preflight at `2026-09-09T06:28:03Z`. It exclusively created only
`/tmp/issue680-attempt1-preflight.txt`. The 891-byte partial record has mode
`0664`, SHA-256
`87527f86c0b85cd61a281614cee75dd4c9f98514c5c31cfedacb1fe90ae994f3`, and
records actual feature `1bfb9f26` instead of authorized `a7e573aa`. Execution
should have stopped at that mismatch. The improvised outer single-quoted shell
then misparsed its unescaped nested census heredoc and returned status 2. The
executor reported, but did not capture in the partial file, `import: command not
found` followed by a syntax error at `roots = (`. No finish/status footer,
census, or temporary/unlisted file exists. Evidence, manifest, and control paths
remain absent; neither verifier invocation ran.

Astra LOW returned **ATTEMPT 1 FAIL / consumed**. The exclusive preflight path
was created and the attempted wrapper ran, so it cannot be relabelled as a
no-attempt stop or continued through an `attempt1b` path. Preserve the partial
record byte-for-byte. It receives no qualification credit. Luna XHIGH
`/root/issue679_luna_verifier` is stopped from attempt execution and has no
production authority. Two attempts remain.

## Attempt 2 preparation and execution

Luna XHIGH `/root/issue679_luna_verifier` alone may prepare an external
attempt-2 runner at `/tmp/issue680-attempt2-runner-draft.py`. Preparation is not
attempt execution: it may create only that ordinary non-symlink draft and must
not invoke it, create any attempt-2 path, read retained evidence, or run a
verifier/product command. Luna HIGH `/root/issue583_luna_impl` remains the sole
eventual attempt-2 executor after all runner reviews and a fresh production
SCOPE PASS; the preparer receives no execution authority. The runner must use
direct Python argument arrays and file APIs; shell `eval`, shell `-c`, and nested
heredocs are forbidden. Before any output creation it must fail closed on
authorization-head, upstream, main, merge-base, executor lease, verifier,
preserved-authority, process-owner, and all fresh-path checks. A rejected head or
path must leave every attempt-2 path absent.

A concurrent preparation already issued to Luna HIGH `/root/issue583_luna_impl`
under feature `ad042156` completed after checkpoint `b7806a90` transferred the
preparer lease. It exclusively created the sole draft path and ran nothing. Root
sealed those exact bytes mode `0444`: 22,402 bytes, SHA-256
`856394652e028095f3b94b4c7e82f86d79a209a8861ff58ff768004ab5cc49f0`.
The proposed runner-control invocation is exactly
`python3 -B /tmp/issue680-attempt2-runner-draft.py --self-test`; the proposed
production invocation is exactly
`ISSUE680_EXECUTOR=/root/issue583_luna_impl python3 -B /tmp/issue680-attempt2-runner-draft.py`.
Neither is authorized yet. All attempt-2 and control paths remain absent.
Because the exclusive draft now exists, the transferred preparer must not
overwrite or append to it. Astra LOW must review this sealed candidate first.
Coordinator static inspection already flags two possible blockers for that
review: `AUTHORIZATION_HEAD` is hard-coded to pre-amendment `ad042156`, and
`target_identity` repeats the `tar` argument sequence.

Astra LOW returned **STATIC FAIL** on the sealed `85639465...5cc49f0`
candidate; it must remain immutable and must not be invoked. The stale hard-coded
authorization creates an amendment cycle, executor identity defaults instead of
being required, target hashing buffers and base64-records the archive, failures
after preflight creation escape the evidence record, manifest behavior is not a
fail-closed equivalent of the frozen command, controls exercise helpers instead
of the execution flow, the process check rejects unrelated Node/npm work, command
checks are tautologies, and production accepts the PASS marker away from the
final nonempty line.

Only Luna HIGH `/root/issue583_luna_impl` may create one replacement candidate at
`/tmp/issue680-attempt2-runner-revision1.py`, which must be fresh under `lexists`.
This is still inert preparation and does not consume attempt 2. It may read the
sealed predecessor runner and this issue spec, but must not invoke either runner,
the verifier, retained evidence, or any product command, and it must create no
attempt/control path. The replacement must:

- require `--authorization-head` as a full lowercase Git SHA for production and
  compare it with live HEAD/upstream; never embed a feature head;
- require `--tracker-head` as a full lowercase Git SHA, compare it with the
  tracker worktree's clean HEAD/upstream and live remote tracker branch, and
  validate that the operative #559/#560 rows name the supplied feature head and
  sole executor lease;
- query live remote `main` and require it, local `origin/main`, and merge base to
  equal the issue's pinned main; a cached remote-tracking ref alone is not proof;
- require `--executor-id /root/issue583_luna_impl` and reject omission or any
  other value, without making OS uid an acceptance condition;
- stream the one exact deterministic target-archive subprocess into SHA-256,
  retaining only argv/cwd/timestamps/status/stderr/digest and never archive bytes;
- append phase, finish time, numeric status, failed assertion or launch error to
  the preflight after it exists, and leave explicit separate command captures;
- implement the self-excluding manifest with Python file APIs using bytewise
  relative-path order, reject symlinks/special entries/path names it cannot encode
  exactly and all traversal errors, then record and run direct
  `sha256sum -c SHA256SUMS` with argv/cwd/timestamps/status/streams and checked rows;
- use isolated dependency-injected execution-flow controls proving changed-head,
  pre-existing regular path, dangling symlink, verifier mismatch, command
  mutation, status 0/1, subprocess launch error, and manifest failure behavior;
- detect only processes whose argv names this runner, the sealed verifier, or an
  attempt-2 path, excluding itself, instead of rejecting unrelated tools;
- validate actual command arrays at the dispatch boundary and require the
  production PASS marker to equal the final nonempty stdout line.

The proposed static-control invocation is
`python3 -B /tmp/issue680-attempt2-runner-revision1.py --self-test`. Production
will use
`python3 -B /tmp/issue680-attempt2-runner-revision1.py --authorization-head <reviewed-full-feature-sha> --tracker-head <reviewed-full-tracker-sha> --executor-id /root/issue583_luna_impl`;
the exact full SHAs are supplied only by a later synchronized exact-head SCOPE
PASS. Neither invocation is authorized by this revision brief.

Revision 1 completed without execution while checkpoint `93dbc383` was
strengthening its authority and manifest contract. Root sealed the resulting
ordinary file mode `0444`: 31,703 bytes, SHA-256
`f4c44699e8c3ea108f4d35021dee918adf3030de234658b1520637d389a24e36`.
No attempt-2 or control path was created. Do not invoke or modify these bytes.
Astra LOW static review is required. Coordinator inspection already finds that
the file still defines and enforces `AUTHORIZATION_HEAD=ad042156...`, accepts no
tracker-head argument, and makes the synthetic status-0 case expect rejection;
therefore it does not yet satisfy the current revision brief.

Astra LOW returned **DRAFT FAIL** for the sealed revision-1 bytes. Preserve them
unchanged and never invoke them. In addition to the coordinator findings, the
runner checks neither a tracker argument nor live remote main/tracker/lease;
tests an imitation rather than the production orchestration; retains tautological
dispatch checks; deletes partial captures; omits terminal preflight, separate
self-test, launch-error, and manifest-verification metadata; permits ambiguous
manifest path spellings; tests literal backslash-x-zero instead of a NUL byte;
records a target-hash cwd not supplied to `Popen`; and serially drains stderr
after stdout, which can deadlock. The explicit executor check, relevant-process
filter, streamed target digest, final-line PASS check, and most traversal checks
are retained design credit only, not execution credit.

The earlier inert preparation lease to Luna HIGH `/root/issue583_luna_impl` is
superseded before it created a revision-2 path or process. Luna XHIGH
`/root/issue679_luna_verifier` alone may prepare one bounded correction at fresh
external `/tmp/issue680-attempt2-runner-revision2.py`. Preparation may read only
this spec and the two sealed runner drafts. It must not import, compile, or invoke
any runner or verifier; inspect retained evidence; create an attempt-2 or control
path; or edit Git. Luna HIGH `/root/issue583_luna_impl` remains the sole eventual
attempt-2 executor after later reviews and receives no current preparation or
execution authority. Revision 2 must satisfy every current runner requirement
and additionally:

- remove every embedded feature/tracker authorization and require both external
  full lowercase SHA arguments, plus the exact explicit executor claim;
- query live remote main and tracker refs, compare feature/tracker HEAD and
  upstream, validate the operative lease text, and do all boundary checks before
  output creation;
- route synthetic observations through the same orchestration and dispatch
  boundary as production, with a positive status-0 flow that succeeds and
  negative head/path/symlink/verifier/command/status/launch/manifest flows that
  fail without output;
- preserve every exclusively created partial capture, append phase/failure/
  finish/status records where possible, and retain separate command, metadata,
  stdout, stderr, and status files for verifier self-test, production, and
  manifest verification;
- restrict manifest paths to an exact unambiguous ASCII spelling, reject newline,
  backslash, non-ASCII, empty, absolute, dot, and dot-dot components, require a
  nonempty inventory, and validate the actual checked rows;
- launch the target archive with the recorded actual cwd and stream stdout to
  SHA-256 while draining stderr concurrently or directly into its durable capture,
  never buffering or copying archive bytes.

Freeze revision 2 by exact SHA-256, size, and mode. Astra LOW must review its
literal bytes before any static-control invocation. No execution is authorized
by this correction brief.

The proposed revision-2 control invocation is
`python3 -B /tmp/issue680-attempt2-runner-revision2.py --self-test`. Its proposed
production invocation is
`python3 -B /tmp/issue680-attempt2-runner-revision2.py --authorization-head <reviewed-full-feature-sha> --tracker-head <reviewed-full-tracker-sha> --executor-id /root/issue583_luna_impl`.
The exact heads are supplied only by a later synchronized SCOPE PASS.

The earlier Luna HIGH preparation completed inertly while checkpoint `07c4afc7`
was transferring the same lease. It exclusively created the sole revision-2
path; root sealed the ordinary file mode `0444`, 33,524 bytes, SHA-256
`2f6ec725d4b6d3e74f2587f0114f065a737976ff33a208d8e0e74cab91f193d5`.
Nothing was invoked and all attempt-2/control paths remain absent. The transferred
preparer must not overwrite or append to this file. Astra LOW must review these
exact bytes. Coordinator inspection flags one likely impossible self-reference:
`validate_lease_rows` requires the supplied tracker commit SHA to occur inside
both tracker files committed by that same SHA.

Astra LOW returned **STATIC FAIL** on revision 2. Its shared flow creates the
evidence directory and then requires that same directory absent, the tracker
lease check is self-referential and conflates historical rows, the live feature
branch is not queried, target stderr draining can still block, `sha256sum`
stdout rows are not parsed, manifest creation metadata is incomplete, and a
duplicate lease helper remains. No runner or verifier byte ran.

## Attempt 2 reduced launcher

Two consecutive runner revisions failed without advancing an executable gate.
The throughput and ceremony-boundary rules now stop that general-runner shape.
The content-addressed verifier already owns target identity, the 77-file SDK
classification, all eight retained gates, and the nine-root census. Repeating
those checks and a general synthetic framework in another program adds failure
surface without strengthening the artifact claim. All earlier clauses that call
for another general runner or runner self-test are superseded; preserve the three
sealed runner candidates outside Git and create no `revision3`.

Only Luna HIGH `/root/issue583_luna_impl` may prepare a small single-purpose
launcher at fresh external `/tmp/issue680-attempt2-launcher.py`. Preparation
creates that ordinary file only and invokes nothing. The launcher must:

- require full lowercase `--feature-head`, `--tracker-head`, and exact
  `--executor-id /root/issue583_luna_impl`; before output, compare those heads
  with clean local HEAD/upstream and live feature/tracker remote refs, require
  live/local main and merge bases at the pinned main, verify the clean preserved
  authority head, verifier hash/size/mode, old preflight hash, and all seven fresh
  paths, and acquire a nonblocking advisory lock on its own ordinary file;
- exclusively create the preflight, invoke the exact verifier self-test once,
  and append literal argv/cwd/start/finish/status plus complete stdout/stderr;
  on launch error, nonzero status, missing final self-test PASS, or control
  residue, append the failure and stop;
- recheck all identities and remaining paths, create the evidence directory,
  invoke the exact production verifier once, preserve the five flat
  `00-production.{command,meta,stdout,stderr,status}` captures, and require status
  0 plus the exact production PASS as the final nonempty stdout line;
- create `SHA256SUMS` over those five explicitly named ordinary files only,
  bytewise sorted, with lowercase digest, two spaces, and `./filename`; reject
  any extra/symlink/special entry, then never write inside evidence again;
- directly dispatch `sha256sum -c SHA256SUMS` once, capture exact argv/cwd/
  timestamps/status/streams in the four external manifest paths, and require
  empty stderr plus the exact five `./filename: OK` stdout rows and matching
  manifest/name/count sets.

Use one dispatch function that compares actual argv with the frozen verifier
arrays immediately before `subprocess.Popen`, maps launch errors to status 127,
and never deletes a partial record. No target archive, retained census, tracker-
body parsing, process scan, general traversal, dependency injection, or launcher
self-test belongs in the reduced program; those are verifier or exact-head scope
responsibilities. Root freezes the launcher hash/size/mode, Astra LOW reviews the
literal bytes once, and a later exact-head SCOPE PASS supplies the sole execution
command:
`python3 -B /tmp/issue680-attempt2-launcher.py --feature-head <reviewed-full-feature-sha> --tracker-head <reviewed-full-tracker-sha> --executor-id /root/issue583_luna_impl`.
No launcher or verifier execution is authorized by this amendment.

The reduced launcher was prepared without execution and root sealed its ordinary
file mode `0444`: 14,665 bytes, SHA-256
`588d17821d368880b41dc6f2420449a39f305458b723dace0ba250a5cd817f8b`.
All attempt-2/control paths remain absent. Astra LOW must review these exact bytes
before any invocation. Coordinator inspection already finds that
`append_json(PREFLIGHT, "selftest_capture", self_record)` passes raw `bytes` to
`json.dumps`, guaranteeing an uncaught `TypeError` after self-test; it also notes
that `exclusive_bytes` deletes a partial file on write failure despite the
reduced launcher's preservation rule.

Astra LOW returned **STATIC FAIL** on the sealed reduced launcher with five
bounded corrections; its verifier argv/markers and exact five-file manifest are
otherwise coherent. Preserve it without execution. Only Luna HIGH
`/root/issue583_luna_impl` may create fresh ordinary
`/tmp/issue680-attempt2-launcher-revision1.py`, reading only the sealed launcher
and this spec and invoking nothing. The revision must serialize the self-test
capture from metadata plus existing base64 streams without raw `bytes`, preserve
every partial exclusive write, remove the unsupported OS-uid gate, append the
final pre-production `boundaries()` observation instead of discarding it, and
catch both `OSError` and `LauncherError` while always emitting stderr if an
after-preflight diagnostic append fails. Do not otherwise expand or generalize
the launcher. Root must seal its exact hash/size/mode and Astra LOW must review
the literal bytes. The earlier launcher invocation is superseded; eventual
execution will use
`python3 -B /tmp/issue680-attempt2-launcher-revision1.py --feature-head <reviewed-full-feature-sha> --tracker-head <reviewed-full-tracker-sha> --executor-id /root/issue583_luna_impl`.
No correction or execution command is authorized beyond inert file creation.

The bounded correction completed without execution. Root sealed the ordinary
file mode `0444`: 15,089 bytes, SHA-256
`cdc44e1c02f27b6cf820c81f09821f0d6d29554000fcea477cb083c64068760d`.
Its diff against the rejected launcher contains only the five authorized fixes.
All attempt-2/control paths remain absent. Astra LOW must review these literal
bytes before any exact-head scope or invocation.

Astra LOW returned **STATIC PASS** on exact launcher SHA-256
`cdc44e1c02f27b6cf820c81f09821f0d6d29554000fcea477cb083c64068760d`,
15,089 bytes, mode `0444`. The diff contains only the five authorized fixes;
exact commands, terminal markers, freshness/live-ref checks, five captures, and
manifest verification are coherent. This verdict ran nothing and grants no
execution by itself. A fresh exact feature/tracker/GitHub/path SCOPE PASS must
supply both full head arguments before the sole Luna executor may invoke it once.

That STATIC PASS is superseded by the controlling independent Astra LOW **DRAFT
FAIL** on the same exact bytes. Revision 1 fixed the UID assumption, raw-byte
serialization, partial-write deletion, and discarded pre-production identity,
but four required lifecycle checks remain: immediately before manifest
verification it must revalidate exactly the five captures plus `SHA256SUMS` as
six ordinary non-symlink files; preflight exclusive creation must enter failure
ownership as soon as it succeeds; manifest creation needs durable
start/finish/status and successful completion needs a terminal status outside the
frozen evidence directory; and `TypeError` must enter the same failure path. No
exact-head SCOPE PASS or execution may rely on the superseded verdict.

Preserve revision 1 unchanged. Only Luna HIGH `/root/issue583_luna_impl` may
prepare the final bounded inert correction at fresh ordinary
`/tmp/issue680-attempt2-launcher-revision2.py`, reading only this spec and the two
sealed reduced-launcher files. It may correct only those four defects and must not
restore general-runner infrastructure, import, compile, or invoke any launcher or
verifier, inspect retained evidence, create an attempt-2/control path, edit Git,
or touch product/promotion files. Root must seal and hash the result and Astra LOW
must statically review its exact bytes. No further reduced-launcher correction is
authorized by this record. Attempt 2 remains unconsumed.

The final inert correction completed and root sealed its ordinary file mode
`0444`: 15,975 bytes, SHA-256
`a6cdc89037e365a78d495551101061bb731b10b135025b8d9ad79f7e8511ec10`.
Nothing ran and all attempt-2/control paths remain absent. Astra LOW must review
these exact bytes. Coordinator inspection finds a regression of the already
required base64-only self-test capture: revision 2 again passes the raw
`self_record` bytes to `append_json`. It also makes manifest-creation metadata
durable only after successful creation, so a creation failure still has no
external creation record. No exact-head or execution authority attaches.

Astra LOW returned **DRAFT FAIL** on final correction
`a6cdc89037e365a78d495551101061bb731b10b135025b8d9ad79f7e8511ec10`.
It confirms the raw-byte regression, incomplete ownership of a partially created
preflight, non-durable manifest-creation failures, and missing durable terminal
success. Six-entry ordinary-file revalidation is fixed. The specification permits
no further reduced-launcher correction, so preserve all three sealed launcher
files and never invoke them. This preparation did not consume attempt 2.

## Attempt 2 direct execution rebrief

The reduced-launcher shape is stopped. Attempt 2 now uses no generated runner,
launcher, helper, nested heredoc, or synthetic framework. It invokes the already
reviewed content-addressed verifier directly and records each phase through
separate, simple filesystem operations. This is a smaller execution shape, not a
third launcher correction.

Before anything executable, root must push and synchronize this amendment. Astra
LOW then performs a fresh exact-head SCOPE review of clean feature/tracker heads,
live feature/tracker/main refs, both merge bases, GitHub body parity, preserved
authority `de542050094f20150f7ec4f106e32f4074798626`, exact verifier and old
attempt-1-preflight identities, absence of every old launcher output, and absence
under both `test -e` and `test -L` of all five fresh paths:

- `/tmp/issue680-attempt2-direct-evidence`;
- `/tmp/issue680-attempt2-direct-manifest.stdout`;
- `/tmp/issue680-attempt2-direct-manifest.stderr`;
- `/tmp/issue680-attempt2-direct-manifest.status`;
- `/tmp/issue679-attempt3-verifier-control`.

No launcher file may be opened for execution.

Only Luna HIGH `/root/issue583_luna_impl` may execute after that SCOPE PASS. It
must use separate tool calls, stopping on the first failure or unexpected output:

1. Exclusively create ordinary mode-0700 directory
   `/tmp/issue680-attempt2-direct-evidence`; record the exact full feature,
   tracker, main, authority, verifier hash/size/mode, executor identity, UTC start,
   literal argv, and cwd in ordinary flat files using exclusive creation.
2. Invoke the exact verifier self-test shown below once, with stdout and stderr
   redirected to fresh ordinary flat files. Persist the tool-reported numeric
   status and UTC finish in fresh files. Require status 0, empty stderr, and the
   exact self-test PASS as final nonempty stdout line before continuing.
3. Recheck all identities and append no existing record; write the second
   observation to a new flat file. Invoke the exact production verifier shown
   below once, redirecting stdout/stderr to fresh files, then persist its numeric
   status and UTC finish in fresh files. Require status 0, empty stderr, and the
   exact production PASS as final nonempty stdout line.
4. Require the evidence directory to contain only the explicitly recorded
   ordinary non-symlink flat files. Create `SHA256SUMS` once over every other file
   in bytewise path order using lowercase digest, two spaces, and `./filename`.
   Make the directory and files read-only, then directly run
   `sha256sum -c SHA256SUMS` once from that directory. Capture its complete stdout,
   stderr, and numeric status in the three fresh external manifest paths; require
   status 0, empty stderr, and exact one-to-one `./filename: OK` rows.

Before `SHA256SUMS`, the exact flat set is
`00-preflight.txt`, `01-selftest.command`, `01-selftest.start`,
`01-selftest.stdout`, `01-selftest.stderr`, `01-selftest.status`,
`01-selftest.finish`, `02-preproduction.txt`, `03-production.command`,
`03-production.start`, `03-production.stdout`, `03-production.stderr`,
`03-production.status`, and `03-production.finish`. Command files contain the
literal argv and cwd; start/finish files contain one UTC timestamp; status files
contain one base-10 integer plus newline; observation files use sorted
`key=value` rows and must contain no unreviewed prose. `SHA256SUMS` is the
fifteenth and final internal file. External manifest captures are never included
in it.

A launch or phase failure consumes attempt 2 and preserves every created byte for
Astra LOW adversarial review. A complete run receives only EVIDENCE review; it
does not authorize promotion. Product/build/SDK/package/browser/resource/PCM/
Cargo/npm/Node gates remain forbidden in this phase. Luna later implements any
accepted three-file promotion, but Astra LOW owns this verifier review.

Astra LOW returned **SCOPE FAIL** at feature
`6ab3295b6e1b3385358a8a4f50206aec731cd5a2` and tracker
`59d483c6a6288e57ebf897bea5076bd5818a26ec`. Every inspected prerequisite passed
except the tracker merge base, which was older delivered main `7d16d9c9` rather
than pinned current main `df0b9b93`. No command ran. Root merged current
`origin/main` into the clean tracker without conflict and pushed reconciliation
`345fce5f3a2c0c3789b1ff9412f13a890c49ae40`; its merge base is now exactly
`df0b9b93636de36a7143da15b83444f280b65e6b`. Fresh Astra LOW exact-head scope
review remains mandatory and is the only possible source of execution authority.

The first review of clarified feature `ce64571b` found the direct protocol
coherent but returned **SCOPE FAIL** because tracker identity advanced from
requested `e74b17f3` to clean pushed `e5cb4ff3` during inspection. Nothing ran
and all five fresh paths remain absent. Freeze the final synchronized feature and
tracker heads, then repeat only the exact-head scope review.

The #680 worktree is the execution cwd and supplies the external authorization
head. The frozen verifier separately requires
`/home/bl/misofm/engine-cp8-mapping-evidence` as its read-only Git-object
authority because its reviewed bytes pin that exact absolute path. Require that
preserved #679 worktree available, clean, upstream-equal at hard-stop
`de542050094f20150f7ec4f106e32f4074798626`, and free of a competing writer.
It grants no #679 retry and receives no write.

The exact attempt-2 verifier remains the ordinary mode-0444 file
`/tmp/issue680-verifier-d3f0803c1a35fe6bfeb0937c944476edb23c9c223c9404bb3d05d0d18618c27b.py`,
72,262 bytes, SHA-256
`d3f0803c1a35fe6bfeb0937c944476edb23c9c223c9404bb3d05d0d18618c27b`.
Astra LOW already returned STATIC DRAFT PASS on these exact bytes under #679;
#680 requires fresh exact-head scope review of the embedded bytes and all paths
before any command.

After the direct flow receives a fresh exact-head production SCOPE PASS, Luna
runs the isolated verifier self-test exactly once:

```text
TMPDIR=/tmp/issue679-attempt3-verifier-control python3 -B /tmp/issue680-verifier-d3f0803c1a35fe6bfeb0937c944476edb23c9c223c9404bb3d05d0d18618c27b.py --self-test
```

Require status 0, terminal PASS, unchanged verifier identity, and complete
control cleanup. Then run the production verifier exactly once:

```text
python3 -B /tmp/issue680-verifier-d3f0803c1a35fe6bfeb0937c944476edb23c9c223c9404bb3d05d0d18618c27b.py /home/bl/misofm/engine-cp8-mapping-evidence 8708c9b998a484d49ccb17a803e79540ca13fcd6 /tmp/issue672-attempt2-candidate-pristine /tmp/issue678-attempt2-candidate-source /tmp/issue672-attempt3-candidate-artifact /tmp/issue678-attempt3-evidence
```

Capture command, cwd, timestamps, numeric status, and complete stdout/stderr in
the fresh evidence directory. Stop on first failure without correction or retry.
Production PASS must include the complete 77-file SDK taxonomy, exact tracked
and overlay identities, all eight gate/postcheck records, correct 516-byte
Playwright command, nondecreasing timezone-aware metadata, lowercase gate-7
marker, all three browsers, initial/final nine-root equality, preserved target,
and terminal `PASS authority/pristine/candidate/artifact/all-eight-gates/nine-root-census`.

After production PASS, finish all evidence writes and create a self-excluding
manifest exactly once by directly invoking `sha256sum` once for each of the 14
frozen input files in the bytewise filename order listed above and appending its
output to an exclusively created `SHA256SUMS`. Before creation require
`SHA256SUMS` absent including symlink and the evidence root ordinary,
non-symlink, and nonempty. Reject every traversal error, symlink, special entry,
nested directory, duplicate, extra file, or filename outside that exact set.
Require every emitted row to contain a lowercase digest, two spaces, and the
literal `./filename`. The manifest excludes only itself.

After creation, permit no write inside the evidence directory. Verify exactly
once with `sha256sum -c SHA256SUMS` from the evidence directory and no shell
indirection. Capture its complete stdout, stderr, and tool-reported numeric status
in the three external manifest paths. Require every covered file and the manifest
ordinary and non-symlink, status 0, empty stderr, exact path sets, 14 manifest
rows, and 14 verification rows. Preserve every partial record on failure.

The complete reviewed verifier bytes follow verbatim:

```python
#!/usr/bin/env python3
"""Read-only disposition verifier for the retained #678 export.

CLI: verifier.py REPO COMMIT PRISTINE CANDIDATE ARTIFACT RETAINED_EVIDENCE
     verifier.py --self-test
"""

from __future__ import annotations

import argparse
import copy
import contextlib
import datetime
import hashlib
import json
import os
import pathlib
import re
import shutil
import stat
import subprocess
import sys
import tempfile


AUTHORITY_COMMIT = "8708c9b998a484d49ccb17a803e79540ca13fcd6"
AUTHORITY_REPO = pathlib.Path("/home/bl/misofm/engine-cp8-mapping-evidence")
NEW_COMMIT = AUTHORITY_COMMIT
NEW_WASM = "93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531"
OLD_COMMIT = "70899de287c23b70c17b3e41a5b2921801ae8052"
OLD_WASM = "580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10"

PIN = "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256"
RESULTS = "hosts/host-web/qualification/results.json"
MATRIX = "hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md"
OVERLAYS = {PIN, RESULTS, MATRIX}
ALLOWED_ROOTS = {
    "sdk/node_modules",
    "hosts/host-web/qualification/node_modules",
    "sdk/dist",
}
ROOT_CENSUS = (
    ("candidate-artifact", "/tmp/issue672-attempt3-candidate-artifact"),
    ("candidate-pristine", "/tmp/issue672-attempt2-candidate-pristine"),
    ("candidate-source", "/tmp/issue672-attempt2-candidate-source"),
    ("main-artifact", "/tmp/issue672-main-artifact"),
    ("issue672-evidence", "/tmp/issue672-attempt3-evidence"),
    ("issue678-evidence", "/tmp/issue678-evidence"),
    ("issue678-attempt2-evidence", "/tmp/issue678-attempt2-evidence"),
    ("issue678-attempt3-evidence", "/tmp/issue678-attempt3-evidence"),
    ("issue678-attempt3-target", "/tmp/issue678-attempt3-target"),
)
SELF_TEST_CONTROL = pathlib.Path("/tmp/issue679-attempt3-verifier-control")
PRESERVED_TARGET = pathlib.Path("/tmp/issue672-attempt2-candidate-source/target")
TARGET_DIGEST = "420c7c4db6427802163e3d08deb8022327722fadbc6bb4178f13212506257151"
SOURCE_CANDIDATE = pathlib.Path("/tmp/issue678-attempt2-candidate-source")
RETAINED_EVIDENCE = pathlib.Path("/tmp/issue678-attempt3-evidence")
RETAINED_VERIFIER_CWD = "/home/bl/misofm/engine-cp8-mapping-delivery"
EXPECTED_TRACKED_COUNT = 12195
TARGET_ENV = "/tmp/issue678-attempt3-target"
PACKAGE_MANIFEST_HASHES = {
    "/tmp/issue678-attempt2-candidate-source/sdk/package.json":
    "62641970eb223ab6c81b97a077e44c3b9d1aa2179513df2f00d235a5546509c0",
    "/tmp/issue678-attempt2-candidate-source/sdk/package-lock.json":
    "dee524dd698d40dbcc99fced7e55e651fd5d96b0957a2ddb1cd59c7c486c03d4",
    "/tmp/issue678-attempt2-candidate-source/hosts/host-web/qualification/package.json":
    "81a76f3233a35f3d387981087a4172fdafe81cee2be6f43417d1a3b3fbfebd6d",
    "/tmp/issue678-attempt2-candidate-source/hosts/host-web/qualification/package-lock.json":
    "37bfedc97da4e4377a45a0d88f2351263b77e160cde109620d23eaa9952ab069",
}
PLAYWRIGHT_COMMAND = (
    "node - <<'NODE'\n"
    "const fs = require('node:fs');\n"
    "const pw = require('./hosts/host-web/qualification/node_modules/playwright');\n"
    "const pkg = require('./hosts/host-web/qualification/node_modules/playwright/package.json');\n"
    "if (pkg.version !== '1.62.1') process.exit(1);\n"
    "for (const [name, browser] of [['chromium', pw.chromium], ['firefox', pw.firefox], ['webkit', pw.webkit]]) {\n"
    "  const executable = browser.executablePath();\n"
    "  fs.accessSync(executable, fs.constants.X_OK);\n"
    "  console.log(`${name}\\t${executable}`);\n"
    "}\n"
    "NODE\n"
)
PLAYWRIGHT_COMMAND_EXPECTED = (
    "node - <<'NODE'\n"
    "const fs = require('node:fs');\n"
    "const pw = require('./hosts/host-web/qualification/node_modules/playwright');\n"
    "const pkg = require('./hosts/host-web/qualification/node_modules/playwright/package.json');\n"
    "if (pkg.version !== '1.62.1') process.exit(1);\n"
    "for (const [name, browser] of [['chromium', pw.chromium], ['firefox', pw.firefox], ['webkit', pw.webkit]]) {\n"
    "  const executable = browser.executablePath();\n"
    "  fs.accessSync(executable, fs.constants.X_OK);\n"
    "  console.log(`${name}\\t${executable}`);\n"
    "}\n"
    "NODE\n"
)
GATE_SPECS = (
    ("07-gate1-resources", "07-gate1-postcheck", "python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue672-attempt3-candidate-artifact"),
    ("08-gate2-web-test", "08-gate2-postcheck", "bash scripts/test-web-audioworklet.sh"),
    ("09-gate3-sdk-deletions", "09-gate3-postcheck", "python3 -B scripts/check-sdk-deletions.py"),
    ("10-gate4-sdk-deletions-selftest", "10-gate4-postcheck", "python3 -B scripts/check-sdk-deletions.py --self-test"),
    ("11-gate5-sdk-types", "11-gate5-postcheck", "bash scripts/check-sdk-types.sh"),
    ("12-gate6-sdk-headless", "12-gate6-postcheck", "bash scripts/check-sdk-headless.sh /tmp/issue672-attempt3-candidate-artifact"),
    ("13-gate7-sdk-package", "13-gate7-postcheck", "bash scripts/sdk-package.sh check /tmp/issue672-attempt3-candidate-artifact"),
    ("14-gate8-qualification", "14-gate8-postcheck", "npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue672-attempt3-candidate-artifact --browser all --check-matrix --self-test-mutations"),
)
POSTCHECK_COMMAND = "CARGO_TARGET_DIR=/tmp/issue678-attempt3-target /tmp/issue678-attempt2-evidence/postcheck.sh"
OVERLAY_COMMAND = "python3 -B /tmp/issue672-attempt2-export-verifier.py /home/bl/misofm/engine-cp8-mapping-delivery 8708c9b998a484d49ccb17a803e79540ca13fcd6 /tmp/issue678-attempt2-candidate-source overlay"
ARTIFACT_NAMES = {
    "miso-engine-v1-abi-layout.json",
    "miso-engine-v1-audio-worklet-host.d.ts",
    "miso-engine-v1-audio-worklet-host.js",
    "miso-engine-v1-audio-worklet.js",
    "miso-engine-v1-audio-worklet.simd128.wasm",
    "miso-engine-v1-parameter-metadata.json",
}
ARTIFACT_HASHES = {
    "miso-engine-v1-abi-layout.json": "40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919",
    "miso-engine-v1-audio-worklet-host.d.ts": "445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf",
    "miso-engine-v1-audio-worklet-host.js": "21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a",
    "miso-engine-v1-audio-worklet.js": "225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb",
    "miso-engine-v1-audio-worklet.simd128.wasm": "93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531",
    "miso-engine-v1-parameter-metadata.json": "6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d",
}
# These output SHA-256 values were acquired after the successful retained gate-7
# package command. They are stability anchors for this disposition, not a claim
# that this verifier reproduces TypeScript or esbuild output bytes. The adjacent
# producer labels classify how each path was produced from the authoritative SDK
# inputs and staging script.
DIST_EXPECTATIONS = {
    'sdk/dist/LICENSE': ('stage-repository-copy', 'cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30', 436),
    'sdk/dist/NOTICE': ('stage-repository-copy', 'b7a2d82a4d67900cd09ddd95d27a20fe9acb964c133328c1c95ea5a44d9655d1', 436),
    'sdk/dist/assets.d.ts': ('typescript-compiler-declaration', 'e4ff336584eaad1794858fc3381b281bc92b23818c7f57bcc11e6c0baab93f2f', 436),
    'sdk/dist/assets.js': ('typescript-compiler-javascript', 'e4afaa81432389c85a1f91a4fe1ba4bfe812f23a302e188818f21b3075bd258b', 436),
    'sdk/dist/assets/miso-engine-v1-abi-layout.json': ('stage-artifact-copy', '40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919', 436),
    'sdk/dist/assets/miso-engine-v1-audio-worklet-host.d.ts': ('stage-artifact-copy', '445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf', 436),
    'sdk/dist/assets/miso-engine-v1-audio-worklet-host.js': ('stage-artifact-copy', '21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a', 436),
    'sdk/dist/assets/miso-engine-v1-audio-worklet.js': ('stage-artifact-copy', '225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb', 436),
    'sdk/dist/assets/miso-engine-v1-audio-worklet.simd128.wasm': ('stage-artifact-copy', '93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531', 509),
    'sdk/dist/assets/miso-engine-v1-parameter-metadata.json': ('stage-artifact-copy', '6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d', 436),
    'sdk/dist/assets/miso-engine-v1-pcm-feed-worklet.js': ('stage-source-copy', 'a4826b08bb0392fdcd0a4d0db05c000de446973f7d7ebaca16fc26677b756135', 436),
    'sdk/dist/assets/miso-engine-v1-sdk-manifest.json': ('stage-package-manifest', 'e868a28a78f3854187d9bf21a2659ed8c32ee063ee0397d340f257e46d0302fb', 436),
    'sdk/dist/browser/console.d.ts': ('typescript-compiler-declaration', '2c7a4e2a704c3dd8513dd81f092ecc61a3fcea4495bf77bd1cd54b545037189d', 436),
    'sdk/dist/browser/console.js': ('typescript-compiler-javascript', '81e8107d051d69080d745c1d76a2f3191a31773b7a9b685331fc86572528e9aa', 436),
    'sdk/dist/browser/default-host.d.ts': ('typescript-compiler-declaration', '80712c0167485831ce6a68c25ecb084a488572def085047c319fa99a515002b9', 436),
    'sdk/dist/browser/default-host.js': ('typescript-compiler-javascript', 'fa774a56e85075a039e8a3af7bf19e1cfd513a1dceeb7350aca531dc9e02b8cf', 436),
    'sdk/dist/browser/engine.d.ts': ('typescript-compiler-declaration', '7325af6884edc9d53d42644e39ff0644cb9d247390a37c4b059f3a779b82df47', 436),
    'sdk/dist/browser/engine.js': ('typescript-compiler-javascript', 'cf24811ea6110a1dedcfd2a613cb5897d9284e9feeccdf8bc6b00392fd69d098', 436),
    'sdk/dist/browser/host-mirror.d.ts': ('typescript-compiler-declaration', 'b11868a19cec3a233751a2907c036df2c7565ccb6a0318ec7c8a303b78cfdb9c', 436),
    'sdk/dist/browser/host-mirror.js': ('typescript-compiler-javascript', '198022132d87eff0a7d9f144b9a5202301dd22f75768effee69a08cc3186ef43', 436),
    'sdk/dist/browser/index.d.ts': ('typescript-compiler-declaration', 'a66dbf369d84cc292873df9c4586f0381676adc7083324f3181b9de953321b32', 436),
    'sdk/dist/browser/index.js': ('typescript-compiler-javascript', '484a0f115da6457a57549ec93f69979d3b98c64a9ca2e2e87592337e005d3ac7', 436),
    'sdk/dist/browser/pcm-feed.d.ts': ('typescript-compiler-declaration', '68c89e8ed58905b881747205a39e88bd7c38057d1921e48893a6387d52ecd7c1', 436),
    'sdk/dist/browser/pcm-feed.js': ('typescript-compiler-javascript', 'af54f5adace60dd74606df797fbf2fb55782ab19acf9e7b81f4eb307fde22149', 436),
    'sdk/dist/browser/pcm-ring.d.ts': ('typescript-compiler-declaration', '40b1ba9c0980f38b4c6d98bc31732d3b996651d7cbb1654ba4fb15a73597b66b', 436),
    'sdk/dist/browser/pcm-ring.js': ('typescript-compiler-javascript', '9412b1578929f2832495a2020f1bb4d455c4f0019cc4eab050ba69ac048e9cd3', 436),
    'sdk/dist/browser/policy.d.ts': ('typescript-compiler-declaration', '951e16db813e79937adfaf2126501feb20885862de43cfd04ee7c779e0764242', 436),
    'sdk/dist/browser/policy.js': ('typescript-compiler-javascript', 'b3ccb790722c584e7a3b4dd2b448182a02f15a123b28d87210604d2bb3de604e', 436),
    'sdk/dist/browser/scratch-worker.d.ts': ('typescript-compiler-declaration', '8e609bb71c20b858c77f0e9f90bb1319db8477b13f9f965f1a1e18524bf50881', 436),
    'sdk/dist/browser/scratch-worker.js': ('stage-esbuild-bundle', '94611b2459cfe5bc13cb5604ec5d77308cf458c42816122f3f001b8a48734363', 436),
    'sdk/dist/browser/scratch.d.ts': ('typescript-compiler-declaration', '208fcf12c13cea2f6c2710cf6e1cedb6d6d1330fa57a2bc37bc5055bf4cc31a8', 436),
    'sdk/dist/browser/scratch.js': ('typescript-compiler-javascript', '6069bbe6d383dcada3960f82220fd0fddcf4d77fe094ee641ce4b161cabd7b1a', 436),
    'sdk/dist/browser/shipped-host.d.ts': ('stage-source-copy', '445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf', 436),
    'sdk/dist/cli/session-request.d.ts': ('typescript-compiler-declaration', 'ba151b3b7d0f655c3560f41f3911a685660c2472b0159ff9d9f6a03c85e9b095', 436),
    'sdk/dist/cli/session-request.js': ('typescript-compiler-javascript', 'c3ce3b7955eea454b2966cd129a21f84a68f976bc5394765833502f24fa649d3', 436),
    'sdk/dist/core/abi.d.ts': ('typescript-compiler-declaration', '6c04f851b95cdb17679d85858d7aa6c4b09fb548015e5385e7f0120550b57b20', 436),
    'sdk/dist/core/abi.js': ('typescript-compiler-javascript', '1b25ff5a4563fd32316d80ffa02363dfeebe245bb3b2882a27831bab1735da95', 436),
    'sdk/dist/core/agent.d.ts': ('typescript-compiler-declaration', '36794bcc079dbf1e2793767e8519ac8359f0a28661937157c50d64592aa6aa57', 436),
    'sdk/dist/core/agent.js': ('typescript-compiler-javascript', '64a9d683bc3579c8cf784b2c30703f1c0140a004536d05a80470eb923d039cb4', 436),
    'sdk/dist/core/asset.d.ts': ('typescript-compiler-declaration', '77cb291d2670af87a48352e47e0a462c2b3a43e2e99636834b88d2271741f01f', 436),
    'sdk/dist/core/asset.js': ('typescript-compiler-javascript', '30a86bdbecd0c00cca83d5a9a9812ae4cbe96131b1c4ecb2b2de92f662ece5fd', 436),
    'sdk/dist/core/boundary.d.ts': ('typescript-compiler-declaration', '0e33997c0dd9e844aba3faac8ef5bd9ed87f91fe7231937f8a302bd35d1244e7', 436),
    'sdk/dist/core/boundary.js': ('typescript-compiler-javascript', '77709ff5004711186ae16dbca7baafcbc75ef1bccf3eda71328614f4f193ceb8', 436),
    'sdk/dist/core/console.d.ts': ('typescript-compiler-declaration', '8dfaa0e54ce1a20eb347ec1b4871d176a415a65f84df94cd9d291a557b84a4ce', 436),
    'sdk/dist/core/console.js': ('typescript-compiler-javascript', '7c77845d7768b980ef0cc92548564a6fbb5e1a1fe8c34c78f6a9959f17ba6e69', 436),
    'sdk/dist/core/decimal.d.ts': ('typescript-compiler-declaration', '7879354d818227436120bde2e84b4cd0a30379f630de8756bd54bd5f7ff29183', 436),
    'sdk/dist/core/decimal.js': ('typescript-compiler-javascript', '0670d35ed9a2a9d460b15f696d04a3305093cb03eda65ec5d4ebdb125551c6af', 436),
    'sdk/dist/core/errors.d.ts': ('typescript-compiler-declaration', '18f6dcdf9a78a6887abc946a9eead28c0b55331d982648576a6078297efe8255', 436),
    'sdk/dist/core/errors.js': ('typescript-compiler-javascript', 'e4fde804fd34dc33335960c7c6d3927bbedd9356e0e5b7278426a1d4e8657226', 436),
    'sdk/dist/core/hex.d.ts': ('typescript-compiler-declaration', 'b6fa36fc54fabbb1259dd8da7ef20ee24eb5fef6bc621e14f39cc6803c0af201', 436),
    'sdk/dist/core/hex.js': ('typescript-compiler-javascript', '6e7a8f8b4b01d7dff632ef3c8686d529a40d5c8fac7db3d80f1367f639342529', 436),
    'sdk/dist/core/lattice.d.ts': ('typescript-compiler-declaration', 'bd8bc30dc74c929e33b22eda53510e9ebfb4b32bc5bc827f8e9394d7255ac02b', 436),
    'sdk/dist/core/lattice.js': ('typescript-compiler-javascript', '0e0f56af712b6b0799082ab736f6abcdfafe303bd0c701feb7065fe13b662015', 436),
    'sdk/dist/core/session.d.ts': ('typescript-compiler-declaration', 'e2c03e4ec668be11b0877e410d2163004416dac68783be020aa01608ef66c12d', 436),
    'sdk/dist/core/session.js': ('typescript-compiler-javascript', 'cab13fb6a195576bffc825c4713b73f90e58a24f9703af62e23a6dc52d4f962b', 436),
    'sdk/dist/core/types.d.ts': ('typescript-compiler-declaration', 'f1e4b15a8c01b9ac1c96f96f95f8dab1ff8c3214430a2b82bd0d383f83ff0fa1', 436),
    'sdk/dist/core/types.js': ('typescript-compiler-javascript', '8e609bb71c20b858c77f0e9f90bb1319db8477b13f9f965f1a1e18524bf50881', 436),
    'sdk/dist/core/writer.d.ts': ('typescript-compiler-declaration', '7f17c6f973c9e87d703e4b7415e53d20ac9d8821c0b2e40409dadc70f16a51ac', 436),
    'sdk/dist/core/writer.js': ('typescript-compiler-javascript', 'a3c87384ce21681706df7d4c92ca9a7fd67696913fa5ed52110bc1a7d8d189c6', 436),
    'sdk/dist/enginectl.d.ts': ('typescript-compiler-declaration', '43e818adf60173644896298637f47b01d5819b17eda46eaa32d0c7d64724d012', 436),
    'sdk/dist/enginectl.js': ('typescript-compiler-javascript', 'c642c6e4d638213abe71ec031f56fd574e44afbbe87f524cc01e1e240831fe4a', 509),
    'sdk/dist/generated/abi.d.ts': ('typescript-compiler-declaration', '9f05c4603ba99915ae71edfee78d17f26d2268dcf13f6f02d32335b043924efa', 436),
    'sdk/dist/generated/abi.js': ('typescript-compiler-javascript', '22c0b1b96a5fa055a8a325c63bce5c30f3c5302571f69c7fa988f95083a9f266', 436),
    'sdk/dist/generated/catalog.d.ts': ('typescript-compiler-declaration', '7192f15c3d0cfdf4e404da86ec6896f08a01379c7ca096bfddba96e15caf60d2', 436),
    'sdk/dist/generated/catalog.js': ('typescript-compiler-javascript', 'e931783bca4bd40beded82aa2de09c811db480d71ae854add97a5af528902839', 436),
    'sdk/dist/generated/provenance.d.ts': ('typescript-compiler-declaration', '2ca0b05367d31a261eafd8a720cd09999b5705ae37ffb3b4fcbf0610f50bb419', 436),
    'sdk/dist/generated/provenance.js': ('typescript-compiler-javascript', '3ae3c2109c7cc470fc6c5c9a275d6baa217d69f510d67875fc4617d46b670d7b', 436),
    'sdk/dist/headless/assets.d.ts': ('typescript-compiler-declaration', '2cf2e9f674b43b10d0c42b37353cfa83095eb7122b137848b7c3ec294e16c00b', 436),
    'sdk/dist/headless/assets.js': ('typescript-compiler-javascript', '4707acae4e1923b749a16d544e39971300007e9fbfe16c3d3dc2de43bdde1cf4', 436),
    'sdk/dist/headless/engine.d.ts': ('typescript-compiler-declaration', '53acff6ab25df2d9f1779e31ce35c2ebab60c90d0fcd5e71d146bd7db87746aa', 436),
    'sdk/dist/headless/engine.js': ('typescript-compiler-javascript', '48b261915fbebfe0ac7b05bd5fb6e84316bf53df28e7ee626462898c53778214', 436),
    'sdk/dist/headless/index.d.ts': ('typescript-compiler-declaration', 'c9865dec5068768d8c2eae7610324f187584dfeb79eeecb42aa9b533ca4c6f05', 436),
    'sdk/dist/headless/index.js': ('typescript-compiler-javascript', '1bd256d6ef58c7808c5e351ec546df6f73918a6e012dfad48dd6d3181c79505b', 436),
    'sdk/dist/index.d.ts': ('typescript-compiler-declaration', 'ae89e56e7db8b99c31a2bebde3f076684006cb76b6839acd940a2aa9e720c1f0', 436),
    'sdk/dist/index.js': ('typescript-compiler-javascript', '12a52d7ee018d751ea7f5b41faf33b42e8ef55bec31a5a261fbeba46de9f3128', 436),
    'sdk/dist/internal/session-json.d.ts': ('typescript-compiler-declaration', '9c619cccd5d2c9578f0906c06d45556ba683a3c59c1ca53839a73a4d0061ac98', 436),
    'sdk/dist/internal/session-json.js': ('typescript-compiler-javascript', '7f4dba589ab98d3e0d6ae8d062ef95f4d74c407c10fedee3212570015c7ea8ee', 436),
}
OLD_SENTENCE = (
    "This matrix is generated from the pinned Playwright 1.62.1 headless Linux "
    "qualification run over candidate `70899de287c23b70c17b3e41a5b2921801ae8052` "
    "and the single shipped simd128 AudioWorklet artifact "
    "`580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`. The "
    "version shown is the lowest version qualified by this run; older versions "
    "are unqualified, not implicitly supported."
)
NEW_SENTENCE = (
    "This matrix is generated from the pinned Playwright 1.62.1 headless Linux "
    "qualification run over candidate `8708c9b998a484d49ccb17a803e79540ca13fcd6` "
    "and the single shipped simd128 AudioWorklet artifact "
    "`93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`. The "
    "version shown is the lowest version qualified by this run; older versions "
    "are unqualified, not implicitly supported."
)


class VerificationError(RuntimeError):
    pass


def git(repo: pathlib.Path, *args: str) -> bytes:
    if not all(isinstance(arg, str) for arg in args):
        raise VerificationError("git arguments must be strings")
    try:
        result = subprocess.run(
            ["git", "-C", str(repo), *args],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise VerificationError(f"git command failed: {args!r}") from error
    return result.stdout


def tracked_inventory(repo: pathlib.Path, ref: str) -> dict[str, tuple[str, str]]:
    if not isinstance(ref, str):
        raise VerificationError("commit/tree authority must be a string")
    try:
        kind = git(repo, "cat-file", "-t", ref).decode("ascii").strip()
    except UnicodeDecodeError as error:
        raise VerificationError("authority type is not ASCII") from error
    if kind == "commit":
        git(repo, "cat-file", "-e", f"{ref}^{{tree}}")
    elif kind != "tree":
        raise VerificationError("authority is not a commit/tree")
    out: dict[str, tuple[str, str]] = {}
    for row in git(repo, "ls-tree", "-rz", "-r", "--full-tree", ref).split(b"\0"):
        if not row:
            continue
        try:
            meta, raw_path = row.split(b"\t", 1)
            mode, entry_kind, oid = meta.split()
            path = raw_path.decode("utf-8")
            mode_text = mode.decode("ascii")
            entry_kind_text = entry_kind.decode("ascii")
            oid_text = oid.decode("ascii")
        except (ValueError, UnicodeDecodeError) as error:
            raise VerificationError("malformed tracked inventory row") from error
        if entry_kind_text != "blob":
            raise VerificationError("authority contains a non-blob tracked entry")
        if path in out:
            raise VerificationError("duplicate tracked path")
        out[path] = (mode_text, oid_text)
    if len(out) != EXPECTED_TRACKED_COUNT and ref == AUTHORITY_COMMIT:
        raise VerificationError(f"tracked path count mismatch: {len(out)}")
    return out


def expected_dirs(paths: set[str]) -> set[str]:
    result: set[str] = set()
    for name in paths:
        parent = pathlib.PurePosixPath(name).parent
        while str(parent) != ".":
            result.add(str(parent))
            parent = parent.parent
    return result


def ignored(name: str, roots: set[str]) -> bool:
    return any(name == root or name.startswith(root + "/") for root in roots)


def ordinary_directory(path: pathlib.Path, label: str) -> None:
    try:
        info = path.lstat()
    except OSError as error:
        raise VerificationError(f"missing {label}: {path}") from error
    if not stat.S_ISDIR(info.st_mode) or stat.S_ISLNK(info.st_mode):
        raise VerificationError(f"{label} is not an ordinary directory: {path}")


def ordinary_file(path: pathlib.Path, label: str) -> None:
    try:
        info = path.lstat()
    except OSError as error:
        raise VerificationError(f"missing {label}: {path}") from error
    if not stat.S_ISREG(info.st_mode) or stat.S_ISLNK(info.st_mode):
        raise VerificationError(f"{label} is not an ordinary file: {path}")


def walk_export(root: pathlib.Path, ignored_roots: set[str]) -> tuple[set[str], set[str]]:
    ordinary_directory(root, "export root")
    files: set[str] = set()
    dirs: set[str] = set()

    def visit(base: pathlib.Path, relbase: str) -> None:
        try:
            entries = sorted(os.scandir(base), key=lambda entry: entry.name)
        except OSError as error:
            raise VerificationError(f"export traversal failed: {base}") from error
        for entry in entries:
            rel = f"{relbase}/{entry.name}".strip("/")
            if ignored(rel, ignored_roots):
                continue
            try:
                info = entry.stat(follow_symlinks=False)
            except OSError as error:
                raise VerificationError(f"export entry cannot be stated: {rel}") from error
            if stat.S_ISDIR(info.st_mode):
                dirs.add(rel)
                visit(pathlib.Path(entry.path), rel)
            else:
                files.add(rel)

    visit(root, "")
    return files, dirs


def check_entry_bytes(
    repo: pathlib.Path,
    root: pathlib.Path,
    path: str,
    mode: str,
    oid: str,
    compare_bytes: bool,
) -> None:
    item = root / path
    try:
        info = item.lstat()
    except OSError as error:
        raise VerificationError(f"missing tracked path: {path}") from error
    blob = git(repo, "cat-file", "blob", oid)
    if mode == "120000":
        if not stat.S_ISLNK(info.st_mode) or os.fsencode(os.readlink(item)) != blob:
            raise VerificationError(f"symlink mismatch: {path}")
        return
    if mode not in {"100644", "100755"} or not stat.S_ISREG(info.st_mode):
        raise VerificationError(f"regular-file mode mismatch: {path}")
    if bool(info.st_mode & 0o111) != (mode == "100755"):
        raise VerificationError(f"executable mode mismatch: {path}")
    if compare_bytes:
        try:
            if item.read_bytes() != blob:
                raise VerificationError(f"tracked bytes mismatch: {path}")
        except OSError as error:
            raise VerificationError(f"tracked file cannot be read: {path}") from error


def verify_export(
    repo: pathlib.Path,
    ref: str,
    root: pathlib.Path,
    overlays: bool,
    artifact_dir: pathlib.Path,
    artifact_hashes: dict[str, str],
    evidence: pathlib.Path,
    emit_inventory: bool,
    dist_expectations: dict[str, tuple[str, str, int]],
    require_authority: bool,
) -> None:
    if require_authority and ref != AUTHORITY_COMMIT:
        raise VerificationError("wrong frozen authority commit")
    expected = tracked_inventory(repo, ref)
    expected_set = set(expected)
    ignored_roots = ALLOWED_ROOTS if overlays else set()
    if overlays:
        for name in sorted(ALLOWED_ROOTS):
            ordinary_directory(root / name, f"allowed root {name}")
        if os.path.lexists(root / "target"):
            raise VerificationError("source-local target exists")
    actual, dirs = walk_export(root, ignored_roots)
    if actual != expected_set:
        raise VerificationError("missing or extra tracked path")
    if dirs != expected_dirs(expected_set):
        raise VerificationError("missing or extra tracked directory")
    if os.path.lexists(root / ".git"):
        raise VerificationError("export contains .git")
    for path, (mode, oid) in expected.items():
        compare = not (overlays and path in OVERLAYS)
        check_entry_bytes(repo, root, path, mode, oid, compare)
    if overlays:
        verify_dist_and_artifacts(
            root, artifact_dir, artifact_hashes, emit_inventory,
            dist_expectations, require_authority,
        )

def strict_json(raw: bytes, label: str) -> object:
    if not raw.endswith(b"\n"):
        raise VerificationError(f"{label} must have exactly its required final newline")

    def reject_duplicates(pairs: list[tuple[str, object]]) -> dict[str, object]:
        result: dict[str, object] = {}
        for key, value in pairs:
            if key in result:
                raise VerificationError(f"duplicate JSON key in {label}: {key}")
            result[key] = value
        return result

    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=reject_duplicates)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise VerificationError(f"invalid JSON in {label}") from error


def replace_json_string(raw: bytes, key: str, old: str, new: str) -> bytes:
    old_literal = json.dumps(old, ensure_ascii=True).encode("ascii")
    new_literal = json.dumps(new, ensure_ascii=True).encode("ascii")
    key_literal = re.escape(json.dumps(key, ensure_ascii=True).encode("ascii"))
    pattern = rb"(?P<prefix>" + key_literal + rb"\s*:\s*)" + re.escape(old_literal)
    matches = list(re.finditer(pattern, raw))
    if len(matches) != 1:
        raise VerificationError(f"expected one JSON field replacement for {key}")
    match = matches[0]
    return raw[:match.start()] + match.group("prefix") + new_literal + raw[match.end():]


def verify_candidate_lineage(
    pristine: pathlib.Path,
    candidate: pathlib.Path,
    lineage: tuple[str, str, str],
) -> None:
    new_commit, new_wasm, new_sentence = lineage
    try:
        pin = (candidate / PIN).read_bytes()
        pristine_result_bytes = (pristine / RESULTS).read_bytes()
        candidate_result_bytes = (candidate / RESULTS).read_bytes()
        pristine_matrix = (pristine / MATRIX).read_bytes()
        candidate_matrix = (candidate / MATRIX).read_bytes()
    except OSError as error:
        raise VerificationError("lineage file cannot be read") from error
    if pin != (new_wasm + "\n").encode():
        raise VerificationError("pin bytes mismatch")
    pristine_results = strict_json(pristine_result_bytes, RESULTS)
    candidate_results = strict_json(candidate_result_bytes, RESULTS)
    expected_results = copy.deepcopy(pristine_results)
    expected_results["candidateCommit"] = new_commit
    expected_results["wasmSha256"] = new_wasm
    if candidate_results != expected_results:
        raise VerificationError("results changed beyond the two lineage fields")
    expected_results_bytes = replace_json_string(
        replace_json_string(pristine_result_bytes, "candidateCommit", OLD_COMMIT, new_commit),
        "wasmSha256", OLD_WASM, new_wasm,
    )
    if candidate_result_bytes != expected_results_bytes:
        raise VerificationError("results bytes changed beyond the two lineage fields")
    if pristine_matrix.count(OLD_SENTENCE.encode()) != 1:
        raise VerificationError("pristine matrix lineage sentence is ambiguous")
    if candidate_matrix != pristine_matrix.replace(OLD_SENTENCE.encode(), new_sentence.encode(), 1):
        raise VerificationError("matrix changed beyond lineage sentence")


def verify_dist_and_artifacts(
    candidate: pathlib.Path,
    artifact_dir: pathlib.Path,
    artifact_hashes: dict[str, str],
    emit_inventory: bool,
    dist_expectations: dict[str, tuple[str, str, int]],
    require_authority: bool,
) -> None:
    if require_authority and dist_expectations is DIST_EXPECTATIONS:
        verify_taxonomy_sources(candidate)
    dist = candidate / "sdk/dist"
    ordinary_directory(dist, "sdk/dist")
    regular: list[pathlib.Path] = []
    seen_dirs: set[str] = set()

    def visit(base: pathlib.Path, relbase: str) -> None:
        try:
            entries = sorted(os.scandir(base), key=lambda entry: entry.name)
        except OSError as error:
            raise VerificationError(f"sdk/dist traversal failed: {base}") from error
        for entry in entries:
            rel = f"{relbase}/{entry.name}".strip("/")
            try:
                info = entry.stat(follow_symlinks=False)
            except OSError as error:
                raise VerificationError(f"sdk/dist entry cannot be stated: {rel}") from error
            if stat.S_ISDIR(info.st_mode):
                seen_dirs.add(rel)
                visit(pathlib.Path(entry.path), rel)
            elif stat.S_ISREG(info.st_mode):
                regular.append(pathlib.Path(entry.path))
            else:
                raise VerificationError("sdk/dist contains a symlink or special entry")

    visit(dist, "")
    expected_dist_dirs: set[str] = set()
    for dist_path in dist_expectations:
        parent = pathlib.PurePosixPath(dist_path.removeprefix("sdk/dist/")).parent
        while str(parent) != ".":
            expected_dist_dirs.add(str(parent))
            parent = parent.parent
    if seen_dirs != expected_dist_dirs:
        raise VerificationError("sdk/dist directory inventory mismatch")
    actual_names = {p.relative_to(candidate).as_posix() for p in regular}
    if actual_names != set(dist_expectations) or len(actual_names) != len(dist_expectations) or len(actual_names) != 77:
        raise VerificationError("sdk/dist names do not match frozen producer taxonomy")
    for item in sorted(regular, key=lambda p: p.relative_to(candidate).as_posix()):
        raw = item.read_bytes()
        rel = item.relative_to(candidate).as_posix()
        mode = stat.S_IMODE(item.stat().st_mode)
        producer, expected_hash, expected_mode = dist_expectations[rel]
        actual_hash = hashlib.sha256(raw).hexdigest()
        if mode != expected_mode:
            raise VerificationError(f"sdk/dist mode mismatch for {rel} ({producer})")
        if actual_hash != expected_hash:
            raise VerificationError(f"sdk/dist bytes mismatch for {rel} ({producer})")
        if emit_inventory:
            print(f"{rel}\t{mode:04o}\t{len(raw)}\t{actual_hash}\t{producer}")
    if require_authority and dist_expectations is DIST_EXPECTATIONS:
        direct_copies = {
            dist / "LICENSE": candidate / "LICENSE",
            dist / "NOTICE": candidate / "NOTICE",
            dist / "assets/miso-engine-v1-pcm-feed-worklet.js":
                candidate / "sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js",
            dist / "browser/shipped-host.d.ts": candidate / "sdk/src/browser/shipped-host.d.ts",
        }
        for packaged, source in direct_copies.items():
            if packaged.read_bytes() != source.read_bytes():
                raise VerificationError(f"direct staged copy differs: {packaged.relative_to(candidate)}")
    ordinary_directory(artifact_dir, "retained artifact root")
    try:
        artifact_entries = list(artifact_dir.iterdir())
    except OSError as error:
        raise VerificationError("retained artifact root cannot be read") from error
    for entry in artifact_entries:
        try:
            info = entry.lstat()
        except OSError as error:
            raise VerificationError("retained artifact entry cannot be stated") from error
        if not stat.S_ISREG(info.st_mode) or stat.S_ISLNK(info.st_mode):
            raise VerificationError("retained artifact root contains a symlink or special entry")
    artifact_names = {p.name for p in artifact_entries}
    if artifact_names != set(artifact_hashes):
        raise VerificationError("retained candidate artifact does not have exactly six files")
    assets = candidate / "sdk/dist/assets"
    for name, expected_hash in artifact_hashes.items():
        source = artifact_dir / name
        packaged = assets / name
        try:
            packaged_info = packaged.lstat()
        except OSError as error:
            raise VerificationError(f"missing packaged artifact: {name}") from error
        if not stat.S_ISREG(packaged_info.st_mode) or stat.S_ISLNK(packaged_info.st_mode):
            raise VerificationError(f"missing packaged artifact: {name}")
        if hashlib.sha256(source.read_bytes()).hexdigest() != expected_hash:
            raise VerificationError(f"retained artifact hash mismatch: {name}")
        if packaged.read_bytes() != source.read_bytes():
            raise VerificationError(f"packaged artifact differs: {name}")
    if require_authority and dist_expectations is DIST_EXPECTATIONS:
        manifest_path = assets / "miso-engine-v1-sdk-manifest.json"
        manifest = strict_json(manifest_path.read_bytes(), "sdk manifest")
        expected_manifest = {
            "schema": "miso.sdk.package-assets.v1",
            "abiVersion": 65536,
            "catalogSchema": "miso.web.parameter-metadata.v1",
            "abiLayoutSchema": "miso.web.abi-layout.v1",
            "artifacts": {
                name: {
                    "bytes": (artifact_dir / name).stat().st_size,
                    "sha256": hashlib.sha256((artifact_dir / name).read_bytes()).hexdigest(),
                }
                for name in sorted(artifact_hashes)
            },
        }
        if manifest != expected_manifest:
            raise VerificationError("SDK manifest does not match authoritative artifact inputs")


def verify_taxonomy_sources(candidate: pathlib.Path) -> None:
    """Cross-check the frozen 77-name table against the SDK's current producers."""
    package = strict_json((candidate / "sdk/package.json").read_bytes(), "sdk/package.json")
    tsconfig = strict_json((candidate / "sdk/tsconfig.build.json").read_bytes(), "sdk/tsconfig.build.json")
    if not isinstance(package, dict) or not isinstance(tsconfig, dict):
        raise VerificationError("SDK metadata is not a JSON object")
    options = tsconfig.get("compilerOptions", {})
    if package.get("files") != ["dist"]:
        raise VerificationError("SDK package does not publish only dist")
    if options.get("outDir") != "./dist" or options.get("rootDir") != "./src":
        raise VerificationError("SDK TypeScript output roots changed")
    if options.get("declaration") is not True:
        raise VerificationError("SDK declarations are not enabled")
    generated: dict[str, str] = {}
    for source in sorted((candidate / "sdk/src").rglob("*.ts")):
        rel = source.relative_to(candidate / "sdk/src").as_posix()
        if rel.endswith(".d.ts"):
            continue
        generated[f"sdk/dist/{pathlib.Path(rel).with_suffix('.js').as_posix()}"] = "typescript-compiler-javascript"
        generated[f"sdk/dist/{pathlib.Path(rel).with_suffix('.d.ts').as_posix()}"] = "typescript-compiler-declaration"
    generated["sdk/dist/browser/scratch-worker.js"] = "stage-esbuild-bundle"
    for name in ARTIFACT_NAMES:
        generated[f"sdk/dist/assets/{name}"] = "stage-artifact-copy"
    generated.update({
        "sdk/dist/assets/miso-engine-v1-sdk-manifest.json": "stage-package-manifest",
        "sdk/dist/assets/miso-engine-v1-pcm-feed-worklet.js": "stage-source-copy",
        "sdk/dist/browser/shipped-host.d.ts": "stage-source-copy",
        "sdk/dist/LICENSE": "stage-repository-copy",
        "sdk/dist/NOTICE": "stage-repository-copy",
    })
    if generated != {name: producer for name, (producer, _, _) in DIST_EXPECTATIONS.items()}:
        raise VerificationError("SDK producer taxonomy differs from authoritative inputs")
    try:
        stage = (candidate / "sdk/codegen/stage-package.mjs").read_text(encoding="utf-8")
    except OSError as error:
        raise VerificationError("SDK staging script cannot be read") from error
    for marker in (
        "miso-engine-v1-sdk-manifest.json", "miso-engine-v1-pcm-feed-worklet.js",
        "shipped-host.d.ts", "resolve(repoRoot, \"LICENSE\")", "resolve(repoRoot, \"NOTICE\")",
        "scratch-worker.js",
    ):
        if marker not in stage:
            raise VerificationError(f"SDK staging producer marker missing: {marker}")


def strict_text(path: pathlib.Path, label: str) -> str:
    ordinary_file(path, label)
    try:
        return path.read_bytes().decode("utf-8")
    except (OSError, UnicodeDecodeError) as error:
        raise VerificationError(f"cannot read {label}: {path}") from error


def strict_status(path: pathlib.Path, expected: str) -> None:
    if strict_text(path, str(path)).splitlines() != [expected]:
        raise VerificationError(f"unexpected status in {path}")


def parse_meta(path: pathlib.Path, expected_keys: set[str], expected_cwd: str | None, expected_status: str) -> None:
    meta: dict[str, str] = {}
    for line in strict_text(path, str(path)).splitlines():
        if not line or "=" not in line:
            raise VerificationError(f"malformed metadata: {path}")
        key, value = line.split("=", 1)
        if key in meta:
            raise VerificationError(f"duplicate metadata key: {path}")
        meta[key] = value
    if set(meta) != expected_keys or (expected_cwd is not None and meta.get("cwd") != expected_cwd) or meta.get("status") != expected_status:
        raise VerificationError(f"metadata identity mismatch: {path}")
    if "CARGO_TARGET_DIR" in expected_keys and meta.get("CARGO_TARGET_DIR") != TARGET_ENV:
        raise VerificationError(f"metadata target mismatch: {path}")
    try:
        start = datetime.datetime.fromisoformat(meta["start_utc"].replace("Z", "+00:00"))
        finish = datetime.datetime.fromisoformat(meta["finish_utc"].replace("Z", "+00:00"))
    except (KeyError, ValueError) as error:
        raise VerificationError(f"malformed metadata timestamps: {path}") from error
    if (
        start.tzinfo is None or finish.tzinfo is None
        or start.utcoffset() is None or finish.utcoffset() is None
        or finish < start
    ):
        raise VerificationError(f"metadata timestamps are not ordered: {path}")


def verify_postcheck(evidence: pathlib.Path, prefix: str, artifact_hashes: dict[str, str]) -> None:
    if strict_text(evidence / f"{prefix}.command", "postcheck command") != POSTCHECK_COMMAND + "\n":
        raise VerificationError(f"postcheck command mismatch: {prefix}")
    strict_text(evidence / f"{prefix}.stdout", "postcheck stdout")
    strict_text(evidence / f"{prefix}.stderr", "postcheck stderr")
    strict_status(evidence / f"{prefix}.status", "0")
    stdout = strict_text(evidence / f"{prefix}.stdout", "postcheck stdout")
    stderr = strict_text(evidence / f"{prefix}.stderr", "postcheck stderr")
    for marker in ("source_target=absent", "artifact_hashes:", "target_identity=manifests:"):
        if marker not in stdout:
            raise VerificationError(f"postcheck lacks {marker}: {prefix}")
    for digest in artifact_hashes.values():
        if digest not in stdout:
            raise VerificationError(f"postcheck lacks artifact hash: {prefix}")
    for path, digest in PACKAGE_MANIFEST_HASHES.items():
        if f"{digest}  {path}" not in stdout:
            raise VerificationError(f"postcheck lacks package identity: {prefix}")
    if TARGET_DIGEST not in stderr:
        raise VerificationError(f"postcheck lacks preserved target identity: {prefix}")


def verify_gate_evidence(evidence: pathlib.Path, artifact_hashes: dict[str, str]) -> None:
    ordinary_directory(evidence, "retained evidence")
    for prefix, postcheck_prefix, expected_command in GATE_SPECS:
        if strict_text(evidence / f"{prefix}.command", "gate command") != expected_command + "\n":
            raise VerificationError(f"gate command mismatch: {prefix}")
        strict_text(evidence / f"{prefix}.stdout", "gate stdout")
        strict_text(evidence / f"{prefix}.stderr", "gate stderr")
        strict_status(evidence / f"{prefix}.status", "0")
        parse_meta(
            evidence / f"{prefix}.meta",
            {"cwd", "CARGO_TARGET_DIR", "start_utc", "finish_utc", "status"},
            str(SOURCE_CANDIDATE),
            "0",
        )
        verify_postcheck(evidence, postcheck_prefix, artifact_hashes)
    qualification = strict_text(evidence / "14-gate8-qualification.stdout", "gate-8 stdout")
    for marker in (
        "session identities: 3 qualification documents declare their fed PCM",
        "artifact set: the exact 6-file shipped set is pinned",
        "chromium: all qualification gates passed (151.0.7922.34)",
        "firefox: all qualification gates passed (153.0)",
        "webkit: all qualification gates passed (26.5)",
    ):
        if marker not in qualification:
            raise VerificationError(f"gate-8 output lacks {marker}")
    package_output = strict_text(evidence / "13-gate7-sdk-package.stdout", "gate-7 stdout")
    for marker in (
        "SDK publishable-tarball gate passed",
        "staged 6 Engine V1 artifacts and package manifest",
    ):
        if marker not in package_output:
            raise VerificationError(f"gate-7 output lacks {marker}")
    require_gate7_marker(package_output)


def verify_playwright_record(evidence: pathlib.Path) -> None:
    command = strict_text(evidence / "05-playwright-api.command", "Playwright command")
    require_playwright_command(command)
    if command != PLAYWRIGHT_COMMAND:
        raise VerificationError("Playwright API command mismatch")
    strict_status(evidence / "05-playwright-api.status", "0")
    parse_meta(
        evidence / "05-playwright-api.meta",
        {"cwd", "start_utc", "finish_utc", "status"},
        str(SOURCE_CANDIDATE),
        "0",
    )
    strict_text(evidence / "05-playwright-api.stderr", "Playwright stderr")
    rows = strict_text(evidence / "05-playwright-api.stdout", "Playwright stdout").splitlines()
    if len(rows) != 3:
        raise VerificationError("Playwright API did not record exactly three browsers")
    names: set[str] = set()
    paths: set[str] = set()
    for row in rows:
        parts = row.split("\t")
        if len(parts) != 2 or parts[0] in names or not parts[1] or not pathlib.Path(parts[1]).is_absolute():
            raise VerificationError("malformed Playwright executable record")
        names.add(parts[0])
        paths.add(parts[1])
    if names != {"chromium", "firefox", "webkit"} or len(paths) != 3:
        raise VerificationError("Playwright executable identity is incomplete")


def verify_overlay_records(evidence: pathlib.Path) -> None:
    if strict_text(evidence / "04-overlay-verifier.command", "initial verifier command") != OVERLAY_COMMAND + "\n":
        raise VerificationError("initial overlay command mismatch")
    strict_status(evidence / "04-overlay-verifier.status", "0")
    if not strict_text(evidence / "04-overlay-verifier.stdout", "initial verifier stdout").startswith("PASS paths=12195 overlay=1"):
        raise VerificationError("initial overlay verifier did not report all tracked paths")
    strict_text(evidence / "04-overlay-verifier.stderr", "initial verifier stderr")
    # The frozen initial record predates cwd emission.  Validate its complete
    # three-key schema and timestamps, while making no unsupported cwd claim.
    parse_meta(
        evidence / "04-overlay-verifier.meta",
        {"start_utc", "finish_utc", "status"},
        None,
        "0",
    )
    if strict_text(evidence / "15-final-overlay-verifier.command", "final verifier command") != OVERLAY_COMMAND + "\n":
        raise VerificationError("final overlay command mismatch")
    strict_status(evidence / "15-final-overlay-verifier.status", "1")
    strict_text(evidence / "15-final-overlay-verifier.stdout", "final verifier stdout")
    parse_meta(
        evidence / "15-final-overlay-verifier.meta",
        {"cwd", "start_utc", "finish_utc", "status"},
        RETAINED_VERIFIER_CWD,
        "1",
    )
    final_stderr = strict_text(evidence / "15-final-overlay-verifier.stderr", "final verifier stderr")
    if "missing or extra tracked path" not in final_stderr:
        raise VerificationError("final verifier failure is not the retained SDK/dist disposition")


def verify_retained_evidence(evidence: pathlib.Path, artifact_hashes: dict[str, str]) -> None:
    verify_playwright_record(evidence)
    verify_gate_evidence(evidence, artifact_hashes)
    verify_overlay_records(evidence)


def file_sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
    except OSError as error:
        raise VerificationError(f"cannot hash file: {path}") from error
    return digest.hexdigest()


def census_root(root: pathlib.Path) -> list[str]:
    ordinary_directory(root, "census root")
    rows: list[str] = []

    def visit(base: pathlib.Path, relbase: str) -> None:
        try:
            entries = sorted(os.scandir(base), key=lambda entry: entry.name)
        except OSError as error:
            raise VerificationError(f"census traversal failed: {base}") from error
        for entry in entries:
            rel = f"{relbase}/{entry.name}".strip("/")
            try:
                info = entry.stat(follow_symlinks=False)
            except OSError as error:
                raise VerificationError(f"census entry cannot be stated: {rel}") from error
            mode = stat.S_IMODE(info.st_mode)
            if stat.S_ISDIR(info.st_mode):
                rows.append(f"d\t{mode:04o}\t{info.st_size}\t-\t{rel}")
                visit(pathlib.Path(entry.path), rel)
            elif stat.S_ISREG(info.st_mode):
                rows.append(f"f\t{mode:04o}\t{info.st_size}\t{file_sha256(pathlib.Path(entry.path))}\t{rel}")
            elif stat.S_ISLNK(info.st_mode):
                target = os.fsencode(os.readlink(entry.path))
                rows.append(f"l\t{mode:04o}\t{info.st_size}\t{hashlib.sha256(target).hexdigest()}\t{rel}")
            else:
                raise VerificationError(f"census contains a special entry: {rel}")

    visit(root, "")
    return rows


def preserved_target_identity(
    root: pathlib.Path = PRESERVED_TARGET,
    expected_digest: str | None = TARGET_DIGEST,
) -> str:
    ordinary_directory(root, "preserved target")
    command = [
        "tar", "--sort=name", "--mtime=@0", "--owner=0", "--group=0",
        "--numeric-owner", "-C", str(root.parent), "-cf", "-", root.name,
    ]
    try:
        process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    except OSError as error:
        raise VerificationError("cannot start target identity command") from error
    digest = hashlib.sha256()
    assert process.stdout is not None
    while True:
        chunk = process.stdout.read(1024 * 1024)
        if not chunk:
            break
        digest.update(chunk)
    stderr = process.stderr.read() if process.stderr is not None else b""
    status = process.wait()
    if status != 0:
        raise VerificationError(f"target identity command failed: {stderr[:200]!r}")
    value = digest.hexdigest()
    if expected_digest is not None and value != expected_digest:
        raise VerificationError(f"preserved target digest mismatch: {value}")
    return value


def capture_nine_root_census(phase: str) -> tuple[tuple[tuple[str, tuple[str, ...]], ...], tuple[str, ...], str]:
    roots: list[tuple[str, tuple[str, ...]]] = []
    for label, raw_path in ROOT_CENSUS:
        root = pathlib.Path(raw_path)
        rows = tuple(census_root(root))
        roots.append((label, rows))
        print(f"{phase} ROOT {label} {raw_path}")
        for row in rows:
            print(row)
    target_rows = tuple(census_root(PRESERVED_TARGET))
    print(f"{phase} PRESERVED_TARGET {PRESERVED_TARGET}")
    for row in target_rows:
        print(row)
    target_digest = preserved_target_identity(PRESERVED_TARGET, TARGET_DIGEST)
    print(f"{phase} PRESERVED_TARGET_TAR_SHA256 {target_digest}")
    return tuple(roots), target_rows, target_digest


def verify_all(
    repo: pathlib.Path,
    ref: str,
    pristine: pathlib.Path,
    candidate: pathlib.Path,
    artifact: pathlib.Path,
    evidence: pathlib.Path,
    lineage: tuple[str, str, str] = (NEW_COMMIT, NEW_WASM, NEW_SENTENCE),
    artifact_hashes: dict[str, str] = ARTIFACT_HASHES,
    dist_expectations: dict[str, tuple[str, str, int]] = DIST_EXPECTATIONS,
    require_authority: bool = True,
    emit_inventory: bool = True,
) -> None:
    if require_authority and ref != AUTHORITY_COMMIT:
        raise VerificationError("wrong authority")
    initial_census = capture_nine_root_census("INITIAL") if require_authority else None
    verify_export(
        repo, ref, pristine, False, artifact, artifact_hashes,
        evidence, False, dist_expectations, require_authority,
    )
    verify_export(
        repo, ref, candidate, True, artifact, artifact_hashes,
        evidence, emit_inventory, dist_expectations, require_authority,
    )
    verify_candidate_lineage(pristine, candidate, lineage)
    if require_authority:
        verify_retained_evidence(evidence, artifact_hashes)
        final_census = capture_nine_root_census("FINAL")
        if final_census != initial_census:
            raise VerificationError("retained-input census changed during verification")
        print("PASS authority/pristine/candidate/artifact/all-eight-gates/nine-root-census")
    else:
        print("PASS authority/pristine/candidate/artifact")


def assert_reject(action) -> None:
    try:
        action()
    except VerificationError:
        return
    raise AssertionError("negative control unexpectedly passed")


def require_playwright_command(command: str) -> None:
    if command != PLAYWRIGHT_COMMAND_EXPECTED:
        raise VerificationError("Playwright command bytes are not the reviewed literal")


GATE7_MARKER = "sdk generated surface is the engine's current output"


def require_gate7_marker(output: str) -> None:
    if GATE7_MARKER not in output:
        raise VerificationError("gate-7 output lacks the exact lowercase generated-surface marker")


def self_test_timestamp_controls(base: pathlib.Path) -> None:
    equal = base / "equal.meta"
    equal.write_text(
        "cwd=/tmp/synthetic\n"
        "start_utc=2026-01-01T00:00:01Z\n"
        "finish_utc=2026-01-01T00:00:01Z\n"
        "status=0\n"
    )
    parse_meta(equal, {"cwd", "start_utc", "finish_utc", "status"}, "/tmp/synthetic", "0")
    equal.unlink()
    reversed_meta = base / "reversed.meta"
    reversed_meta.write_text(
        "cwd=/tmp/synthetic\n"
        "start_utc=2026-01-01T00:00:02Z\n"
        "finish_utc=2026-01-01T00:00:01Z\n"
        "status=0\n"
    )
    assert_reject(lambda: parse_meta(
        reversed_meta, {"cwd", "start_utc", "finish_utc", "status"}, "/tmp/synthetic", "0"
    ))
    reversed_meta.unlink()
    timezone_free = base / "timezone-free.meta"
    timezone_free.write_text(
        "cwd=/tmp/synthetic\n"
        "start_utc=2026-01-01T00:00:01\n"
        "finish_utc=2026-01-01T00:00:01\n"
        "status=0\n"
    )
    assert_reject(lambda: parse_meta(
        timezone_free, {"cwd", "start_utc", "finish_utc", "status"}, "/tmp/synthetic", "0"
    ))
    timezone_free.unlink()


@contextlib.contextmanager
def exact_self_test_control():
    if os.path.lexists(SELF_TEST_CONTROL):
        raise VerificationError(f"self-test control path already exists: {SELF_TEST_CONTROL}")
    try:
        SELF_TEST_CONTROL.mkdir(mode=0o700)
    except OSError as error:
        raise VerificationError(f"cannot create self-test control path: {SELF_TEST_CONTROL}") from error
    try:
        info = SELF_TEST_CONTROL.lstat()
        if not stat.S_ISDIR(info.st_mode) or stat.S_ISLNK(info.st_mode) or stat.S_IMODE(info.st_mode) != 0o700:
            raise VerificationError("self-test control path is not a fresh 0700 directory")
        yield SELF_TEST_CONTROL
    finally:
        try:
            entries = list(os.scandir(SELF_TEST_CONTROL))
        except OSError as error:
            raise VerificationError("cannot inspect self-test control cleanup") from error
        if entries:
            raise VerificationError("self-test control directory was not cleaned")
        try:
            SELF_TEST_CONTROL.rmdir()
        except OSError as error:
            raise VerificationError("cannot remove self-test control directory") from error
        if os.path.lexists(SELF_TEST_CONTROL):
            raise VerificationError("self-test control path remains after cleanup")


def synthetic_self_test() -> None:
    global ROOT_CENSUS, PRESERVED_TARGET, TARGET_DIGEST
    with exact_self_test_control() as control, tempfile.TemporaryDirectory(
        prefix="run-", dir=str(control)
    ) as temp:
        base = pathlib.Path(temp)
        if base.parent != control or base.is_symlink() or not base.is_dir():
            raise VerificationError("self-test tempfile escaped its literal control directory")
        self_test_timestamp_controls(base)
        repo = base / "repo"
        pristine = base / "pristine"
        candidate = base / "candidate"
        artifact = base / "artifact"
        evidence = base / "evidence"
        repo.mkdir()
        artifact.mkdir()
        evidence.mkdir()
        subprocess.run(["git", "init", "-q", str(repo)], check=True)
        tracked = {
            "plain.txt": b"plain\n",
            PIN: (OLD_WASM + "\n").encode(),
            RESULTS: (json.dumps({"candidateCommit": OLD_COMMIT, "wasmSha256": OLD_WASM, "rows": [1]}, indent=2) + "\n").encode(),
            MATRIX: (OLD_SENTENCE + "\n").encode(),
            "sdk/package.json": b"{}\n",
            "sdk/tsconfig.build.json": b"{}\n",
            "hosts/host-web/qualification/package.json": b"{}\n",
            "link": b"plain.txt",
        }
        for name, data in tracked.items():
            item = repo / name
            item.parent.mkdir(parents=True, exist_ok=True)
            if name == "link":
                item.symlink_to(data.decode().strip())
            else:
                item.write_bytes(data)
        subprocess.run(["git", "-C", str(repo), "add", "."], check=True)
        tree = git(repo, "write-tree").decode("ascii").strip()
        env = os.environ | {
            "GIT_AUTHOR_NAME": "issue679-selftest",
            "GIT_AUTHOR_EMAIL": "issue679-selftest@example.invalid",
            "GIT_COMMITTER_NAME": "issue679-selftest",
            "GIT_COMMITTER_EMAIL": "issue679-selftest@example.invalid",
        }
        commit = subprocess.run(
            ["git", "-C", str(repo), "commit-tree", tree, "-m", "synthetic"],
            check=True, stdout=subprocess.PIPE, env=env, text=True,
        ).stdout.strip()
        pristine.mkdir()
        archive = subprocess.run(
            ["git", "-C", str(repo), "archive", "--format=tar", commit],
            check=True, stdout=subprocess.PIPE,
        ).stdout
        subprocess.run(["tar", "-xf", "-", "-C", str(pristine)], input=archive, check=True)
        shutil.copytree(pristine, candidate, symlinks=True)
        (candidate / PIN).write_bytes((NEW_WASM + "\n").encode())
        result_bytes = (candidate / RESULTS).read_bytes()
        result_bytes = replace_json_string(result_bytes, "candidateCommit", OLD_COMMIT, NEW_COMMIT)
        result_bytes = replace_json_string(result_bytes, "wasmSha256", OLD_WASM, NEW_WASM)
        (candidate / RESULTS).write_bytes(result_bytes)
        (candidate / MATRIX).write_bytes((candidate / MATRIX).read_bytes().replace(OLD_SENTENCE.encode(), NEW_SENTENCE.encode(), 1))
        (candidate / "sdk/node_modules").mkdir(parents=True)
        (candidate / "hosts/host-web/qualification/node_modules").mkdir(parents=True)
        dist = candidate / "sdk/dist"
        assets = dist / "assets"
        assets.mkdir(parents=True)
        for name in ARTIFACT_NAMES:
            data = ("artifact:" + name).encode()
            (artifact / name).write_bytes(data)
            (assets / name).write_bytes(data)
        for index in range(71):
            item = dist / f"generated/{index:02d}.js"
            item.parent.mkdir(parents=True, exist_ok=True)
            item.write_bytes(f"generated-{index}\n".encode())
        synthetic_hashes = {name: file_sha256(artifact / name) for name in ARTIFACT_NAMES}
        synthetic_expectations = {
            p.relative_to(candidate).as_posix():
            ("synthetic", file_sha256(p), stat.S_IMODE(p.stat().st_mode))
            for p in dist.rglob("*") if p.is_file() and not p.is_symlink()
        }
        gate_meta = (
            f"cwd={SOURCE_CANDIDATE}\n"
            f"CARGO_TARGET_DIR={TARGET_ENV}\n"
            "start_utc=2026-01-01T00:00:00Z\n"
            "finish_utc=2026-01-01T00:00:01Z\n"
            "status=0\n"
        )
        postcheck_stdout = (
            "source_target=absent\nartifact_hashes:\n"
            + "\n".join(synthetic_hashes.values())
            + "\ntarget_identity=manifests:\n"
            + "\n".join(
                f"{digest}  {path}" for path, digest in PACKAGE_MANIFEST_HASHES.items()
            )
            + "\n"
        )
        for gate_prefix, postcheck_prefix, command in GATE_SPECS:
            (evidence / f"{gate_prefix}.command").write_text(command + "\n")
            (evidence / f"{gate_prefix}.meta").write_text(gate_meta)
            gate_output = "gate passed\n"
            if gate_prefix == "13-gate7-sdk-package":
                gate_output = (
                    "SDK publishable-tarball gate passed\n"
                    "staged 6 Engine V1 artifacts and package manifest\n"
                    "sdk generated surface is the engine's current output\n"
                )
            elif gate_prefix == "14-gate8-qualification":
                gate_output = (
                    "session identities: 3 qualification documents declare their fed PCM\n"
                    "artifact set: the exact 6-file shipped set is pinned\n"
                    "chromium: all qualification gates passed (151.0.7922.34)\n"
                    "firefox: all qualification gates passed (153.0)\n"
                    "webkit: all qualification gates passed (26.5)\n"
                )
            (evidence / f"{gate_prefix}.stdout").write_text(gate_output)
            (evidence / f"{gate_prefix}.stderr").write_text("")
            (evidence / f"{gate_prefix}.status").write_text("0\n")
            (evidence / f"{postcheck_prefix}.command").write_text(POSTCHECK_COMMAND + "\n")
            (evidence / f"{postcheck_prefix}.stdout").write_text(postcheck_stdout)
            (evidence / f"{postcheck_prefix}.stderr").write_text(f"{TARGET_DIGEST}\n")
            (evidence / f"{postcheck_prefix}.status").write_text("0\n")
        require_gate7_marker("sdk generated surface is the engine's current output\n")
        assert_reject(lambda: require_gate7_marker(
            "SDK generated surface is the engine's current output\n"
        ))
        (evidence / "05-playwright-api.command").write_text(PLAYWRIGHT_COMMAND)
        require_playwright_command(PLAYWRIGHT_COMMAND)
        assert_reject(lambda: require_playwright_command(
            PLAYWRIGHT_COMMAND.replace("node - <<'NODE'", "node - <<'NODE", 1)
        ))
        (evidence / "05-playwright-api.meta").write_text(
            f"cwd={SOURCE_CANDIDATE}\n"
            "start_utc=2026-01-01T00:00:00Z\n"
            "finish_utc=2026-01-01T00:00:01Z\n"
            "status=0\n"
        )
        (evidence / "05-playwright-api.stdout").write_text(
            "chromium\t/tmp/chromium\nfirefox\t/tmp/firefox\nwebkit\t/tmp/webkit\n"
        )
        (evidence / "05-playwright-api.stderr").write_text("")
        (evidence / "05-playwright-api.status").write_text("0\n")
        (evidence / "04-overlay-verifier.command").write_text(OVERLAY_COMMAND + "\n")
        (evidence / "04-overlay-verifier.stdout").write_text("PASS paths=12195 overlay=1\n")
        (evidence / "04-overlay-verifier.stderr").write_text("")
        (evidence / "04-overlay-verifier.status").write_text("0\n")
        (evidence / "04-overlay-verifier.meta").write_text(
            "start_utc=2026-01-01T00:00:00Z\n"
            "finish_utc=2026-01-01T00:00:01Z\n"
            "status=0\n"
        )
        (evidence / "15-final-overlay-verifier.command").write_text(OVERLAY_COMMAND + "\n")
        (evidence / "15-final-overlay-verifier.stdout").write_text("")
        (evidence / "15-final-overlay-verifier.stderr").write_text("missing or extra tracked path\n")
        (evidence / "15-final-overlay-verifier.status").write_text("1\n")
        (evidence / "15-final-overlay-verifier.meta").write_text(
            f"cwd={RETAINED_VERIFIER_CWD}\n"
            "start_utc=2026-01-01T00:00:00Z\n"
            "finish_utc=2026-01-01T00:00:01Z\n"
            "status=1\n"
        )
        verify_retained_evidence(evidence, synthetic_hashes)
        missing_postcheck = evidence / "08-gate2-postcheck.command"
        missing_postcheck.unlink()
        assert_reject(lambda: verify_retained_evidence(evidence, synthetic_hashes))
        missing_postcheck.write_text(POSTCHECK_COMMAND + "\n")
        saved_roots, saved_target, saved_digest = ROOT_CENSUS, PRESERVED_TARGET, TARGET_DIGEST
        synthetic_roots = tuple(
            (f"synthetic-{index}", str(base / f"census-{index}")) for index in range(9)
        )
        for _, raw_root in synthetic_roots:
            root = pathlib.Path(raw_root)
            root.mkdir()
            (root / "record").write_text("synthetic\n")
        synthetic_target = base / "target"
        synthetic_target.mkdir()
        (synthetic_target / "record").write_text("target\n")
        ROOT_CENSUS = synthetic_roots
        PRESERVED_TARGET = synthetic_target
        TARGET_DIGEST = preserved_target_identity(synthetic_target, None)
        initial_census = capture_nine_root_census("INITIAL")
        final_census = capture_nine_root_census("FINAL")
        if final_census != initial_census:
            raise AssertionError("synthetic census changed without mutation")
        missing_root = pathlib.Path(synthetic_roots[-1][1]) / "record"
        missing_root.unlink()
        changed_census = capture_nine_root_census("CHANGED")
        if changed_census == initial_census:
            raise AssertionError("synthetic census mutation was not observed")
        missing_root.write_text("synthetic\n")
        ROOT_CENSUS, PRESERVED_TARGET, TARGET_DIGEST = saved_roots, saved_target, saved_digest
        verify_all(repo, commit, pristine, candidate, artifact, evidence,
                   (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                   synthetic_expectations, False, False)
        (candidate / "plain.txt").write_bytes(b"changed\n")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        (candidate / "plain.txt").write_bytes(b"plain\n")
        extra_file = candidate / "fourth-overlay-file"
        extra_file.write_text("bad\n")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        extra_file.unlink()
        extra_root = candidate / "fourth-overlay-directory"
        extra_root.mkdir()
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        extra_root.rmdir()
        extra = dist / "generated/extra.js"
        removed = dist / "generated/70.js"
        removed_data = removed.read_bytes()
        removed.unlink()
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        removed.write_bytes(removed_data)
        extra.write_text("extra\n")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        extra.unlink()
        symlinked = dist / "generated/00.js"
        symlink_data = symlinked.read_bytes()
        symlinked.unlink()
        symlinked.symlink_to("../assets/" + next(iter(ARTIFACT_NAMES)))
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        symlinked.unlink()
        symlinked.write_bytes(symlink_data)
        packaged = assets / next(iter(ARTIFACT_NAMES))
        packaged_data = packaged.read_bytes()
        packaged.write_bytes(b"changed artifact\n")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        packaged.write_bytes(packaged_data)
        artifact_extra = artifact / "extra-dir"
        artifact_extra.mkdir()
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        artifact_extra.rmdir()
        artifact_link = artifact / "extra-link"
        artifact_link.symlink_to(next(iter(ARTIFACT_NAMES)))
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        artifact_link.unlink()
        dependency = candidate / "sdk/node_modules"
        dependency.rmdir()
        dependency.symlink_to(candidate / "hosts/host-web/qualification/node_modules")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        dependency.unlink()
        dependency.mkdir()
        original_results = (candidate / RESULTS).read_bytes()
        duplicate_results = original_results.replace(b'  "rows": [', b'  "rows": [2],\n  "rows": [', 1)
        (candidate / RESULTS).write_bytes(duplicate_results)
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        (candidate / RESULTS).write_bytes(original_results + b"\n")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        (candidate / RESULTS).write_bytes(original_results)
        original_matrix = (candidate / MATRIX).read_bytes()
        (candidate / MATRIX).write_bytes(original_matrix + b"unexpected\n")
        assert_reject(lambda: verify_all(repo, commit, pristine, candidate, artifact, evidence,
                                          (NEW_COMMIT, NEW_WASM, NEW_SENTENCE), synthetic_hashes,
                                          synthetic_expectations, False, False))
        (candidate / MATRIX).write_bytes(original_matrix)
        bad_args = subprocess.run(
            [sys.executable, "-B", str(pathlib.Path(__file__).resolve()), "--self-test", "extra"],
            check=False, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        )
        if bad_args.returncode == 0:
            raise AssertionError("--self-test accepted positional paths")
    print(
        "PASS self-test commit/arguments/overlays/all-eight-gates/nine-root-census/"
        "76-78/sdk-symlink/artifact-special/JSON"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("paths", nargs="*")
    args = parser.parse_args()
    if args.self_test:
        if args.paths:
            parser.error("--self-test takes no paths")
        synthetic_self_test()
        return 0
    if len(args.paths) != 6:
        parser.error("REPO COMMIT PRISTINE CANDIDATE ARTIFACT RETAINED_EVIDENCE required")
    repo = pathlib.Path(args.paths[0])
    ref = args.paths[1]
    pristine, candidate, artifact, evidence = map(pathlib.Path, args.paths[2:])
    if ref != AUTHORITY_COMMIT:
        raise SystemExit("wrong authority commit")
    expected_paths = (
        ("authority repository", repo, AUTHORITY_REPO),
        ("pristine export", pristine, pathlib.Path("/tmp/issue672-attempt2-candidate-pristine")),
        ("candidate export", candidate, SOURCE_CANDIDATE),
        ("candidate artifact", artifact, pathlib.Path("/tmp/issue672-attempt3-candidate-artifact")),
        ("retained evidence", evidence, RETAINED_EVIDENCE),
    )
    for label, actual, expected in expected_paths:
        if actual.resolve() != expected.resolve():
            raise SystemExit(f"wrong {label} path")
    # The production constants are checked independently before any inventory is emitted.
    if set(ARTIFACT_HASHES) != ARTIFACT_NAMES or len(DIST_EXPECTATIONS) != 77:
        raise SystemExit("internal artifact table mismatch")
    artifact_hashes = ARTIFACT_HASHES
    dist_expectations = DIST_EXPECTATIONS
    verify_all(repo, ref, pristine, candidate, artifact, evidence,
               artifact_hashes=artifact_hashes,
               dist_expectations=dist_expectations,
               require_authority=True)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except VerificationError as error:
        print(f"FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
```

## Evidence review and promotion

Astra LOW must return EVIDENCE PASS on exact immutable attempt evidence before
any repository edit. EVIDENCE PASS qualifies the retained candidate only. Root
then appends and pushes a separate promotion amendment and obtains fresh Astra
LOW SCOPE PASS. Promotion may edit only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`;
- `hosts/host-web/qualification/results.json`, only `candidateCommit` and
  `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, only regenerated lineage;
- issue #680 and concise #559/#560 records.

The post-pin stage runs one ordinary no-bypass build in fresh external paths and
requires all six outputs byte-identical to the qualified candidate. It does not
repeat browser or SDK qualification. Astra LOW reviews exact head/current main.
Root opens one PR, waits for required `qualification`, performs guarded merge
review, merges, verifies post-main qualification, synchronizes #669/#670/#680/
#559/#560 and #349 accounting, closes #680, and removes only clean delivered
worktrees. Failed predecessor worktrees remain until delivery evidence is secure.

## Attempt and acceptance rules

Attempt 1 is consumed by its exclusive partial preflight and receives no credit.
Attempt 2 gets one reviewed direct flow, one verifier self-test, one
production invocation, and one manifest flow. Any failure stops it and requires
Astra LOW adversarial review plus a pushed, synchronized amendment before final
attempt 3. After three failed attempts, stop and rescope; never weaken a gate,
reuse a consumed path, append an `a`/`b` suffix to avoid counting an attempt, or
disguise a fourth retry.

Acceptance requires: exact classification of all 77 SDK files; exact tracked,
overlay, artifact, target, and retained eight-gate identities; immutable manifest
verification; Astra LOW EVIDENCE PASS; separately reviewed promotion; ordinary
post-pin six-file byte identity; required PR and post-main qualification success;
GitHub synchronization and closure; and zero generated/compiler evidence payloads
in Git.
