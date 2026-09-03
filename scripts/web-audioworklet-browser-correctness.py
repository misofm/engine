#!/usr/bin/env python3
"""One-shot local raw-WebDriver correctness gate for Issue 075."""

from __future__ import annotations

import argparse
import hashlib
import http.server
import json
import os
import pathlib
import shutil
import socket
import struct
import subprocess
import sys
import threading
import urllib.error
import urllib.request


ROOT = pathlib.Path(__file__).resolve().parent.parent
FIXTURE = ROOT / "hosts/host-web/tests/browser-v1"
# W4-D1: the scalar worklet artifact is no longer built or shipped.
EXPECTED_ARTIFACTS = (
    "miso-engine-v1-audio-worklet.simd128.wasm",
    "miso-engine-v1-audio-worklet.js",
    "miso-engine-v1-audio-worklet-host.js",
    "miso-engine-v1-audio-worklet-host.d.ts",
    # Issue #137 D4: the parameter metadata ships with the module and is sealed with it.
    "miso-engine-v1-parameter-metadata.json",
    # Issue #243: so does the ABI layout. Both are emitted by one generator from the same engine,
    # so a seal that covered one and not the other would attest to half a release.
    "miso-engine-v1-abi-layout.json",
)
SOURCE_SEAL_PATHS = (
    "Cargo.toml",
    "Cargo.lock",
    "hosts/host-web/Cargo.toml",
    "hosts/host-web/web/miso-engine-v1-audio-worklet.js",
    "hosts/host-web/web/miso-engine-v1-audio-worklet-host.js",
    "hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts",
    "scripts/build-web-audioworklet.sh",
    "scripts/check-web-audioworklet.sh",
)

WEBDRIVER_COMMANDS = {
    "status": ("GET", "object"),
    "new-session": ("POST", "object"),
    "navigate-to": ("POST", "null"),
    "execute-async-script": ("POST", "object"),
    "delete-session": ("DELETE", "null"),
}


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(131072), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical_hash(value: object) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


def write_exclusive(path: pathlib.Path, encoded: bytes) -> None:
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(descriptor, "wb") as stream:
        stream.write(encoded)


def file_hashes(directory: pathlib.Path, names: tuple[str, ...]) -> dict[str, str]:
    return {name: sha256(directory / name) for name in names}


def fixture_hashes() -> dict[str, str]:
    return {path.name: sha256(path) for path in sorted(FIXTURE.iterdir()) if path.is_file()}


def candidate() -> str:
    return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()


def require_clean_candidate() -> None:
    status = subprocess.check_output(
        ["git", "status", "--short", "--untracked-files=all"], cwd=ROOT, text=True
    )
    if status:
        raise RuntimeError("browser gate requires a clean committed candidate")


def seal_record(artifacts: pathlib.Path, browser: pathlib.Path, driver: pathlib.Path) -> dict:
    return {
        "schema": "miso.web.browser.seal.v1",
        "candidate": candidate(),
        "artifactSha256": file_hashes(artifacts, EXPECTED_ARTIFACTS),
        "sourceSha256": {name: sha256(ROOT / name) for name in SOURCE_SEAL_PATHS},
        "fixtureSha256": fixture_hashes(),
        "toolSha256": {"browser": sha256(browser), "driver": sha256(driver)},
        "driverIdentity": subprocess.check_output([str(driver), "--version"], text=True).strip(),
        "command": "scripts/run-web-audioworklet-browser-correctness.sh SEALED_INPUT NEW_EVIDENCE_JSON",
        "browserCorrectnessInvocations": 0,
        "workloadInvocations": 0,
        "benchmarkInvocations": 0,
        "timedInvocations": 0,
    }


