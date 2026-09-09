# Qualify and pin the continuous-mapping AudioWorklet artifact

Parent: #560 (lane B, CP8)

Predecessors: #669 and #670

Coordination: #559

## Problem and inherited result

#670 independently qualified the frozen `continuous_mapping_admissible` extraction and its complete typed and borrowed public fixture matrix. Astra LOW returned SOURCE PASS at `fe6ddb4d1f1aadb254a2cd5e95732652fd457351`. The accepted product hashes are:

- `crates/effect-contract/src/lib.rs`: `be709c2293b108feccfe14b0049c08e32d09ce61188a865dca59fa6cee185f98`
- `crates/effect-package/src/wire.rs`: `9a4e833512ab8f70bf4804fc149bfe21e2cb568eb6f53a707c212529e7e66818`

The shipped `host-web` depends on `effect-contract`. #670 therefore ran one repin-report identity probe. It returned candidate digest `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`, different from delivered pin `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`. Astra confirmed the drift, but rejected the probe as qualification evidence because its retained preflight omitted source hashes, live-main identity, pin byte shape, and explicit fresh-path absence, while its postflight contains unlabeled corrected fields. Preserve that record without reconstruction or rerun. The observed digest is only the expected candidate for this fresh qualification.

#670 closed source-qualified and delivery-incomplete at `2cf84f301d6812d3f915a3ade475828d6f425b54`. This successor owns candidate qualification, pin and lineage promotion, exact delivery, and cleanup. It is the second active issue alongside disjoint #671.

## Ownership and immutable scope

Sol HIGH coordinates the brief, checkpoints, artifact decision, GitHub synchronization, PR, merge, and cleanup. Luna HIGH `/root/issue583_luna_impl` is the sole executor. Per current user routing, Astra LOW performs scope, pre-pin artifact, exact-head, integration, and delivery review.

The qualification source is integrated product checkpoint `8708c9b998a484d49ccb17a803e79540ca13fcd6`, whose first parent carries the accepted #670 source and whose second parent is delivered #671 main `acd625d72a57f83f50f26279717464744504b4c4`; the successor branch began at #670 disposition `2cf84f301d6812d3f915a3ade475828d6f425b54`. The two accepted product hashes above remain unchanged after integration. Product source, tests, Cargo manifests/lock, toolchain/config, ABI, JS/TS, metadata, resource and PCM expectations, browser rows, scripts, workflows, and fixtures are frozen. Before pre-pin PASS, repository pins and lineage files are frozen too.

After Astra LOW pre-pin PASS, the only product/config edits authorized are:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, changed to the approved candidate digest plus LF;
- `hosts/host-web/qualification/results.json`, changing only `candidateCommit` to integrated product checkpoint `8708c9b998a484d49ccb17a803e79540ca13fcd6` and `wasmSha256` to the approved digest;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, regenerated from that otherwise unchanged results document so only the corresponding lineage sentence changes;
- this numbered issue record.

Do not change browser results, version floors, gates, resources, PCM identities, source, dependencies, locks, builders, checkers, or expectations. No benchmark, timing, optimization, listening claim, DSP change, new harness, or matrix expansion belongs here.

## Fresh qualification procedure

The exact fresh paths are:

- `/tmp/issue672-main-source`
- `/tmp/issue672-candidate-pristine`
- `/tmp/issue672-candidate-source`
- `/tmp/issue672-main-artifact`
- `/tmp/issue672-candidate-artifact`
- `/tmp/issue672-candidate-target`
- `/tmp/issue672-prepin-evidence`
- `/tmp/issue672-export-verifier.py`
- `/tmp/issue672-verifier-control`
- `/tmp/issue672-prepin-manifest-record.txt`
- `/tmp/issue672-prepin-manifest-verify.stdout`
- `/tmp/issue672-prepin-manifest-verify.status`

Before creating anything, require every path absent including dangling symlinks,
the feature tree clean at its pushed authorization head, live main exactly
`acd625d72a57f83f50f26279717464744504b4c4`, no competing Cargo/rustc/npm or
browser process, and the two frozen product hashes above. Record exact argv, cwd,
executor/time, head/upstream, live main and merge-base, pin bytes/length/newline,
Rust/Cargo/Node/npm versions, the literal value or unset state of Cargo/Rust/CC/
SOURCE_DATE_EPOCH variables, and hashes of the builder, locks, toolchain/config,
artifact gates, SDK manifests, qualification manifests/results, and matrix.
Exercise the capture wrapper with harmless status-0 and expected status-1 controls
and independently read them back.

