#!/usr/bin/env python3
"""Schema gate for the shipped browser parameter metadata (issue #137 D4/E7).

The generator and this validator are deliberately two implementations. The generator walks
`NativeEffectRegistry::descriptors` in Rust; this walks the emitted JSON in Python and knows only
what the schema promises. A generator that starts emitting a field the schema does not describe, or
stops emitting one it does, fails here rather than in an app three repositories away.

`--self-test` runs every rule against a valid document and against its own red mutation, so the
validator itself is proved to discriminate.
"""

from __future__ import annotations

import argparse
import copy
import json
import math
import pathlib
import re
import sys
from decimal import Decimal

SCHEMA = "miso.web.parameter-metadata.v1"
ABI_VERSION = 0x0002_0000

UNITS = {1: "db", 2: "hz", 3: "milliseconds", 4: "samples", 5: "linear", 6: "ratio"}
DOMAINS = {1: "continuous", 2: "boolean", 3: "enumeration"}
MAPPINGS = {1: "linear", 2: "logarithmic", 3: "exponential", 4: "stepped"}
RATES = {1: "sample", 2: "block", 3: "none"}
POLICIES = {1: "shared", 2: "perLane"}
SMOOTHINGS = {1: "none", 2: "linear", 3: "onePole99"}

COMMAND_KINDS = [
    "pan", "matrix", "faderDb", "mute", "effectParam", "effectBypass",
    "observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert",
]
# The two host-level transaction kinds. They are applied -- `admit_commands` binds or unbinds the
# tap and acknowledges `none` -- but what they apply to is the `miso.observe.v1` subscription map,
# not anything the render thread reads. `plane` is what tells the render kinds apart from these
# two. Spelled here rather than imported on purpose, and
# `scripts/check-command-kind-vocabulary.py` is what proves this list, the Rust constants, the
# decode whitelist, the host JS set, the `.d.ts` enum and the generator are one vocabulary.
OBSERVE_COMMAND_KINDS = ["observeSubscribe", "observeUnsubscribe"]
COMMAND_KIND_KEYS = {"value", "name", "applied", "plane"}
PLANE_RENDER = "render"
PLANE_OBSERVATION = "observation"
# Issue #143 added `unknownTap` (10) and `observationUnbound` (11); issue #151 found that this
# list, the host JS acknowledgement bound and the generator all still stopped at `wrongState`.
# The list is spelled here rather than imported on purpose -- this validator is deliberately a
# second implementation -- and `scripts/check-command-reason-vocabulary.py` is what proves the two
# spellings, the Rust constants, the host JS table and the `.d.ts` enum are one vocabulary.
COMMAND_REASONS = [
    "none", "malformed", "unknownTrack", "unknownRack", "unknownEffect", "unknownParameter",
    "domain", "unsupportedKind", "backpressure", "wrongState", "unknownTap", "observationUnbound",
]
BUILTIN_UPDATE_RATES = {"preparedOnly", "blockTarget"}
BUILTIN_SCOPES = {"perLane", "matrixShared"}
BUILTIN_MAPPINGS = {"boolean", "decibelAmplitude", "hertz", "linear"}
BUILTIN_DOMAINS = {"booleanExact", "finiteInclusive", "disabledOrRateKeyedHertz"}
BUILTIN_RESETS = {"restorePreparedValue", "keepTargetResetCurrent"}
BUILTIN_SMOOTHINGS = {"none", "linearNUpdates"}

