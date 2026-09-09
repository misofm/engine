# Qualify the frozen continuous-mapping extraction

GitHub: https://github.com/misofm/engine/issues/670

Parent: #560 (lane B, CP8)

Supersedes stopped #669 for qualification only. #669 exhausted three test and
evidence attempts without SOURCE PASS. Its production extraction is nevertheless
semantically correct per three Astra LOW reviews: `effect-contract` owns pure
`continuous_mapping_admissible`, and the typed and borrowed validators call it
at the former duplicated mapping/minimum condition without moving surrounding
validation. This successor qualifies those frozen source bytes with independently
valid public fixtures. It is a new bounded workflow after the required rescope,
not a fourth #669 attempt.

Sol HIGH coordinates documentation, checkpoints, GitHub synchronization, and
artifact qualification/pinning. The sole implementation executor is Luna HIGH
`/root/issue583_luna_impl`. Per current user routing, Astra LOW performs every
scope, source, exact-head, integration, artifact-applicability, and delivery
verification that would otherwise use Sol HIGH/XHIGH. #668 and #669 are closed;
#670 is the sole active audit child at opening.

## Frozen base and ownership

The isolated successor branch starts at #669 disposition
`c139a0f0788a1e3b1b023ed14dce214b1aa79020`, whose merge-base with main is
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`. Frozen production hashes are:

- `crates/effect-contract/src/lib.rs`:
  `dfe5ba9b47e65998e997f7485e6b8bccbab666cc02c2e1b0f5a6036d0509d8b1`
- `crates/effect-package/src/wire.rs`:
  `a9284433c1ee5a802cd99bbcdd911d48d8c099c6627774f397541a69d81a57e0`

Only inline `#[cfg(test)]` modules in those two source files, mechanical
formatting of those tests, and this numbered spec/evidence may change. The helper,
both production callers, imports needed by production, descriptors, diagnostics,
wire format, formulas, dependencies, lockfile, other tests, artifacts, pins,
workflows, and policies are frozen. Preserve #669's three worktree branches,
temporary evidence/targets, and exact failed patches unchanged.

## Fixture contract

Use one literal nine-minimum sequence for every mapping: negative finite `-1.0`,
negative zero, positive zero, smallest positive subnormal `f32::from_bits(1)`,
smallest positive normal `f32::MIN_POSITIVE`, ordinary positive `1.0`, NaN,
negative infinity, and positive infinity. Every numeric binding is explicitly
`f32`. The full matrix is four mappings by nine minima, exactly 36 cases.

For canonical finite minima at or below zero, use maximum `1.0` and default
`0.5`. For ordinary positive `1.0`, use maximum `2.0` and default `1.5`. The two
tiny positive values may use maximum `1.0` and default `0.5`; they deliberately
reach the independent decimal-lattice rule. Nonfinite spellings are parser
controls and keep otherwise fixed finite maximum/default fields. Derive the
declared lattice for the case's unit/domain/mapping exactly as production encoding
does; do not reuse a lattice derived for another mapping.

The typed projection must build an otherwise-valid public `EffectDescriptor`:
one required dual-mono main input, one required dual-mono main output, Normal
quality rows at 44.1/48/88.2/96 kHz with valid state/latency/tail/scratch, valid
link modes, and exactly the subject parameter. Call public `validate_descriptor`.
For each case assert the exact `parameters`/`Parameter` diagnostic presence or
absence. Fully lattice-valid admissible cases must return `Ok(())`; the two tiny
admissible minima must have no Parameter diagnostic and the exact coupled Lattice
diagnostic. Invalid mapping cases must carry the exact Parameter diagnostic; any
coupled Lattice diagnostic must be stated literally rather than ignored.

The borrowed projection must call only public
`verify_effect_descriptor_wire`; private semantic views/predicates are forbidden.
Assert exact public outcomes for all 36 cases:

- negative zero and all nonfinite minima: `Code::Float`, byte offset
  `HEADER_BYTES + 36`, record index 0;
- canonical finite mapping-law failures: `Code::Semantic`, byte offset
  `HEADER_BYTES + 4`, record index 0;
- the two tiny values with Linear, Logarithmic, or Exponential:
  `Code::Semantic`, byte offset `HEADER_BYTES + 72`, record index 0, proving the
  mapping law passed before the lattice law rejected their decimal spelling;
