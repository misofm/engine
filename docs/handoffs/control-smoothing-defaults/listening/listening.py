#!/usr/bin/env python3
"""Issue #1055 blinded listening packet: prepare, run, validate, reveal, self-test.

Standard library only. The stimuli come from the engine's own ramps
(`control_smoothing_measure stimuli`); this tool only blinds, plays, records and scores them.

    listening.py prepare --stimuli DIR --out PACKET [--seed N] [--commit SHA]
    listening.py run PACKET [--block M|F|R|P] [--player "afplay"]
    listening.py validate PACKET
    listening.py reveal PACKET
    listening.py self-test

Conventions follow the repository's earlier listening packets (dsp-research/listening/issue033):
SplitMix64-v1 randomisation with a Fisher-Yates shuffle, anonymous 32-hex-character stimulus
tokens, a balanced candidate-first schedule per contrast, public files mode 0444, a private
mode-0600 assignment key committed to by hash in the public preparation record, and canonical
JSON (sorted keys, no insignificant whitespace, LF). Synthetic answers exist only inside the
self-test's temporary directory and are never written as listening evidence.
"""

from __future__ import annotations

import contextlib
import datetime
import hashlib
import io
import json
import math
import os
import shutil
import stat
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

SCHEMA_VERSION = 1
RECORD_ID = "issue1055-control-smoothing-v1"
RATE = 48_000
GAP_S = 0.6
MAX_ATTEMPTS = 2
ALPHA = 0.05

BLOCKS = {
    "M": "Which interval has a click, tick or thump at the moment the sound stops or restarts?",
    "F": "Which interval sounds stepped, buzzy or rough while the level moves?",
    "R": "Which interval's cuts sound softer, rounder or later (less tight)?",
    "P": "Which interval has a click, tick, thump or brief dip while the sound plays on?",
}

# Blocks M, F and R decide the three values; block P (PREREGISTRATION.md, Amendment 1) decides none.
VALUE_BLOCKS = ("M", "F", "R")

# (contrast, block, candidate, reference, trials, role). The candidate is the interval the
# question describes when the difference is audible: the shorter ramp for M and F (it may click
# or zip), the longer ramp for R (it may sound soft), the flip or one part of its change for P
# (Amendment 1; the reference is the same note with no flip). Frozen by design_sha256 at
# preparation.
DESIGN = [
    ("M-bass-0", "M", "mute-bass-0ms", "mute-bass-50ms", 2, "control"),
    ("M-bass-5", "M", "mute-bass-5ms", "mute-bass-50ms", 8, "primary"),
    ("M-bass-10", "M", "mute-bass-10ms", "mute-bass-50ms", 8, "primary"),
    ("M-bass-20", "M", "mute-bass-20ms", "mute-bass-50ms", 4, "secondary"),
    ("M-kick-5", "M", "mute-kick-5ms", "mute-kick-50ms", 4, "secondary"),
    ("M-mix-2", "M", "mute-mix-2ms", "mute-mix-50ms", 4, "secondary"),
    ("M-mix-5", "M", "mute-mix-5ms", "mute-mix-50ms", 4, "secondary"),
    ("F-30-0", "F", "drag-bass-30hz-0ms", "drag-bass-30hz-40ms", 2, "control"),
    ("F-30-10", "F", "drag-bass-30hz-10ms", "drag-bass-30hz-40ms", 4, "secondary"),
    ("F-30-20", "F", "drag-bass-30hz-20ms", "drag-bass-30hz-40ms", 8, "primary"),
    ("F-60-10", "F", "drag-bass-60hz-10ms", "drag-bass-60hz-40ms", 4, "secondary"),
    ("F-60-20", "F", "drag-bass-60hz-20ms", "drag-bass-60hz-40ms", 2, "secondary"),
    ("F-flick-20", "F", "flick-bass-30hz-20ms", "flick-bass-30hz-40ms", 4, "secondary"),
    ("R-mix-10", "R", "chop-mix-10ms", "chop-mix-5ms", 4, "secondary"),
    ("R-mix-20", "R", "chop-mix-20ms", "chop-mix-5ms", 4, "secondary"),
    ("R-mix-50", "R", "chop-mix-50ms", "chop-mix-5ms", 2, "control"),
    ("P-bass-0-oob", "P", "polarity-bass-0ms-oob", "polarity-bass-none", 2, "control"),
    ("P-bass-200-inband", "P", "polarity-bass-200ms-inband", "polarity-bass-none", 2, "control"),
    ("P-bass-20", "P", "polarity-bass-20ms", "polarity-bass-none", 8, "primary"),
    ("P-bass-20-inband", "P", "polarity-bass-20ms-inband", "polarity-bass-none", 8, "primary"),
    ("P-bass-20-oob", "P", "polarity-bass-20ms-oob", "polarity-bass-none", 8, "primary"),
    ("P-kick-20", "P", "polarity-kick-20ms", "polarity-kick-none", 4, "secondary"),
    ("P-mix-20", "P", "polarity-mix-20ms", "polarity-mix-none", 4, "secondary"),
]

