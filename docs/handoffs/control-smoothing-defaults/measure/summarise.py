#!/usr/bin/env python3
"""Print the FINDINGS.md tables from the committed CSVs (standard library only).

    python3 summarise.py [DATA_DIR]      # default: ../data next to this script

Each cell is the 48 kHz value; a bracketed range is the spread over the four launch rates.
"""

from __future__ import annotations

import csv
import sys
from pathlib import Path

DATA = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "data"


def load(name: str) -> list[dict]:
    with open(DATA / name, newline="") as handle:
        return list(csv.DictReader(handle))


def cell(rows: list[dict], column: str, spread: bool = True) -> str:
    at48 = [r for r in rows if r["rate_hz"] == "48000"]
    if not at48 or at48[0][column] == "":
        return "n/a"
    value = at48[0][column]
    if not spread:
        return value
    values = [float(r[column]) for r in rows]
    low, high = min(values), max(values)
    return f"{value} [{low:.1f}, {high:.1f}]" if high - low >= 0.05 else value


def table(header: list[str], lines: list[list[str]]) -> None:
    print("| " + " | ".join(header) + " |")
    print("|" + "|".join("---:" for _ in header) + "|")
    for line in lines:
        print("| " + " | ".join(line) + " |")
    print()


def mute() -> None:
    rows = load("mute_click.csv")
    timing = load("mute_timing.csv")
    worst = 0.0
    for r in rows:
        if r["transition"] != "mute":
            continue
        twin = [
            u for u in rows
            if u["transition"] == "unmute"
            and (u["rate_hz"], u["material"], u["ramp_ms"]) == (r["rate_hz"], r["material"], r["ramp_ms"])
        ][0]
        worst = max(worst, abs(float(twin["splatter_db"]) - float(r["splatter_db"])))
    print(f"Unmute versus mute, largest splatter difference: {worst:.2f} dB\n")
    ramps = sorted({r["ramp_ms"] for r in rows}, key=float)
    pick = lambda material, ms: [
        r for r in rows if r["material"] == material and r["transition"] == "mute" and r["ramp_ms"] == ms
    ]
    lines = []
    for ms in ramps:
        bass, kick, mix = pick("bass", ms), pick("kick", ms), pick("mix", ms)
        t = [r for r in timing if r["ramp_ms"] == ms]
        lines.append([
            ms,
            cell(bass, "oob_db"),
            cell(bass, "ctr_max_db"),
            cell(kick, "oob_db"),
            cell(kick, "ctr_max_db"),
            cell(mix, "splatter_db"),
            cell(mix, "hf_splatter_db"),
            cell(t, "mute_to_minus20db_ms", False),
            cell(t, "unmute_to_minus3db_ms", False),
        ])
    table(
        ["ramp ms", "bass OOB dB", "bass CTR dB", "kick OOB dB", "kick CTR dB", "mix splatter dB",
         "mix HF splatter dB", "mute to -20 dB, ms", "unmute to -3 dB, ms"],
        lines,
    )
    bass = [r for r in rows if r["material"] == "bass" and r["transition"] == "mute"]
    print("Bass OOB floor (source alone): " + cell(bass, "oob_floor_db") + " dB; kick floor: "
          + cell([r for r in rows if r["material"] == "kick"], "oob_floor_db") + " dB\n")


