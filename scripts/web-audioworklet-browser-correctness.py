#!/usr/bin/env python3
"""One-shot local raw-WebDriver correctness gate for Issue 075."""

from __future__ import annotations

import argparse
import hashlib
import http.server
import json
import os
import pathlib
import socket
import subprocess
import sys
import threading
import urllib.error
import urllib.request


ROOT = pathlib.Path(__file__).resolve().parent.parent
FIXTURE = ROOT / "hosts/miso-engine-host-web/tests/browser-v1"
EXPECTED_ARTIFACTS = (
    "miso-engine-v2-audio-worklet.scalar.wasm",
    "miso-engine-v2-audio-worklet.simd128.wasm",
    "miso-engine-v2-audio-worklet.js",
    "miso-engine-v2-audio-worklet-host.js",
    "miso-engine-v2-audio-worklet-host.d.ts",
)
SOURCE_SEAL_PATHS = (
    "Cargo.toml",
    "Cargo.lock",
    "hosts/miso-engine-host-web/Cargo.toml",
    "hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js",
    "hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js",
    "hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts",
    "scripts/build-web-audioworklet.sh",
    "scripts/check-web-audioworklet.sh",
)


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
    if subprocess.run(
        ["git", "diff", "--quiet", "--ignore-submodules", "HEAD", "--"], cwd=ROOT, check=False
    ).returncode != 0 or subprocess.run(
        ["git", "diff", "--quiet", "--cached", "--ignore-submodules", "HEAD", "--"],
        cwd=ROOT,
        check=False,
    ).returncode != 0:
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
    session = (FIXTURE / "session.toml").read_text()
    for frozen in ("sample_rate_hz = 48000", "quantum_frames = 128", "length_samples = 256"):
        if frozen not in session:
            raise ValueError(f"session lacks {frozen}")
    return source, expected


class FixtureHandler(http.server.SimpleHTTPRequestHandler):
    artifacts: pathlib.Path

    def translate_path(self, path: str) -> str:
        clean = path.split("?", 1)[0]
        if clean.startswith("/artifacts/"):
            return str(self.artifacts / clean.removeprefix("/artifacts/"))
        if clean.startswith("/fixture/"):
            return str(FIXTURE / clean.removeprefix("/fixture/"))
        return str(FIXTURE / "index.html")

    def log_message(self, _format: str, *_args: object) -> None:
        return


def free_port() -> int:
    with socket.socket() as probe:
        probe.bind(("127.0.0.1", 0))
        return int(probe.getsockname()[1])


def request(method: str, url: str, payload: object | None = None) -> dict:
    data = None if payload is None else json.dumps(payload).encode()
    headers = {} if data is None else {"Content-Type": "application/json"}
    with urllib.request.urlopen(
        urllib.request.Request(url, data=data, headers=headers, method=method)
    ) as response:
        value = json.loads(response.read())
    if value.get("value") is None and method != "DELETE":
        raise RuntimeError(f"WebDriver returned no value for {method} {url}")
    return value


def wait_for_driver(url: str) -> dict:
    failure: Exception | None = None
    for _attempt in range(200):
        try:
            return request("GET", f"{url}/status")["value"]
        except (OSError, urllib.error.URLError, RuntimeError) as error:
            failure = error
            threading.Event().wait(0.025)
    raise RuntimeError("ChromeDriver did not become ready") from failure


def float32(value: float) -> float:
    import struct

    return struct.unpack("<f", struct.pack("<f", value))[0]


def expected_pcm(source: dict) -> list[list[float]]:
    blocks = []
    for description in source["blocks"]:
        blocks.append([
            float32(description["leftBase"] + description["leftStep"] * index)
            for index in range(128)
        ])
    left = blocks[0] + blocks[0] + blocks[1] + [0.0] * 128
    return [left, [0.0] * 512]