The executor must write this literal temporary verifier byte-for-byte to
`/tmp/issue672-export-verifier.py`, read it back, hash it, and run its self-test
before either builder. It uses the feature worktree as the Git object authority;
exports contain no `.git`. No improvised replacement is allowed.

```python
#!/usr/bin/env python3
import os, pathlib, stat, subprocess, sys, tempfile

OVERLAYS = {
    "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256",
    "hosts/host-web/qualification/results.json",
    "hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md",
}
IGNORED = {"sdk/node_modules", "hosts/host-web/qualification/node_modules"}

def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], check=True,
                          stdout=subprocess.PIPE).stdout

def inventory(repo, tree):
    out = {}
    for row in git(repo, "ls-tree", "-rz", "--full-tree", tree).split(b"\0"):
        if not row: continue
        meta, raw = row.split(b"\t", 1)
        mode, kind, oid = meta.split()
        if kind != b"blob": raise RuntimeError("non-blob tracked entry")
        path = raw.decode("utf-8")
        if path in out: raise RuntimeError("duplicate tracked path")
        out[path] = (mode.decode(), oid.decode())
    return out

def ignored(path, overlay):
    return overlay and any(path == root or path.startswith(root + "/")
                           for root in IGNORED)

def verify(repo, tree, root, overlay=False):
    root = pathlib.Path(root)
    if not root.is_dir() or root.is_symlink(): raise RuntimeError("bad root")
    expected = inventory(repo, tree)
    expected_dirs = set()
    for name in expected:
        p = pathlib.PurePosixPath(name).parent
        while str(p) != ".":
            expected_dirs.add(str(p)); p = p.parent
    actual, dirs = set(), set()
    if overlay:
        for name in IGNORED:
            p = root / name
            if not p.is_dir() or p.is_symlink():
                raise RuntimeError("dependency allowance is not a directory")
    def walk_error(error):
        raise error
    for base, names, files in os.walk(root, followlinks=False,
                                      onerror=walk_error):
        relbase = pathlib.Path(base).relative_to(root).as_posix()
        relbase = "" if relbase == "." else relbase
        names[:] = [n for n in names
                    if not ignored(f"{relbase}/{n}".strip("/"), overlay)]
        for name in names:
            rel = f"{relbase}/{name}".strip("/")
            p = root / rel
            if p.is_symlink(): actual.add(rel)
            else: dirs.add(rel)
        for name in files:
            rel = f"{relbase}/{name}".strip("/")
            if not ignored(rel, overlay): actual.add(rel)
    if actual != set(expected): raise RuntimeError("missing or extra tracked path")
    if dirs != expected_dirs: raise RuntimeError("missing or extra directory")
    if (root / ".git").exists() or (root / ".git").is_symlink():
        raise RuntimeError("export contains .git")
    for name, (mode, oid) in expected.items():
        p = root / name
        info = p.lstat()
        blob = git(repo, "cat-file", "blob", oid)
        if mode == "120000":
            if not stat.S_ISLNK(info.st_mode): raise RuntimeError("symlink mode")
            if os.readlink(p).encode() != blob: raise RuntimeError("symlink target")
            continue
        if mode not in {"100644", "100755"} or not stat.S_ISREG(info.st_mode):
            raise RuntimeError("regular-file mode")
        executable = bool(info.st_mode & 0o111)
        if executable != (mode == "100755"): raise RuntimeError("executable mode")
        if not (overlay and name in OVERLAYS) and p.read_bytes() != blob:
            raise RuntimeError("regular-file bytes")
    print(f"PASS paths={len(expected)} overlay={int(overlay)}")

def must_fail(call):
    try: call()
    except RuntimeError: return
    raise RuntimeError("negative control unexpectedly passed")

def self_test():
    parent = os.environ["TMPDIR"]
    with tempfile.TemporaryDirectory(dir=parent) as temp:
        repo = pathlib.Path(temp, "repo"); export = pathlib.Path(temp, "export")
        repo.mkdir(); subprocess.run(["git", "init", "-q", str(repo)], check=True)
        paths = sorted(OVERLAYS | {"plain", "bin/run", "link",
                                  "sdk/package.json",
                                  "hosts/host-web/qualification/package.json"})
        for name in paths:
            p = repo / name; p.parent.mkdir(parents=True, exist_ok=True)
            if name == "link": p.symlink_to("plain")
            else: p.write_bytes((name + "\n").encode())
        (repo / "bin/run").chmod(0o755)
        subprocess.run(["git", "-C", str(repo), "add", "."], check=True)
        tree = git(str(repo), "write-tree").decode().strip()
        export.mkdir()
        subprocess.run(["bash", "-o", "pipefail", "-c",
                        'git -C "$1" archive --format=tar "$2" | tar -xf - -C "$3"',
                        "verify-export", str(repo), tree, str(export)], check=True)
        verify(str(repo), tree, str(export))
        (export / "plain").write_text("changed\n")
        must_fail(lambda: verify(str(repo), tree, str(export)))
        (export / "plain").write_bytes(git(str(repo), "show", f"{tree}:plain"))
        (export / "plain").unlink()
        must_fail(lambda: verify(str(repo), tree, str(export)))
        (export / "plain").write_bytes(git(str(repo), "show", f"{tree}:plain"))
        (export / "extra").write_text("extra\n")
        must_fail(lambda: verify(str(repo), tree, str(export))); (export / "extra").unlink()
        (export / "extra-dir").mkdir()
        must_fail(lambda: verify(str(repo), tree, str(export))); (export / "extra-dir").rmdir()
        (export / "bin/run").chmod(0o644)
        must_fail(lambda: verify(str(repo), tree, str(export))); (export / "bin/run").chmod(0o755)
        (export / "link").unlink(); (export / "link").symlink_to("wrong")
        must_fail(lambda: verify(str(repo), tree, str(export)))
        (export / "link").unlink(); (export / "link").symlink_to("plain")
        for name in IGNORED:
            p = export / name; p.mkdir(parents=True); (p / "ignored").write_text("ok\n")
        must_fail(lambda: verify(str(repo), tree, str(export), False))
        for name in OVERLAYS: (export / name).write_text("overlay\n")
        verify(str(repo), tree, str(export), True)
        subject = export / sorted(IGNORED)[0]
        (subject / "ignored").unlink(); subject.rmdir(); subject.write_text("bad\n")
        must_fail(lambda: verify(str(repo), tree, str(export), True))
        subject.unlink(); subject.symlink_to(export / sorted(IGNORED)[1])
        must_fail(lambda: verify(str(repo), tree, str(export), True))
        subject.unlink(); subject.mkdir(); (subject / "ignored").write_text("ok\n")
        verify(str(repo), tree, str(export), True)
    print("PASS self-test")

if sys.argv[1:] == ["--self-test"]: self_test()
elif len(sys.argv) == 5 and sys.argv[4] in {"exact", "overlay"}:
    verify(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4] == "overlay")
else: raise SystemExit("usage: verifier REPO TREE EXPORT exact|overlay | --self-test")
```