EFFECT_PARAMETER_KEYS = {
    "id", "name", "displayUnit", "unit", "unitName", "domain", "domainName", "minimum", "maximum",
    "default", "mapping", "mappingName", "automationRate", "automationRateName", "channelPolicy",
    "channelPolicyName", "smoothing", "smoothingName", "smoothingSamples", "readable",
    "automatable", "liveUpdatable", "enumChoices", "step",
}
EFFECT_OBSERVATION_KEYS = {
    "id", "name", "displayUnit", "kind", "kindName", "unit", "unitName", "cost", "costName",
    "cadence", "cadenceName", "fold", "foldName", "channels", "channelsName", "minimum", "maximum",
    "subscribable",
}
OBSERVATION_KINDS = {1: "gainReductionDb"}
OBSERVATION_COSTS = {1: "resident", 2: "computed"}
OBSERVATION_CADENCES = {1: "perBlock", 2: "perWindow"}
OBSERVATION_FOLDS = {1: "latest", 2: "peakMagnitude"}
OBSERVATION_CHANNELS = {1: "shared", 2: "perLane"}
OBSERVATION_VOCABULARIES = {
    "kinds": OBSERVATION_KINDS,
    "costs": OBSERVATION_COSTS,
    "cadences": OBSERVATION_CADENCES,
    "folds": OBSERVATION_FOLDS,
    "channels": OBSERVATION_CHANNELS,
}

BUILTIN_PARAMETER_KEYS = {
    "id", "name", "scope", "mapping", "domain", "minimum", "maximum", "maximumByRate", "default",
    "updateRate", "smoothing", "reset", "disabledValue", "liveUpdatable", "step",
}

# Issue #242: the populated lattice declaration that replaced #127's `"nudge": null` slot.
STEP_KEYS = {"unit", "size", "precision", "ladder"}
STEP_UNITS = {"absolute", "cents", "ratio", "index"}
LADDER_SIZES = ["xs", "sm", "md", "lg", "xl"]


def decimal_string(text, what):
    """Return `text` as a Decimal, proving it is an exact decimal literal.

    The step contract crosses the agent boundary as decimal text precisely so no reader has to
    round-trip it through a float; a float here would defeat the point of the slot.
    """
    require(isinstance(text, str) and text != "", f"{what} is a decimal string")
    require(
        re.fullmatch(r"-?(0|[1-9][0-9]*)(\.[0-9]+)?", text) is not None,
        f"{what} is an exact decimal literal",
    )
    return Decimal(text)


def validate_step(step, what):
    """Validate one parameter's `step` object against the #242 lattice contract."""
    require(isinstance(step, dict), f"{what} step is an object")
    require(set(step) == STEP_KEYS, f"{what} step keys")
    require(step["unit"] in STEP_UNITS, f"{what} step unit vocabulary")
    size = decimal_string(step["size"], f"{what} step size")
    require(size > 0, f"{what} step size is positive")
    precision = step["precision"]
    require(
        isinstance(precision, int) and not isinstance(precision, bool) and 0 <= precision <= 8,
        f"{what} step precision",
    )
    if step["unit"] == "ratio":
        # A geometric row's step IS the ratio; at or below 1 it would never advance.
        require(size > 1, f"{what} geometric step ratio exceeds one")
    if step["unit"] == "index":
        require(size == 1 and precision == 0, f"{what} index step is a unit ordinal")
    ladder = step["ladder"]
    require(isinstance(ladder, dict), f"{what} ladder is an object")
    require(list(ladder) == LADDER_SIZES, f"{what} ladder sizes in order")
    multiples = [ladder[size_name] for size_name in LADDER_SIZES]
    require(
        all(
            isinstance(multiple, int) and not isinstance(multiple, bool)
            for multiple in multiples
        ),
        f"{what} ladder multiples are integers",
    )
    # The ladder is defined as INTEGER MULTIPLES of the step, so it can never leave the lattice.
    require(multiples[0] >= 1, f"{what} smallest ladder multiple is at least one step")
    require(
        all(low < high for low, high in zip(multiples, multiples[1:])),
        f"{what} ladder multiples ascend",
    )
    # The frozen descriptor record packs xs..lg in five bits and xl in six.
    require(all(multiple <= 31 for multiple in multiples[:4]), f"{what} ladder wire width")
    require(multiples[4] <= 63, f"{what} extra-large ladder wire width")


