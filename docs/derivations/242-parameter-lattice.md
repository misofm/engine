# Derivations — #242 parameter lattice

Every re-pin in this change, with the arithmetic that discharges it, per the ceremony amended by
[#239 ruling 5462028562 A](https://github.com/misofm/engine-v2/issues/239#issuecomment-5462028562):
a derivation may live in a linked derivations document naming the commit it discharges.

## 1. `scripts/fixtures/parameter-metadata-v1-self-test.json` — regenerated

The fixture is byte-for-byte the generator's `--print` output; it was verified to differ from the
previous revision in exactly two ways before being replaced:

- every parameter's `"nudge": null` slot became a populated `"step"` object (77 slots -> 78 rows:
  66 effect parameters + 12 builtin parameters);
- one appended builtin row, `pan` (stable ID 12), per #239 ruling 5461507633 B4.

`11 builtin rows + 1 appended pan row = 12`; `66 effect parameter rows` is unchanged. No other
key, value or ordering moved: the comparison was made with both slots stripped and the pan row
removed, and the two documents were then identical.

## 2. `scripts/check-parameter-metadata-v1.py` — live builtin set

Before: 8 names. `pan` is declared `BuiltinParameterUpdateRate::BlockTarget`, and the gate derives
`liveUpdatable` from exactly that, so the row is live by the same sentence as the eight already
there. `8 + 1 = 9`. `hpf_hz`, `lpf_hz` and `delay_samples` remain `PreparedOnly` and remain absent.

## 3. `scripts/sweep.sh` — row count

Before: 93 explicit rows. One row added, `scripts/check-step-vocabulary.py --self-test`, the
rename-completeness gate required by #242 eval 7. `93 + 1 = 94`, and `grep -c '^row '` reports 94.

## 4. `miso.delay` `damping` — per-parameter lattice override

The row's declared maximum is `0.995`. The linear/`Linear` unit class pins two decimals, and
`format!("{:.2}", 0.995_f32 as f64)` is `1.00` — a rendering outside the row's own domain, and a
value that does not convert back to the declared `f32`. The row therefore overrides the class
default to `arithmetic(0.01, 3)`: the step is unchanged at `0.01`, the interior runs
`0.010, 0.020, ... 0.990`, and `0.995` is the intrinsic top detent — one irregular adjacency,
which is the hardware-detent precedent #239 ruling 5461507633 B2 cites. `0.995` now renders
`"0.995"` and converts back bit-for-bit.

## 5. `disabledOrRateKeyedHertz` — the rate ceiling is a clamp, not a member

#239 ruling 5461507633 B2 makes both DECLARED bounds lattice members. A rate-keyed cutoff has no
declared maximum: `BuiltinParameterDomain::DisabledOrRateKeyedHertz { disabled, minimum_hz }`
carries only a minimum, and the ceiling comes from `builtin_filter_cutoff_maximum_hz(rate)`. S1's
original sentence for this one shape therefore stands: the top of the lattice is the greatest
generated point at or below that rate's clamp. At 48 kHz the clamp is `23999.43359375` and the top
lattice point is `23798.694` (`10 * 2^(674*20/1200)`); at 44.1 kHz the clamp is `22049.482421875`.

## 6. Enumeration lattices are spelled in choice values

An enumeration's lattice is an index lattice — the step is one over the ordinals, and the ordinal
is what the persist plane carries. The DOCUMENT spells the choice value. The parametric EQ's
`kind` row has six choices valued `1.0 .. 6.0`, so rendering ordinals `0 .. 5` as the canonical
decimals would have refused `notch` (`6.0`) outright and matched every other spelling to the wrong
choice. Canonical renderings are now the choice values; `LatticePoint::index` remains the ordinal.

## 7. Descriptor-wire encoding is canonical, and no sealed identity moved

The decoder reads an all-zero window at parameter offsets 72/76 as
`default_parameter_lattice(unit, domain, mapping)`. An encoder that also wrote that same derived
lattice explicitly would give one lattice two byte spellings; in a format whose bytes are its
identity that is an aliasing bug. The encoder therefore emits the words only when the row's
declaration differs from its derived class default.