Run the following sequence once, in order, stopping at the first failed
precondition or command without correction or retry:

1. Create the evidence and verifier-control directories. Run exactly
   `TMPDIR=/tmp/issue672-verifier-control python3 -B /tmp/issue672-export-verifier.py --self-test`
   and require status 0 with both positive and negative controls passing. Create
   each source directory, then export with
   `bash -o pipefail -c 'git archive --format=tar "$1" | tee "$3" | tar -xf - -C "$2"' export-main acd625d72a57f83f50f26279717464744504b4c4 /tmp/issue672-main-source /tmp/issue672-prepin-evidence/main.tar`,
   then
   `bash -o pipefail -c 'git archive --format=tar "$1" | tee "$3" | tar -xf - -C "$2"' export-pristine 8708c9b998a484d49ccb17a803e79540ca13fcd6 /tmp/issue672-candidate-pristine /tmp/issue672-prepin-evidence/candidate-pristine.tar`,
   then
   `bash -o pipefail -c 'git archive --format=tar "$1" | tee "$3" | tar -xf - -C "$2"' export-candidate 8708c9b998a484d49ccb17a803e79540ca13fcd6 /tmp/issue672-candidate-source /tmp/issue672-prepin-evidence/candidate-source.tar`.
   Require every pipeline status 0, hash
   the three retained tar streams, then delete the tar streams after verification.
   Run the frozen verifier in `exact` mode on all three exports, using
   `/home/bl/misofm/engine-cp8-mapping-artifact` as REPO and the corresponding
   literal commit as TREE (`acd625d7` for main, `8708c9b9` for both candidates).