class Invalid(Exception):
    """One broken schema rule, named by the rule it broke."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def finite(value: object, message: str) -> float:
    require(isinstance(value, (int, float)) and not isinstance(value, bool), message)
    require(math.isfinite(float(value)), message)
    return float(value)


def validate(document: dict) -> None:
    require(set(document) == {
        "schema", "abiVersion", "commandRecordBytes", "maximumCommandRecords", "commandKinds",
        "commandReasons", "observationVocabularies", "builtins", "effects",
    }, "top-level keys")
    require(document["schema"] == SCHEMA, "schema tag")
    require(document["abiVersion"] == ABI_VERSION, "abi version")
    require(document["commandRecordBytes"] == 48, "command record bytes")
    require(document["maximumCommandRecords"] >= 1, "maximum command records")

    kinds = document["commandKinds"]
    require(all(set(kind) == COMMAND_KIND_KEYS for kind in kinds), "command kind keys")
    require([kind["name"] for kind in kinds] == COMMAND_KINDS, "command kinds")
    # Derived, never a literal: this list used to read `range(1, 7)` while the wire decoded eight
    # kinds, which is precisely the drift that went unnoticed for two releases.
    require(
        [kind["value"] for kind in kinds] == list(range(1, len(COMMAND_KINDS) + 1)),
        "command kind values",
    )
    applied = {kind["name"] for kind in kinds if kind["applied"]}
    # Issue #140: every declared kind is applied. Nothing in the ABI is declared-and-refused any
    # more, so a kind that reports `applied: false` is a regression, not a documented gap.
    require(applied == set(COMMAND_KINDS), "applied command kinds")
    # What "applied" means splits by plane, and the split is exact in both directions: the six DSP
    # kinds move state the render thread reads, the two observation kinds move the
    # `miso.observe.v1` subscription map and nothing else. A DSP kind that reports the observation
    # plane would be claiming it renders nothing; an observation kind that reports the render plane
    # would put a subscribe on the applied-DSP list the #140 rule above stands on.
    require(
        all(kind["plane"] in (PLANE_RENDER, PLANE_OBSERVATION) for kind in kinds),
        "command kind planes",
    )
    observation_plane = [kind["name"] for kind in kinds if kind["plane"] == PLANE_OBSERVATION]
    require(observation_plane == OBSERVE_COMMAND_KINDS, "observation-plane command kinds")
    render_plane = [kind["name"] for kind in kinds if kind["plane"] == PLANE_RENDER]
    require(
        render_plane == [name for name in COMMAND_KINDS if name not in OBSERVE_COMMAND_KINDS],
        "render-plane command kinds",
    )
    reasons = document["commandReasons"]
    require([reason["name"] for reason in reasons] == COMMAND_REASONS, "command reasons")
    require(
        [reason["value"] for reason in reasons] == list(range(len(COMMAND_REASONS))),
        "command reason values",
    )

    vocabularies = document["observationVocabularies"]
    require(set(vocabularies) == set(OBSERVATION_VOCABULARIES), "observation vocabulary keys")
    for name, expected in OBSERVATION_VOCABULARIES.items():
        rows = vocabularies[name]
        require(
            {row["value"]: row["name"] for row in rows} == expected,
            f"observation vocabulary {name}",
        )
        require(len(rows) == len(expected), f"observation vocabulary {name} membership")

    builtins = document["builtins"]
    require(set(builtins) == {"parameters"}, "builtin keys")
    seen_ids: set[int] = set()
    live_names: set[str] = set()
    for parameter in builtins["parameters"]:
        require(set(parameter) == BUILTIN_PARAMETER_KEYS, "builtin parameter keys")
        require(isinstance(parameter["id"], int) and parameter["id"] >= 1, "builtin id")
        require(parameter["id"] not in seen_ids, "builtin id uniqueness")
        seen_ids.add(parameter["id"])
        require(isinstance(parameter["name"], str) and parameter["name"], "builtin name")
        require(parameter["scope"] in BUILTIN_SCOPES, "builtin scope")
        require(parameter["mapping"] in BUILTIN_MAPPINGS, "builtin mapping")
        require(parameter["domain"] in BUILTIN_DOMAINS, "builtin domain")
        require(parameter["updateRate"] in BUILTIN_UPDATE_RATES, "builtin update rate")
        require(parameter["smoothing"] in BUILTIN_SMOOTHINGS, "builtin smoothing")
        require(parameter["reset"] in BUILTIN_RESETS, "builtin reset")
        require(isinstance(parameter["liveUpdatable"], bool), "builtin liveUpdatable")
        # The one rule that ties the metadata to the ABI: a parameter is live in the command path
        # exactly when the builtin contract declares it a block target. Nothing else may claim it.
        require(
            parameter["liveUpdatable"] == (parameter["updateRate"] == "blockTarget"),
            "builtin liveUpdatable follows updateRate",
        )
        if parameter["liveUpdatable"]:
            live_names.add(parameter["name"])
        finite(parameter["default"], "builtin default")
        if parameter["domain"] == "finiteInclusive":
            low = finite(parameter["minimum"], "builtin minimum")
            high = finite(parameter["maximum"], "builtin maximum")
            require(low < high, "builtin range order")
            require(low <= parameter["default"] <= high, "builtin default in range")
            require(parameter["maximumByRate"] is None, "builtin fixed range has no rate table")
        if parameter["domain"] == "disabledOrRateKeyedHertz":
            require(isinstance(parameter["maximumByRate"], dict), "builtin rate table")
            require(
                sorted(parameter["maximumByRate"]) == ["44100", "48000", "88200", "96000"],
                "builtin rate table rates",
            )
            for value in parameter["maximumByRate"].values():
                finite(value, "builtin rate table value")
            finite(parameter["disabledValue"], "builtin disabled value")
        validate_step(parameter["step"], "builtin")
    # The live set is pinned by name, not merely counted: a row that silently flipped its
    # `updateRate` would otherwise pass every other rule in this function. Issue #210 phase 3 added
    # `trim_db` and `polarity_invert` -- one coefficient, one ramp, two parameters -- and the two
    # are named here in the same change that flipped their descriptor rows. `hpf_hz`, `lpf_hz` and
    # `delay_samples` are deliberately absent and must stay absent until the ruling that defers
    # them is reopened. #239 ruling 5461507633 B4 appended `pan` as an authoritative persisted
    # descriptor row; it is a block target like the matrix rows it derives coefficients for, so it
    # joins the live set in the same change that added it.
    require(
        live_names
        == {
            "trim_db",
            "polarity_invert",
            "fader_db",
            "mute",
            "matrix_ll",
            "matrix_lr",
            "matrix_rl",
            "matrix_rr",
            "pan",
        },
        "exactly the trim, polarity, fader, mute, matrix and pan parameters are live",
    )

    require(document["effects"], "at least one effect")
    effect_ids = [effect["id"] for effect in document["effects"]]
    require(effect_ids == sorted(effect_ids), "effects in stable id order")
    require(len(set(effect_ids)) == len(effect_ids), "effect id uniqueness")
    for effect in document["effects"]:
        require(set(effect) == {
            "id", "displayName", "contractMajor", "contractMinor", "stateLayoutVersion",
            "parameters", "observations",
        }, "effect keys")
        require(isinstance(effect["id"], str) and effect["id"], "effect id")
        require(effect["contractMajor"] == 1, "effect contract major")
        require(effect["stateLayoutVersion"] >= 1, "effect state layout version")
        require(effect["parameters"], "effect has parameters")
        ids = [parameter["id"] for parameter in effect["parameters"]]
        require(ids == sorted(ids), "parameter ids ascend")
        require(len(set(ids)) == len(ids), "parameter id uniqueness")
        require(all(value >= 1 for value in ids), "parameter ids are nonzero")
        for parameter in effect["parameters"]:
            validate_effect_parameter(parameter)
        # Issue #143: never absent, and possibly empty. A tap menu is ascending and nonzero for the
        # same reason a parameter table is: the id is the addressing authority.
        observations = effect["observations"]
        require(isinstance(observations, list), "effect observations is a list")
        tap_ids = [observation["id"] for observation in observations]
        require(tap_ids == sorted(tap_ids), "observation ids ascend")
        require(len(set(tap_ids)) == len(tap_ids), "observation id uniqueness")
        require(all(value >= 1 for value in tap_ids), "observation ids are nonzero")
        for observation in observations:
            validate_effect_observation(observation)


def validate_effect_observation(observation: dict) -> None:
    require(set(observation) == EFFECT_OBSERVATION_KEYS, "effect observation keys")
    for value_key, name_key, table in (
        ("kind", "kindName", OBSERVATION_KINDS),
        ("unit", "unitName", UNITS),
        ("cost", "costName", OBSERVATION_COSTS),
        ("cadence", "cadenceName", OBSERVATION_CADENCES),
        ("fold", "foldName", OBSERVATION_FOLDS),
        ("channels", "channelsName", OBSERVATION_CHANNELS),
    ):
        require(
            table.get(observation[value_key]) == observation[name_key],
            f"observation {name_key} agrees with value",
        )
    require(isinstance(observation["name"], str) and observation["name"], "observation name")
    require(isinstance(observation["displayUnit"], str), "observation display unit")
    low = finite(observation["minimum"], "observation minimum")
    high = finite(observation["maximum"], "observation maximum")
    require(low < high, "observation range order")
    require(isinstance(observation["subscribable"], bool), "observation subscribable")
    # The one rule that ties the menu to the subscribe path: a `Resident` tap is a copy out of
    # state the block already wrote, and V1 binds it; a `Computed` tap has no implementation and
    # the subscribe path answers `unsupportedKind`. Nothing else may claim to be subscribable.
    require(
        observation["subscribable"] == (observation["costName"] == "resident"),
        "observation subscribable follows cost",
    )
    # A `Computed` tap may not claim per-block cadence: that would put an analysis pass on the
    # render thread, which is exactly what the cost split exists to prevent.
    require(
        not (observation["costName"] == "computed" and observation["cadenceName"] == "perBlock"),
        "a computed tap is not per-block",
    )


def validate_effect_parameter(parameter: dict) -> None:
    require(set(parameter) == EFFECT_PARAMETER_KEYS, "effect parameter keys")
    require(UNITS.get(parameter["unit"]) == parameter["unitName"], "unit name agrees with value")
    require(
        DOMAINS.get(parameter["domain"]) == parameter["domainName"], "domain name agrees with value"
    )
    require(
        MAPPINGS.get(parameter["mapping"]) == parameter["mappingName"],
        "mapping name agrees with value",
    )
    require(
        RATES.get(parameter["automationRate"]) == parameter["automationRateName"],
        "automation rate name agrees with value",
    )
    require(
        POLICIES.get(parameter["channelPolicy"]) == parameter["channelPolicyName"],
        "channel policy name agrees with value",
    )
    require(
        SMOOTHINGS.get(parameter["smoothing"]) == parameter["smoothingName"],
        "smoothing name agrees with value",
    )
    require(isinstance(parameter["name"], str) and parameter["name"], "parameter name")
    require(isinstance(parameter["displayUnit"], str), "parameter display unit")
    require(isinstance(parameter["readable"], bool), "parameter readable")
    require(isinstance(parameter["automatable"], bool), "parameter automatable")
    require(
        parameter["automatable"] == (parameter["automationRateName"] != "none"),
        "automatable follows automation rate",
    )
    require(
        parameter["smoothingSamples"] == 0 or parameter["smoothingName"] != "none",
        "an unsmoothed parameter declares zero smoothing samples",
    )
    # Issue #140 A: an effect parameter is live exactly when it is automatable, because the live
    # path delivers it as a `PreparedAutomationSpan` and an unautomatable parameter has no span an
    # effect would accept.
    require(
        parameter["liveUpdatable"] == parameter["automatable"],
        "effect liveUpdatable follows automatable",
    )
    validate_step(parameter["step"], "effect")
    default = finite(parameter["default"], "parameter default")
    if parameter["domainName"] == "continuous":
        low = finite(parameter["minimum"], "parameter minimum")
        high = finite(parameter["maximum"], "parameter maximum")
        require(low < high, "parameter range order")
        require(low <= default <= high, "parameter default in range")
        require(parameter["enumChoices"] == [], "continuous parameter has no choices")
        require(parameter["mappingName"] != "stepped", "continuous parameter is not stepped")
        if parameter["mappingName"] == "logarithmic":
            require(low > 0.0, "logarithmic parameter has a positive minimum")
    elif parameter["domainName"] == "boolean":
        require(parameter["minimum"] is None and parameter["maximum"] is None, "boolean bounds")
        require(parameter["enumChoices"] == [], "boolean parameter has no choices")
        require(parameter["mappingName"] == "stepped", "boolean parameter is stepped")
        require(default in (0.0, 1.0), "boolean default")
    else:
        require(parameter["minimum"] is None and parameter["maximum"] is None, "enum bounds")
        require(parameter["mappingName"] == "stepped", "enumeration parameter is stepped")
        choices = parameter["enumChoices"]
        require(len(choices) >= 2, "enumeration has at least two choices")
        values = [finite(choice["value"], "choice value") for choice in choices]
        require(values == sorted(values) and len(set(values)) == len(values), "choices ascend")
        labels = [choice["label"] for choice in choices]
        require(len(set(labels)) == len(labels), "choice labels are unique")
        require(default in values, "enumeration default is a choice")


def _first_step(document, unit):
    """Return the first effect parameter step declaring `unit`, for a targeted mutation."""
    for effect in document["effects"]:
        for parameter in effect["parameters"]:
            if parameter["step"]["unit"] == unit:
                return parameter["step"]
    raise AssertionError(f"no {unit} step row to mutate")


def self_test() -> int:
    here = pathlib.Path(__file__).resolve().parent
    sample = json.loads((here / "fixtures/parameter-metadata-v1-self-test.json").read_text())
    validate(sample)
    # Issue #143 E9. The shipped registry declares only `Resident` taps, so the computed-tap rules
    # are proved against a document that carries one: a synthetic menu added here, validated as a
    # positive first, then mutated. Testing them only against the shipped document would prove the
    # rules are never *reached*, not that they discriminate.
    tapped = copy.deepcopy(sample)
    tapped["effects"][0]["observations"] = [
        {
            "id": 1, "name": "Gain Reduction", "displayUnit": "dB",
            "kind": 1, "kindName": "gainReductionDb", "unit": 1, "unitName": "db",
            "cost": 1, "costName": "resident", "cadence": 1, "cadenceName": "perBlock",
            "fold": 2, "foldName": "peakMagnitude", "channels": 2, "channelsName": "perLane",
            "minimum": 0.0, "maximum": 100.0, "subscribable": True,
        },
        {
            "id": 7, "name": "Reduction Envelope", "displayUnit": "dB",
            "kind": 1, "kindName": "gainReductionDb", "unit": 1, "unitName": "db",
            "cost": 2, "costName": "computed", "cadence": 2, "cadenceName": "perWindow",
            "fold": 1, "foldName": "latest", "channels": 1, "channelsName": "shared",
            "minimum": 0.0, "maximum": 60.0, "subscribable": False,
        },
    ]
    validate(tapped)
    tap_mutations: list[tuple[str, object]] = [
        ("hand-edited tap id", lambda d: d["effects"][0]["observations"][0].update(id=9)),
        ("zero tap id", lambda d: d["effects"][0]["observations"][0].update(id=0)),
        (
            "a computed tap claims to be subscribable",
            lambda d: d["effects"][0]["observations"][1].update(subscribable=True),
        ),
        (
            "a resident tap denies being subscribable",
            lambda d: d["effects"][0]["observations"][0].update(subscribable=False),
        ),
        (
            "a computed tap claims per-block cadence",
            lambda d: d["effects"][0]["observations"][1].update(
                cadence=1, cadenceName="perBlock"
            ),
        ),
        (
            "tap fold name disagrees with value",
            lambda d: d["effects"][0]["observations"][0].update(foldName="latest"),
        ),
        (
            "tap bounds are inverted",
            lambda d: d["effects"][0]["observations"][0].update(minimum=100.0, maximum=0.0),
        ),
        (
            "observations key removed",
            lambda d: d["effects"][0].pop("observations"),
        ),
        (
            "observation vocabulary renamed",
            lambda d: d["observationVocabularies"]["costs"][1].update(name="derived"),
        ),
        (
            "observation vocabulary truncated",
            lambda d: d["observationVocabularies"]["folds"].pop(),
        ),
    ]
    mutations: list[tuple[str, object]] = [
        ("schema", lambda d: d.update(schema="miso.web.parameter-metadata.v2")),
        ("abi", lambda d: d.update(abiVersion=1)),
        ("record bytes", lambda d: d.update(commandRecordBytes=32)),
        ("kind not applied", lambda d: d["commandKinds"][2].update(applied=False)),
        # The 6-vs-8 split the kind-vocabulary gate exists to close: the shipped document stopped
        # at `effectBypass` while the wire decoded eight kinds. Its shape is red here now, and so
        # is every way the plane column can lie about what a kind applies to.
        (
            "kind vocabulary truncated at effectBypass",
            lambda d: d["commandKinds"].__delitem__(slice(6, None)),
        ),
        ("kind renamed", lambda d: d["commandKinds"][6].update(name="observeArm")),
        ("kind value renumbered", lambda d: d["commandKinds"][7].update(value=9)),
        (
            "an observation kind claims the render plane",
            lambda d: d["commandKinds"][6].update(plane="render"),
        ),
        (
            "a DSP kind claims the observation plane",
            lambda d: d["commandKinds"][0].update(plane="observation"),
        ),
        ("a kind row drops its plane", lambda d: d["commandKinds"][0].pop("plane")),
        ("a kind row invents a plane", lambda d: d["commandKinds"][0].update(plane="control")),
        ("reason order", lambda d: d["commandReasons"].reverse()),
        # Issue #151's exact defect shape: a vocabulary that stops one short of the reasons the
        # observation path returns, and one whose value column no longer matches its name column.
        ("reason vocabulary truncated at wrongState", lambda d: d["commandReasons"].__delitem__(
            slice(10, None)
        )),
        ("reason renamed", lambda d: d["commandReasons"][10].update(name="unknownObservation")),
        ("reason value renumbered", lambda d: d["commandReasons"][11].update(value=12)),
        (
            "builtin live disagrees with update rate",
            lambda d: d["builtins"]["parameters"][6].update(liveUpdatable=False),
        ),
        (
            # Row index 2 is `hpf_hz`. It was index 1 (`trim_db`) until #210 phase 3 made that row
            # genuinely live; the mutation has to name a row that is still prepared-only or it
            # stops being a mutation.
            "a prepared-only builtin claims to be live",
            lambda d: d["builtins"]["parameters"][2].update(liveUpdatable=True),
        ),
        # The two red mutations on the **live-set literal** itself (#210 phase 3). Both are
        # internally consistent -- `liveUpdatable` still follows `updateRate`, the smoothing
        # policy still matches -- so every other rule in `validate` passes them, and only the
        # `live_names == {...}` pin refuses them. That is what makes the literal load-bearing
        # rather than decorative: without it, a row could be promoted or demoted silently.
        (
            "a deferred builtin is promoted into the live set",
            lambda d: d["builtins"]["parameters"][2].update(
                updateRate="blockTarget",
                smoothing="linearNUpdates",
                liveUpdatable=True,
            ),
        ),
        (
            "the live trim row is demoted out of the live set",
            lambda d: d["builtins"]["parameters"][1].update(
                updateRate="preparedOnly",
                smoothing="none",
                liveUpdatable=False,
            ),
        ),
        # Issue #242 replaced #127's null slot with a populated lattice declaration. Each
        # mutation below breaks exactly one sentence of that contract.
        ("builtin step slot is null", lambda d: d["builtins"]["parameters"][0].update(step=None)),
        ("effect step slot is null", lambda d: d["effects"][0]["parameters"][0].update(step=None)),
        (
            "step size is a float rather than decimal text",
            lambda d: d["effects"][0]["parameters"][0]["step"].update(size=0.1),
        ),
        (
            "step size carries an exponent spelling",
            lambda d: d["effects"][0]["parameters"][0]["step"].update(size="1e-1"),
        ),
        (
            "step unit leaves the vocabulary",
            lambda d: d["effects"][0]["parameters"][0]["step"].update(unit="decibels"),
        ),
        (
            "step precision exceeds the pinned range",
            lambda d: d["effects"][0]["parameters"][0]["step"].update(precision=9),
        ),
        (
            "a geometric row declares a ratio of one",
            lambda d: _first_step(d, "ratio").update(size="1"),
        ),
        (
            "an index row declares a step other than one",
            lambda d: _first_step(d, "index").update(size="2"),
        ),
        (
            "the smallest ladder multiple is zero",
            lambda d: d["effects"][0]["parameters"][0]["step"]["ladder"].update(xs=0),
        ),
        (
            "ladder multiples stop ascending",
            lambda d: d["effects"][0]["parameters"][0]["step"]["ladder"].update(md=3),
        ),
        (
            "a ladder multiple is a float",
            lambda d: d["effects"][0]["parameters"][0]["step"]["ladder"].update(lg=10.0),
        ),
        (
            "the extra-large ladder multiple exceeds its wire width",
            lambda d: d["effects"][0]["parameters"][0]["step"]["ladder"].update(xl=64),
        ),
        (
            "a ladder size is renamed",
            lambda d: d["effects"][0]["parameters"][0]["step"].update(
                ladder={"xs": 1, "sm": 3, "md": 5, "lg": 10, "xxl": 30}
            ),
        ),
        (
            "the step slot keeps its retired nudge spelling",
            lambda d: d["builtins"]["parameters"][0].update(
                nudge=d["builtins"]["parameters"][0].pop("step")
            ),
        ),
        (
            "the appended pan row is dropped from the live set",
            lambda d: d["builtins"]["parameters"][11].update(updateRate="preparedOnly"),
        ),
        ("effect order", lambda d: d["effects"].reverse()),
        (
            "parameter ids descend",
            lambda d: d["effects"][0]["parameters"].reverse(),
        ),
        (
            "an automatable effect parameter denies being live",
            lambda d: d["effects"][0]["parameters"][0].update(liveUpdatable=False),
        ),
        (
            "an unautomatable effect parameter claims to be live",
            lambda d: d["effects"][0]["parameters"][0].update(
                automatable=False, automationRate=3, automationRateName="none"
            ),
        ),
        (
            "a block-target builtin denies being live",
            lambda d: d["builtins"]["parameters"][4].update(liveUpdatable=False),
        ),
        (
            "unit name disagrees with unit value",
            lambda d: d["effects"][0]["parameters"][0].update(unitName="hz"),
        ),
        (
            "default outside range",
            lambda d: d["effects"][0]["parameters"][0].update(default=1e9),
        ),
        (
            "non-finite default",
            lambda d: d["effects"][0]["parameters"][0].update(default=float("inf")),
        ),
    ]
    failures = 0
    for name, mutate in [(name, mutate) for name, mutate in mutations] + [
        (name, mutate) for name, mutate in tap_mutations
    ]:
        base = tapped if any(name == row[0] for row in tap_mutations) else sample
        mutated = copy.deepcopy(base)
        mutate(mutated)
        try:
            validate(mutated)
        except Invalid:
            continue
        except Exception:  # noqa: BLE001 - a mutation that crashes still discriminates
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        failures += 1
    if failures == 0:
        print("parameter metadata schema self-test passed")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("document", nargs="?", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.document is None:
        parser.error("a document path is required")
    try:
        validate(json.loads(args.document.read_text()))
    except Invalid as error:
        print(f"FAIL parameter metadata: {error}", file=sys.stderr)
        return 1
    print(f"parameter metadata schema: ok ({args.document})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
