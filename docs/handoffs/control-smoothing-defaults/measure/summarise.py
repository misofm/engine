#!/usr/bin/env python3
"""Print the FINDINGS.md tables from the committed CSVs (standard library only).

    python3 summarise.py [DATA_DIR]      # default: ../data next to this script

Each cell is the 48 kHz value; a bracketed range is the spread over the four launch rates.
Sections 5-6 come from `measure`'s seven files; section 9 from `live`'s four, when present.
"""

from __future__ import annotations

import csv
import math
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


CLICK = ("oob_db", "oob_worst_db", "ctr_max_db", "hf_splatter_db")
TWIN = {"invert": "mute", "restore": "unmute"}


def span(values: list[float]) -> str:
    return f"{min(values):+.2f} to {max(values):+.2f}"


def flip_minus_mute(rows: list[dict], mute: list[dict], flip_ms: str, mute_ms: str, column: str,
                    material: str | None = None) -> list[float]:
    """The flip at `flip_ms` minus the mute at `mute_ms`, per rate, material and transition
    (invert against mute, restore against unmute)."""
    out = []
    for r in pick(rows, ramp_ms=flip_ms):
        if (material and r["material"] != material) or r[column] == "":
            continue
        for u in pick(mute, rate_hz=r["rate_hz"], material=r["material"],
                      transition=TWIN[r["transition"]], ramp_ms=mute_ms):
            out.append(float(r[column]) - float(u[column]))
    return out


def flip_model_db(seconds: float, edge_hz: float) -> float:
    """FINDINGS 9.2's one-partial model: the energy a polarity flip over `2 * seconds` moves more
    than `edge_hz` from a partial, against a mute over `seconds`, in dB. The flip over 2N is two
    mutes over N back to back, so its gain spectrum is the mute's, `sinc(f T) / (2 pi f)`, times
    `2 cos(pi f T)`. Midpoint rule, 0.25 Hz steps to 20 kHz."""
    num = den = 0.0
    step = 0.25
    f = edge_hz + step / 2
    while f < 20_000.0:
        x = math.pi * f * seconds
        s = (math.sin(x) / x) ** 2 / (f * f)
        den += s
        num += 4.0 * math.cos(x) ** 2 * s
        f += step
    return 10.0 * math.log10(num / den)