2. Create `/tmp/issue672-main-artifact` as an empty non-symlink directory. From
   `/tmp/issue672-main-source`, run exactly once:
   `bash scripts/build-web-audioworklet.sh /tmp/issue672-main-artifact`.
   Require status 0, the exact six filenames, and independently computed Wasm
   digest `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`.
3. In `/tmp/issue672-candidate-source`, replace only
   `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` with exact
   candidate digest `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`
   plus LF. Prove that sole overlay against the pristine export. Create
   `/tmp/issue672-candidate-artifact` as an empty non-symlink directory and, from
   the candidate source, run exactly once:
   `bash scripts/build-web-audioworklet.sh /tmp/issue672-candidate-artifact`.
   Require status 0, the exact six filenames, and that same independently
   computed Wasm digest. A different digest stops and authorizes no replacement.
4. Produce complete size/SHA-256 manifests for both six-file sets. Require all
   five non-Wasm files byte-identical. Classify the Wasm delta using exactly
   `wasm-validate`, `wasm-objdump -h`, `wasm-objdump -x`, and the repository gate
   below; retain only compact section/import/export summaries and hashes. Full
   dumps, `.ll`, `.s`, binaries, objects, archives, Cargo targets, and raw compiler
   streams remain temporary and never enter Git.
5. After candidate identity, apply the other two scratch-only lineage overlays:
   change only `candidateCommit` and `wasmSha256` in
   `hosts/host-web/qualification/results.json`, then run exactly
   `node hosts/host-web/qualification/generate-matrix.mjs`. Require an exact
   three-path diff against the pristine export, semantic equality of every other
   JSON field and browser row, and a matrix diff limited to the lineage sentence.
   Run exactly `node hosts/host-web/qualification/generate-matrix.mjs --check`.
6. Install exact locked dependencies before consumers, from the candidate source:
   `npm --prefix sdk ci --no-audit --no-fund --prefer-offline`, then
   `npm --prefix hosts/host-web/qualification ci --ignore-scripts --no-audit --no-fund`,
   then `(cd hosts/host-web/qualification && npx playwright install chromium firefox webkit)`.
   Require package manifests/locks unchanged, Playwright package version exactly
   `1.62.1`, and all three installed browser executables reported present before
   qualification. Ignored scratch `node_modules` content is allowed.
7. Export `CARGO_TARGET_DIR=/tmp/issue672-candidate-target` and run exactly once
   in order from the candidate source:
   `bash scripts/check-web-audioworklet.sh /tmp/issue672-candidate-artifact`;
   `python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue672-candidate-artifact`;
   `bash scripts/test-web-audioworklet.sh`;
   `python3 -B scripts/check-sdk-deletions.py`;
   `python3 -B scripts/check-sdk-deletions.py --self-test`;
   `bash scripts/check-sdk-types.sh`;
   `bash scripts/check-sdk-headless.sh /tmp/issue672-candidate-artifact`;
   `bash scripts/sdk-package.sh check /tmp/issue672-candidate-artifact`;
   and `npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue672-candidate-artifact --browser all --check-matrix --self-test-mutations`.
   Chromium, Firefox, and WebKit must each execute and pass every existing gate
   and mutation; `--record-matrix` is forbidden.
8. Re-run the literal source/overlay verifier, require only the exact three
   scratch overlays plus ignored dependency directories using exactly
   `python3 -B /tmp/issue672-export-verifier.py /home/bl/misofm/engine-cp8-mapping-artifact 8708c9b998a484d49ccb17a803e79540ca13fcd6 /tmp/issue672-candidate-source overlay`,
   and require the feature
   repository still clean. Finalize all evidence, write a self-excluding
   `SHA256SUMS`, and place only manifest command/completion, verification output,
   and numeric status in the three named sibling files.

