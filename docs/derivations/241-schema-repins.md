# Issue #241 — re-pin derivations for three pushed commits

Three commits on the `#241` schema branch landed their re-pins without the arithmetic that
explains them in the commit body. Per ruling `5462028562-A` (on #239) the derivation record is
decoupled from commit metadata: no history is rewritten, and this document carries the arithmetic
for the three SHAs named below. The amended S6/eval-7 gate is satisfied by
derivation-in-commit-body **or** derivation-in-a-linked-derivations-doc naming the commit.

Every number here was reconstructed from the diffs themselves (`git show <sha>`) and, where a
figure is a measured artifact rather than a byte count of checked-in text, re-measured against the
two trees it separates.

Shared premises used by all three sections:

* BTLV TLV framing (`docs/CONTROL_BTLV_V1.md`): each field is `id:u16, wire:u8, flags:u8,
  len:u32` = 8 header bytes, then the value padded up to the next 8-byte boundary. A `MESSAGE`
  value opens with `nested_count:u32, reserved:u32` = 8 bytes, then its nested TLVs. Therefore a
  scalar field of width <= 8 occupies **16** bytes; a UTF-8 field of `L` bytes occupies
  `8 + 8*ceil(L/8)`; and a message field occupies `16 + sum(nested field sizes)`.
* `SessionLimits` is `{ pcm_ring_frames: u64, control_queue_messages: u64, memory_bytes: u64 }`
  = **24 bytes** held by value inside `SessionToml`.
* The `#241` acceptance condition (ruling `5460535644` §4) is that no frozen whole-render output
  digest moves. The five `output:*` rows of `fixtures/native-pcm-runner/v1/MANIFEST.tsv` are
  byte-identical to `origin/main` across all of the work below.

---

## 04d291dd — `Implement canonical PCM source schema`

Deletes the `limits` table and the nested `content`/`mapping`/`region` source shape, and replaces
the source declaration with `{ id, content, channels, bit_depth, frames }`.

### 1. Session document byte deltas (the generator's output, not hand edits)

Two text deletions occur in every affected document: the whole `limits = { ... }` line, and the
replacement of the nested source line by the flat one.

`fixtures/session/v1/canonical.toml`

| term | bytes |
| --- | ---: |
| `limits = { pcm_ring_frames = 1024, control_queue_messages = 64, memory_bytes = 1048576 }\n` | -89 |
| old source line (`sample_rate_hz` + nested `content`/`mapping`/`region`), 196 bytes | -196 |
| new source line (`content`/`channels`/`bit_depth`/`frames`), 154 bytes | +154 |
| **total** | **-131** |

1894 - 131 = **1763** bytes, and its SHA-256 moves
`323768dd664277651ad79b6c5bae97eab0a4458cc533bd3e9267c41c24111999` ->
`36232a437c0280ad1166aeed4cc6a3c95d1260d088664757e776c2b3a065aa80`. That digest is the input to
section `c78de14a` below.

`fixtures/session/v1/parametric-eq-nine-track.toml` (`limits` line is 90 bytes here, because
`memory_bytes = 16777216` is one digit wider; source line 244 -> 163):

-90 + (163 - 244) = **-171**, so 9,817 - 171 = **9,646** bytes. This is exactly the pin spelled
`assert_eq!(PARAMETRIC_EQ.len(), 9_817 - 171)` in
`crates/miso-engine-session/tests/canonical_schema.rs`, whose FNV-1a-64 companion moves
`0xa7e3_594d_10fa_c382` -> `0x7bee_179a_a903_f382`.

The five `fixtures/native-pcm-runner/v1/*.toml` documents are generated from that same template,
so they take the same -90 on `limits` plus a -111 source line (their identity is a 64-hex digest
rather than the template's short name): **-201** each. 9,846 - 201 = **9,645** for the four RIFF
documents; the RF64 document is one byte shorter in both arms (`frames = 514` vs `frames = 1024`),
9,845 - 201 = **9,644**.

### 2. RF64 asset pre-slice (ruling 5460535644 §3)

Region selection is deleted, so the asset is regenerated as exactly the frames the old region
selected: the old document read `start_sample = 1, length_samples = 514` out of a 516-frame
asset, and `generate.py` now emits `wave(48000, True, 514, start=1)`.

516 - 514 = 2 frames removed x 2 channels x 4 bytes = **16 bytes**; 4,208 - 16 = **4,192**. The
80-byte RF64 header is unchanged in both arms: 4,208 - 516*8 = 80 and 4,192 - 514*8 = 80. The
514 retained frames are the same float bit patterns the render consumed before, which is why the
`output:rf64-48000` digest does not move.

That last sentence is checkable rather than asserted. Taking the two assets at `04d291dd^` and
`04d291dd`, stripping the 80-byte header from each, the new 4,112-byte sample payload is
**byte-identical** to `old_payload[8 .. 8 + 4112]` — that is, to old frames `[1, 515)`, exactly the
range the deleted `start_sample = 1, length_samples = 514` selected. The headers differ only in
their size words. No sample was re-encoded, re-rounded or re-ordered; the file simply starts where
the region used to.

### 3. Protocol conformance corpus (`crates/miso-engine-protocol/tests/conformance_corpus.rs`)

Frame count is unchanged at **46**; exactly one frame's bytes move, so the FNV-1a-64 roll over
`(name, bytes)` moves `0xeb7a_a549_b666_77a8` -> `0xbdeb_b0f8_1c38_ec42`.

`complete_all_opcode_fixture()` goes from **42 to 39** edits: opcodes `0x0006` `SetLimits`,
`0x0102` `SetSourceSampleRateHz` and `0x0104` `SetSourceMapping` are deleted, and `0x0103`
`SetSourceContent` is re-shaped. The fixture derives its values from `canonical.toml`, whose
source has `id = "voice"` (5 bytes), old `identity = "sha256:demo"` (11) and
`locator = "host:voice"` (10), and new `content = "sha256:<64 hex>"` (71).

Each edit is framed as `EDIT: MESSAGE { OPCODE: u16, PAYLOAD: MESSAGE }`, so
`edit_size = 16 + 16 + payload_message_size` = `32 + payload_message_size`.

Deleted (bytes removed from the transaction payload):

| opcode | payload composition | edit bytes |
| --- | --- | ---: |
| `0x0006` SetLimits | msg(3 x u64 = 48) = 64 -> payload msg 80 | 112 |
| `0x0102` SetSourceSampleRateHz | utf8("voice") 16 + u32 16 = 32 -> payload msg 48 | 80 |
| `0x0104` SetSourceMapping | utf8 16 + mapping msg(u8 16 + region msg 48) = 96 -> payload msg 112 | 144 |
| | **subtotal removed** | **336** |

Re-shaped:

| opcode | old edit bytes | new edit bytes | delta |
| --- | ---: | ---: | ---: |
| `0x0103` SetSourceContent | utf8 16 + content msg 64 = 80 -> 96 -> **128** | utf8 16 + utf8(71)=80 + u8 16 + u8 16 + u64 16 = 144 -> 160 -> **192** | +64 |
| `0x0100` UpsertSource | source msg(16+16+16+64+80)=192 -> 208 -> **240** | source msg(16+16+80+16+16+16)=160 -> 176 -> **208** | -32 |

Net: **-336 + 64 - 32 = -304 bytes**.

Measured against both trees (`complete_schema_corpus()`, frame
`command.session_transaction_apply`):

| | 04d291dd^ | 04d291dd | delta |
| --- | ---: | ---: | ---: |
| frame bytes | 6,344 | 6,040 | -304 |
| header payload length | 6,296 | 5,992 | -304 |
| header top-level TLV count | 42 | 39 | -3 |
| whole-corpus bytes (46 frames) | 10,480 | 10,176 | -304 |

The measured -304 equals the derived -304, and the only frame that moved is the transaction, which
is why the frame count survives untouched at 46.

### 4. Canonical consumers of the deleted 24 bytes

`SessionLimits` (24 bytes) leaves `SessionToml` by value. Both structures that embed exactly one
session by value lose exactly that:

* `size_of::<PreparedStructuralCommand>()`: 776 - 24 = **752**
* `size_of::<ProtocolController<MockProvider>>()`: 6,088 - 24 = **6,064**

`ReplayEntry` (56) and `ReplayCache` (88) are unchanged; the twelve embedded spsc endpoints are
untouched, so no other term in the controller moves.

### 5. Resource-estimate case counts

`estimate.rs` previously projected runtime storage from the document's own words:
`queue_bytes = control_queue_messages * 64`, `source_ring_bytes = pcm_ring_frames * channels * 4`,
`requested_runtime_bytes = queue_bytes + source_ring_bytes`. All four now read literal `0`,
because the ring is the host's choice under #240 and is re-derived after that choice.

The invalid-matrix corpus shrinks accordingly:

* the 20-case runtime/platform overflow test loses its subject entirely: **-20**.
* `configured_resource_category`: 2 zero/non-zero arms x 6 capped fields = 12, plus 2
  `memory_bytes` cases, plus 2 `capacity.zero` limits cases = 16. Four capped fields
  (`runtime`, `queue`, `frames`, `ring-bytes`) and all four limits cases are deleted, leaving
  2 x 2 = **4**.
* corpus distribution `[16, 20, 24, 16, 20, 20, 20, 16]` (sum 152) becomes
  `[16, 20, 24, 16, 20, 20, 4]` (sum 120): 152 - 20 - 12 = **120**.

Verification follow-up (Fable REVISE round): three `bit_depth` refusal rows were added to the
source-identity category, taking it 16 -> 19 and the distribution to
`[16, 20, 24, 19, 20, 20, 4]` = **123**.

### 6. The four caps this made inert, and why they were kept

Setting the four runtime terms to literal `0` makes four `CompileCaps` rows unable to refuse
anything: `check_caps` tests `value > limit`, and `0 > limit` is false for every `u64`. The
affected rows are `max_requested_runtime_bytes`, `max_queue_items`, `max_source_ring_frames` and
`max_source_ring_bytes` — exactly the four whose case count fell from 16 to 4 above.

Ruled choice (verifier fix 6): **pin, do not delete.** The two options and their arithmetic:

| option | diff | behavioural change |
| --- | ---: | --- |
| delete the fields, their `check_caps` rows and every construction site | 4 fields x 89 `max_queue_items`-class lines across **38 files**, plus a public-API break to `CompileCaps` | none |
| pin the inertness with one test and field docs | **1 test + 3 doc blocks** | none |

Neither option changes behaviour, because the caps already cannot refuse; the only question is
whether the dead-ness is stated or silent. The deletion is the larger diff by two orders of
magnitude, breaks a public struct that #240, #242 and #244 are constructing on three in-flight
branches, and buys nothing a test does not. The hand-off is real and already enforced elsewhere:
`miso-engine-host-core`'s prepare checks `total_engine_owned_bytes > maximum_source_total_bytes`
against the ring the host actually chose, and the C ABI compile path does the same — which is the
#240 S3.7 ordering, budget checked after the choice rather than against a document word.

`dead_resource_caps_cannot_refuse_any_session` therefore asserts both halves: that the estimate
reports zero for all four terms, and that a session compiles with all four caps set to `0`. If a
later issue re-populates the estimate, that row goes red and forces the decision instead of
letting a budget silently start biting.

---

## c78de14a — `test(audit): repin migrated builtins manifest`

Discharges the checked-in builtins corpus manifest whose contents moved one commit earlier, in
`1254cc1f`.

The two `prepare_256_tracks` benchmark documents name the canonical session by digest. That digest
is the `canonical.toml` SHA-256 derived in `04d291dd` section 1:
`323768dd...` -> `36232a43...`. Both are 64 lowercase hex characters, substituted in place, so
each benchmark document stays exactly **963 bytes** and every `size` column in the manifest is
unchanged.

| quantity | 1254cc1f^ | 1254cc1f |
| --- | ---: | ---: |
| `fixtures/builtins/v1/MANIFEST.tsv` lines (1 header + 50 payload rows) | 51 | 51 |
| `MANIFEST.tsv` bytes | 4,820 | 4,820 |
| payload rows whose sha256 column moved | — | 2 of 50 |
| `parse_manifest(...).entries.len()` pin | 50 | 50 |

48 of 50 rows are byte-identical; two rows change 64 hex characters each; the row lengths, the
row count and therefore the file length are all invariant. Only the manifest's own SHA-256 moves:

`1d2f8ffe8f56d08314e480f0aba7ee5068ae8448721504ae7f64db11a33f06c8` ->
`4b7b6ac1f1c2f16aecebb003c62b37420a96ca6f0bc1b75fa471654dcbc38ba5`

(verified by hashing the file at both revisions). No PCM, meter, response, diagnostics, resources
or benchmark-output fixture is touched, which is the arithmetic proof that this is a naming move
and not a render move: `cases.toml` stays 391,992 bytes, `diagnostics.jsonl` 1,324, `metadata.toml`
395, and every `f32le` payload keeps its digest.

---

## 097da5c2 — `test(graph): repin zero-delay semantic identity`

Re-pins `the_zero_delay_plan_digest_is_the_pre_feature_digest` in
`crates/miso-engine-graph-compiler/tests/track_delay.rs`.

The graph's canonical text was dumped for the nine-track fixture at both revisions
(`GraphCompiler::evidence(...).canonical_bytes`) and diffed. The full text is **34,051 bytes in
both arms**, and exactly one of its 685 lines differs — the trailing `estimate` row, in exactly
one of its twenty fields:

```
- estimate 82 82 81 82 10 1 9 9 23305 0 0 48915 5544 0 0 0 0 48915 147679 159967
+ estimate 82 82 81 82 10 1 9 9 23305 0 0 48915 5544 0 0 0 0 48915 147679 147679
```

Every structural term is identical: 82 logical nodes, 82 materialized nodes, 81 edges, 82 schedule
items, 10 dependency levels, 1 reduction, 9 routes, 9 effects, 23,305 audio buffer samples,
**0 total delay samples and 0 delay bytes** (the zero-delay claim itself), 48,915 graph metadata
bytes, 5,544 declared effect bytes, 0 effect-bank terms, and 147,679 incremental plan bytes.

Only `session_plus_plan_bytes` moves: **159,967 -> 147,679**, a delta of **-12,288**, and it now
equals `incremental_plan_bytes` exactly, because the session term went to zero:

```
requested_runtime_bytes = queue_bytes + source_ring_bytes
                        = control_queue_messages * 64  +  pcm_ring_frames * channels * 4
                        = 64 * 64                      +  1024 * 2 * 4
                        = 4096                         +  8192
                        = 12288   ->   0
159967 - 12288 = 147679
```

That is the whole cause. The digest therefore moves

`60a22fd833ca1a2ffcb1329e7ba228e51a0b91246c4dd93fb805e7c47221ab96` ->
`213617ba7e5774e831785e725f8cb70bdd0f043cba9ae071e139888935acf4b0`

(both re-measured; the old value reproduces the constant the test carried before, and the new
value reproduces the constant it carries now).

Note for the record: this commit's body — and, until the Fable REVISE round, the in-tree comment
at `track_delay.rs:225` — attributes the move to source content identity entering the compiled
graph's semantic text. That is not what happened — `node_text`/`edge_text` never
carried a source identity, and `write_canonical` emits no source row at all. The mover is the
`limits`-derived runtime projection in the estimate row, as derived above. The pin is correct; the
prose reason recorded in the commit was not, and this document supersedes it.

The companion assertion `a_delayed_session_is_a_different_plan` keeps its meaning: a 480-sample
delay changes `total_delay_samples` and `delay_bytes` away from 0, so the digest is not inert.
