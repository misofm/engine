# Design issue — parameter lattice v2: objective base units, integer canonical values

**Supersedes:** the geometric parameter lattice of #239 ruling 5461507633-B, per owner direction
of 2026-08-30. **Sequenced AFTER #246 merges** (§9).

## Verification record

Designed by **Opus**; adversarially verified by **Fable**; errata folded; coordinator rulings baked
in as settled (§11).

| verification | result |
|---|---|
| blessed-site sweep (§4.3) | **21 M conversions bit-exact** against a correctly-rounded reference, including **disassembly-level confirmation of MXCSR immunity** — no FP-arithmetic opcode present |
| fixture sweep (§5.3) | **reproduced exactly**: 7 128 checked / 7 057 legal / 71 illegal, and the three findings F1/F2/F3 confirmed independently |
| rulings consistency (§7) | all **seven** rulings consistency-checked; A7 abridgement confirmed faithful, with two caveats now folded in as their own rows |
| errata | **5 confirmed and folded**: two ulp distances (114, and 114/1564), the F3 token provenance, four row counts in §1.2, and the SDK generator / README / `decimalToFloat32` dispositions |

Verdict: **SOUND**. Every erratum was a stated number that was wrong, not a conclusion that was
wrong; no finding, law, or disposition changed under verification.

**Status:** design + issue. No implementation, no worktree, no repo edits.
**Anchor:** engine `origin/main` @ `d6d6359edb5575c33d7e68b7c808359d37098f96` (fetched; all
file/line citations below are at that commit).
**Supersedes in part:** #242 as merged (`docs/derivations/242-parameter-lattice.md`), #239 ruling
A12, #242 S1's unit-class table.
**Lands after:** #246 (the app's vendored SDK is pinned there; see §9).

---

## 0. The redirection, in one paragraph

The owner ruled two sentences:

1. *"Maybe the step shouldn't be perceptual and should be unit-specific in an objective way
   instead. The perceptual layer could be built on the client side instead as an abstraction over
   the base step unit."*
2. *"Why don't we store parameter values in a numerical unit that doesn't have decimals?"*

Ratified interpretation, and the whole of this design: **every continuous parameter gets one
objective decimal resolution `10^-k` in its own native unit (the *base unit*), and its canonical
persisted value is an INTEGER count `n` of that base unit.** The lattice stops being a set of
generated geometric points and becomes the closed integer interval `[n_min, n_max]`. Documents
still spell human decimals; the decimal ↔ integer map is pure string arithmetic with zero
floating-point operations, which removes the rounding-mode hazard *by construction* rather than by
containment. `f32` is entered at exactly one integer→float site with a specified,
hardware-independent algorithm. Perceptual stepping — cents for frequency, ratios for time and Q —
moves out of the descriptor and into the SDK, where it computes a target and snaps to the integer
grid.

This is a strict simplification. It deletes `StepUnit::Cents`, `StepUnit::Ratio`, geometric point
generation, `MAXIMUM_LATTICE_POINTS` as a validation concern, the `f32` step word on the wire, and
the entire "does this row's rendering precision survive its own bound" class of defect. It costs
one new law (the f32-distinctness ceiling, §1) and one surviving exception (the Butterworth Q
default, §2.4).

---

## 1. Per-unit precision, with the f32-resolution argument

### 1.1 The law

> **L1 (distinctness).** For a row with base unit `10^-k` and declared value magnitudes bounded by
> `V = max(|min|, |max|, |default|)`, adjacent integer counts must map to distinct `f32`s
> everywhere in the row's range: `10^-k > ulp_f32(v)` for every `|v| <= V`.

`ulp_f32(v)` for `v` in the binade `[2^e, 2^(e+1))` is `2^(e-23)`. `ulp` is monotone in `|v|`, so
L1 reduces to a single test at the top of the range. Solving `2^(E-23) >= 10^-k` for the first
*unsafe* binade `E` gives a closed-form ceiling per `k`:

| `k` | base unit | first unsafe binade `E = ceil(23 + log2(10^-k))` | **distinct-safe while `\|v\| < 2^E`** |
|---:|---|---:|---:|
| 0 | 1 | 23 | 8 388 608 |
| 1 | 0.1 | 20 | 1 048 576 |
| 2 | 0.01 | 17 | 131 072 |
| 3 | 0.001 | 14 | 16 384 |
| 4 | 0.0001 | 10 | 1 024 |
| 5 | 0.00001 | 7 | 128 |
| 6 | 1e-6 | 4 | 16 |
| 7 | 1e-7 | 0 | 1 |
| 8 | 1e-8 | -3 | 0.125 |

Worked check of the owner's own example, which is exactly right: at `k = 3` (0.001 Hz) the ceiling
is `2^14 = 16384 Hz`. At 8192 Hz `ulp = 2^(13-23) = 2^-10 = 0.0009765625`, so the margin has
already collapsed to `0.001 / 0.0009765625 = 1.024x` — one part in forty. At 16384 Hz
`ulp = 2^-9 = 0.001953125 > 0.001` and distinctness is **gone**: two adjacent 0.001 Hz counts are
the same `f32`. So "0.001 Hz fails above ~8 kHz" is the right practical statement and 16384 Hz is
the hard wall. At `k = 2` the ceiling is `2^17 = 131072 Hz`, which covers the 96 kHz launch rate's
Nyquist clamp (47998.867 Hz) with a **2.56x** margin and the 20 kHz EQ rows with **5.12x**.

### 1.2 The table

The owner's unit list maps onto the engine's `ParameterUnit` / builtin `BuiltinParameterMapping`
vocabulary as follows. `Q` and `%` and `pan` are not units in the enum; they are display faces of
`Ratio` and `Linear`.

| owner's unit | engine unit | `k` | base unit | worst shipped `\|v\|` | `ulp` there | `k_max` | margin | rows |
|---|---|---:|---|---:|---:|---:|---:|---|
| Hz | `ParameterUnit::Hz` | **2** | 0.01 Hz | 47998.867 (96 kHz clamp) | 3.906e-3 | 2 | 2.56x | `hpf_hz`, `lpf_hz`, 4x `band-N-frequency`, `crossover` |
| dB | `ParameterUnit::Db` | **4** | 0.0001 dB | 144 (`trim_db`/`fader_db` floor) | 1.526e-5 | 4 | 6.55x | 18 rows (+1 override) |
| dB (one override) | `ParameterUnit::Db` | **5** | 0.00001 dB | 24 (`ceiling` range) | 1.907e-6 | 5 | 5.24x | `miso.true-peak-limiter` `ceiling` only — see §5.4 |
| ms | `ParameterUnit::Milliseconds` | **3** | 1 µs | 5000 (compressor/multiband release) | 4.883e-4 | 3 | 2.05x | 15 rows |
| ratio / Q | `ParameterUnit::Ratio` | **4** | 0.0001 | 20 (compressor ratio) | 1.907e-6 | 5 | 52.4x | 12 rows (incl. the 4 `band-N-shelf-slope` rows) |
| % / pan / mix / matrix | `ParameterUnit::Linear` | **4** | 0.0001 | 1.0 | 1.192e-7 | 6 | 839x | 14 rows (9 effect + 5 builtin) |
| samples | `ParameterUnit::Samples` | **0** | 1 sample | 48000 | 3.906e-3 | 2 | 256x | `delay_samples` |
| choice / boolean | domain `Enumeration`/`Boolean` | **0** | 1 ordinal | — | — | — | — | 8 enum + 2 boolean rows |

Two rows are deliberately *not* at their `k_max`:

- **ratio/Q at 4 rather than 5.** `k = 5` is legal (52x -> 5.2x margin) but buys nothing: no
  authored value in the tree needs more than 2 fraction digits of ratio, and `k = 4` leaves a 52x
  cushion against a future row whose declared maximum grows.
- **linear at 4 rather than 6.** `k = 6` is legal, but 0.0001 already means 0.01 % on a normalized
  control, and it keeps the whole unit's rank span at 20 000 rather than 2 000 000.

**Verified against every shipped row.** All 78 continuous+builtin rows were checked against L1 and
against the span law below; the only two flags are the two known exceptions carried forward into
§2.4 and §5.4. Largest rank span in the whole catalog is **4 995 000** (compressor `release`, ms,
`k = 3`), which is `0.00116x` of `u32::MAX`.