`sdk-package.sh check` already invokes `check-sdk-generated.sh`; the sequence does
not run that gate separately. Accepted #670 source tests and policies are inherited and are not rerun: this
issue changes no source. The candidate structural/resource/PCM/SDK/browser gates
above qualify the artifact boundary.

Any source/config drift, second candidate identity, artifact-census error, unexplained delta, structural/resource/PCM/SDK/browser failure, zero-selected browser run, or evidence-capture defect stops for root disposition. Do not rerun or repin.

## Review, promotion, and delivery

Astra LOW must review the exact frozen source, live-main baseline, retained candidate, six-file and Wasm delta, every gate record, scratch overlays, incomplete #670 probe disposition, and verified manifest. Candidate novelty or green compilation alone is insufficient. No repository pin or lineage edit occurs before explicit PRE-PIN PASS.

PRE-PIN PASS authorizes no repository edit by itself. Root must append an exact
three-path promotion and post-pin command sequence, push/synchronize it, and
obtain a fresh Astra LOW scope PASS before Luna changes the pin or lineage. That
later amendment will name fresh post-pin paths and literal commands, require one
ordinary no-bypass build whose six files byte-match the prequalified candidate,
and avoid repeating browser execution. No vague or inferred post-pin action is
authorized by this initial scope.

Astra LOW then performs exact feature-head/current-main PR-readiness review. Root opens one PR, waits for required `qualification` SUCCESS on its immutable head/current base, performs guarded live-head/base review, merges, and verifies post-main `qualification` SUCCESS. Synchronize #559/#560 and this issue before closure. The source-qualified #669/#670 and this successor receive product-delivery credit only after merge and post-main success. Remove their clean delivered worktrees only after all checkpoints are pushed and required evidence remains available outside them.

## Acceptance

- The accepted #670 source bytes remain unchanged and integrate on current main.
- Fresh baseline and candidate ordinary builds reproduce the delivered and expected digests respectively.
- Every artifact delta is explained by the accepted mapping extraction; ABI, static realtime properties, resource/PCM identities, SDK/package behavior, and all three browsers remain green.
- Astra LOW returns PRE-PIN PASS before the exact pin/lineage promotion.
- The post-pin ordinary build reproduces all six prequalified files byte-for-byte.
- Required PR and post-main `qualification` runs succeed on their stated immutable commits.
- Git contains no generated Wasm, Cargo target, `.ll`, `.s`, full compiler stream, or raw qualification payload.

## Initial scope review — FAIL and correction

Astra LOW returned **SCOPE FAIL** at clean pushed feature
`28f7ac5697bf92a6e04919124b9156e29c829ecd` because the qualification
sequence still used path placeholders, omitted exact dependency setup and target
isolation, did not require the five non-Wasm files equal, left lineage timing and
post-pin commands vague, and redundantly repeated accepted source gates. No build
or qualification command ran. This amendment replaces every placeholder with an
exact fresh path, freezes setup and gate order, installs locked dependencies
before use, requires the five files byte-identical, fixes lineage overlays before
lineage-sensitive gates, removes redundant source execution, and makes promotion
a separate reviewed scope boundary. The concurrent tracker checkpoint was
preserved and GitHub #559/#560 were resynchronized before this correction.

Astra LOW returned a second **SCOPE FAIL** at exact clean feature `d0e1f511`
and tracker `ccbd655a`. The sole remaining blocker was that step 1 still asked
the executor to invent the export verifier immediately before use; it also noted
the duplicated SDK generated check. No export, build, install, or browser command
ran. This correction freezes the literal verifier and its missing/extra/bytes/
mode/symlink/overlay self-controls, uses fail-closed archive pipelines with an
external Git object authority, limits final allowances to the three overlays and
two named dependency directories, and removes the redundant direct generated
check because `sdk-package.sh check` owns it. Fresh scope review remains required.

Astra LOW returned a third **SCOPE FAIL** at exact clean feature `40dbdd3d`
and tracker `7ef99874`. The embedded verifier incorrectly applied dependency
allowances even in exact mode, could accept a file or symlink at an allowed
directory name, and let `os.walk` suppress traversal errors. No workload ran.
This bounded correction makes exact mode allow no ignored path, requires both
post-install allowances to exist as ordinary non-symlink directories, raises all
walk errors, and adds negative controls for exact-mode dependency content plus
file and symlink substitutions. All other scope remains frozen.