def polarity() -> None:
    rows = load("polarity_click.csv")
    mute = load("mute_click.csv")
    worst = max(
        abs(float(r[c]) - float(u[c]))
        for r in rows if r["transition"] == "invert"
        for u in pick(rows, rate_hz=r["rate_hz"], material=r["material"], transition="restore",
                      ramp_ms=r["ramp_ms"])
        for c in CLICK + ("splatter_db",) if r[c] != ""
    )
    print(f"Polarity: restore versus invert, largest difference on any measure: {worst:.2f} dB\n")
    ramps = sorted({r["ramp_ms"] for r in rows}, key=float)
    lines = []
    for ms in ramps:
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
    same = ", ".join(
        f"{c} {span([d for ms in ramps for d in flip_minus_mute(rows, mute, ms, ms, c)])}"
        for c in CLICK + ("splatter_db",)
    )
    print(f"Flip minus mute at the same length (every length, rate, material, transition): {same}\n")
    lines = []
    for ms in ramps:
        half = f"{float(ms) / 2:g}"
        if ms == "0" or not pick(mute, ramp_ms=half):
            continue
        line = [f"{ms} / {half}"]
        line += [span(flip_minus_mute(rows, mute, ms, half, c)) for c in CLICK]
        line += [span(flip_minus_mute(rows, mute, ms, half, "splatter_db", m))
                 for m in ("bass", "kick", "mix")]
        lines.append(line)
    print("Flip at twice the mute's length minus the mute, over the four rates, every material and "
          "both transitions (dB)\n")
    table(["flip / mute ms", "OOB", "worst OOB", "CTR", "HF splatter", "splatter, bass",
           "splatter, kick", "splatter, mix"], lines)
    lines = []
    for ms in ("1", "5", "10", "20"):
        t = float(ms) / 1000.0
        near = [flip_model_db(t, float(edge)) for edge in range(56, 83, 2)]
        lines.append([f"{2 * float(ms):g} / {ms}", span(near), f"{flip_model_db(t, 510.0):+.2f}"])
    print("One-partial model (`flip_model_db`), flip minus mute: edge 56-82 Hz (the splatter "
          "edge of the bass's partials below 600 Hz) and 510 Hz (its top partial to the 1.5 kHz "
          "out-of-band edge)\n")
    table(["flip / mute ms", "edge 56-82 Hz", "edge 510 Hz"], lines)


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
    # Against the mute reference at the same rate, material, transition and length: the largest
    # excess of an effect's click on each measure, and on the bass and kick, how closely the
    # click is the mute's plus the step energy.
    excess: dict[str, list] = {c: [float("-inf"), set()] for c in ("splatter_db",) + CLICK}
    scale = []
    for r in rows:
        if r["effect"] == "mute-reference":
            continue
        ref = pick(rows, rate_hz=r["rate_hz"], effect="mute-reference", material=r["material"],
                   transition=r["transition"], ramp_ms=r["ramp_ms"])[0]
        for column, best in excess.items():
            if r[column] == "":
                continue
            over = round(float(r[column]) - float(ref[column]), 2)
            where = (r["effect"], r["material"], int(r["rate_hz"]), float(r["ramp_ms"]))
            if over > best[0]:
                best[0], best[1] = over, {where}
            elif over == best[0]:
                best[1].add(where)
        if r["oob_db"] and float(r["step_db"]) > -40.0 and r["ramp_ms"] != "0":
            scale.append(float(r["oob_db"]) - float(ref["oob_db"]) - float(r["step_db"]))
    print("Largest excess of an effect's click over the mute reference's at the same rate, "
          "material, transition and length, per measure (every tie listed):")
    for column, (over, where) in excess.items():
        cases = "; ".join(f"{e} {m} {rate} Hz {ms:g} ms" for e, m, rate, ms in sorted(where))
        print(f"  {column}: {over:+.2f} dB ({cases})")
    print(f"Bass and kick, 2-20 ms, step above -40 dB: OOB minus (mute OOB + step) from "
          f"{min(scale):+.2f} to {max(scale):+.2f} dB\n")
    # Today's step (0 ms) minus the crossfade, OOB on bass and kick, HF splatter on the mix; the
    # transient shaper on the kick (step about -60 dB, switches in the body) apart.
    for ms in ("10", "20"):
        for rate in ("all", "44100", "48000", "88200", "96000"):
            oob, hf, shaper = [], [], []
            for r in pick(rows, ramp_ms="0"):
                if r["effect"] == "mute-reference" or rate not in ("all", r["rate_hz"]):
                    continue
                late = pick(rows, rate_hz=r["rate_hz"], effect=r["effect"], material=r["material"],
                            transition=r["transition"], ramp_ms=ms)[0]
                if r["effect"] == "transient-shaper" and r["material"] == "kick":
                    shaper.append(float(r["oob_db"]) - float(late["oob_db"]))
                elif r["oob_db"]:
                    oob.append(float(r["oob_db"]) - float(late["oob_db"]))
                elif r["material"] == "mix":
                    hf.append(float(r["hf_splatter_db"]) - float(late["hf_splatter_db"]))
            print(f"Today's step minus the {ms} ms crossfade, {rate}: OOB {min(oob):.1f} to "
                  f"{max(oob):.1f} dB, mix HF splatter {min(hf):.1f} to {max(hf):.1f} dB; transient "
                  f"shaper on the kick, OOB {min(shaper):.1f} to {max(shaper):.1f} dB")
    print()
    # The mute-reference rows against mute_click.csv (bypass with mute, unbypass with unmute).
    mute = load("mute_click.csv")
    twin = {"bypass": "mute", "unbypass": "unmute"}
    for column in ("splatter_db",) + CLICK:
        worst = (0.0, "")
        for r in pick(rows, effect="mute-reference"):
            if r[column] == "":
                continue
            u = pick(mute, rate_hz=r["rate_hz"], material=r["material"],
                     transition=twin[r["transition"]], ramp_ms=r["ramp_ms"])[0]
            gap = abs(float(r[column]) - float(u[column]))
            if gap > worst[0]:
                worst = (gap, f"{r['rate_hz']} Hz {r['material']} {r['transition']} "
                              f"{r['ramp_ms']} ms: {r[column]} against {u[column]}")
        print(f"mute-reference against mute_click.csv, {column}: within {worst[0]:.2f} dB "
              f"({worst[1]})")
    print()


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
    # The margin at the defaults: the 20 ms glide under the 10 ms mute (mute_click.csv), on the
    # bass for bass-kick and on the mix for the two mixes, at each rate.
    mute = load("mute_click.csv")
    reads = {"bass-kick": ("bass", ("oob_db", "oob_worst_db", "ctr_max_db")),
             "mix": ("mix", ("hf_splatter_db", "splatter_db")),
             "mix-wide": ("mix", ("hf_splatter_db", "splatter_db"))}
    for material, (reference, columns) in reads.items():
        for column in columns:
            margins = []
            for r in pick(rows, material=material, ramp_ms="20"):
                u = pick(mute, rate_hz=r["rate_hz"], material=reference, transition="mute",
                         ramp_ms="10")[0]
                margins.append(float(u[column]) - float(r[column]))
            print(f"20 ms glide under the 10 ms mute ({reference}), {material} {column}: "
                  f"{min(margins):.1f} to {max(margins):.1f} dB")
    print()


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