- lattice-valid Linear/Exponential cases at `-1.0`, positive zero, and `1.0`,
  plus Logarithmic at `1.0`: successful verification with exact input bytes.

Add a combined nonfinite-minimum/mapping-failure control proving the earlier
Float diagnostic. For each successful case, independently compute SHA-256 over
`IDENTITY_DOMAIN`, the little-endian `u64` input length, and exact input bytes,
then compare with `verified.identity().as_bytes()`. Do not use
`effect_descriptor_identity` to compute expected bytes. Retain and explicitly
exercise the existing phase-order, overflow/tie-break, typed/borrowed differential,
roundtrip/identity, and binding-mismatch tests named in #669.

## Attempt 1 execution contract

Before any edit, Luna must record exact clean HEAD/upstream, merge-base/live
main, cwd, `rustc --version`, `cargo --version`, frozen production hashes, and
the explicit unset/value state of `CARGO_TARGET_DIR`, `RUSTFLAGS`,
`RUSTDOCFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `CC`, `CFLAGS`, and
`SOURCE_DATE_EPOCH`. Verify every #669 path is preserved. These five #670 paths
must be absent and non-symlinks:

- `/tmp/issue670-attempt1-evidence`
- `/tmp/issue670-attempt1-target`
- `/tmp/issue670-attempt1-manifest-record.txt`
- `/tmp/issue670-attempt1-manifest-verify.stdout`
- `/tmp/issue670-attempt1-manifest-verify.status`

Create only the evidence and target directories. Prove harmless success status
0 and harmless failure status 1, persist and independently read back both. After
test edits and mechanical formatting, run these commands exactly once in order,
exporting `CARGO_TARGET_DIR=/tmp/issue670-attempt1-target` for every Cargo command:

1. `cargo test --locked -p effect-contract`
2. `cargo test --locked -p effect-package`
3. `cargo test --locked --release -p effect-contract`
4. `cargo test --locked --release -p effect-package`
5. `cargo clippy --locked -p effect-contract -p effect-package --all-targets --all-features -- -D warnings`
6. `cargo fmt --all -- --check`
7. `bash scripts/check-effect-runtime-policy.sh`
8. `bash scripts/check-effect-package-v1.sh`
9. `bash scripts/check-effect-descriptor-v1.sh`
10. `bash scripts/check-workspace-policy.sh`
11. `git diff --check`

Capture every exact command, separate stdout/stderr, and numeric status under the
evidence directory. Stop on the first unexpected status without correction,
retry, or later gate. If all pass, finalize every file in the evidence directory,
write its self-excluding `SHA256SUMS`, and place all manifest command/completion,
verification output, and numeric status only in the three sibling paths. Current
`sha256sum -c` must pass every entry. Commit no temporary evidence, target output,
`.ll`, `.s`, or full compiler stream.

Implementation remains unauthorized until the clean pushed brief and synchronized
#559/#560/#670 bodies receive Astra LOW scope PASS. One Luna attempt is initially
available; subsequent attempts follow the repository's three-attempt rule. After
SOURCE PASS, root owns artifact applicability and any required qualification/pin
successor. No artifact, pin, PR, or merge work is included in this brief.

## Astra LOW scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean feature HEAD/upstream
`a68e418232d9512a86df4cfd2901ee0ec33a66a3`, main merge-base
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`, synchronized #670/#559/#560
bodies, and all five fresh paths absent including symlinks. Independent source
inspection confirmed precision-1 dB Linear/Exponential accept the proposed
ordinary values; logarithmic ratio 1.02/precision 3 accepts `1.0/2.0/1.5`; and
both tiny positive minima retain the intended later-lattice rejection.

Only Luna HIGH `/root/issue583_luna_impl` is authorized for one qualification-
only attempt limited to inline tests and formatting with production frozen. The
literal typed diagnostic sets and all 36 public borrowed outcomes are mandatory.
Run the eleven gates once under the evidence contract and stop on the first
unexpected failure without correction or retry. Preserve all #669 history and
temporary records. No artifact, pin, PR, or merge work is authorized.

## Luna HIGH attempt 1 checkpoint — FAIL