Astra LOW returned a fourth **SCOPE FAIL** at exact clean feature `5144422c`
and tracker `aaa21c78` because the exact-mode dependency negative control ran
after overlay bytes changed and was therefore non-discriminating. No workload
ran. The verifier logic itself passed review. This correction moves that control
to the pristine-byte state with only the two dependency directories present,
then changes the three overlay bytes and retains both overlay PASS plus file and
symlink rejection controls. No other scope changes.

## Pre-pin scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean pushed feature
`4e5e5dc826da38cbc15ca204a13d24ccbaedb762` and tracker
`aa8a4f57acdfff3ce157d649ee4a7898fa66bb9d`; GitHub #672/#559/#560
match. The reordered control isolates exact-mode dependency-directory rejection
while tracked bytes are pristine, and the overlay/file/symlink controls pass.
Only Luna HIGH `/root/issue583_luna_impl` may execute the frozen pre-pin sequence
after immediate identity, path-absence, and verifier-control preflight. Stop on
the first unexpected failure without correction or retry. Repository promotion,
post-pin build, PR, and merge remain unauthorized pending their separate reviews.

## Delivered-main integration and reauthorization requirement

Luna HIGH stopped at the first authorization precondition on clean
`d9051b342c4ce2c15b541cf1213f872d59862a92`: #671 had delivered and
`origin/main` was `acd625d72a57f83f50f26279717464744504b4c4`, not the scoped
`7d16d9c9`. All twelve #672 paths were absent including dangling symlinks. No
verifier, export, build, install, browser, or gate command ran, no path was
created, and no attempt was consumed.

Root merged that exact delivered main conflict-free at clean pushed checkpoint
`8708c9b998a484d49ccb17a803e79540ca13fcd6`. Its parents are the prior
#672 authorization and delivered main in that order. The #670 product hashes
remain exact. This amendment freezes the integrated checkpoint as candidate and
the delivered commit as baseline/current main throughout the literal commands.
Fresh Astra LOW scope PASS is required before execution resumes.

Astra LOW returned integrated-base **SCOPE PASS** at exact clean pushed feature
`c29329e53e0e094679fdd03cf98bd605cee3b93b` and tracker
`25a1a3c3a6c70dea0382931a6b626e799b5b00b1`; GitHub bodies match. The
merge parents, unchanged accepted source hashes, literal baseline/candidate
exports, verifier authority, and candidate lineage are consistent. All twelve
paths remain absent including symlinks. Only Luna HIGH
`/root/issue583_luna_impl` may execute the frozen pre-pin sequence after immediate
fresh preflight. Stop at any unexpected failure. The earlier precondition stop
supplies no workload credit; repository promotion and post-pin work remain
separately gated.

## Pre-pin attempt 1 — EVIDENCE FAIL

Luna HIGH reported passing preflight at clean `9729f0448b11b840e249b7a83ee5a31e0dc17e10`;
the retained evidence directory contains no durable preflight/context/control
record, so that statement retains Luna attribution and is not independent
contemporaneous evidence.
The frozen verifier self-test, all three fail-closed exports, tar hashes/removal,
and exact export verification passed. The current-main ordinary builder then ran
exactly once and returned 0 with the exact six-file set and delivered Wasm digest
`580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`.

The post-build capture referenced nonexistent `10-main-build.stdout` instead of
retained `09-main-build.stdout`, causing an uncaptured shell error. Luna stopped
immediately. No candidate build, overlay, install, later gate, correction, or
retry ran; the repository remained clean. Preserve `/tmp/issue672-prepin-evidence`
and `/tmp/issue672-main-artifact` unchanged. Attempt 1 is consumed as an evidence
failure. Astra LOW must review it before any attempt-2 amendment. No compiler or
generated artifact payload enters Git, and no promotion is authorized.

## Attempt 1 review and bounded attempt 2

Astra LOW confirmed **ATTEMPT-1 FAIL** at exact clean pushed
`1d207c54741c1c963db4d8b8d13bd0184bb31e05`. The baseline command has
contemporaneous timestamps/status 0 and its current six hashes match the retained
census, but the failed filename reference, absent failure status/final manifest,
and absent durable preflight/context/control record consume the attempt. Do not
reconstruct those records or repeat the successful baseline builder.