def load_inputs() -> tuple[dict, dict]:
    source = json.loads((FIXTURE / "source.json").read_text())
    expected = json.loads((FIXTURE / "expected.json").read_text())
    if source.get("schema") != "miso.web.browser.source.v1":
        raise ValueError("unexpected source fixture schema")
    if expected.get("schema") != "miso.web.browser.expected.v1":
        raise ValueError("unexpected expected fixture schema")
    if source.get("sampleRateHz") != 48000 or source.get("quantumFrames") != 128:
        raise ValueError("fixture rate/quantum mismatch")
    if len(source.get("blocks", [])) != 2:
        raise ValueError("fixture requires exactly two source blocks")
    if set(source) != {"schema", "sourceId", "sampleRateHz", "quantumFrames", "blocks"}:
        raise ValueError("source fixture keys")
    if set(expected) != {
        "schema",
        "sampleRateHz",
        "quantumFrames",
        "renderedQuantaBeforeDispose",
        "nextAbsoluteSampleBeforeDispose",
        "oracle",
        "pcm",
        "directOracle",
    }:
        raise ValueError("expected fixture keys")
    direct = expected.get("directOracle", {})
    if set(direct) != {
        "schema",
        "simd128",
        "nativePcmF32leSha256",
        # Issue #137 E2: the command-timeline leg and its native pin.
        "commandTimeline",
        "nativeCommandTimelinePcmF32leSha256",
        # Issue #143 E1/E4/E12: the observation-timeline leg and its native pin.
        "observationTimeline",
        "nativeObservationPcmF32leSha256",
    }:
        raise ValueError("direct oracle keys")
    # Issue #143. The observation leg's contract, stated here so a browser run that silently
    # stopped observing anything cannot pass: an armed tap publishes a positive magnitude, the
    # designated master is that track's own reading, the window is the declared two blocks, an
    # unsubscribed tap publishes exactly zero, a session with no capacity reports no master
    # reading at all, and the two typed refusals keep their frozen reasons.
    observation = direct.get("observationTimeline", {})
    if set(observation) != {
        "armedTrackGrDbPositive",
        "armedMasterEqualsTrack",
        "armedWindowSamples",
        "disarmedTrackGrDb",
        "unobservedMasterGrDb",
        "subscribeReason",
        "unknownTapReason",
        "unboundReason",
        "pcmF32leSha256",
    }:
        raise ValueError("observation timeline keys")
    if observation.get("pcmF32leSha256") != direct.get("nativeObservationPcmF32leSha256"):
        raise ValueError("observation timeline native parity")
    if observation.get("armedTrackGrDbPositive") is not True:
        raise ValueError("observation timeline armed magnitude")
    if observation.get("armedMasterEqualsTrack") is not True:
        raise ValueError("observation timeline master designation")
    if observation.get("armedWindowSamples") != "256":
        raise ValueError("observation timeline window length")
    if observation.get("disarmedTrackGrDb") != 0:
        raise ValueError("observation timeline unsubscribe")
    if observation.get("unobservedMasterGrDb") is not None:
        raise ValueError("observation timeline absent master")
    if (observation.get("subscribeReason"), observation.get("unknownTapReason"),
            observation.get("unboundReason")) != (0, 10, 11):
        raise ValueError("observation timeline typed reasons")
    timeline = direct.get("commandTimeline", {})
    if set(timeline) != {"reports", "beforeDisposeStatus", "pcmF32leSha256"}:
        raise ValueError("command timeline keys")
    if timeline.get("pcmF32leSha256") != direct.get("nativeCommandTimelinePcmF32leSha256"):
        raise ValueError("command timeline native parity")
    reports = timeline.get("reports", {})
    # Issue #140 C: every declared kind is live, so every declared kind is in the timeline.
    if set(reports) != {
        "matrix",
        "unknownTrack",
        "flood",
        "unknownParameter",
        "pan",
        "fader",
        "mute",
        "effectParam",
        "effectBypass",
        "mixedBatch",
    }:
        raise ValueError("command timeline report keys")
    # The application sample of each admitted kind is part of the pin: an ABI that starts applying
    # a command one block early or late is a changed ABI, not a passing test.
    for name, sample, admitted in (
        ("matrix", "128", 1),
        ("pan", "384", 1),
        ("fader", "512", 1),
        ("mute", "640", 1),
        ("effectParam", "768", 1),
        ("effectBypass", "896", 1),
        ("mixedBatch", "1024", 2),
    ):
        if reports[name]["appliedAtSample"] != sample:
            raise ValueError(f"{name} command application sample")
        if reports[name]["admitted"] != admitted or reports[name]["result"] != 0:
            raise ValueError(f"{name} admitted records")
    # The typed refusals are part of the pin too: an ABI that silently starts accepting an unknown
    # track, a flood or an undeclared parameter is a changed ABI.
    for name, result, reason in (
        ("unknownTrack", 1, 2),
        ("flood", 6, 8),
        ("unknownParameter", 1, 5),
    ):
        if reports[name]["result"] != result or reports[name]["reason"] != reason:
            raise ValueError(f"{name} typed refusal")
        if reports[name]["admitted"] != 0:
            raise ValueError(f"{name} admitted records")
    if direct.get("schema") != "miso.web.browser.direct-oracle.v1":
        raise ValueError("direct oracle schema")
    # Issue #241 reshaped the source declaration: the session-level `limits` table and the nested
    # `mapping`/`region` are gone, so a source's length is the row's own `frames` and its content is
    # a canonical-PCM identity (docs/STEM_IDENTITY_V1.md) rather than a locator name. The pins below
    # are the post-#241 spelling of the same frozen facts, plus the identity itself. Shape and
    # identity are separate pins so each can go red alone: the shape states the preimage's length,
    # the identity states which bytes filled it.
    def check_session_fixture(name: str, frames: str, content: str, effect_id: str | None = None) -> None:
        session = json.loads((FIXTURE / name).read_text())
        source = session["sources"][0]
        expected = {
            "sample_rate_hz": 48000,
            "quantum_frames": 128,
            "channels": 2,
            "bit_depth": "32f",
            "frames": frames,
            "content": content,
        }
        actual = {
            "sample_rate_hz": session.get("sample_rate_hz"),
            "quantum_frames": session.get("quantum_frames"),
            "channels": source.get("channels"),
            "bit_depth": source.get("bit_depth"),
            "frames": source.get("frames"),
            "content": source.get("content"),
        }
        if actual != expected:
            raise ValueError(f"{name} frozen session shape or identity changed")
        if effect_id is not None:
            effects = session["tracks"][0]["dynamic"]["effects"]
            if effects[0]["identity"].get("effect_id") != effect_id:
                raise ValueError(f"{name} effect identity changed")

    # SHA-256 over 256 frames x 2 channels x 4 bytes, interleaved little-endian f32 bits, of
    # the `source.json` ramp the fixture feeds: per 128-frame block, left[i] = leftBase +
    # leftStep * i and right[i] = 0.
    check_session_fixture(
        "session.json",
        "256",
        "sha256:a7d052a7f6b3b881f4bde6090d87c4226d39e62010e9b6038088bb28b8742949",
    )
    # Issue #137 E2, extended by #140 C: the command timeline runs the identity session plus one
    # dynamic-rack parametric EQ over a longer region, and both legs read this exact file. The EQ
    # is what makes an effect-addressed command have something real to address, and its band 1 is
    # a low shelf so a DC fixture can witness the parameter move at all.
    # Issue #143: the observation timeline's own session -- one track, one compressor in the
    # dynamic rack, so the per-node scalar publish path is what the browser exercises.
    # SHA-256 over 2048 frames x 2 channels x 4 bytes of constant 0.5 -- the level
    # `direct-oracle.mjs` fills into every observation-timeline block.
    check_session_fixture(
        "observation-session.json",
        "2048",
        "sha256:66e39e41bccc0a57ae90a77b426f4075e81ba877b0653c3aabe0a9e00762769c",
        "miso.compressor",
    )
    # SHA-256 over 2048 frames x 2 channels x 4 bytes of constant 0.25 -- the level
    # `direct-oracle.mjs` fills into every command-timeline block. It is deliberately not the
    # identity session's digest: the same content string under two different `frames` would be
    # two different preimage lengths, which STEM_IDENTITY_V1 forbids.
    check_session_fixture(
        "command-session.json",
        "2048",
        "sha256:680aca77ba6b819a4489730f3e42f69ba9f6d7a5921e748a8a46eb1974d0867c",
        "miso.parametric-eq",
    )
    return source, expected