def drag() -> None:
    rows = load("fader_drag.csv")
    for material in ("bass", "mix"):
        for move in ("800", "200"):
            print(f"Fader, {material}, move {move} ms each way (0 -> -24 -> 0 dB)\n")
            lines = []
            ramps = sorted({r["ramp_ms"] for r in rows}, key=float)
            for ms in ramps:
                line = [ms]
                for hz in ("30", "60"):
                    sel = [
                        r for r in rows
                        if (r["material"], r["move_ms"], r["ramp_ms"], r["update_hz"]) == (material, move, ms, hz)
                    ]
                    if material == "bass":
                        line += [cell(sel, "oob_db"), cell(sel, "ctr_max_db")]
                    else:
                        line += [cell(sel, "hf_splatter_db")]
                    line += [cell(sel, "splatter_db"), cell(sel, "ripple_rms_db", False),
                             cell(sel, "ripple_peak_db", False), cell(sel, "added_lag_ms", False),
                             cell(sel, "settle_0p5db_ms", False)]
                lines.append(line)
            ideal = [r for r in rows if (r["material"], r["move_ms"], r["update_hz"]) == (material, move, "30")]
            per = (["OOB", "CTR"] if material == "bass" else ["HF splatter"]) + [
                "splatter", "ripple rms", "ripple peak", "added lag ms", "settle 0.5 dB ms"]
            header = ["ramp ms"] + [f"{hz} Hz {name}" for hz in ("30", "60") for name in per]
            table(header, lines)
            ideal_cols = ["ideal_splatter_db"] + (["ideal_oob_db", "ideal_ctr_max_db"] if material == "bass" else ["ideal_hf_splatter_db"])
            print("Continuous (per-sample) drag, the floor: " + ", ".join(
                f"{c.removeprefix('ideal_')} {cell(ideal, c)}" for c in ideal_cols) + "\n")


def pan() -> None:
    rows = load("pan_drag.csv")
    lines = []
    for ms in sorted({r["ramp_ms"] for r in rows}, key=float):
        line = [ms]
        for hz in ("30", "60"):
            sel = [r for r in rows if (r["move_ms"], r["ramp_ms"], r["update_hz"]) == ("800", ms, hz)]
            line += [cell(sel, "oob_db"), cell(sel, "splatter_db"), cell(sel, "gain_error_rms_db", False),
                     cell(sel, "added_lag_ms", False), cell(sel, "power_dev_max_db", False)]
        lines.append(line)
    header = ["ramp ms"] + [f"{hz} Hz {n}" for hz in ("30", "60")
                            for n in ("OOB", "splatter", "gain error rms", "added lag ms", "power dev dB")]
    print("Pan, bass in the left lane, -0.5 -> +0.5 -> -0.5 over 800 ms each way\n")
    table(header, lines)
    ideal = [r for r in rows if (r["move_ms"], r["update_hz"]) == ("800", "30")]
    print(f"Continuous pan floor: splatter {cell(ideal, 'ideal_splatter_db')}, OOB {cell(ideal, 'ideal_oob_db')}\n")
    jumps = load("pan_jump.csv")
    lines = []
    for (a, b) in (("-1", "1"), ("0", "-1"), ("-0.5", "0.5")):
        sel = [r for r in jumps if (r["from"], r["to"], r["rate_hz"]) == (a, b, "48000")]
        lines.append([f"{a} -> {b}"] + [f"{r['power_min_db']} / {r['time_below_minus1db_ms']}" for r in sel])
    header = ["pan jump"] + [f"{r['ramp_ms']} ms" for r in jumps if (r["from"], r["to"], r["rate_hz"]) == ("-1", "1", "48000")]
    print("Pan jump in one record: minimum power dB / time below -1 dB (ms), 48 kHz\n")
    table(header, lines)


def whatif() -> None:
    rows = load("mute_shape_whatif.csv")
    lines = []
    for ms in sorted({r["ramp_ms"] for r in rows}, key=float):
        line = [ms]
        for material in ("bass", "kick"):
            for shape in ("engine-linear", "raised-cosine"):
                r = [x for x in rows if (x["material"], x["ramp_ms"], x["shape"]) == (material, ms, shape)][0]
                line.append(f"{r['oob_db']} / {r['ctr_max_db']}")
        lines.append(line)
    print("What-if (not the engine): mute OOB dB / CTR dB, 48 kHz\n")
    table(["ramp ms", "bass linear (engine)", "bass raised-cosine", "kick linear (engine)",
           "kick raised-cosine"], lines)


if __name__ == "__main__":
    print(f"Data: {DATA}\n")
    mute()
    drag()
    pan()
    whatif()