Luna recorded the required clean pre-edit provenance and numeric 0/1 controls at
authorization head `3d8c1ee025b15a8f0ef412fd8c3fdee2b7fa8c17`, preserved all #669
records, and changed only the two authorized inline test modules plus their
mechanical formatting. Production helper, callers, and imports remain frozen.

Both affected crates' full debug and release suites passed once. `effect-contract`
reported 14 unit tests plus integration groups of 12, 7, 9, and 0 passing in each
profile. `effect-package` reported 35 unit tests plus integration groups of 6, 2,
1, 5 with one ignored, 3, 2, 15, and 0 passing in each profile. Strict affected
Clippy then returned 101 for one `clippy::useless_conversion`: the typed matrix
called `.into_iter()` on `expected` inside `zip`, although `zip` accepts the array
directly. Luna stopped before formatting and policy gates 6–11, with no correction
or retry. Attempt 1 is consumed.

Temporary evidence and target output remain unchanged at
`/tmp/issue670-attempt1-evidence` and `/tmp/issue670-attempt1-target`; no final
manifest was produced after the failed gate. Test-checkpoint hashes are
`1b45ea20a10e283f21b242bfef90779c5148333f705efc43a062b37afeec2688` for
`effect-contract/src/lib.rs` and
`9a4e833512ab8f70bf4804fc149bfe21e2cb568eb6f53a707c212529e7e66818` for
`effect-package/src/wire.rs`. This compiling, debug/release-green checkpoint is
preserved unchanged for Astra LOW source/failure review. No artifact, pin, PR,
merge, compiler payload, or SOURCE PASS is claimed.

## Astra LOW attempt 1 review — FAIL

Astra LOW returned **ATTEMPT-1 FAIL** at exact clean HEAD/upstream
`7752376e176a6ea151480273410af1e44035dd15`. The four test commands passed and
strict Clippy stopped only at the redundant `expected.into_iter()` expression.
Production is unchanged. Both 36-case tables, public borrowed diagnostics,
tiny-minimum lattice distinction, accepted exact bytes, independent SHA oracle,
and Float-before-Semantic controls are sound.

One test gap remains: typed validation reduces the public error collection to two
`.any()` booleans, so a rejected case could carry an unexpected extra diagnostic
without failing. Attempt 1 is consumed.

## Attempt 2 correction scope

Freeze production and every borrowed-wire test byte at checkpoint `7752376e`.
Only the typed inline test module and its mechanical formatting may change:

1. replace `.zip(expected.into_iter())` with `.zip(expected)`;
2. project the complete public typed error collection to ordered literal
   `(path, DescriptorDiagnosticCode)` pairs and compare it with the exact
   expected set for every one of the 36 cases; retained accepted cases must
   still require `validate_descriptor` to return `Ok(())`.

The literal typed sets are: no diagnostics for lattice-valid admissible cases;
only `("parameters", Lattice)` for the two tiny minima under Linear,
Logarithmic, or Exponential; and both `("parameters", Parameter)` then
`("parameters", Lattice)` for nonfinite/negative-zero minima, Logarithmic with
canonical nonpositive minimum, and every Stepped case. Do not ignore, filter, or
collapse another diagnostic. If actual public ordering differs, stop rather than
editing the expected set during the run.

Before edits, preserve attempt-1 paths and use fresh absent/non-symlink paths
`/tmp/issue670-attempt2-evidence`, `/tmp/issue670-attempt2-target`,
`/tmp/issue670-attempt2-manifest-record.txt`,
`/tmp/issue670-attempt2-manifest-verify.stdout`, and
`/tmp/issue670-attempt2-manifest-verify.status`. Record the same clean exact
provenance, production/borrowed hashes, safe environment fields, and numeric 0/1
capture controls.

Run these exact commands once in order, using the attempt-2 target for Cargo:

1. `cargo test --locked -p effect-contract continuous_mapping_validity_tests::`
2. `cargo test --locked --release -p effect-contract continuous_mapping_validity_tests::`
3. `cargo clippy --locked -p effect-contract -p effect-package --all-targets --all-features -- -D warnings`
4. `cargo fmt --all -- --check`
5. `bash scripts/check-effect-runtime-policy.sh`
6. `bash scripts/check-effect-package-v1.sh`
7. `bash scripts/check-effect-descriptor-v1.sh`
8. `bash scripts/check-workspace-policy.sh`
9. `git diff --check`