> **L2 (span).** `n_max - n_min < u32::MAX` strictly. This is what keeps
> `miso_engine_builtins::DISABLED_LATTICE_INDEX = u32::MAX`
> (`crates/miso-engine-builtins/src/lib.rs:349`) permanently outside every lattice, which today is
> true only by accident of the geometric point counts.

### 1.3 What L1 is actually protecting

L1 is not aesthetic. Without it, two distinct canonical integers produce one `f32`, and then:

- the persist plane is injective but the *engine* is not, so a load→prepare→observe→save cycle can
  silently move a document if any surface ever reconstructs a value from the prepared word;
- a one-base-unit nudge produces no audible change, which makes the smallest rung of any client's
  ladder a no-op — the UX property the ladder exists to deliver.

---

## 2. The integer value model

### 2.1 Declaration

`ParameterLattice` collapses from four fields to one:

```rust
/// One parameter's persisted-value base unit (issue NNN, superseding #242 S1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParameterLattice {
    /// Decimal exponent of the base unit: the value's resolution is `10^-precision`
    /// in the parameter's declared `ParameterUnit`. `0..=8`.
    pub precision: u8,
}
```

`step: f32`, `step_unit: StepUnit`, and `ladder: StepLadder` all leave the descriptor.
`StepUnit` is deleted outright. `StepLadder` survives *as catalog metadata only* — see §2.6.

`default_parameter_lattice(unit, domain, mapping)` becomes
`default_precision(unit, domain) -> u8` and no longer consults `mapping` at all, because the
mapping no longer changes the storage grid. That is the single largest simplification in this
design: **`Logarithmic` becomes a purely presentational hint** describing how a client should map a
normalized control onto the range. It stops being a storage law.

### 2.2 The value

For a continuous row, the legal values are exactly

```
{ n / 10^k  :  n integer, n_min <= n <= n_max }        (plus at most one intrinsic escape, §2.4)
```

where `n_min = scaled(shortest(min), k)` and `n_max = scaled(shortest(max), k)` — see §2.5 for why
the declared `f32` bounds can stay `f32` and still yield exact integers.

Membership is a pure decimal predicate on the document text, decided in `i128`:

```
member(text) :=
    let d = ExactDecimal::parse(text)?            # existing, already exact, zero FP
    d.fraction_digits_after_trailing_zero_strip() <= k
    && n_min <= d.scaled(k) <= n_max
```

Cost: O(len(text)). No point generation, no allocation, no `MAXIMUM_LATTICE_POINTS`, no binary
search over a materialized vector. `parameter_lattice_points` becomes an optional control-plane
convenience (and must be bounded, because a full materialization of the catalog is 57 683 390
points).

### 2.3 The rank law

> **L3 (rank).** For a row with no off-grid intrinsic member, `rank(n) = n - n_min`, exactly.
> Rank 0 is the declared minimum; rank `n_max - n_min` is the declared maximum; `step(±1)` is
> `n ± 1`.

This is the coincidence the redirection buys. #242 S3 originally spelled the wire carrier as
"`u32` k relative to `min`"; adopted ruling **5462139867 finding 7** superseded that with *rank in
the totally-ordered member set*, because geometric generation made the two different numbers (see
`sdk/src/core/lattice.ts:27-33`, the only statement of finding 7 in the tree). Under a linear
integer grid, **`k`-relative-to-min and rank are the same number again** for every row except the
four Q rows. Finding 7 is therefore *preserved as the definition* and *restored to agreement* with
S3's original spelling.

With one off-grid intrinsic `d` lying strictly between grid counts `n_lo` and `n_lo + 1`:

```
rank(n) = n - n_min                     for n <= n_lo
rank(d) = n_lo - n_min + 1
rank(n) = n - n_min + 1                 for n >= n_lo + 1
```

Still O(1), still closed-form, still monotone. `stepsTo` (SDK) remains rank subtraction.

### 2.4 Intrinsic members: are they subsumed by a per-row precision override?

**No.** The question is whether the `miso.delay` `damping` precedent — a row overriding its class
precision so it can spell its own bound — can absorb the verbatim-spelling intrinsic members like
`band-N-q`'s Butterworth default `0.70710677`. Three independent kills:

1. **Spellability.** `f32(1/√2) = 0.707106769084930419921875` exactly; its `f32` rounding interval
   is `[0.7071067392826080322265625, 0.7071067988872528076171875]`. Exhaustive check of the
   nearest decimal on each side at every `k`: **no decimal with 7 or fewer fraction digits lies
   inside that interval.** `k = 8` is the minimum precision that names this `f32`.
   (`0.7071` -> `0x3f350481`, `0.7071068` -> `0x3f3504f4`, `0.70710677` -> `0x3f3504f3`.)
2. **L1.** `k = 8`'s distinctness ceiling is `|v| < 0.125`. The `band-N-q` row declares
   `[0.1, 18.0]`. At 18.0, `ulp = 2^-19 = 1.907e-6` and `10^-8` is **190x smaller** — 190
   consecutive integer counts share one `f32`. The override is unlawful under L1.
3. **L2 / tractability.** `(18.0 - 0.1) x 10^8 = 1 790 000 000` ranks. It fits `u32` but it is
   1790x the current control-plane cap and it makes the smallest rung of any ladder a no-op.

So: the override cannot subsume it, and the intrinsic-member mechanism must survive — **tightened**
to its minimum useful form:

> **L4 (intrinsic escape).** A row may carry **at most one** off-grid intrinsic member: its
> declared *default*. Declared bounds may **not** be off-grid (they define `n_min`/`n_max`; an
> off-grid bound is a declaration error). The off-grid default is declared as an exact decimal
> string in the descriptor, its `f32` must be distinct from both flanking grid points' `f32`s, and
> validation checks all of that in exact decimal.

**Verified: exactly four rows in the shipped catalog need L4** — `miso.parametric-eq`
`band-{1,2,3,4}-q`, all with the same default. Every other bound and default in all 78 rows has a
shortest `f32` spelling with `<= k` fraction digits under the §1.2 table, so is on-grid by
construction.

The mirror-image result is just as informative: **the `damping` override dissolves.** `0.995` at
`linear k = 4` is `9950` exactly, so `miso.delay` `damping` no longer overrides anything, its
descriptor window returns to canonical zeros, and its descriptor identity returns to its
pre-#242 value (§5.5).

The alternative — respell the Butterworth default onto the grid — is **rejected as a
recommendation, and the owner has RULED it out (§11, Q1a)**, because `f32("0.7071") != f32(1/√2)` (114 ulps
apart), so it moves the prepared biquad coefficient and therefore moves render digests for every
document that uses the default Q — a class-A change. **Ruled: keep the escape** (§11, Q1a).

### 2.5 Why the declared bounds can stay `f32` on the wire

Naively, an integer model wants `n_min`/`n_max`/`n_default` as `i32` counts in the descriptor
record at offsets 36/40/44 — which would move **every** descriptor identity. It is not necessary:

> **L5 (exact bound recovery).** `n = scaled(shortest_display(bound_f32), k)`, where
> `shortest_display` is the software float formatter's shortest round-tripping spelling. This is
> exact, allocation-free (the existing `fraction_digits` stack-buffer trick,
> `crates/miso-engine-effect-contract/src/step.rs:280-313`), and contains **zero floating-point
> arithmetic instructions** — the shortest-`f32` formatter is integer code. Validation refuses any
> bound whose shortest spelling exceeds `k` fraction digits, which is *exactly today's*
> `intrinsic_spellable` check, unchanged.

The critical soundness point: `f32(0.1)` is **not** `1/10`; it is
`0.100000001490116119384765625`. A law demanding "the bound is an exact multiple of the base unit
as a real number" would refuse `miso.compressor` `attack`'s minimum. L5 asks the right question
instead — *does this `f32`'s own decimal NAME land on the grid?* — and the answer is yes for 77 of
78 rows.

### 2.6 The ladder leaves the wire

The ladder cannot survive at offsets 72/76: its multiples are counts of the base unit, and a UI
nudge of 1 Hz is 100 base units, which does not fit the frozen 5-bit fields. More importantly it
**should not** be there:

- nothing in preparation, rendering, descriptor verification or wire binding reads it;
- it is a UX recommendation, and putting a UX recommendation inside a sealed identity means a taste
  change re-pins descriptor identities and the `#108` benchmark digest.

Ruled: `ParameterDescriptor` keeps a `recommended_steps` field, the descriptor **wire does not
encode it**, `bind_effect_descriptor_wire`'s semantic-equality leg does not compare it, and the
catalog carries it for the SDK. Its shape becomes gesture-typed rather than base-unit-typed, so it
survives the perceptual move:

```rust
pub enum StepGesture { Units(u32), Cents(u16), Ratio(ExactDecimalLiteral) }
pub struct RecommendedSteps { pub gesture: StepGesture, pub multiples: [u16; 5] }
```

`FADER_STEP_LADDER` (`[1,5,10,30,60]` over `Units(1000)` = 0.1 dB) and `DEFAULT_STEP_LADDER`
(`[1,3,5,10,30]`) both restate cleanly; the Hz rows get `Cents(20)`, the log-ms and ratio rows get
`Ratio("1.02")`. #242's A12 finding is thereby **preserved where it was right** (a frequency dial
should step in cents) and **relocated to where it belongs** (the client), which is the owner's
sentence 1 exactly.

### 2.7 The disabled sentinel

Unchanged. `DISABLED_LATTICE_INDEX = u32::MAX` stays the reserved rank for
`DisabledOrRateKeyedHertz`; the domain's escape is still matched by `to_bits()` against
`disabled: 0.0` (`crates/miso-engine-builtins/src/lib.rs:312-314`); the canonical rendering of the
sentinel is now `"0.00"` (Hz `k = 2`) rather than `"0.000"`. L2 makes the sentinel's disjointness
from the lattice a *proved* property rather than an observed one.

### 2.8 The rate-keyed ceiling

Per #239 ruling 5461507633 B2 and #242 derivation §5 the rate clamp is **not** a declared bound and
**not** a lattice member, so L5 does not apply to it (which is why `47998.867` carrying 3 fraction
digits at Hz `k = 2` is not a finding). Its law restates as:

> `n_top(rate) = floor(clamp(rate) x 10^k)`, computed in exact integer arithmetic from the `f32`
> clamp's dyadic decomposition (`f32` = `m x 2^e` exactly; the product and floor are `i128`).
> No FP instruction.

At the four launch rates this gives top points `22049.48 / 23999.43 / 44098.96 / 47998.86`, versus
today's geometric `23798.694` at 48 kHz. The lattice top moves *closer* to the clamp, which is a
strict improvement and a re-pin in
`crates/miso-engine-effect-compiler/tests/parameter_lattice.rs:309+`.

---

## 3. Parsing: decimal spelling ↔ integer, exact, both directions

### 3.1 Accepted spellings (unchanged from #242's `ExactDecimal`)

`ExactDecimal::parse` (`crates/miso-engine-effect-contract/src/step.rs:731-826`) already does
exactly the right thing and needs **no change**: optional sign, `_` separators, optional fraction,
optional decimal exponent, leading integer zeros and trailing fraction zeros stripped, `-0` folded
to `0`, hex/inf/NaN refused, 512-character cap. So `0.3`, `0.30`, `+0.300`, `3e-1` and `0.3000000`
are one value; and per ruling 5462028562-B all of them are accepted.

New, and the only addition: after normalization, the fraction must have `<= k` digits and the
scaled integer must lie in `[n_min, n_max]`.

```
decimal -> integer:   n = sign * (integer_digits ++ fraction_digits.pad_right(k)) as i64
integer -> decimal:   text = sign ++ (|n| / 10^k) ++ "." ++ zero_pad_left(|n| % 10^k, k)
```

Both directions are string/`i128` only. **Zero floating-point operations in either direction** —
which is the whole point: #242 derivation §8's open hazard is not contained, it is *deleted*, since
`str::parse::<f32>()` no longer appears on the parsing path at all.

### 3.2 The canonical writer

Ruling **5462028562-B** descoped descriptor-aware re-spelling from #242 and stated: *"the canonical
writer preserves accepted spellings byte-stably and performs no descriptor-aware re-rendering."*