def validate_result(result: dict, source: dict, expected: dict) -> None:
    if result.get("schema") != "miso.web.browser.result.v1":
        raise AssertionError("browser result schema")
    runs = result.get("runs")
    if not isinstance(runs, list) or len(runs) != 4:
        raise AssertionError("exactly four fresh contexts required")
    expected_backends = ["scalar", "scalar", "simd128", "simd128"]
    direct_pcm = expected_pcm(source)
    tolerance = expected["pcm"]["absoluteTolerance"]
    baseline_resources = None
    for index, (run, backend) in enumerate(zip(runs, expected_backends, strict=True)):
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
        numeric_backend = 0 if backend == "scalar" else 1
        if (
            status.get("tag") != "miso.status.v1"
            or status.get("result") != 0
            or status.get("state") != 2
            or status.get("lastResult") != 0
            or status.get("backend") != numeric_backend
            or status.get("sampleRateHz") != 48000
            or status.get("quantumFrames") != 128
            or status.get("nextAbsoluteSample") != expected["nextAbsoluteSampleBeforeDispose"]
            or status.get("renderedQuanta") != str(expected["renderedQuantaBeforeDispose"])
            or status.get("memoryBytes") != run.get("memoryBytes")
        ):
            raise AssertionError(f"run {index} status")
        resources = run.get("resources", {})
        if resources.get("backend") != numeric_backend:
            raise AssertionError(f"run {index} resource backend")
        for name, value in expected["resourceExact"].items():
            if str(resources.get(name)) != str(value):
                raise AssertionError(f"run {index} resource {name}")
        for name in expected["resourcePositive"]:
            if int(resources.get(name, "0")) <= 0:
                raise AssertionError(f"run {index} positive resource {name}")
        comparable = {name: value for name, value in resources.items() if name != "backend"}
        if baseline_resources is None:
            baseline_resources = comparable
        elif comparable != baseline_resources:
            raise AssertionError(f"run {index} resource determinism")
        pcm = run.get("pcm")
        if not isinstance(pcm, list) or len(pcm) != 2:
            raise AssertionError(f"run {index} PCM shape")
        for channel in range(2):
            if len(pcm[channel]) != 512:
                raise AssertionError(f"run {index} PCM frames")
            for actual, wanted in zip(pcm[channel], direct_pcm[channel], strict=True):
                if abs(actual - wanted) > tolerance:
                    raise AssertionError(f"run {index} PCM mismatch")
    if runs[0]["pcm"] != runs[1]["pcm"] or runs[2]["pcm"] != runs[3]["pcm"]:
        raise AssertionError("fresh-context determinism")
    for scalar, simd in zip(runs[0]["pcm"], runs[2]["pcm"], strict=True):
        for left, right in zip(scalar, simd, strict=True):
            if abs(left - right) > tolerance:
                raise AssertionError("scalar/simd parity")


def run(args: argparse.Namespace, source: dict, expected: dict) -> None:
    artifacts = args.artifacts.resolve()
    if sorted(path.name for path in artifacts.iterdir()) != sorted(EXPECTED_ARTIFACTS):
        raise ValueError("artifact directory is not the exact frozen five-file set")
    if args.output.exists() or args.output.with_suffix(args.output.suffix + ".sha256").exists():
        raise FileExistsError("refusing to overwrite browser evidence")
    require_clean_candidate()
    sealed = json.loads(args.seal_input.read_text())
    actual_seal = seal_record(artifacts, args.browser, args.driver)
    if sealed != actual_seal:
        raise RuntimeError("candidate/artifact/fixture/tool seal mismatch; browser not launched")

    handler = type("BoundFixtureHandler", (FixtureHandler,), {"artifacts": artifacts})
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
        created = request("POST", f"{driver_url}/session", capability)["value"]
        session_id = created["sessionId"]
        capabilities = created["capabilities"]
        base = f"{driver_url}/session/{session_id}"
        fixture_url = f"http://127.0.0.1:{server.server_port}/fixture/index.html"
        request("POST", f"{base}/url", {"url": fixture_url})
        script = """
          const done = arguments[arguments.length - 1];
          import('/fixture/browser-correctness.js')
            .then((module) => module.runMisoBrowserCorrectness())
            .then((value) => done({ok: true, value}),
                  () => done({ok: false, error: 'browser-correctness-failed'}));
        """
        envelope = request("POST", f"{base}/execute/async", {"script": script, "args": []})["value"]
        if envelope != {"ok": True, "value": envelope.get("value")}:
            raise RuntimeError("browser correctness module failed")
        result = envelope["value"]
        validate_result(result, source, expected)
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
                request("DELETE", f"{driver_url}/session/{session_id}")
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
    parser.add_argument("--seal", action="store_true")
    parser.add_argument("--seal-input", type=pathlib.Path)
    parser.add_argument("--artifacts", type=pathlib.Path)
    parser.add_argument("--output", type=pathlib.Path)
    parser.add_argument("--browser", type=pathlib.Path)
    parser.add_argument("--driver", type=pathlib.Path)
    args = parser.parse_args()
    source, expected = load_inputs()
    if args.check:
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
