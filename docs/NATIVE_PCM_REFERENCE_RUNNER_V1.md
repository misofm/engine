# Native PCM reference runner V1

`native-pcm-runner` is a native, offline reference tool. It resolves strict session
sources to checked RIFF/WAVE or RF64/WAVE files, decodes bounded chunks with the public native
decoder, and performs engine preparation, source submission, resource inspection, and rendering
only through the frozen Engine V2 C ABI V1. It is not a render-thread component, host adapter,
codec, player, benchmark, or sample-rate converter.

## Command and launch envelope

The complete command shape is:

```text
native-pcm-runner \
  --session SESSION.toml --source-root DIRECTORY \
  --frames POSITIVE_U64 --output OUTPUT.f32le
```

Each named option is required exactly once. Positional and unknown arguments are rejected. The
frame count must be positive, arithmetically representable as `frames * 8` output bytes, and an
exact multiple of the session quantum. Accepted session rates are exactly 44,100, 48,000, 88,200,
and 96,000 Hz. Source/session rate mismatch is rejected; no implicit SRC exists.

The runner parses the strict session only to inspect declarations. It does not call the Rust
session compiler, graph compiler/renderer, or source ring. Engine creation, transactional session
compilation, resource query, planar source submission, stereo render, and destruction use the
installed C ABI V1 entry points.

## Local file adapter

The session carries no locator. The runner canonicalizes the source root, enumerates its bounded
set of `.wav` entries in sorted order, and considers only regular nonsymlink files whose canonical
paths remain under that root. For each declaration it parses RIFF/WAVE or RF64/WAVE, hashes the raw
canonical sample bytes (never the wrapper), and selects the file whose digest matches the exact
`sha256:<64 lowercase hexadecimal digits>` content identity. No session string becomes a path.

The selected file must exactly match the session root rate and the declaration's channels, bit
depth, and full frame count. All identities, WAVE/RF64 structures, and declared shapes are checked
before C compilation or output creation.

## Streaming and PCM bytes

For each output quantum, sources are visited in source-ID byte order. Each source contributes at
most one decoder chunk bounded by the quantum: a full chunk, short final chunk, or no chunk after
the full source ends. The chunk carries generation 1 and its exact absolute source frame beginning
at zero. Any C ABI
backpressure or non-OK submission/render result is terminal and is never retried.

One render call produces one stereo quantum. The file record is left-plane samples followed by
right-plane samples. Each scalar is written as its exact IEEE-754 `f32` bits in little-endian byte
order; finite values and signed zero are not canonicalized. Only source decode scratch, one output
quantum, C-owned prepared state, and small hash/write buffers are retained. Whole stems and whole
outputs are never retained.

## Atomic publication and diagnostics

The caller must provide an existing output directory exclusively owned for the complete runner
invocation. From entry through return, no other thread or process may create, remove, rename, link,
replace, chmod, or otherwise mutate an entry in that directory. The runner cannot infer this
authority from ownership bits, ACLs, or process identity. It does not claim safety against a
same-privilege concurrent directory-entry mutator; callers needing concurrency must coordinate it
externally.

Within that precondition, both the final path and exact sibling `OUTPUT.issue073.partial` must be
absent. The runner creates the partial with create-new semantics, streams and hashes bytes, flushes
and synchronizes it, and retains the create-new handle through exact `frames * 8` length, digest,
and identity checks. Publication is explicit per supported host family and never falls back to a
pathname hard link:

- Linux and Android use atomic `renameat2(RENAME_NOREPLACE)`;
- Apple Unix uses atomic `renameatx_np(RENAME_EXCL)`; and
- Windows uses `SetFileInformationByHandle(FileRenameInfo)` on the retained handle with
  replacement disabled.

Unix compares device/inode identity and Windows compares volume serial/file index identity from
opened handles before and after publication. Each primitive rejects every existing final kind and
atomically consumes the partial, leaving one accepted final. Failure cleanup is bounded and
identity-checked under the exclusive-directory precondition. Existing regular files, directories,
symlinks, and hardlinks are never followed, truncated, or replaced. A native platform without one
of these adapters returns `preflight/platform.unsupported` before output creation, source
resolution, or engine invocation.

Stderr failures have the stable tab-separated form:

```text
native-pcm-runner.v1<TAB>PHASE<TAB>CODE
```

Phases are `cli`, `preflight`, `resolve`, `decode`, `compile`, `submit`, `render`, `output`, and
`publish`. Codes are stable lowercase dotted tokens. They deliberately carry no paths, addresses,
audio data, or unbounded product diagnostics.

## Frozen fixtures

[`fixtures/native-pcm-runner/v1/MANIFEST.tsv`](../fixtures/native-pcm-runner/v1/MANIFEST.tsv)
freezes RIFF files at all four launch rates, an RF64 `ds64` file, their strict sessions, sizes,
canonical-content SHA-256 identities, and exact 8,192-byte output digests. The RF64 source asset is
pre-sliced to the 514 decoded frames that the former region selected, so its full-source stream
starts at zero and still has a two-frame final chunk. Source values include positive and negative
zero, a NaN, a subnormal, and finite nonzero witnesses; the public decoder sanitizes the
nonfinite/subnormal witnesses. Rust
tests independently split accepted output into little-endian words, verify exact length/finiteness,
and compare the whole-file digest rather than reusing the production encoder as an oracle.

The tool is native-only because the public file decoder is absent from `wasm32`; the static policy
checker rejects every reverse dependency from crates, hosts, or other tools. Cross-target
qualification belongs to Issue 114.