Carry forward the unchanged borrowed debug/release results from attempt 1; do not
rerun them. Apply the same separate-stream/status, first-failure stop, finalized
self-excluding manifest, and sibling verification rules as attempt 1. Preserve
attempt-1 evidence unchanged. No correction, retry, later gate, production edit,
artifact, pin, PR, merge, `.ll`, `.s`, or compiler-payload publication is
authorized.

Attempt 2 remains unauthorized until this amendment is pushed, synchronized,
and passes fresh Astra LOW scope review.

## Astra LOW attempt 2 scope review — PASS

Astra LOW returned **ATTEMPT-2 SCOPE PASS** at exact clean HEAD/upstream
`c1ae10df2416f3e519cacda36fb049504c501d36`. GitHub #670/#559/#560 match and
all five fresh paths are absent including symlinks. The literal public diagnostic
order is correct: Parameter precedes Lattice after the validator's final sort.

Only Luna HIGH `/root/issue583_luna_impl` may change the typed test module and its
formatting, remove the redundant conversion, and compare complete ordered error
collections. Production and borrowed tests remain frozen. Run the nine commands
once, preserve attempt-1 evidence, and stop at first failure without correction
or retry. No artifact, pin, PR, or merge is authorized.

## Luna HIGH attempt 2 checkpoint — PASS pending review

Luna changed only `effect-contract`'s typed inline test module. The redundant
iterator call is removed, and each of the 36 cases now compares public
`validate_descriptor` output with a complete ordered literal diagnostic set or
requires `Ok(())`. Production helper/callers/imports and all borrowed-wire source
remain byte-unchanged; `effect-package/src/wire.rs` retains SHA-256
`9a4e833512ab8f70bf4804fc149bfe21e2cb568eb6f53a707c212529e7e66818`.
The final `effect-contract/src/lib.rs` SHA-256 is
`be709c2293b108feccfe14b0049c08e32d09ce61188a865dca59fa6cee185f98`.

All nine attempt-2 commands returned 0 once: focused typed tests in debug and
release, strict affected Clippy, formatting, effect-runtime policy,
effect-package V1, effect-descriptor V1, workspace policy, and diff hygiene. The
focused test reported 1 pass in both profiles; attempt-1's unchanged full
borrowed debug/release results remain applicable.

Evidence is under `/tmp/issue670-attempt2-evidence` with target output under
`/tmp/issue670-attempt2-target`. Current independent manifest verification passes
every entry; `SHA256SUMS` has SHA-256
`4120fd24d202d35f9d5c4b9abbec081b9e58e92d4feb24d0a111765024bde317`, and
sibling verification status is 0. No temporary evidence, target output, compiler
payload, artifact, or pin is committed. This is a green implementation checkpoint,
not SOURCE PASS; Astra LOW must review the exact pushed head.

## Astra LOW attempt 2 source review — PASS

