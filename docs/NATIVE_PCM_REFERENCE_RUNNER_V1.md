# Native PCM reference runner V1

`miso-engine-native-pcm-runner` is a native, offline reference tool. It resolves strict session
sources to checked RIFF/WAVE or RF64/WAVE files, decodes bounded chunks with the public native
decoder, and performs engine preparation, source submission, resource inspection, and rendering
only through the frozen Engine V2 C ABI V1. It is not a render-thread component, host adapter,
codec, player, benchmark, or sample-rate converter.

## Command and launch envelope

The complete command shape is:

```text
miso-engine-native-pcm-runner \
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

This tool alone interprets two otherwise opaque session strings:

- a locator is `file:<relative-path>` using `/` separators. The suffix is nonempty UTF-8 and may
  contain only normal, nonempty components: no `.`, `..`, backslash, NUL, root, prefix, or absolute
  path is accepted;
- an identity is `sha256:<64 lowercase hexadecimal digits>` and must equal SHA-256 of the complete
  resolved file.

The source root is canonicalized. Each requested source must itself be a regular, nonsymlink file,
and its canonical path must remain under that root. Aliased canonical files are legal only with the
same digest. All locators, identities, WAVE/RF64 structures, declared rates/channels, and finite
regions are checked before C compilation or output creation.

## Streaming and PCM bytes

For each output quantum, sources are visited in source-ID byte order. Each source contributes at
most one decoder chunk bounded by the quantum: a full chunk, short final chunk, or no chunk after
its region ends. The chunk carries generation 1 and its exact absolute source frame. Any C ABI
backpressure or non-OK submission/render result is terminal and is never retried.

One render call produces one stereo quantum. The file record is left-plane samples followed by
right-plane samples. Each scalar is written as its exact IEEE-754 `f32` bits in little-endian byte
order; finite values and signed zero are not canonicalized. Only source decode scratch, one output
quantum, C-owned prepared state, and small hash/write buffers are retained. Whole stems and whole
outputs are never retained.

## Atomic publication and diagnostics

Both the final path and exact sibling `OUTPUT.issue073.partial` must be absent. The runner creates
the partial with create-new semantics, streams and hashes bytes, flushes and synchronizes it, and
retains the create-new handle through exact `frames * 8` length, digest, and identity checks.
Publication is explicit per supported host family and never falls back to a checked pathname:

- Linux and Android use `linkat(AT_EMPTY_PATH)` on the retained descriptor;
- Apple Unix uses no-replace `linkat` through the retained `/dev/fd/N` descriptor name; and
- Windows uses no-replace `SetFileInformationByHandle(FileLinkInfo)` on the retained handle.

Unix compares device/inode identity and Windows compares volume serial/file index identity from
opened handles. Each primitive publishes the exact retained file only while the final name is
absent. Removal is identity-checked and leaves a single final name. Existing regular files,
symlinks, and hardlinks are never followed, truncated, or replaced. A native platform without one
of these proved adapters returns `preflight/platform.unsupported` before output creation or engine
invocation.

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
SHA-256 identities, and exact 8,192-byte output digests. The RF64 row has source-region origin 1
and a two-frame final chunk. Source values include positive and negative zero, a NaN, a subnormal,
and finite nonzero witnesses; the public decoder sanitizes the nonfinite/subnormal witnesses. Rust
tests independently split accepted output into little-endian words, verify exact length/finiteness,
and compare the whole-file digest rather than reusing the production encoder as an oracle.

The tool is native-only because the public file decoder is absent from `wasm32`; the static policy
checker rejects every reverse dependency from crates, hosts, or other tools. Cross-target
qualification belongs to Issue 114.
