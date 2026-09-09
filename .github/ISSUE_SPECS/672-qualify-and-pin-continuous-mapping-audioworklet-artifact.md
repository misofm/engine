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

The qualification source is the accepted product checkpoint `fe6ddb4d1f1aadb254a2cd5e95732652fd457351`; the successor branch begins at #670 disposition `2cf84f301d6812d3f915a3ade475828d6f425b54`. Product source, tests, Cargo manifests/lock, toolchain/config, ABI, JS/TS, metadata, resource and PCM expectations, browser rows, scripts, workflows, and fixtures are frozen. Before pre-pin PASS, repository pins and lineage files are frozen too.

After Astra LOW pre-pin PASS, the only product/config edits authorized are:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, changed to the approved candidate digest plus LF;
- `hosts/host-web/qualification/results.json`, changing only `candidateCommit` to the accepted product checkpoint and `wasmSha256` to the approved digest;
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
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`, no competing Cargo/rustc/npm or
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
   `bash -o pipefail -c 'git archive --format=tar "$1" | tee "$3" | tar -xf - -C "$2"' export-main 7d16d9c9752c9ac2d31e69008fe075df86ce3c26 /tmp/issue672-main-source /tmp/issue672-prepin-evidence/main.tar`,
   then
   `bash -o pipefail -c 'git archive --format=tar "$1" | tee "$3" | tar -xf - -C "$2"' export-pristine fe6ddb4d1f1aadb254a2cd5e95732652fd457351 /tmp/issue672-candidate-pristine /tmp/issue672-prepin-evidence/candidate-pristine.tar`,
   then
   `bash -o pipefail -c 'git archive --format=tar "$1" | tee "$3" | tar -xf - -C "$2"' export-candidate fe6ddb4d1f1aadb254a2cd5e95732652fd457351 /tmp/issue672-candidate-source /tmp/issue672-prepin-evidence/candidate-source.tar`.
   Require every pipeline status 0, hash
   the three retained tar streams, then delete the tar streams after verification.
   Run the frozen verifier in `exact` mode on all three exports, using
   `/home/bl/misofm/engine-cp8-mapping-artifact` as REPO and the corresponding
   literal commit as TREE.
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
   `python3 -B /tmp/issue672-export-verifier.py /home/bl/misofm/engine-cp8-mapping-artifact fe6ddb4d1f1aadb254a2cd5e95732652fd457351 /tmp/issue672-candidate-source overlay`,
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
