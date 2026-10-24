# Arm a later mono chain across a per-node op between its chains

Filed by root as the successor that #987's decision record names (backlog sprint, 2026-10-24).

## Product outcome

#987 (PR #1898) arms the mono collapse of a later chain on a split strip when every lane's producer is a same-track bank unit that collapsed in the same block. When a per-node op sits between the two chains (a TrackDelay, a pan or matrix op, a route or a sum), that op becomes the colour's last writer, `input_producer_table` (`crates/graph/src/runtime.rs`) gives the later chain no row, and the chain renders dual. The #970 measurement put the cost of rendering dual on realistic mono sessions at +42 % to +48 %.

## Smallest closable slice

1. For each per-node op kind that can sit between two chains of one strip, state whether it maps an L == R input to an L == R output exactly (bit for bit), and under which parameters. A TrackDelay with equal left and right delays does; a pan or a 2x2 matrix in general does not; state the exact condition for each.
2. Carry the producer's per-block symmetric fact through each op that preserves it, and arm the later chain on that fact, as #987 does for a bank producer.
3. One red-first test in the crate that owns the table: a later mono chain behind a symmetry-preserving per-node op arms and renders the dual bits; behind a symmetry-breaking op it stays dual.

## Constraints

Class A: no rendered bit moves. Owner rule kept: arm only when the consumer's own tracks are eligible (mono source). The memory rule "dual mono by source file" holds.

## Paths

`crates/graph/src/runtime.rs`, `crates/graph-compiler` tests, `crates/host-core/tests/collapse_arming.rs`, this spec.

## Narrowed outcome (scope note of 2026-10-24, root decisions R1-R3)

Only a per-node **insert effect** (`NodeKind::Effect`, `NodeKind::LiveControlEffect`) can sit between
two chains of one track in production. The later chain behind such an op is armed on it, and
collapses on a block exactly when the op's sticky agreement fact holds.

## Decision record

Op table (slice item 1), verified on main `c6e295947`:

| op | L == R in maps to L == R out? | decision |
|---|---|---|
| `TrackDelay` | iff the two lanes declare the same delay (`channels_agree`) | **moot**: it is the track's input node (no graph inputs), never between chains; its fact is already in `SOURCE` (`session_structural_symmetry`). |
| per-node pan / 2x2 matrix | `Route([m00,m01,m10,m11])` maps L == R to L == R iff `(m00 == m10 && m01 == m11) \|\| (m00 == m11 && m01 == m10)` bitwise; a live route also needs equal ramp state | **moot**: the per-node strip triple exists only in test builds (`builtins-compiler` `strip_bindings`, `cfg(test-support)`); in production the strip builtins and console slots always bank. |
| route, sum (`Identity` of a submix `Input`), `SumDelay` | n/a | **excluded**: the consumer is a bus, never collapse-eligible (`mono_source` is tracks only); the owner rule "arm only when the consumer's own tracks are eligible" holds. |
| per-node insert effect | iff the designed words are bit-equal L/R (`channel_symmetry()` with the live terms), the input planes are identical, **and** the L/R state is bit-identical | **implemented** (below). |

- R1: scope is the per-node native-effect op only.
- R2: the state term is a sticky per-op bit with reset-only recovery: `channels_agree[op] &&= input
  && op.kind.channel_symmetry().preserves_channel_agreement()` after every block the op renders,
  seeded at bind from the op's prepared witness. `preserves_channel_agreement` (every term but
  `UNBYPASSED`) and not `eligible`: a live-bypassed per-node instance still runs its processor on
  both channels and mixes one scalar crossfade, so equal input over equal state stays equal (the
  same argument as `ChannelSymmetryWitness::AGREEING`). No proof-based recovery (an effect-contract
  L/R state comparison) in this slice. Nothing in a plan resets it: there is no runtime-level effect
  reset in `crates/graph`.
- R3: on a plan swap, a per-node successor that takes a per-node predecessor's instance
  (`move_from`) takes the predecessor's fact with it (`false` if nothing maintained it there); one
  that takes a bank lane's payload gets `false`, since a lane has no per-instance fact. Follow-up:
  carry the bank chain's agreement into the instance.
- Admission (`input_producer_table`): a plain unit holding one per-node effect op with one input,
  no sidechain and no staging, whose input is an admissible producer's output of the same track
  (a same-track bank final lane, or recursively another admitted op), gets a one-producer row; a
  bank consumer's lane may now name such an op. Op rows that no bank row reaches are pruned, so a
  stereo or bus-only path maintains nothing. `channels_agree` is one bind-allocated `Box<[bool]>`
  per runtime (empty when no op is admitted); render reads and writes it in place.
- The host-core probe uses an upstream episode, not a write on the op itself: every live
  one-channel write clears the op's own `LIVE` term for the rest of the plan, which already masks
  "arming on this block's words alone". The case only the sticky bit catches is an op whose input
  was unequal and is equal again while its state still differs: the producer chain's silent skip
  (`resident_zero`) after an asymmetric episode.
- `Identity` with one input between two banks of one track: **moot**. The only identity stages
  inside a strip are the alias candidates `PostSimd1`, `PostDynamic` and `PostSimd2PreFader`
  (`program::is_alias_candidate`), and `lower` elides each one that has one undelayed main input
  and no sidechain; one that keeps its op has a PDC-delayed input, which admission excludes.

- Gate change (root's gate list named a host-core sticky probe): the sticky term is moved to a
  graph-compiler rig. Measured: with the sticky `&&` dropped, no host-core probe goes red, because
  a later chain whose input term failed once clears its own agreement and re-engages only on a
  `channels_agree` proof, which no launch effect bank gives (`PreparedNativeEffectBank` default
  `false`). The rig's later chain holds a stateless pass-through bank that proves it, so the
  mutation is visible there. The bit is still owed: it is what keeps a future proving bank sound.

## Evidence (attempt 1, AVX2 host, debug unless named)

- Red first: on main's `runtime.rs`, graph-compiler
  `a_later_mono_chain_arms_on_its_producers_and_renders_the_dual_bits` (odd tracks now carry a
  per-node delay) and host-core `both_chains_of_a_strip_split_by_a_per_node_delay_are_armed`
  fail; both pass on the branch.
- New graph-compiler rigs: `a_chain_behind_a_per_node_effect_whose_channels_parted_stays_dual`
  (sticky term), `a_chain_behind_a_sidechained_per_node_effect_is_not_armed_on_it` (admission).
- Mutations: `crates/graph/tests/MUTATIONS.md` rows 1899-1..3, all red.
- `cargo test --locked -p graph -p graph-compiler -p host-core -p rack`: pass.
  `cargo test --locked --release -p console-workload`: 55 pass (`chain_shape` 22, counts
  unchanged). `scripts/check-graph-determinism.sh`: PASS (100/100). `realtime_policy`: ok
  (212 regions). fmt, clippy `-D warnings`, aarch64 `--tests` check, `cargo doc -D warnings`, the
  lint-checks script: pass.
- PR evidence (one-time, not committed): every `native_session_rows()` row rendered 64 blocks armed
  and forced off gives equal digests; no standing row has a chain armed on input across a per-node
  op (`armed_on_input` 0 everywhere), so no standing row's bits or counters move.