Attempt 2 preserves every attempt-1 byte and uses its retained main artifact only
as a read-only comparator. Its exact fresh paths are:

- `/tmp/issue672-attempt2-candidate-pristine`
- `/tmp/issue672-attempt2-candidate-source`
- `/tmp/issue672-attempt2-candidate-artifact`
- `/tmp/issue672-attempt2-candidate-target`
- `/tmp/issue672-attempt2-evidence`
- `/tmp/issue672-attempt2-export-verifier.py`
- `/tmp/issue672-attempt2-verifier-control`
- `/tmp/issue672-attempt2-manifest-record.txt`
- `/tmp/issue672-attempt2-manifest-verify.stdout`
- `/tmp/issue672-attempt2-manifest-verify.status`

Before creation require all ten listed paths absent including dangling symlinks, the feature
clean at its pushed authorization head, `origin/main` exactly `acd625d7`, and no
competing workload. Record all identity, toolchain, environment, source/config/
pin byte-shape, path-absence, and harmless capture status-0/status-1 controls
required by the original preflight in the fresh evidence directory and read each
back independently.

Read-only reconcile attempt 1: verify the retained literal verifier hash, its
self-test/status, three export statuses, all retained export identities against
Git objects, baseline command/timestamps/status, and these exact six comparator
hashes; record that attempt 1 has no durable original preflight or final manifest:

- `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919` ABI JSON
- `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf` host d.ts
- `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a` host JS
- `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb` worklet JS
- `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10` Wasm
- `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d` parameter metadata

Write the identical embedded verifier to the new verifier path, require SHA-256
`2bd12b45cc916faacbb75cda3ae7df48d228022e121da5288f81e46b566f049c`,
read it back, and run its self-test with the new control directory. Export
candidate checkpoint `8708c9b9` twice using the original two literal fail-closed
candidate archive commands with only the attempt-2 paths substituted. Hash then
delete both tar streams and verify both exports in exact mode.

Continue original steps 3 through 8 exactly once and in order using only the
attempt-2 candidate source/artifact/target/evidence/verifier/manifest paths. The
retained `/tmp/issue672-main-artifact` replaces the original main artifact only
for exact five-file equality and Wasm delta comparison; no command may mutate it.
All overlay, dependency, static/resource/PCM/SDK, three-browser, final verifier,
and manifest requirements remain unchanged. Stop on the first failure without
correction or retry. No repository edit, promotion, post-pin work, PR, or merge
is authorized until attempt 2 receives fresh Astra LOW scope PASS and then
pre-pin evidence PASS.

Astra LOW returned attempt-2 **SCOPE FAIL** at exact clean feature `58367843`
and tracker `647c6479` solely because the brief listed ten fresh paths but said
all nine must be absent. All ten were absent. This correction synchronizes that
count; no substantive scope changes and no execution occurred.

Astra LOW returned attempt-2 **SCOPE PASS** at exact clean feature
`e4a9624d8b49f623a1dcc187288eae59dcb2b8c6` and tracker
`818f655e21e37ad48aa458f0d8145afd67eaa09a`; GitHub matches and all ten
paths are absent including symlinks. Only Luna HIGH `/root/issue583_luna_impl`
may preserve/reconcile attempt 1, use its baseline read-only, and execute the
fresh candidate and remaining gates once. Stop on first failure. No baseline
rebuild, promotion, post-pin work, PR, or merge is authorized.

## Pre-pin attempt 2 — FAIL

Luna HIGH ran at exact clean `3c6add6c68a5c47c235b332c35d307d49d0b3dfd`.
Fresh preflight, attempt-1 reconciliation, verifier identity/self-test, capture
controls, both candidate exports, and exact 12,195-path verification passed; tar
hashes matched. The cleanup invoked `unlink` with two operands, returned an error,
and left both temporary tar streams preserved. Luna stopped immediately. No
candidate build, overlay, install, browser, later gate, correction, or retry ran;
the repository remained clean. Preserve all attempt-2 paths unchanged. Attempt 2
is consumed. Astra LOW failure review and a frozen final-attempt scope are
required before further execution. No promotion or compiler/generated payload
commit is authorized.

