#!/usr/bin/env python3
"""Derive the mono 64-track console fixture from the standing intended-placement one.

The mono-collapse work needs a checked-in session whose tracks are *collapse-eligible*: a track
whose two channels would compute bit-identical work everywhere upstream of the fader/matrix seam.
``crates/effect-contract/src/symmetry.rs`` names the two structural terms that decide
that at preparation, and this script is what makes them true of every track:

  * ``SOURCE`` -- "the track's two channels read one source channel, or a one-channel source". The
    standing fixture maps ``left_source_channel = 0, right_source_channel = 1``; here both read
    channel **0**.
  * ``DESIGNED`` -- "every designed per-lane word the stage's kernel reads compares bit-equal
    between the channels". The standing fixture is deliberately asymmetric everywhere: all 64
    tracks carry a different ``trim_db``/``hpf_hz``/``lpf_hz`` per channel, and 124 of its 128
    left/right EQ parameter pairs carry different values. Here the **left** value wins in both
    places. Since issue #210 phase 2 the builtins table also carries ``delay_samples``; copying the
    whole ``builtins.left`` table carries it across with everything else, which is what keeps an
    asymmetric input delay -- a term this witness genuinely declines -- out of the mono fixture.

So the transformation is exactly three edits, and every one of them is upstream of the seam:

  1. ``right_source_channel = 1`` becomes ``right_source_channel = 0`` on all 64 tracks;
  2. each track's ``builtins.right`` table becomes a copy of its ``builtins.left`` table;
  3. each ``{ parameter_id = N, channel = "right", ... }`` parameter takes the value of the
     ``{ parameter_id = N, channel = "left", ... }`` entry beside it.

# What is deliberately *kept*, and why

**The fader and pan asymmetry.** 49 of the 64 tracks carry a different ``left_db`` from their
``right_db``, and 50 carry a different pan ``left`` from their pan ``right``. Both stay exactly as
the standing fixture wrote them. They are the *seam*: the collapse duplicates the single computed
plane **into** the fader and the matrix, so those two stages' per-channel words are free to differ
and must not gate anything (``SeamSide::SeamSide``). A mono fixture that also symmetrised its
faders would be a fixture on which a collapse that wrongly gated on seam-side words would still
pass, and the row-pair this fixture exists for would prove nothing.

**The limiter's ``link_mode = "maximum"``.** A true-peak limiter is one of the three effects whose
link mode is not inert: ``maximum`` links the two lane detectors by their peak. On a track whose
two channels carry identical samples the linked maximum *is* each lane's own peak, so the link is
compatible with collapse rather than an obstacle to it -- and that is precisely why it is worth
keeping. A mono fixture whose only stereo-linked effect had been quietly unlinked would be a
fixture on which "collapse and link mode interact" could never be observed.

**``channels = 2`` on the source, and the source identity.** The witness admits *either* "two
channels read one source channel" *or* "a one-channel source"; this fixture is the first form. That
is the form the ``sixty_four_track_console_half_mono`` bench row needs, because that row is derived
**in code** from this fixture by putting ``right_source_channel = 1`` back on the odd tracks, and a
one-channel source would make that derivation illegal rather than merely asymmetric. The source's
content identity is left byte-identical to the standing fixture's: the two sessions are fed the
same input, which is half of what makes their numbers comparable.

**Both ``channel = "left"`` and ``channel = "right"`` parameter entries.** The symmetrised pairs are
kept as *pairs* carrying equal values rather than collapsed into a single ``channel = "both"``
entry. A collapse to ``both`` would make the fixture prove that the ``DESIGNED`` witness reads the
*declaration shape*; keeping the pair makes it prove that the witness compares the designed
per-lane **words**, which is what it claims to do.

# The output is a draft

Canonical spelling, key order and float formatting come from the session validator, which is also
what proves the result is a legal session:

    python3 scripts/derive-mono-console-fixture.py > fixtures/session/v1/console-sixty-four-track-mono.toml

``scripts/check-mono-console-fixture.sh`` regenerates the committed file and compares it byte for
byte, so a hand-edit is caught rather than silently carried into a measurement.
"""

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STANDING = ROOT / "fixtures" / "session" / "v1" / "console-sixty-four-track-intended.toml"