Consequence, and the reason it was found: `bench`'s three interchange descriptors are
`Linear`/`Continuous`/`Linear`, whose class default IS `arithmetic(0.01, 2)` -- exactly what they
declare. Writing that explicitly moved the `migration_two_step_bank_restore` envelope digest from
the value sealed by issue #108's authorized one-shot run,
`5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777`, to
`ce6060818a06a265e5e1637aa53d008b92a827605445fdece1f352182b44cc65`. That digest is pinned in five
places (the bench itself, `scripts/check-effect-interchange-benchmark-108.sh`,
`scripts/test-effect-interchange-benchmark-108-policy.sh`,
`scripts/effect-interchange-benchmark-108-validator.py`, and #108's own spec and brief), and
re-pinning it would have invalidated recorded one-shot benchmark evidence. Under the canonical
rule those descriptors encode zeros again and the sealed digest is restored exactly; nothing was
re-pinned.

The only shipped effect row whose descriptor bytes move under #242 is `miso.delay` `damping`,
because it is the only row that genuinely overrides its class default (section 4). Per #127's
recorded position, a descriptor that declares its own ladder changing bytes is correct and
intended.

**Coordinator ruling: the alias is rejected, not merely unused.** A canonical encoder is not
enough on its own -- the verifier accepted 8 of 8 hand-built explicit-default windows, so a
non-conforming or older encoder could still mint a second identity for a descriptor that means
exactly the same thing. `verify_effect_descriptor_wire` now refuses a non-zero window whose
unpacked lattice equals `default_parameter_lattice(unit, domain, mapping)`, typed `Reserved` at
offset 72 of the offending record. The historical zero spelling is the sole canonical spelling of
a derived lattice, so the encoder's rule and the format's rule are one sentence rather than a
convention the verifier declines to enforce.

Proven by `an_explicitly_spelled_derived_lattice_is_refused_as_a_second_spelling` in
`crates/effect-package/tests/descriptor_v1_qualification.rs`, which rebuilds the alias
by hand for every derived row of all three comprehensive descriptors and asserts the refusal names
the aliased window. Deleting the rule turns it red.

**The byte-equality seals are restored.** Checkpoint `9f2a8ec8` relaxed
`state_vectors.rs::independent_reference_vector_binds_verifies_and_reencodes_byte_identically`
from `assert_eq!(rust_wire, descriptor_fixture)` to a verify-only call, and
`descriptor_v1_qualification.rs::checked_vectors_match_independent_wire_identity_and_port_permutation`
from comparing the encoder's output against the sealed vector to comparing the sealed vector
against itself. Both relaxations were residue of the pre-canonical encoder, and while they stood
no effect-package gate could see an encoder that moved these bytes -- the regression surfaced only
in the #108 bench digest, three crates away. Restored, they kill that mutation directly: removing
the canonical-zeros rule fails `descriptor_v1_qualification.rs:578` and `state_vectors.rs:283`.

## 8. The blessed conversion is not rounding-mode independent — OPEN

Found while making section 4's rule hold under issue #146's floating-point arms. Descriptor
validation runs inside `prepare_host_session`, and `crates/host-core/tests/fp_environment.rs`
calls it with the caller's MXCSR set to flush-to-zero, denormals-are-zero AND **round-toward-zero**.
A first attempt at section 4 proved the rendering by converting it back with `decimal_to_f32` and
comparing words; under that caller's word the whole parametric-EQ descriptor became invalid.

The cause is not the check: `str::parse::<f32>()` has a hardware fast path that multiplies or
divides by a power of ten in `f64`, so under a non-default rounding mode it can land one unit in
the last place away from the correctly rounded result. `decimal_to_f32` is that parse, and it is
the site #242 calls the SINGLE conversion authority whose whole purpose is that there be no float
drift. Today that guarantee holds only in a default rounding environment.

Section 4's rule was restated in the decimal domain instead -- a rendering must equal the value's
shortest round-tripping spelling, both produced by the software float formatter -- so descriptor
validation is now environment-independent and the #146 arms pass. **The underlying hazard is not
fixed and is not #242's to rule on.** When S2 enforcement lands, preparation will convert
persisted text through `decimal_to_f32`, and a host whose caller word differs will then prepare
different words from the same document. Two candidate fixes, both outside this brief:

1. Convert in exact integer arithmetic. Every lattice rendering is `units / 10^precision` with at
   most eight fraction digits, so the correctly rounded `f32` is computable with `i128` and an
   explicit round-half-even, with no hardware float and no environment dependence at all.
2. Enter the canonical floating-point environment around preparation, the way
   `StartedRenderSession` already does around render (#146). This is the larger change and is
   #146's architecture to extend.

## 9. Descriptor validation is declaration-only, not point generation

The inherited tranche answered "is this lattice lawful?" by generating every point. Descriptor
validation runs on the preparation path, and soft clip's three rows alone hold `601 + 481 + 101 =
1183` points, each rendered to its own `String`: `crates/soft-clip/tests/allocation.rs`
measured **1290 allocations per prepare against its pinned bound of 32**. This is precisely the
defect #127 recorded from the v1 audit -- an un-memoized per-call derivation on a hot descriptor
path -- arriving in v2 by a different route.

Lawfulness is a property of the DECLARATION and its intrinsic points, not of the interior, so
`validate_parameter_lattice_parts` decides it in constant time with no allocation: the step, unit,
precision and ladder are checked directly, and each intrinsic point is proved spellable by
counting the fraction digits of its shortest round-trip spelling in a stack buffer. Point
generation remains available for the control plane, where a registry can cache it. Measured after
the change: **15 allocations per prepare**, inside the bound. Both the static-descriptor validator
and the descriptor-wire binder call the one shared implementation.

## 10. Two surfaces the inherited tranche did not reach

**`fuzz/fuzz_targets/effect_state.rs`** declares a `ParameterDescriptor` literal and lives outside
the workspace, so `cargo build --workspace` never compiled it and adding the `lattice` field broke
`scripts/check-effect-package-v1.sh`. Its row is linear/`Linear`, so it now states its class
default and its descriptor bytes are unchanged.

**`scripts/effect-descriptor-v1-reference.py` is SEALED and was left alone — OPEN.** It is a row of
`fixtures/effect-interchange/v1/ACCEPTED.sha256`, whose own identity
`e3896726979aa746cfda50fc10c1985c0ecef117f87b39e692f18226b7b4fa14` is pinned in three scripts. That
file still requires parameter offsets 72/76 to be zero, so as an independent model of the wire it
no longer describes the format the encoder emits: it would refuse the `miso.delay` descriptor. No
gate is red, because the reference builds and mutates its own corpus rather than reading the
encoder's output, so the divergence is latent.

Teaching it the packing was implemented and works -- the torn-declaration rule, the re-pack rule,
a decoder that round-trips the words, and a mutation matrix gaining three proven-red rows plus one
proven-ACCEPTED row -- and was then REVERTED, because editing a sealed reference is exactly what
the seal exists to prevent. Landing it is the effect-interchange re-seal ceremony: re-run
`scripts/run-effect-interchange-reference-processes.sh`, re-pin the manifest identity in
`check-effect-interchange-qualification.sh`, `preflight-effect-interchange-benchmark.sh` and
`run-effect-interchange-benchmark.sh` together (a partial re-seal is caught by
`scripts/test-effect-interchange-benchmark.sh`), and record the derivation. That is an authorized
ceremony, not a side effect of this brief.

## 11. Fixture migration ledger — NOT APPLIED, ruling required

Enforcing the lattice at validation/preparation refuses **2320 persisted values across 8 of the 14
shipped session fixtures**. These edits are deliberately NOT made: they would move render digests,
a sealed fixture byte hash, a graph plan digest and five native-PCM output digests, which the
brief's class-A rule forbids without a ruling. The ledger below is the complete migration, so the
decision can be taken on the arithmetic rather than on a sample.

### Occurrences per fixture

| fixture | off-lattice values |
|---|---:|
| `builtins-automation.toml` | 2 |
| `canonical.toml` | 2 |
| `console-sixty-four-track-intended.toml` | 803 |
| `console-sixty-four-track-mono.toml` | 804 |
| `console-sixty-four-track.toml` | 680 |
| `observation-frame-shape.toml` | 1 |
| `parametric-eq-bank-console.toml` | 8 |
| `parametric-eq-nine-track.toml` | 20 |
| **total** | **2320** |

The six fixtures not listed (`canonical-minimal.toml`, `toml-1.0-escapes.toml`,
`toml-1.0-invalid-duplicate-key.toml`, `compressor-dynamic-observation.toml`,
`compressor-bank-observation.toml`, `compressor-dynamic-bank-observation.toml`) are already
entirely on-lattice and need no edit.

### Every distinct (row, authored value) pair and its two nearest legal values


#### builtin `hpf_hz` — 8 distinct values, 340 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `30.0` | `29.966` | `30.314` | 32 |
| `35.0` | `34.822` | `35.227` | 44 |
| `45.0` | `44.898` | `45.420` | 44 |
| `50.0` | `49.818` | `50.397` | 44 |
| `55.0` | `54.642` | `55.277` | 44 |
| `60.0` | `59.932` | `60.629` | 44 |
| `65.0` | `64.980` | `65.735` | 44 |
| `70.0` | `69.644` | `70.453` | 44 |

#### builtin `lpf_hz` — 9 distinct values, 406 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `17250.0` | `17221.559` | `17421.663` | 20 |
| `17500.0` | `17421.663` | `17624.093` | 56 |
| `17750.0` | `17624.093` | `17828.876` | 58 |
| `18000.0` | `17828.876` | `18036.037` | 58 |
| `18250.0` | `18245.606` | `18457.609` | 58 |
| `18500.0` | `18457.609` | `18672.077` | 58 |
| `18750.0` | `18672.077` | `18889.036` | 36 |
| `19000.0` | `18889.036` | `19108.516` | 40 |
| `20000.0` | `19782.376` | `20012.236` | 22 |

#### builtin `fader_db` — 10 distinct values, 180 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `-2.75` | `-2.8` | `-2.7` | 12 |
| `-2.25` | `-2.3` | `-2.2` | 24 |
| `-1.75` | `-1.8` | `-1.7` | 24 |
| `-1.25` | `-1.3` | `-1.2` | 24 |
| `-0.75` | `-0.8` | `-0.7` | 21 |
| `-0.25` | `-0.3` | `-0.2` | 21 |
| `0.25` | `0.2` | `0.3` | 18 |
| `0.75` | `0.7` | `0.8` | 18 |
| `1.25` | `1.2` | `1.3` | 9 |
| `1.75` | `1.7` | `1.8` | 9 |

#### `miso.compressor` ratio — 9 distinct values, 192 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `1.5` | `1.48594740` | `1.51566634` | 24 |
| `2.25` | `2.20803966` | `2.25220046` | 21 |
| `3.0` | `2.97173067` | `3.03116529` | 21 |
| `3.75` | `3.69497357` | `3.76887304` | 21 |
| `4.5` | `4.41583546` | `4.50415216` | 21 |
| `5.25` | `5.17385504` | `5.27733214` | 21 |
| `6.0` | `5.94313313` | `6.06199579` | 21 |
| `6.75` | `6.69293318` | `6.82679184` | 21 |
| `7.5` | `7.38953904` | `7.53732982` | 21 |

#### `miso.compressor` attack — 11 distinct values, 192 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `2.0` | `1.989` | `2.029` | 18 |
| `3.5` | `3.463` | `3.532` | 18 |
| `5.0` | `4.946` | `5.045` | 18 |
| `6.5` | `6.398` | `6.526` | 18 |
| `8.0` | `7.955` | `8.114` | 18 |
| `9.5` | `9.321` | `9.507` | 18 |
| `11.0` | `10.920` | `11.139` | 18 |
| `12.5` | `12.298` | `12.544` | 18 |
| `14.0` | `13.850` | `14.127` | 18 |
| `15.5` | `15.291` | `15.597` | 15 |
| `17.0` | `16.883` | `17.220` | 15 |

#### `miso.compressor` release — 12 distinct values, 177 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `40.0` | `39.993` | `40.793` | 15 |
| `55.0` | `54.902` | `56.000` | 15 |
| `70.0` | `69.629` | `71.022` | 15 |
| `85.0` | `84.878` | `86.575` | 15 |
| `115.0` | `114.234` | `116.519` | 15 |
| `130.0` | `128.647` | `131.219` | 15 |
| `145.0` | `144.877` | `147.774` | 15 |
| `160.0` | `159.956` | `163.155` | 15 |
| `175.0` | `173.141` | `176.604` | 15 |
| `190.0` | `187.414` | `191.162` | 15 |
| `205.0` | `202.863` | `206.920` | 15 |
| `220.0` | `219.585` | `223.977` | 12 |

#### `miso.parametric-eq` band-1 frequency — 32 distinct values, 395 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `90.0` | `89.797` | `90.840` | 16 |
| `110.0` | `109.283` | `110.553` | 10 |
| `120.0` | `119.865` | `121.257` | 1 |
| `135.0` | `134.543` | `136.107` | 16 |
| `165.0` | `163.740` | `165.642` | 10 |
| `180.0` | `179.594` | `181.681` | 16 |
| `220.0` | `218.566` | `221.106` | 10 |
| `225.0` | `223.675` | `226.274` | 16 |
| `270.0` | `269.087` | `272.213` | 16 |
| `275.0` | `272.213` | `275.376` | 10 |
| `315.0` | `312.691` | `316.324` | 16 |
| `330.0` | `327.480` | `331.285` | 10 |
| `360.0` | `359.188` | `363.361` | 16 |
| `385.0` | `384.968` | `389.441` | 10 |
| `405.0` | `403.175` | `407.859` | 16 |
| `440.0` | `437.133` | `442.212` | 10 |
| `450.0` | `447.350` | `452.548` | 16 |
| `495.0` | `490.665` | `496.366` | 26 |
| `540.0` | `538.174` | `544.427` | 16 |
| `550.0` | `544.427` | `550.753` | 8 |
| `585.0` | `583.502` | `590.282` | 16 |
| `605.0` | `604.080` | `611.099` | 8 |
| `630.0` | `625.382` | `632.649` | 16 |
| `660.0` | `654.959` | `662.570` | 8 |
| `675.0` | `670.268` | `678.056` | 16 |
| `715.0` | `710.124` | `718.376` | 8 |
| `720.0` | `718.376` | `726.723` | 16 |
| `765.0` | `761.093` | `769.936` | 16 |
| `770.0` | `769.936` | `778.882` | 8 |
| `825.0` | `815.719` | `825.197` | 8 |
| `1000.0` | `992.733` | `1004.268` | 9 |
| `2400.0` | `2388.564` | `2416.318` | 1 |

#### `miso.parametric-eq` band-1 Q — 11 distinct values, 192 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `0.5` | `0.49729479` | `0.50724069` | 21 |
| `0.65` | `0.64330384` | `0.65616992` | 21 |
| `0.8` | `0.79986747` | `0.81586482` | 21 |
| `0.95` | `0.93717223` | `0.95591567` | 21 |
| `1.1` | `1.09804663` | `1.12000756` | 18 |
| `1.25` | `1.23657885` | `1.26131043` | 18 |
| `1.4` | `1.39258863` | `1.42044040` | 18 |
| `1.55` | `1.53753037` | `1.56828098` | 18 |
| `1.7` | `1.69755777` | `1.73150892` | 18 |
| `1.8499999999999999` | `1.83749112` | `1.87424094` | 6 |
| `1.85` | `1.83749112` | `1.87424094` | 12 |

#### `miso.true-peak-limiter` ceiling — 60 distinct values, 120 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `-2.46875` | `-2.5` | `-2.4` | 2 |
| `-2.4375` | `-2.5` | `-2.4` | 2 |
| `-2.40625` | `-2.5` | `-2.4` | 2 |
| `-2.375` | `-2.4` | `-2.3` | 2 |
| `-2.34375` | `-2.4` | `-2.3` | 2 |
| `-2.3125` | `-2.4` | `-2.3` | 2 |
| `-2.28125` | `-2.3` | `-2.2` | 2 |
| `-2.25` | `-2.3` | `-2.2` | 2 |
| `-2.21875` | `-2.3` | `-2.2` | 2 |
| `-2.1875` | `-2.2` | `-2.1` | 2 |
| `-2.15625` | `-2.2` | `-2.1` | 2 |
| `-2.125` | `-2.2` | `-2.1` | 2 |
| `-2.09375` | `-2.1` | `-2.0` | 2 |
| `-2.0625` | `-2.1` | `-2.0` | 2 |
| `-2.03125` | `-2.1` | `-2.0` | 2 |
| `-1.96875` | `-2.0` | `-1.9` | 2 |
| `-1.9375` | `-2.0` | `-1.9` | 2 |
| `-1.90625` | `-2.0` | `-1.9` | 2 |
| `-1.875` | `-1.9` | `-1.8` | 2 |
| `-1.84375` | `-1.9` | `-1.8` | 2 |
| `-1.8125` | `-1.9` | `-1.8` | 2 |
| `-1.78125` | `-1.8` | `-1.7` | 2 |
| `-1.75` | `-1.8` | `-1.7` | 2 |
| `-1.71875` | `-1.8` | `-1.7` | 2 |
| `-1.6875` | `-1.7` | `-1.6` | 2 |
| `-1.65625` | `-1.7` | `-1.6` | 2 |
| `-1.625` | `-1.7` | `-1.6` | 2 |
| `-1.59375` | `-1.6` | `-1.5` | 2 |
| `-1.5625` | `-1.6` | `-1.5` | 2 |
| `-1.53125` | `-1.6` | `-1.5` | 2 |
| `-1.46875` | `-1.5` | `-1.4` | 2 |
| `-1.4375` | `-1.5` | `-1.4` | 2 |
| `-1.40625` | `-1.5` | `-1.4` | 2 |
| `-1.375` | `-1.4` | `-1.3` | 2 |
| `-1.34375` | `-1.4` | `-1.3` | 2 |
| `-1.3125` | `-1.4` | `-1.3` | 2 |
| `-1.28125` | `-1.3` | `-1.2` | 2 |
| `-1.25` | `-1.3` | `-1.2` | 2 |
| `-1.21875` | `-1.3` | `-1.2` | 2 |
| `-1.1875` | `-1.2` | `-1.1` | 2 |
| `-1.15625` | `-1.2` | `-1.1` | 2 |
| `-1.125` | `-1.2` | `-1.1` | 2 |
| `-1.09375` | `-1.1` | `-1.0` | 2 |
| `-1.0625` | `-1.1` | `-1.0` | 2 |
| `-1.03125` | `-1.1` | `-1.0` | 2 |
| `-0.96875` | `-1.0` | `-0.9` | 2 |
| `-0.9375` | `-1.0` | `-0.9` | 2 |
| `-0.90625` | `-1.0` | `-0.9` | 2 |
| `-0.875` | `-0.9` | `-0.8` | 2 |
| `-0.84375` | `-0.9` | `-0.8` | 2 |
| `-0.8125` | `-0.9` | `-0.8` | 2 |
| `-0.78125` | `-0.8` | `-0.7` | 2 |
| `-0.75` | `-0.8` | `-0.7` | 2 |
| `-0.71875` | `-0.8` | `-0.7` | 2 |
| `-0.6875` | `-0.7` | `-0.6` | 2 |
| `-0.65625` | `-0.7` | `-0.6` | 2 |
| `-0.625` | `-0.7` | `-0.6` | 2 |
| `-0.59375` | `-0.6` | `-0.5` | 2 |
| `-0.5625` | `-0.6` | `-0.5` | 2 |
| `-0.53125` | `-0.6` | `-0.5` | 2 |

#### `miso.true-peak-limiter` release — 63 distinct values, 126 occurrences

| authored | nearest below | nearest above | occurrences |
|---:|---:|---:|---:|
| `60.0` | `59.431` | `60.620` | 2 |
| `61.25` | `60.620` | `61.832` | 2 |
| `62.5` | `61.832` | `63.069` | 2 |
| `63.75` | `63.069` | `64.330` | 2 |
| `65.0` | `64.330` | `65.617` | 2 |
| `66.25` | `65.617` | `66.929` | 2 |
| `67.5` | `66.929` | `68.268` | 2 |
| `68.75` | `68.268` | `69.633` | 2 |
| `70.0` | `69.633` | `71.026` | 2 |
| `71.25` | `71.026` | `72.446` | 2 |
| `72.5` | `72.446` | `73.895` | 2 |
| `73.75` | `72.446` | `73.895` | 2 |
| `75.0` | `73.895` | `75.373` | 2 |
| `76.25` | `75.373` | `76.881` | 2 |
| `77.5` | `76.881` | `78.418` | 2 |
| `78.75` | `78.418` | `79.987` | 2 |
| `80.0` | `79.987` | `81.586` | 2 |
| `81.25` | `79.987` | `81.586` | 2 |
| `82.5` | `81.586` | `83.218` | 2 |
| `83.75` | `83.218` | `84.883` | 2 |
| `85.0` | `84.883` | `86.580` | 2 |
| `86.25` | `84.883` | `86.580` | 2 |
| `87.5` | `86.580` | `88.312` | 2 |
| `88.75` | `88.312` | `90.078` | 2 |
| `90.0` | `88.312` | `90.078` | 2 |
| `91.25` | `90.078` | `91.880` | 2 |
| `92.5` | `91.880` | `93.717` | 2 |
| `93.75` | `93.717` | `95.592` | 2 |
| `95.0` | `93.717` | `95.592` | 2 |
| `96.25` | `95.592` | `97.503` | 2 |
| `97.5` | `95.592` | `97.503` | 2 |
| `98.75` | `97.503` | `99.453` | 2 |
| `101.25` | `100.000` | `101.443` | 2 |
| `102.5` | `101.443` | `103.471` | 2 |
| `103.75` | `103.471` | `105.541` | 2 |
| `105.0` | `103.471` | `105.541` | 2 |
| `106.25` | `105.541` | `107.652` | 2 |
| `107.5` | `105.541` | `107.652` | 2 |
| `108.75` | `107.652` | `109.805` | 2 |
| `110.0` | `109.805` | `112.001` | 2 |
| `111.25` | `109.805` | `112.001` | 2 |
| `112.5` | `112.001` | `114.241` | 2 |
| `113.75` | `112.001` | `114.241` | 2 |
| `115.0` | `114.241` | `116.526` | 2 |
| `116.25` | `114.241` | `116.526` | 2 |
| `117.5` | `116.526` | `118.856` | 2 |
| `118.75` | `116.526` | `118.856` | 2 |
| `120.0` | `118.856` | `121.233` | 2 |
| `121.25` | `121.233` | `123.658` | 2 |
| `122.5` | `121.233` | `123.658` | 2 |
| `123.75` | `123.658` | `126.131` | 2 |
| `125.0` | `123.658` | `126.131` | 2 |
| `126.25` | `126.131` | `128.654` | 2 |
| `127.5` | `126.131` | `128.654` | 2 |
| `128.75` | `128.654` | `131.227` | 2 |
| `130.0` | `128.654` | `131.227` | 2 |
| `131.25` | `131.227` | `133.851` | 2 |
| `132.5` | `131.227` | `133.851` | 2 |
| `133.75` | `131.227` | `133.851` | 2 |
| `135.0` | `133.851` | `136.528` | 2 |
| `136.25` | `133.851` | `136.528` | 2 |
| `137.5` | `136.528` | `139.259` | 2 |
| `138.75` | `136.528` | `139.259` | 2 |