## Attempt 2 review and final-attempt rescope

Astra LOW confirmed **ATTEMPT-2 FAIL** at exact clean pushed
`be6e4fe02ede8b4b472d7dd0e8aa74a4f6f0697e`. Both retained exports have
status-0 exact verification of 12,195 paths and their tar hashes match
`1dc665c502ab20e215597ce8e93db80c30a11e8f58f99f2e8a9cba7eeb434d75`.
The failed cleanup has no retained command/status record and remains executor-
attributed; do not reconstruct it. No final manifest exists. Attempt 2 is
consumed, while its successful export records remain usable.

Root integrated delivered #675 main
`df0b9b93636de36a7143da15b83444f280b65e6b` conflict-free at pushed
checkpoint `755f3b70395d1498c76d8ef1d2851cf54151e46d`. The merge parents are
attempt-2 disposition then current main. #675 changes exactly its issue record,
`crates/builtins/src/lib.rs`, and new `crates/builtins/src/tests.rs`; Astra LOW's
delivered #675 SOURCE PASS proves the moved body remains behind `cfg(test)` and
the production input/artifact applicability is unchanged. The two accepted CP8
source hashes, host-web/SDK/config/lock/pin bytes, and retained artifact source
checkpoint `8708c9b9` remain unchanged.

Final attempt 3 uses only these fresh paths:

- `/tmp/issue672-attempt3-candidate-artifact`
- `/tmp/issue672-attempt3-target`
- `/tmp/issue672-attempt3-evidence`
- `/tmp/issue672-attempt3-manifest-record.txt`
- `/tmp/issue672-attempt3-manifest-verify.stdout`
- `/tmp/issue672-attempt3-manifest-verify.status`

Require all six absent including dangling symlinks before creation. Record fresh
feature/upstream/main/merge identities, toolchains/environment, all frozen source/
config/pin byte shapes, exact #675 three-path applicability, complete attempt-1/2
path census and hashes, and harmless captured status-0/status-1 controls. Preserve
all prior bytes, exports, artifacts, tar streams, and failed attribution.

Use retained `/tmp/issue672-attempt2-candidate-pristine` and
`/tmp/issue672-attempt2-candidate-source` without re-exporting. Re-run the frozen
verifier in exact mode against `8708c9b9` before any overlay; require both 12,195-
path results and the retained tar hashes, but do not delete either tar. Then
continue the original candidate steps 3 through 8 once, substituting only the
attempt-3 artifact/target/evidence/manifest paths. Apply the same exact three
scratch overlays to the retained candidate source, build the candidate once,
require expected digest `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`, compare its five non-Wasm files to
the immutable attempt-1 baseline, run every frozen structural/resource/PCM/SDK/
three-browser gate, verify final overlay state, and finalize the self-excluding
manifest. Omit all tar cleanup from the qualification path; cleanup is deferred
until delivered-worktree cleanup.

Stop on the first failure without correction or retry. Any failure exhausts #672
and requires a genuinely rescoped successor under the repository hard stop. No
repository promotion, post-pin execution, PR, merge, compiler dump, or generated
payload commit is authorized until this pushed amendment receives Astra LOW
FINAL-ATTEMPT SCOPE PASS and the completed evidence receives PRE-PIN PASS.

Astra LOW returned **FINAL-ATTEMPT SCOPE PASS** at exact clean pushed feature
`34cff816268df4243a89c4f0e2f64c7eeb281ad0`, tracker
`e6dec2b2a8ef99f726141c47ca5d3e731d7fec8b`, and current main
`df0b9b93636de36a7143da15b83444f280b65e6b`; GitHub bodies match. Merge
`755f3b70` has the recorded attempt-2 disposition then delivered main as its
parents, #675 is test-only and leaves artifact inputs unchanged, and all six
fresh attempt-3 paths are absent including symlinks. Only Luna HIGH
`/root/issue583_luna_impl` may reverify the retained exports/tars and run the
unexecuted candidate/downstream sequence once. Do not rebuild exports or the
baseline, clean tar streams, promote repository pins, or run post-pin work. Any
unexpected failure exhausts #672; PRE-PIN PASS and a separate promotion scope
review remain required before repository edits.