# Block P's rule (Amendment 1): the cue each primary contrast tests at the shipped flip, which is
# twice the shipped muteMs. It changes no value.
SHIPPED_MUTE_MS = 10
POLARITY_FLIP_MS = 2 * SHIPPED_MUTE_MS
POLARITY_CUES = {"flip": "P-bass-20", "dip": "P-bass-20-inband", "click": "P-bass-20-oob"}

# Labelled familiarisation pairs: they carry no answer and are not trials.
TRAINING = [
    ("01-mute-bass-hard-switch-0ms", "mute-bass-0ms"),
    ("02-mute-bass-50ms", "mute-bass-50ms"),
    ("03-drag-bass-30hz-stepped-0ms", "drag-bass-30hz-0ms"),
    ("04-drag-bass-30hz-40ms", "drag-bass-30hz-40ms"),
    ("05-chop-mix-5ms", "chop-mix-5ms"),
    ("06-chop-mix-soft-50ms", "chop-mix-50ms"),
    ("07-polarity-bass-no-flip", "polarity-bass-none"),
    ("08-polarity-bass-click-of-a-hard-flip", "polarity-bass-0ms-oob"),
    ("09-polarity-bass-dip-of-a-slow-flip-200ms", "polarity-bass-200ms-inband"),
]


class Invalid(ValueError):
    pass


def canonical(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def design_sha256() -> str:
    return sha256(canonical({"blocks": BLOCKS, "design": DESIGN, "training": TRAINING}))


class SplitMix64:
    """SplitMix64-v1, bit-identical to tools/audit/src/fixture_builtins_listening.rs."""

    def __init__(self, seed: int) -> None:
        self.state = seed & 0xFFFF_FFFF_FFFF_FFFF

    def next(self) -> int:
        mask = 0xFFFF_FFFF_FFFF_FFFF
        self.state = (self.state + 0x9E37_79B9_7F4A_7C15) & mask
        z = self.state
        z = ((z ^ (z >> 30)) * 0xBF58_476D_1CE4_E5B9) & mask
        z = ((z ^ (z >> 27)) * 0x94D0_49BB_1331_11EB) & mask
        return z ^ (z >> 31)

    def bounded(self, upper: int) -> int:
        threshold = ((1 << 64) - upper) % upper
        while True:
            value = self.next()
            if value >= threshold:
                return value % upper


def shuffle(values: list, rng: SplitMix64) -> None:
    for upper in range(len(values) - 1, 0, -1):
        index = rng.bounded(upper + 1)
        values[upper], values[index] = values[index], values[upper]


# ---- WAV (the renderer's 44-byte RIFF, stereo interleaved 32-bit float) -------------------------


def read_wav(path: Path) -> bytes:
    data = path.read_bytes()
    if len(data) < 44 or data[0:4] != b"RIFF" or data[8:16] != b"WAVEfmt ":
        raise Invalid(f"not a RIFF/WAVE file: {path.name}")
    tag, channels, rate = struct.unpack("<HHI", data[20:28])
    bits = struct.unpack("<H", data[34:36])[0]
    if (tag, channels, rate, bits) != (3, 2, RATE, 32) or data[36:40] != b"data":
        raise Invalid(f"expected 48 kHz stereo float32 RIFF44: {path.name}")
    size = struct.unpack("<I", data[40:44])[0]
    body = data[44 : 44 + size]
    if len(body) != size or size % 8:
        raise Invalid(f"truncated WAV: {path.name}")
    return body


def wav_bytes(body: bytes) -> bytes:
    header = b"RIFF" + struct.pack("<I", 36 + len(body)) + b"WAVEfmt "
    header += struct.pack("<IHHIIHH", 16, 3, 2, RATE, RATE * 8, 8, 32)
    return header + b"data" + struct.pack("<I", len(body)) + body


def write_mode(path: Path, data: bytes, mode: int) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "xb") as handle:
        handle.write(data)
    path.chmod(mode)