TRACKS = 64

# Every left/right parameter pair in the standing fixture is adjacent and shares a unit, because
# the canonical spelling sorts a slot's parameters by id and then by channel. Anchoring on that
# adjacency is what keeps the rewrite scoped to a *pair* -- an unpaired `channel = "right"` entry
# would be left alone and would then fail the post-condition below.
PARAM_PAIR = re.compile(
    r'\{ parameter_id = (?P<id>\d+), channel = "left", unit = "(?P<unit>[a-z_]+)", '
    r'value = (?P<left>-?[\d.]+) \}, '
    r'\{ parameter_id = (?P=id), channel = "right", unit = "(?P=unit)", value = -?[\d.]+ \}'
)
BUILTINS = re.compile(
    r"builtins = \{ left = \{ (?P<left>[^{}]*) \}, right = \{ [^{}]* \} \}"
)
BUILTINS_PAIR = re.compile(
    r"builtins = \{ left = \{ ([^{}]*) \}, right = \{ ([^{}]*) \} \}"
)
FADER = re.compile(
    r"fader = \{ left_db = (-?[\d.]+), right_db = (-?[\d.]+), "
    r"left_mute = (\w+), right_mute = (\w+) \}"
)
PAN = re.compile(
    r"pan = \{ left = (-?[\d.]+), right = (-?[\d.]+), smoothing_samples = (\d+) \}"
)

# This string is written byte-for-byte into a frozen fixture (fixtures/session/v1/
# console-sixty-four-track-mono.toml, docs/rulings/prefix-strip-inventory.md): the
# `crates/miso-engine-effect-contract/...` mention below stays pinned to the pre-rename
# spelling deliberately -- the fixture was not rewritten, and this generator must keep
# producing its exact bytes.
HEADER = """\
# The mono 64-track console session: the same strip, collapse-eligible upstream of the seam.
#
# Generated from `console-sixty-four-track-intended.toml` by
# `scripts/derive-mono-console-fixture.py`, which makes exactly three edits and every one of them
# is upstream of the fader/matrix seam:
#
#   1. both of every track's channels read source channel 0 (`right_source_channel = 1` -> `0`);
#   2. every track's `builtins.right` is a copy of its `builtins.left`;
#   3. every `channel = "right"` effect parameter takes the value of the `channel = "left"` entry
#      beside it.
#
# Those are the two structural terms of the per-track channel-symmetry witness
# (`crates/miso-engine-effect-contract/src/symmetry.rs`): SOURCE and DESIGNED. Every track here
# satisfies both, so every track is collapse-eligible -- which is the whole point of the fixture.
#
# What is deliberately kept, because it is what documents the seam:
#
#   * **The fader and pan asymmetry.** 49 tracks carry a different `left_db` from their `right_db`
#     and 50 carry a different pan `left` from their pan `right`, exactly as the standing fixture
#     wrote them. The collapse duplicates the one computed plane *into* those two stages, so their
#     per-channel words are free to differ and must not gate anything. A fixture that symmetrised
#     them too would pass even for a collapse that wrongly gated on seam-side words.
#   * **The limiter's `link_mode = "maximum"`.** On a track whose channels carry identical samples
#     the linked maximum is each lane's own peak, so a stereo link and a collapse are compatible --
#     and a mono fixture that had quietly unlinked its one stereo-linked effect could never show
#     that.
#   * **`channels = 2` on the source.** The witness admits "two channels read one source
#     channel" as well as "a one-channel source"; this is the first form, and it is the form the
#     `sixty_four_track_console_half_mono` bench row needs, because that row is derived in code
#     from this fixture by putting `right_source_channel = 1` back on the odd tracks.
#   * **Both `channel = "left"` and `channel = "right"` parameter entries**, carrying equal values
#     rather than collapsed into one `channel = "both"` entry. The witness claims to compare
#     designed per-lane *words*, not declaration shapes, and a collapsed pair could not tell the
#     two apart.
#
# The source's content identity is byte-identical to the standing fixture's: the two sessions are
# fed the same input, which is half of what makes their numbers comparable. Everything
# else -- 64 tracks, eight full eight-lane banks and no scalar tail, EQ and compressor as one
# two-slot `simd1` chain, a true-peak limiter alone on `simd2`, no automation -- is the standing
# fixture's, unchanged.
"""