**Finding (verified, and it complicates the ruling's first clause):** the shipped session model
does **not** preserve float spellings. `crates/miso-engine-session/src/model.rs:324` stores
`pub value: f32`, the accepted token is discarded at `value.rs:11-19`, and
`canonical.rs:139` re-renders through `value.rs::write_f32` — shortest `f32` `Display`, with an
exact-`f64` `Display` fallback for the two double-rounding values, plus a `.0` suffix for integral
spellings. So `0.30` in an accepted document canonicalizes to `0.3`. The half of B that *is*
honoured is the operative half — no *descriptor-aware* re-rendering, and both spellings prepare the
same `f32` — and the required "spelling variance is render-inert" eval holds. But the sentence
"the DOCUMENT'S accepted spelling is preserved as written" is true only of documents that are never
canonicalized.

Two options, and this design **recommends the first** for landing:

- **W-A (recommended, zero fixture churn).** Leave `write_f32` alone. Add one proof obligation:
  *lattice closure of the writer* — for every legal `n` of every shipped row,
  `scaled(shortest_display(blessed(n, k)), k) == n`. Sampled verification over 964 495 values
  across the six worst-margin rows found **zero mismatches**; the exhaustive sweep (57 683 390
  values) is an implementer eval (§8.6). Every fixture stays byte-identical.
- **W-B (cleaner, expensive).** Store `value: i32` in the session model and write the canonical
  base-unit decimal (`0.0` -> `0.0000`). This makes the writer idempotent by construction, deletes
  the `f64` double-rounding fallback, and makes the writer's output the lattice's own vocabulary —
  but it re-spells every float in every session fixture, moving the session-fixture byte hashes
  pinned in `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`,
  `fixtures/builtins/v1/MANIFEST.tsv` and `tools/miso-engine-audit/src/fixture_builtins.rs`.

W-B is the right end state; it belongs in its own separately-ruled writer issue, exactly as
5462028562-B anticipated.

---

## 4. The one blessed integer→`f32` site

### 4.1 Signature and contract

```rust
/// The sole integer→engine-`f32` conversion. Returns the correctly rounded (nearest,
/// ties-to-even) `f32` nearest the exact rational `n / 10^k`.
///
/// Executes NO floating-point instruction. The result is therefore identical under every
/// MXCSR/FPCR word: rounding mode, FTZ and DAZ have nothing to act on.
#[must_use]
pub fn base_units_to_f32(n: i64, k: u8) -> Option<f32>;
```

`decimal_to_f32(&str)` is **deleted**. (It has, today, *zero production call sites* — every one of
its 13 invocations is in a test; see `crates/miso-engine-effect-contract/tests/lattice.rs` and
`crates/miso-engine-effect-compiler/tests/parameter_lattice.rs`. Its whole stated value was
"auditing that boundary is one grep"; the grep currently finds only tests, so deleting it costs
nothing and replacing it with an integer-domain function is a strict upgrade.)

### 4.2 Why not the f64 route

The alternative floated in #242 derivation §8 — "integer→`f64` exact for `|n| < 2^53`, then a
halving chain" — is **rejected on two grounds**. First, the division `n as f64 / 10^k as f64` is an
FP instruction and is therefore MXCSR-sensitive, which is the entire defect being fixed. Second,
even in a default environment `f64`-then-`f32` **double-rounds**: the `f64` result is correctly
rounded to 53 bits and then re-rounded to 24, which is not in general the correctly rounded 24-bit
result. Integer arithmetic has neither problem.

### 4.3 The algorithm

Preconditions, all checked and all satisfied by every shipped row (§1.2): `k <= 8`,
`|n| < 2^40`. These bound the exact value `v = |n| / 10^k` into `[10^-8, 2^40)`, strictly inside
the `f32` normal range `[2^-126, 2^128)`, so there is **no subnormal case and no overflow case** —
stated as an invariant and asserted, not handled.

```
fn base_units_to_f32(n: i64, k: u8) -> Option<f32> {
    if k > 8 || n.unsigned_abs() >= 1u64 << 40 { return None; }
    if n == 0 { return Some(f32::from_bits(0)); }               // exact +0.0
    let sign: u32 = if n < 0 { 1 << 31 } else { 0 };
    let m: u128 = n.unsigned_abs() as u128;                      // 1 <= m < 2^40
    let d: u128 = 10u128.pow(k as u32);                          // 1 <= d < 2^27

    // 1. Bracket the binary exponent.  bits(x) = 128 - x.leading_zeros().
    //    2^(bits(m)-bits(d)-1) <= m/d < 2^(bits(m)-bits(d)+1).
    let bm = 128 - m.leading_zeros() as i32;
    let bd = 128 - d.leading_zeros() as i32;

    // 2. Produce a quotient with 26 or 27 significant bits, plus a sticky remainder.
    //    t <= 26 + 27 = 53, and m << t <= 2^40 * 2^53 = 2^93 < 2^128, so u128 suffices.
    let t: i32 = 26 - (bm - bd);
    let (num, den) = if t >= 0 { (m << t, d) } else { (m, d << (-t)) };
    let mut q: u128 = num / den;
    let mut sticky: bool = (num % den) != 0;
    let mut e2: i32 = -t;                                        // v == q * 2^e2 (+ sticky)

    // 3. Normalize down to exactly 25 bits: 24 significand bits + 1 round bit.
    while q >= (1u128 << 25) { sticky |= (q & 1) != 0; q >>= 1; e2 += 1; }
    debug_assert!(q >= (1u128 << 24));                            // 25 bits by construction

    // 4. Round to nearest, ties to even, on the round bit + sticky.
    let round_bit = (q & 1) != 0;
    q >>= 1; e2 += 1;                                             // q now 24 bits
    if round_bit && (sticky || (q & 1) != 0) {
        q += 1;
        if q == (1u128 << 24) { q >>= 1; e2 += 1; }               // carry out of the significand
    }

    // 5. Pack.  value = q * 2^e2 with 2^23 <= q < 2^24, so the unbiased exponent is e2 + 23.
    let exponent: i32 = e2 + 23 + 127;
    if !(1..=254).contains(&exponent) { return None; }             // invariant, never taken
    let mantissa = (q as u32) & 0x007f_ffff;
    Some(f32::from_bits(sign | ((exponent as u32) << 23) | mantissa))
}
```

Every operation is `u128`/`i32` integer arithmetic, a shift, or `f32::from_bits` — which is a bit
reinterpretation (`movd`), not an arithmetic instruction. **There is no instruction in this function
whose result the MXCSR rounding-mode field, FTZ bit or DAZ bit can change.** That is the immunity
argument, and it is structural rather than empirical.

### 4.4 Where it is called

Exactly one production path: `prepare`, converting a validated integer count to the prepared word.
Automation segment endpoints go through the same site (they are persisted state). Live/command-plane
values do **not** — they stay continuous `f32` and never touch this function (§7, A7/D1).

---

## 5. Migration from the merged #242 surface

### 5.1 What changes

| surface | change |
|---|---|
| `crates/miso-engine-effect-contract/src/step.rs` | `ParameterLattice` -> `{ precision: u8 }`; `StepUnit` deleted; `StepLadder` -> `RecommendedSteps` and moved off the wire; `parameter_lattice_points*` become bounded control-plane helpers; `lattice_index_for_decimal` becomes O(1) arithmetic; `decimal_to_f32` -> `base_units_to_f32`; geometric generation deleted (~200 lines) |
| `crates/miso-engine-effect-contract/src/lib.rs` | re-export list; `ParameterDescriptor.lattice` type; `DescriptorDiagnosticCode::Lattice` semantics |
| `crates/miso-engine-effect-package/src/wire.rs` | `pack/unpack_lattice_spec` replaced (§5.2); canonical-zeros rule preserved verbatim in spirit; torn-window rule restated |
| `crates/miso-engine-effect-package/include/*.h`, `src/ffi.rs` | the seven `STEP_*_MASK_V1` macros collapse to one `PRECISION` mask; `step_bits` field renamed |
| `crates/miso-engine-builtins/src/lib.rs` | 12 lattice declarations; `builtin_parameter_lattice_points` adapter; rate-clamp floor law (§2.8) |
| 8 effect crates | one `lattice:` literal per row; `miso-engine-delay`'s `with_lattice` helper **deleted** (its reason evaporates) |
| `crates/miso-engine-session/src/*` | **W-A: no change.** (W-B would touch `model.rs`, `value.rs`, `canonical.rs`.) |
| `tools/miso-engine-parameter-metadata/src/lib.rs` | `step_object` -> `{ "precision": k }` + a sibling `recommendedSteps` object; **schema tag bumps `miso.web.parameter-metadata.v1` -> `.v2`** (ruling Q4), which re-pins `provenance.ts`'s `schemas.catalog` and every consumer that asserts the tag |
| `tools/.../bin/lattice_oracle.rs` | **must stop digesting every point** — 57.7M points is not a test. Redesign: digest `(k, n_min, n_max, intrinsic escapes, rank(min), rank(max), rank(default))` plus a pinned pseudo-random sample of 1024 ranks per row |
| `sdk/src/core/lattice.ts` | `latticePoints()` **must not materialize** (5M objects for one row); becomes `LatticeView` with `rankOf(decimal)`, `decimalOf(rank)`, `count`, `nearest(decimal)`, all O(1) |
| `sdk/src/core/agent.ts` | `ParameterHandle` keeps its rank-holding shape unchanged; `points: readonly LatticePoint[]` -> `view: LatticeView`; `step()` moves to the perceptual layer (§6) |
| `sdk/codegen/generate.mjs` | **the generator must learn the new step object.** It emits `export type StepDeclaration = EffectParameter<EffectId>["step"]` and `export type StepSizeName = keyof StepDeclaration["ladder"]` (`generate.mjs:83-84`); the second line breaks outright once `ladder` leaves the `step` object, so `StepSizeName` must be re-derived from `RecommendedSteps["multiples"]` |
| `sdk/src/core/agent.ts` — `decimalToFloat32` | **survives, re-scoped and re-documented.** See §5.7 |
| `sdk/README.md`, `sdk/PROVENANCE.*` | the vendoring contract's engine-commit pin and the `schemas.catalog` tag (now `.v2`, ruling Q4) |
| `sdk/src/generated/catalog.ts`, `sdk/assets/*.json` | regenerate |
| `scripts/check-parameter-metadata-v1.py` | `validate_step` rewritten; `STEP_UNITS` deleted; new self-test mutations |
| `scripts/check-step-vocabulary.py` | unaffected in substance; its allow-list rows pointing at `check-parameter-metadata-v1.py` and `242-parameter-lattice.md` must be re-checked (a stale row fails the gate) |
| `docs/EFFECT_CONTRACT_V1.md` | the default-domain table (lines ~78-90) and the builtin table (~92-107) replace wholesale |
| `docs/derivations/` | new derivation doc; **and finding 7 gets written into `docs/` for the first time** — today it exists only in two source comments |

### 5.2 The wire

The 80-byte parameter record does not grow; offsets 72/76 stay the lattice window. New meaning:

| offset | old | new |
|---:|---|---|
| 72 | `f32::to_bits(step)` | `precision` in bits 0..3; **bits 4..31 must be zero** (`Code::Reserved`) |
| 76 | ladder 5/5/5/5/6 + precision 4 + step_unit 2 | **must be zero** (`Code::Reserved`) — the ladder is no longer wire state |

The three enforcement legs are **preserved verbatim in structure**:

- **canonical zeros.** All-zero window = `default_precision(unit, domain)`. A window that spells the
  derived default explicitly is still refused as a second spelling, `Code::Reserved` at
  `record + 72`. The proof test
  `an_explicitly_spelled_derived_lattice_is_refused_as_a_second_spelling`
  (`crates/miso-engine-effect-package/tests/descriptor_v1_qualification.rs:880-936`) restates
  directly.
- **torn window.** Today `(step_bits == 0) != (lattice_spec == 0)` is `Code::Flags`. Since 76 is now
  always zero, the rule restates as: *offset 76 must be zero, always*, `Code::Reserved` at
  `record + 76`.
- **semantic equality on bind.** Unchanged shape; compares `precision` only.

### 5.3 Fixture story — VERIFIED

The claim to test is #242 derivation §11's ledger: **2 320 persisted values across 8 of 14 session
fixtures are off-lattice today.** The design's claim is that the new grid legalizes them.

Method: every `params = [{ parameter_id, channel, unit, value }]` record and every builtin key in
`fixtures/session/v1/*.toml` was extracted, joined to its row's unit and domain from
`sdk/assets/miso-engine-v2-parameter-metadata.json`, normalized (trailing fraction zeros stripped,
so `1.0` at `k = 0` is the integer 1), and tested against the §1.2 table.

**Result: 7 128 values checked, 7 057 legal, 71 illegal.** Every one of the 2 320 formerly
off-lattice values in §11's ledger is legal under the new grid — `hpf_hz 30.0` -> 3000 units,
`fader_db -2.75` -> -27 500 units, `compressor ratio 1.5` -> 15 000 units, `band-1-frequency 90.0`
-> 9 000 units, `attack 2.0` -> 2 000 units, and so on — **except** the following, which are
findings with dispositions:

| # | row | values | occ. | fixtures | disposition |
|---|---|---|---:|---|---|
| **F1** | `miso.true-peak-limiter` `ceiling` (dB) | 32 distinct odd multiples of 1/32 dB, `-0.53125` … `-2.46875`, all 5 fraction digits | 64 | `console-sixty-four-track-intended.toml` (32), `console-sixty-four-track-mono.toml` (32) | **per-row `k = 5` override**, §5.4 |
| **F2** | `miso.parametric-eq` `band-N-q` (ratio) | `0.70710677` (8 digits) | 1 | `parametric-eq-nine-track.toml` | **L4 intrinsic escape** (§2.4), ruled Q1(a) |
| **F3** | `miso.parametric-eq` `band-N-q` (ratio) | `1.8499999999999999` (16 digits) | 6 | `console-sixty-four-track.toml` | **re-author to `1.85`**, §5.6 |

### 5.4 F1 — the true-peak ceiling, and why the per-row override must stay

These values are not authored by hand. `scripts/derive-intended-console-fixture.py:136-138`
generates them: `ceiling[i] = -0.5 - i/32`, with the comment *"Exact binary fractions: 1/32 and
5/4"*. They are deliberately dyadic, they are exactly representable in `f32`, and they are the
premise of the controlled 64-track pair.

`k = 5` is unavailable to the dB unit globally: `trim_db`/`fader_db` declare `-144.0`, and `k = 5`'s
distinctness ceiling is `|v| < 128`. But the **`ceiling` row's own range is `[-24, 0]`**, whose
`k_max` is 5 with a 5.24x margin. So the per-row override is *lawful for this row and unlawful for
the unit* — which is precisely the argument that L1 is a **per-row** law and the §1.2 table is a
table of *defaults*, not of unit constants. The descriptor keeps a per-row precision word for
exactly this reason. (Every alternative — re-deriving the fixture on a coarser grid — moves the
`f32` and therefore moves rendered digests of two 64-track fixtures. Rejected.)

### 5.5 Digest impact statement

| digest class | impact |
|---|---|
| **render digests / native-PCM output digests** | **ZERO**, under W-A + F1's override + L4's escape + F3's disposition. Every persisted value's `f32` is unchanged: the on-grid ones because L5 guarantees the shortest spelling round-trips to the same word; F1's because the override admits them verbatim; F2's because L4 admits it verbatim; F3's because `f32("1.8499999999999999") == f32("1.85") == 0x3feccccd` (verified: the two `f64`
neighbours narrow to one `f32`). |
| **graph plan digest, sealed session-fixture byte hashes** | **ZERO** for 13 of 14 fixtures. `console-sixty-four-track.toml` moves under F3 (§5.6). |
| **effect-descriptor identities** | move for exactly **two** descriptors: `miso.true-peak-limiter` (gains a non-zero window for `ceiling`) and `miso.delay` (**loses** its window — `damping` returns to canonical zeros, restoring its pre-#242 identity). The other six shipped descriptors, and `comprehensive-{a,b,c}`, encode zeros before and after. Re-pin: `fixtures/effect-descriptor/v1/MANIFEST.sha256` (9 rows). |
| **`#108` `migration_two_step_bank_restore` envelope digest** `5f23e630…` | **UNMOVED.** `miso-engine-bench`'s three interchange rows are `Linear`/`Continuous`/`Linear`; their new class default is `precision 4` and they declare exactly that, so they encode zeros before and after. The five pins stay as they are. |
| **`fixtures/effect-state/v1`** state vectors | descriptors there declare class defaults -> zeros -> **unmoved**. The two restored byte-equality seals (`state_vectors.rs:283`, `descriptor_v1_qualification.rs:~578`) keep holding. |
| **`fixtures/effect-interchange/v1/ACCEPTED.sha256`** `e3896726…` | **must move.** `scripts/effect-descriptor-v1-reference.py` is a sealed row of that manifest and still requires offsets 72/76 to be zero — already latently diverged (#242 derivation §10, OPEN). This design's wire change is the natural moment to discharge that ceremony: re-run `run-effect-interchange-reference-processes.sh` and re-pin the identity in `check-effect-interchange-qualification.sh`, `preflight-effect-interchange-benchmark.sh` and `run-effect-interchange-benchmark.sh` **together**. |
| **catalog / metadata assets** | regenerate `sdk/assets/*.json`, `sdk/src/generated/catalog.ts`, `scripts/fixtures/parameter-metadata-v1-self-test.json`. `check-sdk-generated.sh` re-derives both arrows byte-for-byte. |
| **ABI version word `131072`** | unchanged — no ABI struct moves. |

### 5.6 F3 disposition

The six `1.8499999999999999` occurrences are an authoring artifact, not writer output. Two facts
establish it (both re-verified after a first draft got the token distribution wrong):

- **The token distribution is split across the fixture family, not mixed within one file.**
  `console-sixty-four-track.toml` carries `1.8499999999999999` x6 and `1.85` **x0**;
  `console-sixty-four-track-intended.toml` and `console-sixty-four-track-mono.toml` each carry
  `1.85` x6 and `1.8499999999999999` x0. The two derived fixtures are produced by
  `scripts/derive-*-console-fixture.py` piped through the session validator's canonical writer,
  and that writer emits `1.85`. So the clean spelling is exactly where a canonical writer touched
  the document, and the long spelling is exactly where one did not — which is the provenance
  argument, and it is stronger than the "both spellings in one file" claim it replaces.
- **The long token is an `f64` artifact, not an `f32` one.** `1.8499999999999999` is the shortest
  `Display` of the `f64` one ulp **below** `f64(1.85)` — i.e. of `nextafter(1.85, -inf)`, not of
  `f64(1.85)` itself and not of `f32(1.85)` (whose exact `f64` value is `1.850000023841858`).
  That is the signature of a value that made a round trip through a decimal-emitting `f64`
  toolchain outside the engine.

Both `f64` neighbours narrow to the same `f32`, `0x3feccccd`. Replace the six tokens with `1.85`;
`f32` is bit-identical, so **no render digest moves**; only
`console-sixty-four-track.toml`'s own bytes, and hence any hash of that file, move. That fixture is
consumed by the console-benchmark scripts by *path*, not by content hash, in the seven
`scripts/*console*` files checked — but this needs the implementer to re-confirm before landing.

### 5.7 The SDK's `decimalToFloat32` mirror — its fate

Deleting Rust's `decimal_to_f32` (§4.1, zero production call sites) raises the obvious question
about its TypeScript mirror, `sdk/src/core/agent.ts:276`. **The mirror survives**, because unlike
its Rust counterpart it has a real production job — and the two now do different work:

- **Persisted values** get a new integer-domain mirror, `baseUnitsToFloat32(n, k)`, implementing
  §4.3 in `BigInt` arithmetic and `Float32Array`/`Uint32Array` bit packing. This is what the SDK
  uses whenever it must predict the word preparation will compute.
- **Live/command-plane values** keep `decimalToFloat32`, whose body (`Math.fround(Number(text))`)
  is *correct* for that job: a live target is a performance value, not a lattice point, and A7/D1
  keeps that plane continuous. It fills the 48-byte command record and nothing else.

Two documentation corrections are mandatory, because the current comment is now false:

1. `agent.ts:266-267` claims *"Its Rust counterpart is `miso_engine_effect_contract::decimal_to_f32`,
   and the two carry the same precondition"*. The counterpart is being deleted and the precondition
   is no longer shared — the live path has no lattice precondition at all. Rewrite.
2. The MXCSR hazard that motivates §4 **never existed on the TypeScript side**: ECMAScript
   specifies `Number()` and `Math.fround` as round-to-nearest-ties-to-even with no host rounding
   mode exposed. Say so explicitly, so a future reader does not "fix" the mirror by mimicking the
   Rust integer algorithm on the live path, where it would be wrong.

The existing eval `assert.equal(String(decimalToFloat32("0.30")), "0.30000001192092896")`
(`sdk/test/agent-evals.mjs:157-158`) is preserved unchanged: it pins the live plane's continuity,
which this design does not touch.

---

## 6. The SDK's perceptual layer

### 6.1 What moves

The SDK gains `sdk/src/core/perceptual.ts`. The lattice module keeps only integer/decimal
arithmetic; nothing in `core/lattice.ts` knows what a cent is.

```ts
export type Gesture =
  | { readonly kind: "units"; readonly count: number }      // linear rows
  | { readonly kind: "cents"; readonly cents: number }       // Hz rows
  | { readonly kind: "ratio"; readonly ratio: string };      // log ms / ratio rows

/** Move `rank` by `count` gestures. Always moves at least one base unit per gesture. */
export function stepByGesture(
  view: LatticeView, rank: number, gesture: Gesture, count: number,
): number;
```

### 6.2 Algorithm, one gesture, direction `s = sign(count)`

```
n      = view.unitsOfRank(rank)                 // exact integer
v      = n / 10^k                               // exact rational; evaluated in f64 for the target
target = gesture.kind === "units" ? v + s * gesture.count / 10^k
       : gesture.kind === "cents" ? v * 2 ** (s * gesture.cents / 1200)
       :                            v * (s > 0 ? r : 1 / r)          // r = Number(gesture.ratio)
m      = roundTowardTravel(target * 10^k, s)    // §6.3
if (m === n) m = n + s                          // §6.4 — the no-stall guarantee
return view.rankOfUnits(clamp(m, view.nMin, view.nMax))
```

Repeat `|count|` times, or apply once with the gesture pre-multiplied for the multiplicative
kinds (`2 ** (count * cents / 1200)`) — the two differ by accumulated rounding; **the design
specifies iterate-per-gesture**, so that `step(xs, 5)` and five `step(xs, 1)` land on the same rank.

### 6.3 Tie-breaking

`roundTowardTravel(x, s)` = round-half-**toward the direction of travel**:
`s > 0 ? Math.floor(x + 0.5) : Math.ceil(x - 0.5)`. Rationale: it is direction-symmetric under
negation, it never rounds a gesture backwards, and it makes the "one gesture up then one gesture
down" asymmetry (§6.5) as small as it can be.

### 6.4 The no-stall guarantee

> **L6.** `stepByGesture(view, rank, g, ±1)` moves the rank by at least 1, unless `rank` is already
> the endpoint in that direction.

Proved by the explicit `if (m === n) m = n + s` bump. The bump is reachable in practice: a 1-cent
gesture at the bottom of a Hz row is `10 x (2^(1/1200) - 1) = 0.00578 Hz = 0.58 base units`, which
rounds to 1 — but a 1-cent gesture at 10.00 Hz with a *coarser* future `k` would round to 0. The
bump makes the guarantee independent of the numbers.

### 6.5 What is explicitly NOT guaranteed

`stepByGesture(·, g, +1)` followed by `stepByGesture(·, g, -1)` **need not return to the starting
rank**, because both directions snap. A client that needs an exact undo must retain the anchor
rank, not replay the inverse gesture. This is stated as a documented property with an eval
(§8.7), not left to be discovered.

### 6.6 The engine's own step verb

`resolve_parameter_step` (`step.rs:695-710`) and `ParameterHandle.step()` become thin wrappers:
`step(size, count)` = `stepByGesture(view, rank, recommendedSteps.gesture scaled by
multiples[size], count)`, clamped. The `setSteps` refuse-vs-clamp asymmetry
(`sdk/src/core/agent.ts:177,196-201`) is preserved unchanged: an index is not a gesture.

---

## 7. Rulings superseded vs. preserved

| ruling | text (abridged) | status | why |
|---|---|---|---|
| **#239 A7** (comment 5452209916) — *"min/max/step per parameter; the lattice governs state, continuity governs performance"* | every controllable parameter's contract is `min`/`max`/`step`, decimal-defined; documents enforced in exact decimal on TOML text; f32 conversion happens exactly once | **PRESERVED, strengthened** | the contract stays `min`/`max`/resolution; exact-decimal document enforcement becomes *total* (zero FP anywhere in parsing, not merely one blessed parse); the single conversion site becomes integer-domain and mode-independent |
| **#239 A7 / D1** — *"Live commands stay continuous: transient performance (D1 — never persisted) and declick ramps glide through all intermediate values regardless"* | | **PRESERVED VERBATIM** | nothing in this design touches `miso.command.v1`, the 48-byte command record, `live.rs`, or smoothing. The lattice governs persisted state only. `base_units_to_f32` is never called on the live path |
| **#239 A7, ladder clause** — *"The ladder (#127's xs-xl) is retained, re-based: sizes `xs/sm/md/lg/xl` are per-parameter integer multiples of step"* | | **PARTIALLY SUPERSEDED** | the ladder is retained, but it can no longer be integer multiples of the *base unit* (a 1 Hz nudge is 100 base units; the 5-bit wire fields cannot hold it, §2.6). It becomes integer multiples of a **gesture** (§2.6's `RecommendedSteps`). A7's own rationale is what licenses this: it re-based the ladder onto step precisely so that *"one source of truth; the ladder can never drift off-lattice; UX retuning never touches the schema."* Gesture-typed multiples keep all three properties and strengthen the third — UX retuning now cannot touch the *wire* either |
| **#239 A7, refusal clause** — an off-lattice persisted value is refused with a typed error naming the two nearest legal values | | **PRESERVED VERBATIM** | `NearestLatticeValues { lower, upper }` survives unchanged in shape; only its computation changes, from a binary search over materialized points to `n_lo = floor`/`n_hi = ceil` arithmetic (and, on the four escape rows, a comparison against the escape). Eval 8.1's refusal legs and the existing validator mutations carry over unmodified |
| **#239 A7, canonical-writer clause** — the engine converts a lattice point to `f32` exactly once; the writer is not a descriptor-aware re-renderer | | **PRESERVED — and it corroborates W-A** | A7 already located the single conversion at preparation, not at serialization. That is the same boundary W-A (§3.2) keeps: the writer stays descriptor-unaware and the lattice is enforced where interpretation happens. W-A is therefore the option *consistent with A7 as written*, and W-B would be the departure |
| **#239 A12** — *"Steps are perceptually natural, never equal-unit over log domains. Equal-unit stepping of log parameters is rejected on the record"* | #127's JND research; LV2 equal-ratio precedent; v1's `1005.79 Hz` field lesson | **SUPERSEDED — split in two** | its *storage* half is overturned: the persisted grid becomes equal-unit in the native unit, because the storage grid's job is exactness, not perception. Its *gesture* half is **retained and relocated** to §6: a frequency dial still steps in cents, a Q dial still steps by ratio. #127's research is not overruled; it is re-homed where a JND belongs — in the client's gesture, not in the document's alphabet |
| **#239 5461507633 B1/B2/B3** — declared bounds and the declared default are lattice members by declaration; endpoint/default detents may make one irregular adjacency | | **B2/B3 PRESERVED and mostly made vacuous** | bounds are members by construction (they are `n_min`/`n_max`); the default is a member by construction for 74 of 78 rows. The "irregular adjacency" licence survives for exactly the four Q rows under L4 |
| **#239 5461507633 B2** — the rate-keyed ceiling is a clamp, not a declared bound | | **PRESERVED** | restated as the exact-integer floor law, §2.8 |
| **#239 5461507633 B4** — `pan` is persisted intent and owns a descriptor row | | **PRESERVED, untouched** | stable ID 12, per-lane, live, `Linear`, `[-1, 1]`; only its `k` changes 2 -> 4 |
| **#239 5462028562-A** — a derivation may live in a linked derivations doc naming the commit | | **PRESERVED, used** | this issue's derivation lands in `docs/derivations/` |
| **#239 5462028562-B** — the lattice is enforced at validation/preparation; the canonical writer preserves accepted spellings byte-stably and performs no descriptor-aware re-rendering; `0.3`/`0.30` are render-inert | | **PRESERVED, with a correction filed** | option W-A keeps the writer untouched and adds the lattice-closure proof. The correction: the shipped session writer does not in fact preserve float spellings (`model.rs:324` stores `f32`); the render-inertness half holds, the byte-preservation half does not. **Ruled Q3: correcting amendment to B, adopt W-A** |
| **#239 5462139867 finding 7** — a point's index is its RANK, superseding S3's "k relative to min" | | **PRESERVED as the definition; the two spellings RECONVERGE** | under a linear integer grid, rank and `n - n_min` are the same number for 74 of 78 rows (L3), and differ by exactly 1 above the escape on the other four |
| **#242 S3 finding 8 / derivation §8** — *"the blessed conversion is not rounding-mode independent — OPEN"* | `str::parse::<f32>()`'s hardware fast path lands 1 ulp off under round-toward-zero | **DISCHARGED — this design is candidate fix 1** | §4. The hazard is not contained, it is removed: no FP instruction remains on the path |
| **#242 derivation §9** — descriptor validation is declaration-only, not point generation | 1290 allocations/prepare -> 15 | **PRESERVED and made trivial** | membership is O(len(text)); there is nothing to generate |
| **#242 derivation §10** — the sealed `effect-descriptor-v1-reference.py` is latently diverged, OPEN | | **DISCHARGED as part of this issue's ceremony** | §5.5 |
| **#242 derivation §11** — the 2 320-value fixture migration ledger, NOT APPLIED, ruling required | | **RESOLVED WITHOUT A RULING for 2 320 of 2 320** | §5.3; three residual findings F1/F2/F3 with dispositions |
| **#127** — named step sizes research (xs..xl ladder, JND anchoring) | | **PRESERVED, re-homed** | the ladder survives as `RecommendedSteps` in the catalog (§2.6); the JND anchoring survives as the SDK's gesture units (§6) |

---

## 8. Built-in acceptance evals for the implementing brief

Each is stated so it can be proven red.

**8.1 Round-values eval.** The named values are all legal, at the named rows, with the named
integer counts:

| value | row | `k` | integer |
|---|---|---:|---:|
| `1000` Hz | `miso.multiband-compressor` `crossover` | 2 | 100 000 |
| `120` Hz | `miso.parametric-eq` `band-1-frequency` | 2 | 12 000 |
| `20000` Hz | `miso.parametric-eq` `band-1-frequency` (its declared max) | 2 | 2 000 000 |
| `0.25` | `miso.delay` `damping` (its declared default) | 4 | 2 500 |
| `1.5:1` | `miso.compressor` `ratio` | 4 | 15 000 |
| `0.5` | `miso.parametric-eq` `band-1-q` | 4 | 5 000 |
Red mutation: set Hz `k` back to 3 -> `20000` still passes but `47998.86` at 96 kHz collapses;
set ratio `k` to 1 -> `1.5` passes, `0.25`/`0.5` on ratio rows fail. Better mutation: revert
`default_precision` for `Ratio` to the old `ratio 1.02 p8` and assert `1.5` is refused.

**8.2 Exactness eval — parse has zero FP ops.** Three legs, all required:
- *(a) source policy.* A new `scripts/test-lattice-fp-policy.sh` in the shape of the existing
  `scripts/test-math-policy.sh` / `test-effect-runtime-policy.sh` mutation-testing scripts: apply
  each mutation to a scratch copy, assert the policy script rejects it, restore. Forbidden in the
  parsing/conversion module: the tokens `f32`/`f64` as arithmetic operands, `parse::<f32>`,
  `parse::<f64>`, `as f32`/`as f64` on a numeric expression, and any `core::fmt` float formatting
  other than the shortest-`Display` bound-recovery helper. Proven red by five mutations.
- *(b) disassembly.* Optional but cheap: `objdump` `base_units_to_f32` and assert the only
  SSE opcodes present are `movd`/`movss` moves — no `add/sub/mul/div/cvt` in the `ss`/`sd`/`ps`/`pd`
  families. Proven red by re-introducing one `f64` division.
- *(c) behavioural.* §8.3.

**8.3 Mode-independence eval under forced MXCSR.** Extend
`crates/miso-engine-host-core/tests/fp_environment.rs`'s existing arms (FTZ + DAZ +
round-toward-zero). For **every legal integer of every shipped row** — 57 683 390 conversions —
assert `base_units_to_f32(n, k)` returns bit-identical words under the default word and under each
of the four rounding modes x {FTZ, DAZ} combinations. Release profile; a few seconds. Red mutation:
replace the integer core with `(n as f64 / 10f64.powi(k)) as f32` -> red under RZ *and* red on the
double-rounding cases even under RN.

**8.4 Rank-law evals.**
- `rank(n) == n - n_min` for every legal `n` of every row **with no escape** (74 rows,
  exhaustive).
- For the four Q rows, `rank` is the §2.3 piecewise formula, and `rank(default) == 7071 - 1000 + 1`.
- `rank(min) == 0` and `rank(max) == count - 1` for all 78 rows.
- `stepsTo` is rank subtraction and `applying that many xs steps` reproduces the target (the
  existing SDK eval, restated).
- `DISABLED_LATTICE_INDEX` is outside every row's rank range (L2), for all four launch rates.
Red mutation: reintroduce a geometric interior for `Logarithmic` rows -> the identity fails at every
row after the first point.

**8.5 Bound-recovery eval (L5).** For every declared min/max/default of every row,
`scaled(shortest_display(bound), k)` is an integer, and `base_units_to_f32(that integer, k)`
reproduces the declared `f32` **bit for bit**. Red mutation: bump `Milliseconds` to `k = 5` ->
`attack`'s minimum `f32(0.1)` still has shortest spelling `0.1` and still passes; bump it to
`k = 1` -> `miso.true-peak-limiter` `release`'s authored `61.25` fails. Better: mutate
`shortest_display` to `format!("{:.*}", k, ...)` -> `0.995` at `k = 2` renders `1.00` and the
`damping` row goes red, which is #242's original launch case restated.

**8.6 Writer lattice-closure eval (W-A's proof obligation).** For every legal `n` of every row,
`scaled(shortest_display(base_units_to_f32(n, k)), k) == n`. Sampled at design time over 964 495
values across the six worst-margin rows: zero mismatches. The implementer runs it exhaustively.
Red mutation: set `linear` `k` to 6 -> still passes (6 <= `k_max`); set `Hz` `k` to 3 -> fails above
16384 Hz, which is L1 caught from the other side.

**8.7 SDK perceptual-step evals.**
- **no stall:** for every row and every gesture in `recommendedSteps`, one gesture from every rank
  in a 4096-rank sample moves the rank by `>= 1` unless clamped (L6).
- **monotone:** `stepByGesture(r, g, +1) > r` and `stepByGesture(r, g, -1) < r`, except at
  endpoints.
- **composition:** `stepByGesture(r, g, n)` equals `n` applications of `stepByGesture(·, g, 1)`.
- **cents fidelity:** for the Hz rows, the ratio between the value at `r` and at
  `stepByGesture(r, cents(20), 1)` is within `±1` base unit of `2^(20/1200)`, at every rank.
  This is #127's `1005.79 Hz` lesson, restated as a *client* gate.
- **asymmetry is documented, not accidental:** an explicit test asserting that up-then-down does
  **not** in general return to the start, at a named rank, so a future "fix" that silently changes
  the tie-break is caught (§6.5).
- **oracle parity:** the redesigned `lattice_oracle` digest (declaration + 1024 sampled ranks)
  reproduces between Rust and TS for all 78 rows, replacing the every-point digest.
- Red mutation: delete the `if (m === n) m = n + s` bump -> the no-stall eval goes red at the
  bottom of the Hz rows.

**8.8 Gate self-tests.** `check-parameter-metadata-v1.py --self-test` mutations rewritten for the
new `step` object: precision non-integer, precision 9, precision missing, `recommendedSteps`
gesture outside vocabulary, multiples non-ascending, multiple zero, retired `unit`/`size` keys
resurfacing. Each proven red. The `check-step-vocabulary.py` allow-list rows re-verified (a stale
row is itself a red).

**8.9 L4 validation evals (ruling Q1a).** The escape mechanism is the one hole this design
opens, so its validator gets three proven-red mutations of its own:
- **an off-grid declared BOUND is refused.** L4 permits an escape only for the declared *default*;
  a `minimum` or `maximum` whose shortest spelling exceeds `k` is a `Declaration` error, because
  bounds define `n_min`/`n_max` and an off-grid bound would make the rank law ill-posed. Red
  mutation: relax the check to accept any intrinsic -> a hand-built row with `maximum = 0.99999`
  at `k = 4` is accepted and its `rank(max) == count - 1` eval goes red one test over.
- **an escape whose `f32` collides with a flank is refused.** Red mutation: declare a default of
  `0.70710000000001` on a `k = 4` ratio row — off-grid, but `f32`-identical to the grid point
  `0.7071` — and assert refusal. Without the distinctness clause it binds, and two ranks then
  render the same word (attack A4, as a gate).
- **a second escape is refused.** Red mutation: hand-build a row with both an off-grid default and
  an off-grid enum choice; assert `Declaration`. This pins "at most one" so the mechanism cannot
  grow back into the general intrinsic set.
- **the shipped case is positively pinned:** `band-{1,2,3,4}-q` each carry exactly one escape,
  `0.70710677`, at rank `7071 - 1000 + 1 = 6072`, whose `f32` is `0x3f3504f3` — 114 ulps above
  `f32("0.7071")` and 1564 ulps below `f32("0.7072")`.

**8.10 Automation segment endpoint eval.** #242 S2 makes automation segment values persisted
state, so `start_value` and `end_value` are lattice-validated like any stored value while segment
*interpolation* stays continuous (A7/D1). Assert: every segment endpoint in every session fixture
is a lattice member of its target row; an off-grid endpoint is refused naming the two nearest legal
values; and an interpolated *interior* sample is **not** validated — a segment from rank 0 to rank 3
passes through values that are not lattice points, by design. Red mutation: lattice-validate the
interpolator -> the automation fixtures go red, which is #242 eval 5 restated for the integer grid.
(The pending #178 builtin-automation arm inherits this rule when it lands.)

**8.11 Full session-fixture corpus revalidation — a STANDING gate.** The §5.3 sweep must not be a
one-off design artifact; it becomes a checked-in gate that re-runs on every commit, so that a new
fixture cannot be authored off-grid and a `k` cannot be changed without the corpus objecting.

*Scope must be pinned in the gate itself*, because it is the one thing that can silently rot. The
verified populations, with their scoping:

| population | count | scope |
|---|---:|---|
| descriptor-backed effect+builtin rows (the §5.3 sweep) | **7 128** | `params[].value` joined to a descriptor row, plus the 11 builtin lattice keys |
| + the two `pan` gain keys (`pan.left`, `pan.right`) | **7 590** | adds 462 values |
| the verifier's standing-gate population | **8 283** | the broader sweep; the 693-value difference is a *scoping* difference over which keys count as lattice-bearing |

The gate must therefore assert its own population count as a pinned integer alongside the pass
count, and the brief must settle the 693-value scope question explicitly rather than leaving two
defensible numbers in circulation. Expected steady state under this design: **all values legal**,
with F1/F2 admitted by the override and the escape and F3 re-authored. Red mutation: revert the dB
default to `k = 1` -> 90 `fader_db` values go red; revert Hz to the geometric grid -> 2 320 go red.

**8.12 Signed-zero fold.** The integer model has no `-0`: `ExactDecimal::parse` already folds
`-0` to `0` (`step.rs:815-818`), and `n = 0` converts to `+0.0` by construction (§4.3 step 2). The
session writer, however, preserves `-0.0` exactly today (`docs/SESSION_SCHEMA_V1.md:22`). These do
not conflict under W-A — the writer is descriptor-unaware and never re-renders from a rank — but
the asymmetry must be *recorded in the derivation* rather than discovered later. **Verified: no
shipped session fixture carries a `-0.0` token** (zero matches across all 14 files), so nothing
moves. Eval: a hand-built document with `value = -0.0` on a lattice row is accepted, matches rank
`n = 0`, and prepares `+0.0`; and the writer's own `-0.0` round-trip test is left untouched.

**8.13 Wire evals.** The three enforcement legs of §5.2 each proven red by hand-built windows:
explicit-derived alias at 72, non-zero at 76, semantic mismatch on bind. The two restored
byte-equality seals still hold. `miso.delay`'s identity is asserted to equal its **pre-#242** value
(a positive, checkable consequence).

---

## 9. Sequencing and blast radius

**Order.** Lands **after #246 merges.** #246 owns the app-side adoption of the vendored SDK; the
SDK is vendored as source (`sdk/README.md`: *"There is no `dist/`, and there is not going to be
one"*), `sdk/package.json` is `0.0.0`/`private`, and the pin is by engine commit + `PROVENANCE`.
Landing this before #246 would hand the app a `LatticeView` API where it expects
`readonly LatticePoint[]`, with no semver to express the break. After #246, this is one vendor
re-pin with a named engine commit.

**Also gated on:** the effect-interchange re-seal ceremony (§5.5) must be authorized in the same
brief, because `scripts/effect-descriptor-v1-reference.py` is sealed and this design changes the
format it independently models. That is the discharge of #242 derivation §10's OPEN item, not a
side effect.

**Blast radius.**

| area | files | notes |
|---|---:|---|
| effect-contract | ~4 (`step.rs`, `lib.rs`, 2 test files) | `step.rs` shrinks by roughly 200-250 lines net |
| effect-package (wire + FFI + header) | ~5 | the packing, the C macros, 2 qualification test files |
| builtins + builtins-compiler | ~4 | 12 declarations + the lattice adapter + the clamp floor |
| shipped effect crates | 8 | one `lattice:` literal per row; `miso-engine-delay` loses `with_lattice` |
| non-shipped descriptor literals | 4 | `conformance`, `graph`, `bench/effect_interchange.rs` (x3 rows), `fuzz/effect_state.rs` |
| effect-compiler tests | ~2 | `parameter_lattice.rs` is the catalog sweep; several pins move |
| metadata tool + oracle | 3 | `lib.rs::step_object`, `abi_layout.rs` untouched, `lattice_oracle.rs` redesigned |
| SDK | ~10 | `lattice.ts`, `agent.ts` (incl. the `decimalToFloat32` re-scope, §5.7), new `perceptual.ts`, new `baseUnits.ts`, `decimal.ts` (trimmed), `index.ts`, **`codegen/generate.mjs`**, 2 generated files, `barrel-surface.ts` |
| SDK vendoring surface | 2 | **`sdk/README.md`** (engine-commit pin), `sdk/PROVENANCE.*` (`schemas.catalog` -> `.v2`) |
| SDK tests | 4 | `agent-evals.mjs`, `package-evals.mjs`, new `perceptual-evals.mjs`, new `base-units-evals.mjs` |
| scripts / gates | ~4 | `check-parameter-metadata-v1.py`, the new FP-policy script + its sweep row, 3 interchange re-pin scripts |
| fixtures | 2 files edited | `console-sixty-four-track.toml` (F3, 6 tokens); `fixtures/effect-descriptor/v1/MANIFEST.sha256` + `ACCEPTED.sha256` re-pinned |
| docs | 4 | `EFFECT_CONTRACT_V1.md`, `EFFECT_DESCRIPTOR_WIRE_V1.md`, new derivation, finding 7 written into `docs/` |
| **crates touched** | **~11** | contract, package, builtins, builtins-compiler, effect-compiler, delay, parametric-eq, true-peak-limiter, + 5 other effect crates for one literal each, conformance, graph |
| **`sweep.sh` rows affected** | ~13 | 117, 119, 120, 122, 128, 171, 175, 183, 186, plus the 4 interchange rows; 1 row added |

**Not touched:** `miso-engine-session` (under W-A), `miso-engine-protocol`'s live command record,
`live.rs`, smoothing/declick, the ABI version word, `abi-layout.v1`, the graph compiler, any
kernel.

---


---

*Sections 10–12 (the twelve self-adversarial attacks with outcomes, the ruled decisions table, and per-section confidence) are in the first comment below — GitHub body-size limit.*