def require_mode(path: Path, mode: int) -> None:
    info = path.lstat()
    if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1:
        raise Invalid(f"regular one-link file: {path}")
    if stat.S_IMODE(info.st_mode) != mode:
        raise Invalid(f"mode {oct(mode)} required: {path}")


# ---- prepare --------------------------------------------------------------------------------------


def load_stimuli(directory: Path) -> dict[str, bytes]:
    manifest = (directory / "manifest.tsv").read_text().splitlines()
    if not manifest or manifest[0].split("\t")[:2] != ["condition", "file"]:
        raise Invalid("stimulus manifest header")
    files = {}
    for line in manifest[1:]:
        condition, file = line.split("\t")[:2]
        files[condition] = directory / file
    needed = {c for row in DESIGN for c in row[2:4]} | {c for _, c in TRAINING}
    missing = sorted(needed - files.keys())
    if missing:
        raise Invalid(f"stimuli missing: {missing}")
    return {condition: read_wav(files[condition]) for condition in sorted(needed)}


def prepare(stimuli_dir: Path, out: Path, seed: int | None, commit: str) -> None:
    stimuli = load_stimuli(stimuli_dir)
    if out.exists():
        raise Invalid(f"refusing to overwrite {out}")
    if seed is None:
        seed = int.from_bytes(os.urandom(8), "little")
    rng = SplitMix64(seed)
    trials = []
    for block in BLOCKS:
        entries = []
        for contrast, b, candidate, reference, count, role in DESIGN:
            if b != block:
                continue
            schedule = [True] * (count // 2) + [False] * (count - count // 2)
            shuffle(schedule, rng)
            entries += [(contrast, candidate, reference, first) for first in schedule]
        shuffle(entries, rng)
        trials += [(block, *entry) for entry in entries]
    tokens: list[str] = []
    while len(tokens) < len(trials):
        token = f"{rng.next():016x}{rng.next():016x}.wav"
        if token not in tokens:
            tokens.append(token)
    public, private = out / "public", out / "private"
    out.mkdir(parents=True)
    public.mkdir(mode=0o755)
    private.mkdir(mode=0o700)
    gap = bytes(8 * round(GAP_S * RATE))
    listed, keyed = [], []
    for sequence, ((block, contrast, candidate, reference, first), token) in enumerate(
        zip(trials, tokens), start=1
    ):
        one, two = (candidate, reference) if first else (reference, candidate)
        write_mode(public / token, wav_bytes(stimuli[one] + gap + stimuli[two]), 0o444)
        listed.append({"block": block, "file": token, "question": BLOCKS[block], "sequence": sequence})
        keyed.append(
            {
                "candidate": candidate,
                "candidate_first": first,
                "contrast": contrast,
                "reference": reference,
                "sequence": sequence,
            }
        )
    for label, condition in TRAINING:
        write_mode(public / "training" / f"{label}.wav", wav_bytes(stimuli[condition]), 0o444)
    write_mode(public / "trials.json", canonical(listed), 0o444)
    key = canonical({"record_id": RECORD_ID, "schema_version": SCHEMA_VERSION, "seed": str(seed), "trials": keyed})
    write_mode(private / "assignment-key.json", key, 0o600)
    members = {
        str(path.relative_to(out)): sha256(path.read_bytes())
        for path in sorted(public.rglob("*"))
        if path.is_file()
    }
    preparation = {
        "assignment_key_sha256": sha256(key),
        "engine_commit": commit,
        "counters": {"human_listening_sessions": 0, "valid_human_responses": 0},
        "design_sha256": design_sha256(),
        "public_member_sha256": members,
        "record_id": RECORD_ID,
        "schema_version": SCHEMA_VERSION,
        "status": "prepared",
        "stimulus_sha256": {c: sha256(wav_bytes(body)) for c, body in stimuli.items()},
        "trials": len(trials),
    }
    write_mode(public / "preparation.json", canonical(preparation), 0o444)
    validate(out)
    print(f"prepared {len(trials)} trials in {out}; keep {private} closed until reveal")


# ---- validate -------------------------------------------------------------------------------------


def load_json(path: Path) -> object:
    data = path.read_bytes()
    value = json.loads(data)
    if canonical(value) != data:
        raise Invalid(f"non-canonical JSON: {path.name}")
    return value


def validate(packet: Path) -> tuple[dict, list, dict]:
    public, private = packet / "public", packet / "private"
    if stat.S_IMODE(private.stat().st_mode) != 0o700:
        raise Invalid("private directory must be mode 0700")
    require_mode(public / "preparation.json", 0o444)
    preparation = load_json(public / "preparation.json")
    if preparation.get("record_id") != RECORD_ID or preparation.get("schema_version") != SCHEMA_VERSION:
        raise Invalid("preparation identity")
    if preparation["design_sha256"] != design_sha256():
        raise Invalid("design drift: listening.py no longer matches the prepared packet")
    actual = {
        str(path.relative_to(packet)): path
        for path in public.rglob("*")
        if path.is_file() and path.name != "preparation.json"
    }
    if set(actual) != set(preparation["public_member_sha256"]):
        raise Invalid("public member set")
    for name, path in actual.items():
        require_mode(path, 0o444)
        if sha256(path.read_bytes()) != preparation["public_member_sha256"][name]:
            raise Invalid(f"public member hash: {name}")
    key_path = private / "assignment-key.json"
    require_mode(key_path, 0o600)
    if sha256(key_path.read_bytes()) != preparation["assignment_key_sha256"]:
        raise Invalid("assignment-key commitment")
    trials = load_json(public / "trials.json")
    if [t["sequence"] for t in trials] != list(range(1, preparation["trials"] + 1)):
        raise Invalid("trial sequence")
    key = load_json(key_path)
    if [t["sequence"] for t in key["trials"]] != [t["sequence"] for t in trials]:
        raise Invalid("key/trial alignment")
    for row in key["trials"]:
        match = [d for d in DESIGN if d[0] == row["contrast"]]
        if len(match) != 1 or (match[0][2], match[0][3]) != (row["candidate"], row["reference"]):
            raise Invalid("key contrast")
    for contrast, _, _, _, count, _ in DESIGN:
        rows = [r for r in key["trials"] if r["contrast"] == contrast]
        if len(rows) != count or sum(r["candidate_first"] for r in rows) != count // 2:
            raise Invalid(f"balanced schedule: {contrast}")
    responses_path = packet / "responses.jsonl"
    if responses_path.exists():
        require_mode(responses_path, 0o600)
        check_responses(read_responses(responses_path), len(trials))
    return preparation, trials, key


def read_responses(path: Path) -> list[dict]:
    rows = []
    for line in path.read_bytes().splitlines(keepends=True):
        row = json.loads(line)
        if canonical(row) != line:
            raise Invalid("non-canonical response row")
        rows.append(row)
    return rows


def check_responses(rows: list[dict], trials: int) -> None:
    fields = {"answer", "attempt", "confidence", "observation", "reason", "schema_version", "sequence", "trial", "utc", "valid"}
    attempts: dict[int, int] = {}
    answered = set()
    for index, row in enumerate(rows, start=1):
        if set(row) != fields or row["schema_version"] != SCHEMA_VERSION or row["sequence"] != index:
            raise Invalid(f"response row {index} shape")
        trial = row["trial"]
        if type(trial) is not int or not 1 <= trial <= trials or trial in answered:
            raise Invalid(f"response row {index} trial")
        attempts[trial] = attempts.get(trial, 0) + 1
        if row["attempt"] != attempts[trial] or row["attempt"] > MAX_ATTEMPTS:
            raise Invalid(f"response row {index} attempt")
        if row["valid"] is True:
            if row["answer"] not in ("1", "2") or row["reason"] is not None:
                raise Invalid(f"response row {index} answer")
            confidence = row["confidence"]
            if confidence is not None and (type(confidence) is not int or not 0 <= confidence <= 100):
                raise Invalid(f"response row {index} confidence")
            answered.add(trial)
        elif row["valid"] is False:
            if row["answer"] is not None or not row["reason"]:
                raise Invalid(f"response row {index} invalid attempt needs a reason and no answer")
        else:
            raise Invalid(f"response row {index} valid flag")


# ---- run ------------------------------------------------------------------------------------------


def default_player() -> list[str]:
    for command in (["afplay"], ["paplay"], ["aplay", "-q"], ["ffplay", "-nodisp", "-autoexit", "-loglevel", "quiet"]):
        if shutil.which(command[0]):
            return command
    raise Invalid("no audio player found; pass --player")


def run(packet: Path, block: str | None, player: list[str] | None) -> None:
    _, trials, _ = validate(packet)
    player = player or default_player()
    path = packet / "responses.jsonl"
    rows = read_responses(path) if path.exists() else []
    if not path.exists():
        path.touch(mode=0o600)
        path.chmod(0o600)
    done = {r["trial"] for r in rows if r["valid"]}
    tries = {}
    for r in rows:
        tries[r["trial"]] = tries.get(r["trial"], 0) + 1
    print("Answer 1 or 2. r = replay, x = mark this attempt invalid (playback problem), q = quit.")
    for trial in trials:
        if trial["sequence"] in done or (block and trial["block"] != block):
            continue
        if tries.get(trial["sequence"], 0) >= MAX_ATTEMPTS:
            continue
        print(f"\nTrial {trial['sequence']}/{len(trials)} [{trial['block']}] {trial['question']}")
        while True:
            subprocess.run([*player, str(packet / "public" / trial["file"])], check=False)
            answer = input("answer [1/2/r/x/q]: ").strip().lower()
            if answer == "q":
                return
            if answer in ("1", "2", "x"):
                break
        confidence, reason = None, None
        if answer == "x":
            reason = input("reason (required): ").strip() or "unspecified playback problem"
        else:
            text = input("confidence 0-100 (blank to skip): ").strip()
            confidence = int(text) if text.isdigit() and int(text) <= 100 else None
        observation = input("observation (optional): ").strip()
        tries[trial["sequence"]] = tries.get(trial["sequence"], 0) + 1
        row = {
            "answer": None if answer == "x" else answer,
            "attempt": tries[trial["sequence"]],
            "confidence": confidence,
            "observation": observation,
            "reason": reason,
            "schema_version": SCHEMA_VERSION,
            "sequence": len(rows) + 1,
            "trial": trial["sequence"],
            "utc": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "valid": answer != "x",
        }
        rows.append(row)
        with open(path, "ab") as handle:
            handle.write(canonical(row))


# ---- reveal ---------------------------------------------------------------------------------------


def binomial_tail(k: int, n: int) -> float:
    """P(X >= k) for X ~ Binomial(n, 1/2): the one-sided exact p-value of k correct of n."""
    return sum(math.comb(n, i) for i in range(k, n + 1)) / 2**n


def score(rows: list[dict], key: dict) -> dict:
    answers = {r["trial"]: r["answer"] for r in rows if r["valid"]}
    results = {}
    for contrast, block, candidate, reference, count, role in DESIGN:
        keyed = [t for t in key["trials"] if t["contrast"] == contrast]
        missing = [t["sequence"] for t in keyed if t["sequence"] not in answers]
        if missing:
            raise Invalid(f"{contrast}: trials without a valid answer: {missing}")
        correct = sum(answers[t["sequence"]] == ("1" if t["candidate_first"] else "2") for t in keyed)
        p = binomial_tail(correct, count)
        results[contrast] = {
            "block": block,
            "candidate": candidate,
            "correct": correct,
            "detected": role != "control" and p <= ALPHA,
            "p_one_sided": round(p, 6),
            "reference": reference,
            "role": role,
            "trials": count,
        }

    def passed(blocks: tuple[str, ...]) -> bool:
        return all(
            r["correct"] == r["trials"]
            for r in results.values()
            if r["role"] == "control" and r["block"] in blocks
        )

    # Each part's positive controls gate only its own non-detections (Amendment 1).
    controls, polarity_controls = passed(VALUE_BLOCKS), passed(("P",))
    decisions = decide(results, controls)
    decisions["polarity"] = decide_polarity(
        results, polarity_controls, decisions.get("mute_ms", SHIPPED_MUTE_MS)
    )
    return {
        "contrasts": results,
        "controls_passed": controls,
        "decisions": decisions,
        "polarity_controls_passed": polarity_controls,
    }


def decide(results: dict, controls: bool) -> dict:
    """The preregistered decision rules of PREREGISTRATION.md, applied mechanically."""
    if not controls:
        return {
            "status": "inconclusive",
            "reason": "a positive control was missed, so a non-detection cannot be read as inaudibility",
        }
    detected = {name: r["detected"] for name, r in results.items()}
    four_of_four = {name: r["correct"] == r["trials"] == 4 for name, r in results.items()}
    if not detected["M-bass-5"]:
        mute = 5
    elif not detected["M-bass-10"]:
        mute = 10
    elif not four_of_four["M-bass-20"] and not four_of_four["R-mix-20"]:
        mute = 20
    else:
        mute = 10
    fader = 20 if not detected["F-30-20"] else 35
    return {
        "status": "decided",
        "mute_ms": mute,
        "fader_ms": fader,
        "pan_ms": fader,
        "note": "bounded to this listener, playback chain and level; see PREREGISTRATION.md",
    }


def decide_polarity(results: dict, controls: bool, mute_ms: int) -> dict:
    """Amendment 1's block-P rule, applied mechanically. It changes no value.

    Each cue at the shipped flip is heard when its contrast is detected, not heard when it is not
    detected and both P controls are 2/2, and inconclusive otherwise. The answer is carried to the
    flip of the decided muteMs (twice it) where the length makes it certain: a longer flip has a
    longer dip and a weaker click, a shorter one the reverse; anything else is not assessed.
    """

    def status(contrast: str) -> str:
        if results[contrast]["detected"]:
            return "heard"
        return "not heard" if controls else "inconclusive"

    shipped = {cue: status(contrast) for cue, contrast in POLARITY_CUES.items()}
    if "heard" in shipped.values():
        outcome = "heard"
    elif set(shipped.values()) == {"not heard"}:
        outcome = "not heard"
    else:
        outcome = "inconclusive"
    flip_ms = 2 * mute_ms
    decided = {cue: shipped[cue] for cue in ("dip", "click")}
    if flip_ms > POLARITY_FLIP_MS:
        decided = {
            "dip": "heard" if shipped["dip"] == "heard" else "not assessed",
            "click": "not heard" if shipped["click"] == "not heard" else "not assessed",
        }
    elif flip_ms < POLARITY_FLIP_MS:
        decided = {
            "dip": "not heard" if shipped["dip"] == "not heard" else "not assessed",
            "click": "heard" if shipped["click"] == "heard" else "not assessed",
        }
    return {
        "at_decided_flip": {"flip_ms": flip_ms, **decided},
        "at_shipped_flip": {"flip_ms": POLARITY_FLIP_MS, **shipped},
        "heard_as": [cue for cue in ("dip", "click") if shipped[cue] == "heard"],
        "outcome": outcome,
        "record": {
            "heard": "a finding for root; defaults unchanged",
            "not heard": "answers FINDINGS 9.7 for this listener, chain and level; defaults unchanged",
            "inconclusive": "a P control was missed, so no P non-detection is read; defaults unchanged",
        }[outcome],
        "values_changed": [],
    }


def reveal(packet: Path) -> None:
    preparation, trials, key = validate(packet)
    path = packet / "responses.jsonl"
    rows = read_responses(path)
    result = score(rows, key)
    report = {
        "assignment_key_sha256": preparation["assignment_key_sha256"],
        "record_id": RECORD_ID,
        "responses_sha256": sha256(path.read_bytes()),
        "reveal_utc": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "schema_version": SCHEMA_VERSION,
        **result,
    }
    out = packet / "reveal.json"
    write_mode(out, canonical(report), 0o600)
    print(f"{'contrast':18} {'role':9} {'correct':>9} {'p':>8}  detected")
    for name, r in result["contrasts"].items():
        print(f"{name:18} {r['role']:9} {r['correct']:>4}/{r['trials']:<4} {r['p_one_sided']:8.4f}  {r['detected']}")
    print(json.dumps(result["decisions"], indent=2, sort_keys=True))
    print(f"wrote {out}")


# ---- self-test ------------------------------------------------------------------------------------


def self_test() -> None:
    """Exercises prepare/validate/reveal on tiny synthetic stimuli in a temporary directory."""
    rng = SplitMix64(0)
    assert [rng.next() for _ in range(2)] == [0xE220A8397B1DCDAF, 0x6E789E6AA1B965F4]
    assert binomial_tail(7, 8) == 9 / 256 and binomial_tail(0, 4) == 1.0
    with tempfile.TemporaryDirectory() as temp:
        root = Path(temp)
        stimuli = root / "stimuli"
        stimuli.mkdir()
        conditions = sorted({c for row in DESIGN for c in row[2:4]} | {c for _, c in TRAINING})
        lines = ["condition\tfile\tframes\tpeak_dbfs\trms_dbfs"]
        for index, condition in enumerate(conditions):
            body = struct.pack("<ff", index / 100.0, -index / 100.0) * 480
            (stimuli / f"{condition}.wav").write_bytes(wav_bytes(body))
            lines.append(f"{condition}\t{condition}.wav\t480\t0\t0")
        (stimuli / "manifest.tsv").write_text("\n".join(lines) + "\n")
        packet = root / "packet"
        prepare(stimuli, packet, 42, "0" * 40)
        _, trials, key = validate(packet)
        # Amendment 1: block P's 36 trials follow the 68 of M, F and R, balanced like them.
        blocks = [t["block"] for t in trials]
        assert blocks == ["M"] * 34 + ["F"] * 24 + ["R"] * 10 + ["P"] * 36, blocks
        assert all(t["contrast"].startswith("P-") == (t["sequence"] > 68) for t in key["trials"])
        # The public records never carry the assignment: no role words, no seed.
        public_text = b"".join(p.read_bytes() for p in (packet / "public").glob("*.json"))
        for word in (b"candidate", b"reference", b"seed", b"contrast"):
            assert word not in public_text, word
        # A perfect synthetic listener, written only inside this temporary directory.
        path = packet / "responses.jsonl"
        path.touch(mode=0o600)
        path.chmod(0o600)
        with open(path, "ab") as handle:
            for index, t in enumerate(key["trials"], start=1):
                row = {
                    "answer": "1" if t["candidate_first"] else "2", "attempt": 1, "confidence": None,
                    "observation": "", "reason": None, "schema_version": 1, "sequence": index,
                    "trial": t["sequence"], "utc": "2026-01-01T00:00:00Z", "valid": True,
                }
                handle.write(canonical(row))
        # The reveal record carries every block-P contrast and the P decision (run quietly here).
        with contextlib.redirect_stdout(io.StringIO()):
            reveal(packet)
        revealed = load_json(packet / "reveal.json")
        assert {n for n in revealed["contrasts"] if n.startswith("P-")} == {d[0] for d in DESIGN if d[1] == "P"}
        assert revealed["decisions"]["polarity"]["outcome"] == "heard", revealed["decisions"]
        # A listener who hears every difference: 5, 10 and 20 ms all detected, and 20 ms audibly
        # softens the chops, so the rules fall back to 10 ms; the 30 Hz zipper moves faders to 35.
        result = score(read_responses(path), key)
        assert result["controls_passed"] and result["decisions"]["mute_ms"] == 10
        assert result["decisions"]["fader_ms"] == 35
        # A listener at chance on every non-control contrast keeps the proposal.
        chance = {t["sequence"]: ("1" if t["candidate_first"] else "2") for t in key["trials"]}
        flipped = []
        for t in key["trials"]:
            control = [d for d in DESIGN if d[0] == t["contrast"]][0][5] == "control"
            answer = chance[t["sequence"]] if control else ("2" if chance[t["sequence"]] == "1" else "1")
            flipped.append({"answer": answer, "trial": t["sequence"], "valid": True})
        decisions = score(flipped, key)["decisions"]
        assert (decisions["mute_ms"], decisions["fader_ms"]) == (5, 20), decisions

        # Block P (Amendment 1). A synthetic listener who hears exactly `hears` (every other trial
        # answered wrong); it exists only in memory here.
        controls = {d[0] for d in DESIGN if d[5] == "control"}

        def listener(hears: set[str]) -> dict:
            rows = []
            for t in key["trials"]:
                right = "1" if t["candidate_first"] else "2"
                wrong = "2" if right == "1" else "1"
                rows.append({"answer": right if t["contrast"] in hears else wrong, "trial": t["sequence"], "valid": True})
            return score(rows, key)

        def polarity(result: dict) -> tuple:
            p = result["decisions"]["polarity"]
            assert p["values_changed"] == [], p
            shipped, decided = p["at_shipped_flip"], p["at_decided_flip"]
            return (p["outcome"], tuple(p["heard_as"]), shipped["flip"], decided["flip_ms"], decided["dip"], decided["click"])

        # The perfect listener hears the flip, its dip and its click at the shipped 20 ms flip.
        assert polarity(result) == ("heard", ("dip", "click"), "heard", 20, "heard", "heard"), polarity(result)
        # At chance on every non-control contrast: not heard. muteMs falls to 5, so the 10 ms flip's
        # shorter dip is not heard either and its stronger click is not assessed.
        chance = listener(controls)
        assert polarity(chance) == ("not heard", (), "not heard", 10, "not heard", "not assessed"), polarity(chance)
        # A missed P control makes P's non-detections inconclusive and leaves M, F and R decided.
        missed = listener(controls - {"P-bass-0-oob"})
        assert missed["controls_passed"] and not missed["polarity_controls_passed"]
        assert (missed["decisions"]["mute_ms"], missed["decisions"]["fader_ms"]) == (5, 20)
        assert polarity(missed) == ("inconclusive", (), "inconclusive", 10, "not assessed", "not assessed"), polarity(missed)
        # A missed M control leaves the values at the defaults and P decided on its own controls.
        missed = listener(controls - {"M-bass-0"})
        assert missed["decisions"]["status"] == "inconclusive" and missed["polarity_controls_passed"]
        assert polarity(missed) == ("not heard", (), "not heard", 20, "not heard", "not heard"), polarity(missed)
        # The dip alone, with muteMs moved to 20 by M: the 40 ms flip's longer dip is heard too and
        # its weaker click not heard.
        dip = listener(controls | {"M-bass-5", "M-bass-10", "P-bass-20-inband"})
        assert dip["decisions"]["mute_ms"] == 20
        assert polarity(dip) == ("heard", ("dip",), "not heard", 40, "heard", "not heard"), polarity(dip)
        # The click alone, with muteMs 5: the 10 ms flip's stronger click is heard too.
        click = listener(controls | {"P-bass-20-oob"})
        assert polarity(click) == ("heard", ("click",), "not heard", 10, "not heard", "heard"), polarity(click)
        # The flip alone, neither part: heard, cue not separated.
        flip = listener(controls | {"P-bass-20"})
        assert polarity(flip) == ("heard", (), "heard", 10, "not heard", "not assessed"), polarity(flip)
        # Block P never moves a value: the same listeners without any P detection decide the same.
        for hears in ({"M-bass-5", "M-bass-10"}, {"F-30-20"}, set()):
            with_p = listener(controls | hears | set(POLARITY_CUES.values()))["decisions"]
            without = listener(controls | hears)["decisions"]
            assert {k: v for k, v in with_p.items() if k != "polarity"} == {k: v for k, v in without.items() if k != "polarity"}
        validate(packet)
        target = packet / "public" / trials[0]["file"]
        target.chmod(0o644)
        target.write_bytes(b"tampered")
        target.chmod(0o444)
        try:
            validate(packet)
        except Invalid:
            pass
        else:
            raise AssertionError("tampering was not detected")
    print("self-test passed")


def main(argv: list[str]) -> int:
    def option(name: str) -> str | None:
        return argv[argv.index(name) + 1] if name in argv else None

    try:
        command = argv[1] if len(argv) > 1 else ""
        if command == "prepare":
            seed = option("--seed")
            prepare(
                Path(option("--stimuli") or ""),
                Path(option("--out") or ""),
                int(seed) if seed is not None else None,
                option("--commit") or "unrecorded",
            )
        elif command == "run":
            player = option("--player")
            run(Path(argv[2]), option("--block"), player.split() if player else None)
        elif command == "validate":
            validate(Path(argv[2]))
            print("packet valid")
        elif command == "reveal":
            reveal(Path(argv[2]))
        elif command == "self-test":
            self_test()
        else:
            print(__doc__)
            return 2
    except Invalid as error:
        print(f"listening packet error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
