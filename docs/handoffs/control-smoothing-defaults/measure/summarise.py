#!/usr/bin/env python3
"""Print the FINDINGS.md tables from the committed CSVs (standard library only).

    python3 summarise.py [DATA_DIR]      # default: ../data next to this script

Each cell is the 48 kHz value; a bracketed range is the spread over the four launch rates.
Sections 5-6 come from `measure`'s seven files; section 9 from `live`'s four, when present.
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


def pick(rows: list[dict], **match: str) -> list[dict]:
    return [r for r in rows if all(r[k] == v for k, v in match.items())]


def polarity() -> None:
    rows = load("polarity_click.csv")
    mute = load("mute_click.csv")
    worst = max(
        abs(float(r["oob_db"] or 0) - float(u["oob_db"] or 0))
        for r in rows if r["transition"] == "invert"
        for u in pick(rows, rate_hz=r["rate_hz"], material=r["material"], transition="restore",
                      ramp_ms=r["ramp_ms"])
    )
    print(f"Polarity: restore versus invert, largest out-of-band difference: {worst:.2f} dB\n")
    lines = []
    for ms in sorted({r["ramp_ms"] for r in rows}, key=float):
        flip = lambda material: pick(rows, material=material, transition="invert", ramp_ms=ms)
        line = [ms, cell(flip("bass"), "oob_db"), cell(flip("bass"), "ctr_max_db"),
                cell(flip("kick"), "oob_db"), cell(flip("kick"), "ctr_max_db"),
                cell(flip("mix"), "hf_splatter_db")]
        same = pick(mute, rate_hz="48000", material="bass", transition="mute", ramp_ms=ms)
        half = pick(mute, rate_hz="48000", material="bass", transition="mute",
                    ramp_ms=f"{float(ms) / 2:g}")
        b = pick(flip("bass"), rate_hz="48000")[0]
        line.append(f"{float(b['oob_db']) - float(same[0]['oob_db']):+.2f}" if same else "n/a")
        line.append(f"{float(b['oob_db']) - float(half[0]['oob_db']):+.2f} / "
                    f"{float(b['ctr_max_db']) - float(half[0]['ctr_max_db']):+.2f}" if half else "n/a")
        lines.append(line)
    print("Polarity flip (invert), 48 kHz\n")
    table(["ramp ms", "bass OOB", "bass CTR", "kick OOB", "kick CTR", "mix HF splatter",
           "bass OOB minus mute at same length", "bass OOB / CTR minus mute at half length"], lines)


def transfer() -> None:
    rows = load("law_transfer.csv")
    for law in ("route-indexed", "input-trim"):
        sel = pick(rows, law=law)
        diff = max(float(r["max_abs_diff"]) for r in sel)
        settle = all(r["both_settle_exactly"] == "true" for r in sel)
        measured = [r for r in sel if r["d11_bass_oob_db"]]
        oob = max(abs(float(r["d11_bass_oob_db"]) - float(r["law_bass_oob_db"])) for r in measured)
        ctr = max(abs(float(r["d11_bass_ctr_max_db"]) - float(r["law_bass_ctr_max_db"]))
                  for r in measured)
        print(f"{law}: {len(sel)} rows, largest |coefficient difference| {diff:.3e}, "
              f"all settle on the same bits: {settle}; bass OOB within {oob:.2f} dB, "
              f"CTR within {ctr:.2f} dB")
        for case in sorted({r["case"] for r in sel}):
            print(f"  {case}: largest difference "
                  f"{max(float(r['max_abs_diff']) for r in pick(sel, case=case)):.3e}")
    print()


def crossfade_cells(rows: list[dict], material: str) -> list[str]:
    if material in ("mix", "mix-wide"):
        return [cell(rows, "hf_splatter_db")]
    return [cell(rows, "oob_db"), cell(rows, "ctr_max_db")]


def bypass() -> None:
    rows = load("bypass_crossfade.csv")
    ramps = sorted({r["ramp_ms"] for r in rows}, key=float)
    for material in ("bass", "kick", "mix"):
        lines = []
        effects = []
        for r in rows:
            if r["material"] == material and r["effect"] not in effects:
                effects.append(r["effect"])
        for effect in effects:
            sel = pick(rows, effect=effect, material=material, transition="bypass")
            line = [effect, cell(pick(sel, ramp_ms=ramps[0]), "step_db")]
            for ms in ramps:
                line.append(" / ".join(crossfade_cells(pick(sel, ramp_ms=ms), material)))
            lines.append(line)
        measure = "HF splatter dB" if material == "mix" else "OOB / CTR dB"
        print(f"Bypass crossfade, {material}, {measure} (unbypass mirrors bypass)\n")
        table(["effect", "step dB"] + [f"{ms} ms" for ms in ramps], lines)
    worst = max(
        abs(float(r["oob_db"] or r["hf_splatter_db"]) - float(u["oob_db"] or u["hf_splatter_db"]))
        for r in rows if r["transition"] == "bypass"
        for u in pick(rows, rate_hz=r["rate_hz"], effect=r["effect"], material=r["material"],
                      transition="unbypass", ramp_ms=r["ramp_ms"])
    )
    print(f"Unbypass versus bypass, largest click difference: {worst:.2f} dB\n")
    # Against the mute reference at the same rate, material and length: the largest excess of
    # an effect's click (OOB and CTR on bass and kick, HF splatter on the mix), and on the bass
    # and kick, how closely the click is the mute's plus the step energy.
    excess = (float("-inf"), "")
    scale = []
    for r in rows:
        if r["effect"] == "mute-reference" or r["transition"] != "bypass":
            continue
        ref = pick(rows, rate_hz=r["rate_hz"], effect="mute-reference", material=r["material"],
                   transition="bypass", ramp_ms=r["ramp_ms"])[0]
        columns = ["oob_db", "ctr_max_db"] if r["oob_db"] else ["hf_splatter_db"]
        for column in columns:
            over = float(r[column]) - float(ref[column])
            if over > excess[0]:
                where = f"{r['effect']} {r['material']} {r['rate_hz']} Hz {r['ramp_ms']} ms {column}"
                excess = (over, where)
        if r["oob_db"] and float(r["step_db"]) > -40.0 and r["ramp_ms"] != "0":
            scale.append(float(r["oob_db"]) - float(ref["oob_db"]) - float(r["step_db"]))
    print(f"Largest excess of an effect's click over the mute reference's at the same length: "
          f"{excess[0]:+.2f} dB ({excess[1]})")
    print(f"Bass and kick, 2-20 ms, step above -40 dB: OOB minus (mute OOB + step) from "
          f"{min(scale):+.2f} to {max(scale):+.2f} dB\n")


def link() -> None:
    rows = load("link_glide.csv")
    ramps = sorted({r["ramp_ms"] for r in rows}, key=float)
    lines = []
    for material in ("mix", "mix-wide", "bass-kick"):
        sel = pick(rows, material=material, transition="link")
        r48 = pick(sel, rate_hz="48000")[0]
        line = [material, r48["channel_correlation"], cell(pick(sel, ramp_ms=ramps[0]), "step_db")]
        for ms in ramps:
            line.append(" / ".join(crossfade_cells(pick(sel, ramp_ms=ms), material)))
        lines.append(line)
    print("Link glide dual_mono -> maximum (unlink mirrors link): HF splatter dB (mix), "
          "OOB / CTR dB (bass-kick)\n")
    table(["material", "correlation", "step dB"] + [f"{ms} ms" for ms in ramps], lines)


if __name__ == "__main__":
    print(f"Data: {DATA}\n")
    mute()
    drag()
    pan()
    whatif()
    if (DATA / "polarity_click.csv").exists():
        polarity()
        transfer()
        bypass()
        link()