class FixtureHandler(http.server.BaseHTTPRequestHandler):
    payloads: dict[str, tuple[str, bytes]]

    def do_GET(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler callback name
        clean = self.path.split("?", 1)[0]
        record = self.payloads.get(clean)
        if record is None:
            self.send_error(404)
            return
        content_type, payload = record
        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(payload)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(payload)

    def log_message(self, _format: str, *_args: object) -> None:
        return


def immutable_payloads(artifacts: pathlib.Path) -> dict[str, tuple[str, bytes]]:
    content_types = {
        ".wasm": "application/wasm",
        ".js": "text/javascript; charset=utf-8",
        ".mjs": "text/javascript; charset=utf-8",
        ".json": "application/json; charset=utf-8",
        ".json": "application/json; charset=utf-8",
        ".html": "text/html; charset=utf-8",
        ".ts": "text/plain; charset=utf-8",
    }
    payloads = {}
    for name in EXPECTED_ARTIFACTS:
        path = artifacts / name
        payloads[f"/artifacts/{name}"] = (content_types[path.suffix], path.read_bytes())
    for path in sorted(FIXTURE.iterdir()):
        if path.is_file():
            payloads[f"/fixture/{path.name}"] = (
                content_types.get(path.suffix, "application/octet-stream"),
                path.read_bytes(),
            )
    payloads["/"] = payloads["/fixture/index.html"]
    return payloads


def free_port() -> int:
    with socket.socket() as probe:
        probe.bind(("127.0.0.1", 0))
        return int(probe.getsockname()[1])


def validate_webdriver_response(command: str, method: str, response: object) -> dict:
    contract = WEBDRIVER_COMMANDS.get(command)
    if contract is None:
        raise RuntimeError(f"unknown WebDriver command: {command}")
    expected_method, result_kind = contract
    if method != expected_method:
        raise RuntimeError(f"invalid method for WebDriver command {command}: {method}")
    if not isinstance(response, dict) or set(response) != {"value"}:
        raise RuntimeError(f"malformed WebDriver response for {command}")
    value = response["value"]
    if isinstance(value, dict) and isinstance(value.get("error"), str):
        raise RuntimeError(f"WebDriver protocol error for {command}: {value['error']}")
    if result_kind == "null":
        if value is not None:
            raise RuntimeError(f"WebDriver returned non-null success for {command}")
    elif not isinstance(value, dict):
        raise RuntimeError(f"WebDriver returned no typed value for {command}")
    return response


def request(command: str, method: str, url: str, payload: object | None = None) -> dict:
    data = None if payload is None else json.dumps(payload).encode()
    headers = {} if data is None else {"Content-Type": "application/json"}
    try:
        with urllib.request.urlopen(
            urllib.request.Request(url, data=data, headers=headers, method=method)
        ) as http_response:
            if http_response.status != 200:
                raise RuntimeError(
                    f"unexpected WebDriver HTTP status for {command}: {http_response.status}"
                )
            response = json.loads(http_response.read())
    except (json.JSONDecodeError, UnicodeDecodeError) as error:
        raise RuntimeError(f"malformed WebDriver response for {command}") from error
    return validate_webdriver_response(command, method, response)


def self_test_webdriver_responses() -> None:
    responses: dict[str, tuple[int, bytes]] = {
        "/navigate": (200, b'{"value":null}'),
        "/delete": (200, b'{"value":null}'),
        "/status": (200, b'{"value":{"ready":true}}'),
        "/new-session": (200, b'{"value":{"sessionId":"test","capabilities":{}}}'),
        "/script": (200, b'{"value":{"ok":true,"value":{}}}'),
        "/missing": (200, b"{}"),
        "/malformed-envelope": (200, b"[]"),
        "/extra-envelope-key": (200, b'{"value":{},"extra":true}'),
        "/malformed-json": (200, b"not-json"),
        "/protocol-error": (
            200,
            b'{"value":{"error":"unknown error","message":"failed","stacktrace":""}}',
        ),
        "/http-error": (
            500,
            b'{"value":{"error":"unknown error","message":"failed","stacktrace":""}}',
        ),
        "/typed-null": (200, b'{"value":null}'),
        "/navigate-object": (200, b'{"value":{}}'),
    }

    class ResponseHandler(http.server.BaseHTTPRequestHandler):
        def respond(self) -> None:
            length = int(self.headers.get("Content-Length", "0"))
            if length:
                self.rfile.read(length)
            status, body = responses[self.path]
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        do_GET = respond
        do_POST = respond
        do_DELETE = respond

        def log_message(self, _format: str, *_args: object) -> None:
            return

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), ResponseHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    base = f"http://127.0.0.1:{server.server_port}"

    def expect_rejected(command: str, method: str, path: str) -> None:
        try:
            request(command, method, f"{base}{path}")
        except (RuntimeError, urllib.error.HTTPError):
            return
        raise AssertionError(f"WebDriver response unexpectedly accepted: {path}")

    try:
        if request("navigate-to", "POST", f"{base}/navigate") != {"value": None}:
            raise AssertionError("navigation null success")
        if request("delete-session", "DELETE", f"{base}/delete") != {"value": None}:
            raise AssertionError("session deletion null success")
        for command, method, path in (
            ("status", "GET", "/status"),
            ("new-session", "POST", "/new-session"),
            ("execute-async-script", "POST", "/script"),
        ):
            request(command, method, f"{base}{path}")
        for command, method, path in (
            ("status", "GET", "/missing"),
            ("status", "GET", "/malformed-envelope"),
            ("status", "GET", "/extra-envelope-key"),
            ("status", "GET", "/malformed-json"),
            ("status", "GET", "/protocol-error"),
            ("status", "GET", "/http-error"),
            ("status", "GET", "/typed-null"),
            ("new-session", "POST", "/typed-null"),
            ("execute-async-script", "POST", "/typed-null"),
            ("navigate-to", "POST", "/navigate-object"),
        ):
            expect_rejected(command, method, path)
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