def symmetrise_params(text: str) -> tuple[str, int]:
    """Give every `channel = "right"` parameter its `channel = "left"` sibling's value."""

    def replace(match: re.Match[str]) -> str:
        identifier, unit, left = match["id"], match["unit"], match["left"]
        entry = f'parameter_id = {identifier}, channel = "%s", unit = "{unit}", value = {left}'
        return "{ " + entry % "left" + " }, { " + entry % "right" + " }"

    return PARAM_PAIR.subn(replace, text)


def symmetrise_builtins(text: str) -> tuple[str, int]:
    """Make every track's `builtins.right` a copy of its `builtins.left`."""

    def replace(match: re.Match[str]) -> str:
        left = match["left"]
        return f"builtins = {{ left = {{ {left} }}, right = {{ {left} }} }}"

    return BUILTINS.subn(replace, text)


def main() -> int:
    text = STANDING.read_text()
    # Drop the standing fixture's header comment; this fixture carries its own.
    text = text[text.index("schema_version = 1") :]

    seam_faders = sum(1 for f in FADER.findall(text) if f[0] != f[1] or f[2] != f[3])
    seam_pans = sum(1 for p in PAN.findall(text) if p[0] != p[1])
    limiters = text.count('link_mode = "maximum"')

    text, sources = re.subn(
        r"right_source_channel = 1", "right_source_channel = 0", text
    )
    text, builtins = symmetrise_builtins(text)
    text, params = symmetrise_params(text)
    text = re.sub(
        r'^session_id = "console-sixty-four-track-intended"$',
        'session_id = "console-sixty-four-track-mono"',
        text,
        count=1,
        flags=re.M,
    )

    # Post-conditions. Each is a fact the fixture's whole reason for existing rests on, so it is
    # asserted here rather than left to the shell check that reads the finished file.
    assert sources == TRACKS, f"expected {TRACKS} source mappings to monoise, saw {sources}"
    assert builtins == TRACKS, f"expected {TRACKS} builtin pairs to symmetrise, saw {builtins}"
    assert params == 2 * TRACKS, f"expected {2 * TRACKS} parameter pairs, saw {params}"
    assert text.count('channel = "right"') == 2 * TRACKS, (
        "the right-channel entries must survive as pairs, not be collapsed into `both`"
    )
    survivors = BUILTINS_PAIR.findall(text)
    assert len(survivors) == TRACKS and all(left == right for left, right in survivors), (
        "a builtins pair survived symmetrisation"
    )
    assert all(
        match["left"] == match.group(0).rsplit("value = ", 1)[1].rstrip(" }")
        for match in PARAM_PAIR.finditer(text)
    ), "a parameter pair survived symmetrisation"
    assert text.count("right_source_channel = 1") == 0, "a stereo source mapping survived"
    assert "console-sixty-four-track-mono" in text, "session id was not renamed"

    # And the three things that must *not* have moved.
    assert (
        sum(1 for f in FADER.findall(text) if f[0] != f[1] or f[2] != f[3]) == seam_faders
    ), "the fader asymmetry that documents the seam was lost"
    assert sum(1 for p in PAN.findall(text) if p[0] != p[1]) == seam_pans, (
        "the pan asymmetry that documents the seam was lost"
    )
    assert text.count('link_mode = "maximum"') == limiters == TRACKS, (
        "the limiter's stereo link was lost"
    )

    sys.stdout.write(HEADER + canonicalise(text))
    return 0


def canonicalise(draft: str) -> str:
    """Return the session validator's canonical spelling of `draft`.

    Identical in shape and in reason to `derive-intended-console-fixture.py`'s: the draft only has
    to be *legal*, and canonical key order, spacing and float spelling are the validator's to
    decide. A stage failure is fatal -- a draft this script cannot get through the real four-stage
    pipeline is a defect in the derivation, not a file to ship.
    """
    with tempfile.TemporaryDirectory() as directory:
        path = Path(directory) / "draft.toml"
        path.write_text(draft)
        result = subprocess.run(
            [
                "cargo", "run", "-q", "-p", "session-validator", "--",
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