Astra LOW returned **SOURCE PASS** at exact clean HEAD/upstream
`fe6ddb4d1f1aadb254a2cd5e95732652fd457351` against recorded main
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`. Only the typed inline test changed
from attempt 1; production and borrowed tests remain frozen. The complete ordered
public diagnostic collections cover all 36 typed cases, reject unexpected errors,
and require `Ok(())` for accepted descriptors.

All nine attempt-2 gates returned 0. The current 36-entry evidence manifest
verifies and the sibling status is 0; unchanged full borrowed debug/release
results from attempt 1 remain applicable. No untracked deliverable exists.
SOURCE PASS is complete. PR readiness remains conditional on root's artifact-
applicability decision and final exact-head/current-main review. Preserve all
predecessor failures and temporary evidence; this verdict authorizes no artifact
execution or cleanup by an implementation agent.

## AudioWorklet artifact applicability scope

Root's post-SOURCE-PASS dependency trace shows `host-web` depends on changed
`effect-contract` through its compiler/runtime graph. The production helper can
therefore enter the shipped AudioWorklet even though #670 changes descriptor
validation rather than render arithmetic. Artifact applicability requires one
exact-source identity probe; no retained-artifact assumption is allowed.

The sole probe executor remains Luna HIGH `/root/issue583_luna_impl` after Astra
LOW scope PASS. At clean pushed #670 head, record HEAD/upstream, live main and
merge-base, cwd, Rust/Cargo versions, source hashes, exact 65-byte pin content and
newline shape, literal environment, clean tree, and unchanged builder/copied
inputs. Verify absence including dangling symlinks of
`/tmp/issue670-artifact-probe-output` and
`/tmp/issue670-artifact-probe-evidence`. Prove harmless capture statuses 0 and 1
and read both back, then create the output/evidence directories once. Immediately
before invocation, verify that the output path is an existing empty non-symlink
directory.

Run exactly once:

`MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue670-artifact-probe-output`

Require status 0, stdout exactly one lowercase 64-hex digest plus LF, an empty
output directory, unchanged repository/pin, and no competing Cargo/rustc process.
Retain complete stderr and inspect it for unrelated errors without requiring it
to be empty or suppressing the unchanged builder's normal Cargo progress. Capture
complete stdout/stderr/status/context/postflight and hashes in a finalized self-
excluding manifest whose verification status and hash are recorded outside the
evidence directory. Stop on any failed precondition or result without correction
or retry.

Do not run the ordinary builder, inspect or retain its temporary Cargo target,
create a six-file candidate, edit the pin, or invoke static/resource/browser/SDK
gates. If the digest matches the pin, Astra LOW decides whether prior qualified
artifact evidence carries. If it differs, this proves drift only; qualification
and pin promotion require a separately numbered issue when an active slot is
available. Full build streams and every generated artifact remain temporary. No
`.ll`, assembly, object, archive, binary, `rlib`, `rmeta`, Cargo target,
benchmark, timing, allocation, or performance evidence enters Git.

The probe is unauthorized until this pushed amendment and synchronized trackers
receive Astra LOW scope PASS. Source PASS alone does not authorize execution.

## Initial artifact-probe scope review — FAIL

Astra LOW returned **ARTIFACT-PROBE SCOPE FAIL** at exact clean HEAD/upstream
`901c49cab18efdd8326f2382d5c805c6642d2dc2`. The sole blocker was the brief's
incorrect empty-stderr requirement: the unchanged Cargo builder writes normal
progress to stderr. This correction retains and inspects complete stderr without
suppressing it, and explicitly checks the created output directory is existing,
empty, and non-symlink immediately before invocation. No probe ran.

## Corrected artifact-probe scope review — PASS

Astra LOW returned **ARTIFACT-PROBE SCOPE PASS** at exact clean pushed
HEAD/upstream `bdc8051fd81fb5052e8c131d7841a16b39d2037e`; synchronized tracker
HEAD/upstream was `395c1de18e3df46b1b9e7add8ef713f588ce11f3` and GitHub
#670/#559/#560 matched their local specs. Both fresh probe paths were absent,
including symlinks. The corrected scope preserves complete stderr and checks the
created output directory immediately before invocation. Only Luna HIGH
`/root/issue583_luna_impl` is authorized to perform the one declared repin-report
invocation after the specified fresh preflight. Stop on failure without retry.
No ordinary build, retained artifact/compiler payload, candidate qualification,
pin change, browser/SDK execution, commit, or push is authorized.

## Artifact identity probe result — DRIFT

Luna HIGH ran the authorized repin-report command exactly once at clean
HEAD/upstream `49f4161fc6e4b1f5d2c2107ca36353aebcd89dc4`. It returned status
0 and exact stdout digest
`93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`
plus LF. Complete stderr contains normal Cargo compilation progress and no
builder error. The output directory remained empty and non-symlink, the tree
remained clean, and the delivered pin remained
`580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`.
The self-excluding evidence manifest verifies with status 0 and SHA-256
`acf943231631e0caa21761860c6183819cfb7a22978fb26d417fef8cdfb01a48`.
Root independently reverified the manifest, stream shape, stderr, empty output,
clean exact head/upstream, and unchanged pin. No artifact, Cargo target, LLVM IR,
assembly capture, or other compiler payload enters Git.

The candidate identity differs from the delivered pin, so prior artifact
qualification cannot carry. Do not repin or open a pull request. A separately
numbered artifact-qualification successor must inherit this source chain after
one of the two active issue slots (#670/#671) clears. Astra LOW must first review
this drift disposition.