def wait_for_driver(url: str) -> dict:
    failure: Exception | None = None
    for _attempt in range(200):
        try:
            return request("status", "GET", f"{url}/status")["value"]
        except (OSError, urllib.error.URLError, RuntimeError) as error:
            failure = error
            threading.Event().wait(0.025)
    raise RuntimeError("ChromeDriver did not become ready") from failure


def float32(value: float) -> float:
    import struct

    return struct.unpack("<f", struct.pack("<f", value))[0]


def pcm_bits(pcm: list[list[float]]) -> bytes:
    """Little-endian f32 words. `==` on floats would equate `-0.0` and `+0.0`."""
    return b"".join(struct.pack("<f", sample) for channel in pcm for sample in channel)


def pcm_f32le_sha256(pcm: list[list[float]]) -> str:
    digest = hashlib.sha256()
    for channel in pcm:
        for sample in channel[:384]:
            digest.update(struct.pack("<f", sample))
    return digest.hexdigest()


def validate_result(result: dict, source: dict, expected: dict) -> None:
    if set(result) != {"schema", "runs", "failure"}:
        raise AssertionError("browser result keys")
    if result.get("schema") != "miso.web.browser.result.v1":
        raise AssertionError("browser result schema")
    runs = result.get("runs")
    if not isinstance(runs, list) or len(runs) != 2:
        raise AssertionError("exactly two fresh contexts required")
    # W4-D1: one shipped artifact, so both runs are simd128. The cross-backend leg is #83's G5
    # corpus plus the native digest asserted by direct-oracle.mjs, both `to_bits` identities.
    expected_backends = ["simd128", "simd128"]
    for index, (run, backend) in enumerate(zip(runs, expected_backends, strict=True)):
        if set(run) != {
            "backend", "exposedMainQuantum", "memoryBytes", "memoryStable",
            "positiveZeroSilence", "resources", "status", "acknowledgements", "pcm",
        }:
            raise AssertionError(f"run {index} keys")
        if run.get("backend") != backend:
            raise AssertionError(f"run {index} backend")
        if run.get("exposedMainQuantum") not in (0, 128):
            raise AssertionError(f"run {index} main quantum")
        if not run.get("memoryStable") or not run.get("positiveZeroSilence"):
            raise AssertionError(f"run {index} memory/silence")
        acknowledgements = run.get("acknowledgements", {})
        exact_acks = {
            "first": 0,
            "firstOwnership": True,
            "initialBackpressure": 6,
            "initialBackpressureOwnership": True,
            "seek": 0,
            "repeat": 0,
            "repeatOwnership": True,
            "repeatBackpressure": 6,
            "repeatBackpressureOwnership": True,
            "final": 0,
            "finalOwnership": True,
        }
        if acknowledgements != exact_acks:
            raise AssertionError(f"run {index} acknowledgement transcript")
        status = run.get("status", {})
        if set(status) != {
            "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
            "quantumFrames", "nextAbsoluteSample", "renderedQuanta", "memoryBytes",
        }:
            raise AssertionError(f"run {index} status keys")
        direct_status = expected["directOracle"][backend]["beforeDisposeStatus"]
        comparable_status = {
            name: status[name]
            for name in direct_status
        }
        if status.get("tag") != "miso.status.v1" or status.get("result") != 0 \
                or comparable_status != direct_status \
                or run.get("memoryBytes") != direct_status["memoryBytes"]:
            raise AssertionError(f"run {index} status")
        resources = run.get("resources", {})
        if resources != expected["directOracle"][backend]["resources"]:
            raise AssertionError(f"run {index} complete resources")
        pcm = run.get("pcm")
        if not isinstance(pcm, list) or len(pcm) != 2:
            raise AssertionError(f"run {index} PCM shape")
        for channel in range(2):
            if len(pcm[channel]) != 512:
                raise AssertionError(f"run {index} PCM frames")
        if pcm_f32le_sha256(pcm) != expected["directOracle"][backend]["pcmF32leSha256"]:
            raise AssertionError(f"run {index} independent direct PCM")
    # Packed bytes, not `==`: Python and JavaScript both equate `-0.0` with `+0.0`.
    if pcm_bits(runs[0]["pcm"]) != pcm_bits(runs[1]["pcm"]):
        raise AssertionError("fresh-context determinism")
    failure = result.get("failure")
    if failure != {
        "tag": "miso.error.v1",
        "requestId": 0,
        "result": 9,
        "exposedMainQuantum": failure.get("exposedMainQuantum"),
        "frames": 128,
        "positiveZeroSilence": True,
    } or failure.get("exposedMainQuantum") not in (0, 128):
        raise AssertionError("observable failure silence")


