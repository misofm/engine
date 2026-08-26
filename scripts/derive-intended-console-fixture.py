#!/usr/bin/env python3
"""Derive the intended-placement console fixture from the standing 64-track one.

Issue #175. The owner set the intended production rack layout: the EQ and the compressor share
one two-slot chain on ``simd1``, and a true-peak limiter sits alone on ``simd2``. The standing
fixture ``fixtures/session/v1/console-sixty-four-track.toml`` places the EQ on ``simd1`` and the
compressor in the ``dynamic`` rack -- two one-slot chains -- and carries no limiter at all.

This script performs that one transformation and nothing else:

  1. the ``dynamic`` rack's compressor moves, **verbatim**, to the end of the ``simd1`` chain;
  2. the ``dynamic`` rack is left empty;
  3. a ``miso.true-peak-limiter`` is added to the empty ``simd2`` rack.

Steps 1 and 2 are a pure *text* move of the compressor's inline table, so every EQ and compressor
coefficient in the derived fixture is byte-identical to the standing fixture's. That is what makes
the two fixtures a controlled pair: the only arithmetic difference between them is the limiter,
and the only structural difference is which chain the compressor is a slot of. Post-#166 bank
eligibility follows the effect's kernel contract rather than its rack, so the move is a layout
change and must not move a rendered bit -- which the bench and the graph-compiler tests assert
rather than assume.

The output is a *draft*. Canonical spelling, key order and float formatting come from the session
validator, which is also what proves the result is a legal session:

    python3 scripts/derive-intended-console-fixture.py > /tmp/draft.toml
    cargo run -q -p miso-engine-session-validator -- validate --canonical /tmp/draft.toml \
        > fixtures/session/v1/console-sixty-four-track-intended.toml

# The limiter's parameters, and where they come from

Every value below is inside the domain the contract publishes. Generate the authority with
``cargo run -q -p miso-engine-parameter-metadata -- --print`` and read
``effects[] | select(.id == "miso.true-peak-limiter")``; at the time of writing it declares:

  | id | name      | unit         | domain        | default |
  |----|-----------|--------------|---------------|---------|
  | 1  | ceiling   | db           | [-24.0, 0.0]  | -1.0    |
  | 2  | release   | milliseconds | [10.0, 2000.0]| 100.0   |
  | 3  | lookahead | milliseconds | [0.0, 10.0]   | 5.0     |

``link_mode`` is ``maximum``. A true-peak limiter is one of the three effects whose link mode is
not inert, and independent per-lane gain reduction shifts the stereo image of a track that is
being limited -- so a console strip links the two lane detectors by their peak. The limiter
declares support for exactly ``dual_mono`` and ``maximum``
(``TRUE_PEAK_LIMITER_DESCRIPTOR_V1.supported_link_modes``), and homogeneous banking requires one
link mode across the cohort, so ``maximum`` is applied uniformly to all sixty-four tracks.

**ceiling** and **release** vary per track, because the standing fixture's whole premise is that no
two tracks share a coefficient set -- identical strips would let the measurement collapse work a
real console cannot collapse. Both progressions are centred on the contract default and stay well
inside the published domain, and both steps are exact binary fractions (2^-5 and 5/4) so that
canonicalisation has no float spelling to round:

  ceiling[i] = -0.5  - i/32   ->  -0.5 dBTP .. -2.46875 dBTP
  release[i] =  60.0 + i*1.25 ->  60 ms .. 138.75 ms

Those are ordinary channel-strip safety-limiter settings: a ceiling a little under 0 dBTP (the
contract's own default, -1.0, sits inside the range, as does the -1 dBTP that delivery practice
conventionally leaves for inter-sample peaks) and a release in the tens-to-low-hundreds of
milliseconds.

**lookahead** is deliberately *uniform* at the contract default of 5.0 ms, and is the one limiter
parameter that does not vary. Two reasons, both structural rather than aesthetic:

  * It is the limiter's design constant, not a per-channel musical choice. It is the only one of
    the three the contract marks ``automationRate: none`` and ``automatable: false``.
  * It selects the width of the sliding-minimum and box-ramp windows (``Wb = clamp(L+1, 32, R)``
    in the crate's gain law). Varying it per lane would vary a *window length* across the lanes of
    one bank rather than varying a coefficient, which is a different kind of difference from the
    one this fixture exists to carry.

Note that the declared latency does **not** follow this parameter: the crate declares a fixed
``N + 6`` samples where ``N = Fs/100`` (486 samples at 48 kHz), independent of the lookahead value,
so nothing here perturbs the plan's latency or its tail.
"""

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STANDING = ROOT / "fixtures" / "session" / "v1" / "console-sixty-four-track.toml"

SIMD1_OPEN = "simd1 = { effects = ["
DYNAMIC_OPEN = "dynamic = { effects = ["
SIMD2_LINE = "simd2 = { effects = [] }"
SIMD2_OPEN = "simd2 = { effects = ["
ARRAY_CLOSE = "] }"

HEADER = """\
# Issue #175: the intended-placement 64-track qualification session.
#
# This is the standing console qualification fixture. It replaces
# `console-sixty-four-track.toml`, which remains in the tree as the retired authority the
# transition record is measured against.
#
# The owner set the intended production rack layout: the EQ and the compressor share **one
# two-slot chain on `simd1`**, and a **true-peak limiter sits alone on `simd2`**. The retired
# fixture placed the EQ on `simd1` and the compressor in the `dynamic` rack -- two one-slot chains
# -- and carried no limiter on any track.
#
# Both differences are deliberate and each is measured on its own:
#
#   * The compressor's move is a pure chain-shape change. Post-#166 bank eligibility follows the
#     effect's kernel contract and not the rack it was placed in, so merging two one-slot chains
#     into one two-slot chain regroups lanes without changing any lane's arithmetic. It must not
#     move a rendered bit, and it saves one planar/AoSoA transpose round-trip per bank per block
#     (the G5 shape gate: one round-trip per bank chain per block).
#   * The limiter is genuinely new arithmetic. It is the first effect on `simd2` in any checked-in
#     fixture, and it is the one part of this fixture that is not a restatement of the retired
#     one.
#
# Every EQ and compressor coefficient here is byte-identical to the retired fixture's -- this file
# is generated from it by `scripts/derive-intended-console-fixture.py`, which moves the
# compressor's inline table verbatim and adds the limiter. That is what makes the two fixtures a
# controlled pair rather than two different sessions.
#
# The limiter's parameters, their provenance in the published parameter metadata, and why
# `lookahead` is the one that does not vary per track, are documented in that script's header.
#
# Sixty-four tracks is eight full banks at the launch eight-lane width and no scalar tail, so the
# per-track cost this fixture reports is the cost of a full bank rather than the cost of a
# remainder. No two tracks share a coefficient set.
#
# The `sources` entry is deliberately identical to the retired fixture's, down to the content
# identity: the two sessions are fed the same input, which is the other half of what makes their
# numbers comparable.
"""


def limiter_effect(index: int) -> str:
    """The `simd2` limiter for track `index`, as an inline TOML table."""
    # Exact binary fractions: 1/32 and 5/4. See the module docstring for the domains these sit in
    # and for why `lookahead` is uniform.
    ceiling = -0.5 - index / 32
    release = 60.0 + index * 1.25
    return (
        '{ id = "limiter", identity = { kind = "native", '
        'effect_id = "miso.true-peak-limiter" }, quality = "normal", bypass = false, '
        'link_mode = "maximum", params = ['
        f'{{ parameter_id = 1, channel = "both", unit = "db", value = {ceiling} }}, '
        f'{{ parameter_id = 2, channel = "both", unit = "milliseconds", value = {release} }}, '
        '{ parameter_id = 3, channel = "both", unit = "milliseconds", value = 5.0 }'
        "], sidechain = { kind = \"none\" } }"
    )


def inner(line: str, opening: str) -> str:
    """The contents of a `rack = { effects = [ ... ] }` line."""
    assert line.startswith(opening), line
    assert line.endswith(ARRAY_CLOSE), line
    return line[len(opening) : -len(ARRAY_CLOSE)]


def main() -> int:
    lines = STANDING.read_text().splitlines()
    out: list[str] = []
    track = -1
    pending_compressor: str | None = None
    seen_simd1 = seen_dynamic = seen_simd2 = 0

    for line in lines:
        if line == "[[tracks]]":
            assert pending_compressor is None, "a track ended without an emitted simd2"
            track += 1
            out.append(line)
        elif line.startswith(SIMD1_OPEN):
            # Held back: the compressor is on the *next* line and has to be appended here.
            assert pending_compressor is None
            pending_compressor = inner(line, SIMD1_OPEN)
            seen_simd1 += 1
        elif line.startswith(DYNAMIC_OPEN):
            assert pending_compressor is not None, "dynamic rack before simd1"
            compressor = inner(line, DYNAMIC_OPEN)
            assert "miso.compressor" in compressor, compressor
            # The two-slot chain, in strip order: EQ first, then the compressor, exactly as the
            # retired fixture's `simd1 -> dynamic` traversal ran them.
            out.append(f"{SIMD1_OPEN}{pending_compressor}, {compressor}{ARRAY_CLOSE}")
            out.append("dynamic = { effects = [] }")
            pending_compressor = None
            seen_dynamic += 1
        elif line == SIMD2_LINE:
            out.append(f"{SIMD2_OPEN}{limiter_effect(track)}{ARRAY_CLOSE}")
            seen_simd2 += 1
        else:
            out.append(line)

    assert pending_compressor is None
    assert seen_simd1 == seen_dynamic == seen_simd2 == 64, (
        f"expected 64 of each rack line, saw {seen_simd1}/{seen_dynamic}/{seen_simd2}"
    )

    text = "\n".join(out) + "\n"
    # Drop the retired fixture's header comment; this fixture carries its own.
    text = text[text.index("schema_version = 1") :]
    text = re.sub(
        r'^session_id = "console-sixty-four-track"$',
        'session_id = "console-sixty-four-track-intended"',
        text,
        count=1,
        flags=re.M,
    )
    assert "console-sixty-four-track-intended" in text, "session id was not renamed"
    sys.stdout.write(HEADER + canonicalise(text))
    return 0


def canonicalise(draft: str) -> str:
    """Return the session validator's canonical spelling of `draft`.

    This is the #179 dogfood, and it is the reason nothing in this script tries to emit
    presentable TOML. The draft only has to be *legal*; canonical key order, spacing and float
    spelling are the validator's to decide, and taking them from it rather than from a second
    opinion here is what keeps the fixture from acquiring a formatting drift the pipeline would
    not accept. A stage failure is fatal: a draft this script cannot get through the real
    four-stage pipeline is a defect in the derivation, not a file to ship.
    """
    with tempfile.TemporaryDirectory() as directory:
        path = Path(directory) / "draft.toml"
        path.write_text(draft)
        result = subprocess.run(
            [
                "cargo", "run", "-q", "-p", "miso-engine-session-validator", "--",
                "validate", "--canonical", str(path),
            ],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(f"session validator refused the derived draft (exit {result.returncode})")
    return result.stdout


if __name__ == "__main__":
    raise SystemExit(main())