def check_oracle(artifacts: pathlib.Path) -> None:
    if sorted(path.name for path in artifacts.iterdir()) != sorted(EXPECTED_ARTIFACTS):
        raise ValueError(
            f"artifact directory is not the exact frozen "
            f"{len(EXPECTED_ARTIFACTS)}-file set"
        )
    runtime = shutil.which("node") or shutil.which("bun")
    if runtime is None:
        raise RuntimeError("Node.js-compatible runtime required for raw-Wasm oracle")
    subprocess.run(
        [
            runtime,
            str(FIXTURE / "direct-oracle.mjs"),
            str(artifacts),
            str(FIXTURE / "expected.json"),
        ],
        cwd=ROOT,
        check=True,
    )


def run(args: argparse.Namespace, source: dict, expected: dict) -> None:
    artifacts = args.artifacts.resolve()
    if sorted(path.name for path in artifacts.iterdir()) != sorted(EXPECTED_ARTIFACTS):
        raise ValueError(
            f"artifact directory is not the exact frozen "
            f"{len(EXPECTED_ARTIFACTS)}-file set"
        )
    if args.output.exists() or args.output.with_suffix(args.output.suffix + ".sha256").exists():
        raise FileExistsError("refusing to overwrite browser evidence")
    require_clean_candidate()
    sealed = json.loads(args.seal_input.read_text())
    actual_seal = seal_record(artifacts, args.browser, args.driver)
    if sealed != actual_seal:
        raise RuntimeError("candidate/artifact/fixture/tool seal mismatch; browser not launched")
    payloads = immutable_payloads(artifacts)

    handler = type("BoundFixtureHandler", (FixtureHandler,), {"payloads": payloads})
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()
    driver_port = free_port()
    driver = subprocess.Popen(
        [str(args.driver), f"--port={driver_port}", "--url-base=/"],
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    driver_url = f"http://127.0.0.1:{driver_port}"
    session_id = None
    try:
        driver_status = wait_for_driver(driver_url)
        capability = {
            "capabilities": {
                "alwaysMatch": {
                    "browserName": "chrome",
                    "goog:chromeOptions": {
                        "binary": str(args.browser),
                        "args": [
                            "--headless=new",
                            "--no-sandbox",
                            "--disable-dev-shm-usage",
                            "--autoplay-policy=no-user-gesture-required",
                        ],
                    },
                }
            }
        }
        created = request("new-session", "POST", f"{driver_url}/session", capability)["value"]
        session_id = created["sessionId"]
        capabilities = created["capabilities"]
        base = f"{driver_url}/session/{session_id}"
        fixture_url = f"http://127.0.0.1:{server.server_port}/fixture/index.html"
        request("navigate-to", "POST", f"{base}/url", {"url": fixture_url})
        script = """
          const done = arguments[arguments.length - 1];
          import('/fixture/browser-correctness.js')
            .then((module) => module.runMisoBrowserCorrectness())
            .then((value) => done({ok: true, value}),
                  () => done({ok: false, error: 'browser-correctness-failed'}));
        """
        envelope = request(
            "execute-async-script",
            "POST",
            f"{base}/execute/async",
            {"script": script, "args": []},
        )["value"]
        if envelope != {"ok": True, "value": envelope.get("value")}:
            raise RuntimeError("browser correctness module failed")
        result = envelope["value"]
        validate_result(result, source, expected)
        require_clean_candidate()
        final_seal = seal_record(artifacts, args.browser, args.driver)
        if final_seal != sealed or final_seal != actual_seal:
            raise RuntimeError("candidate/artifact/fixture/tool seal changed during browser run")
        record = {
            "schema": "miso.web.browser.evidence.v1",
            "candidate": candidate(),
            "browser": {
                "name": capabilities.get("browserName"),
                "version": capabilities.get("browserVersion"),
                "driver": driver_status,
            },
            "sealSha256": sha256(args.seal_input),
            "artifactSha256": actual_seal["artifactSha256"],
            "fixtureSha256": actual_seal["fixtureSha256"],
            "resultSha256": canonical_hash(result),
            "result": result,
            "browserCorrectnessInvocations": 1,
            "workloadInvocations": 0,
            "benchmarkInvocations": 0,
            "timedInvocations": 0,
        }
        encoded = (json.dumps(record, indent=2, sort_keys=True) + "\n").encode()
        write_exclusive(args.output, encoded)
        checksum = sha256(args.output)
        checksum_path = args.output.with_suffix(args.output.suffix + ".sha256")
        write_exclusive(checksum_path, f"{checksum}  {args.output.name}\n".encode())
    finally:
        if session_id is not None:
            try:
                request("delete-session", "DELETE", f"{driver_url}/session/{session_id}")
            except Exception:
                pass
        driver.terminate()
        driver.wait()
        server.shutdown()
        server.server_close()
        server_thread.join()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--self-test-webdriver-responses", action="store_true")
    parser.add_argument("--seal", action="store_true")
    parser.add_argument("--seal-input", type=pathlib.Path)
    parser.add_argument("--artifacts", type=pathlib.Path)
    parser.add_argument("--output", type=pathlib.Path)
    parser.add_argument("--browser", type=pathlib.Path)
    parser.add_argument("--driver", type=pathlib.Path)
    args = parser.parse_args()
    if args.self_test_webdriver_responses:
        self_test_webdriver_responses()
        print("web AudioWorklet WebDriver response tests passed")
        return 0
    source, expected = load_inputs()
    if args.check:
        if args.artifacts is None:
            parser.error("check mode requires --artifacts")
        check_oracle(args.artifacts.resolve())
        print("web AudioWorklet browser fixture/runner static check passed")
        return 0
    if args.seal:
        if None in (args.artifacts, args.output, args.browser, args.driver):
            parser.error("seal mode requires --artifacts, --output, --browser and --driver")
        require_clean_candidate()
        if args.output.exists() or args.output.is_symlink():
            raise FileExistsError("refusing to overwrite browser seal")
        record = seal_record(args.artifacts.resolve(), args.browser.resolve(), args.driver.resolve())
        write_exclusive(args.output, (json.dumps(record, indent=2, sort_keys=True) + "\n").encode())
        print(sha256(args.output))
        return 0
    if None in (args.artifacts, args.output, args.browser, args.driver, args.seal_input):
        parser.error("run mode requires --seal-input, --artifacts, --output, --browser and --driver")
    run(args, source, expected)
    return 0


if __name__ == "__main__":
    sys.exit(main())
