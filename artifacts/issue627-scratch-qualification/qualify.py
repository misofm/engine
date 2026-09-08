import datetime
import hashlib
import json
import os
import pathlib
import subprocess
import sys

E = pathlib.Path('/home/bl/misofm/engine-limiter-audioworklet-qualification/artifacts/issue627-scratch-qualification')
R = pathlib.Path('/home/bl/misofm/engine-issue627-scratch')
O = pathlib.Path('/tmp/issue627-qualified-output')
SOURCE = 'dc14ca856e10cb5ad7f6fdacb0ea9322342a9251'
CANDIDATE = '63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1'
DELIVERED = pathlib.Path('/tmp/issue623-qualified-output')
PIN = 'hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256'
RESULTS = 'hosts/host-web/qualification/results.json'
MATRIX = 'hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md'
commands = []


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(['git', *args], cwd=R, text=True)


def run(name, argv, extra=None):
    env = dict(os.environ, CARGO_TARGET_DIR='/tmp/issue627-qualification-target')
    env.pop('MISO_ENGINE_WEB_REPIN', None)
    env.pop('MISO_ENGINE_WEB_AUDIOWORKLET_REPIN', None)
    env.pop('MISO_ENGINE_WEB_STRIP', None)
    env.update(extra or {})
    record = {
        'name': name,
        'argv': argv,
        'cwd': str(R),
        'head': git('rev-parse', 'HEAD').strip(),
        'start': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'env': {k: v for k, v in env.items() if k.startswith(('CARGO', 'RUST', 'MISO_ENGINE_WEB', 'PLAYWRIGHT')) or k in ('PATH', 'NODE_OPTIONS')},
    }
    (E / (name + '.command.json')).write_text(json.dumps(record, indent=2) + '\n')
    with (E / (name + '.stdout')).open('wb') as stdout, (E / (name + '.stderr')).open('wb') as stderr:
        result = subprocess.run(argv, cwd=R, env=env, stdout=stdout, stderr=stderr)
    record['status'] = result.returncode
    record['end'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    commands.append(record)
    (E / (name + '.status')).write_text(str(result.returncode) + '\n')
    (E / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
    print(name, result.returncode, flush=True)
    if result.returncode:
        raise RuntimeError(name + ' failed; no retry')


def assert_scratch_names(expected):
    actual = git('diff', '--name-only').splitlines()
    assert actual == sorted(expected), (actual, expected)


def normalized_matrix(text, old_commit, old_digest):
    return text.replace(old_commit, '<candidate-commit>').replace(old_digest, '<wasm-digest>')


try:
    assert git('rev-parse', 'HEAD').strip() == SOURCE
    assert not git('status', '--porcelain')
    assert not O.exists()
    assert not pathlib.Path('/tmp/issue627-qualification-target').exists()
    assert not pathlib.Path('/tmp/issue627-hermetic-target').exists()
    (E / 'scratch-head.txt').write_text(SOURCE + '\n')
    (E / 'source-status-before.txt').write_text(git('status', '--porcelain'))
    input_paths = [
        'Cargo.lock', 'Cargo.toml', 'rust-toolchain.toml', '.cargo/config.toml',
        'scripts/build-web-audioworklet.sh', 'crates/effect-contract/src/lib.rs',
        'crates/effect-package/src/wire.rs',
        'hosts/host-web/web/miso-engine-v1-audio-worklet.js',
        'hosts/host-web/web/miso-engine-v1-audio-worklet-host.js',
        'hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts', PIN,
    ]
    (E / 'inputs.sha256').write_text(''.join(digest(R / p) + '  ' + p + '\n' for p in input_paths))
    run('00-toolchain', ['rustc', '-vV'])

    pathlib.Path('/tmp/issue627-qualified-output').mkdir()
    (R / PIN).write_text(CANDIDATE + '\n')
    assert git('diff', '--name-only').splitlines() == [PIN]
    assert (R / PIN).read_bytes() == (CANDIDATE + '\n').encode()
    (E / 'first-overlay.diff').write_text(git('diff'))
    run('01-build', ['bash', 'scripts/build-web-audioworklet.sh', str(O)])

    expected_files = {
        p.name: digest(p) for p in DELIVERED.iterdir() if p.is_file()
    }
    actual_files = {p.name: digest(p) for p in O.iterdir() if p.is_file()}
    assert len(actual_files) == 6 and set(actual_files) == set(expected_files), (actual_files, expected_files)
    assert actual_files['miso-engine-v1-audio-worklet.simd128.wasm'] == CANDIDATE
    assert all(actual_files[name] == value for name, value in expected_files.items() if name != 'miso-engine-v1-audio-worklet.simd128.wasm')
    (E / 'six-file.sha256').write_text(''.join(actual_files[name] + '  ' + str(O / name) + '\n' for name in sorted(actual_files)))
    (E / 'identity.json').write_text(json.dumps({'candidate': CANDIDATE, 'non_wasm_identical': True, 'files': actual_files}, indent=2) + '\n')

    old_results = json.loads((R / RESULTS).read_text())
    old_commit = old_results['candidateCommit']
    old_digest = old_results['wasmSha256']
    old_matrix = (R / MATRIX).read_text()
    new_results = dict(old_results, candidateCommit=SOURCE, wasmSha256=CANDIDATE)
    (R / RESULTS).write_text(json.dumps(new_results, indent=2) + '\n')
    run('02-matrix-generate', ['node', 'hosts/host-web/qualification/generate-matrix.mjs'])
    assert_scratch_names([PIN, RESULTS, MATRIX])
    checked_results = json.loads((R / RESULTS).read_text())
    assert checked_results['candidateCommit'] == SOURCE and checked_results['wasmSha256'] == CANDIDATE
    assert {k: v for k, v in checked_results.items() if k not in ('candidateCommit', 'wasmSha256')} == {k: v for k, v in old_results.items() if k not in ('candidateCommit', 'wasmSha256')}
    new_matrix = (R / MATRIX).read_text()
    assert normalized_matrix(new_matrix, SOURCE, CANDIDATE) == normalized_matrix(old_matrix, old_commit, old_digest)
    (E / 'second-overlay.diff').write_text(git('diff'))

    run('03-static', ['bash', 'scripts/check-web-audioworklet.sh', str(O)])
    run('04-resources', ['python3', '-B', 'scripts/check-browser-expected-resources.py', '--artifacts', str(O)])
    run('05-hermetic', ['bash', 'scripts/test-web-audioworklet.sh'], {'CARGO_TARGET_DIR': '/tmp/issue627-hermetic-target'})
    run('06-sdk-install', ['npm', '--prefix', 'sdk', 'ci', '--ignore-scripts'])
    run('07-sdk', ['bash', 'scripts/sdk-package.sh', 'check', str(O)])
    run('08-browser-install', ['npm', '--prefix', 'hosts/host-web/qualification', 'ci', '--ignore-scripts'])
    run('09-browsers', ['npm', '--prefix', 'hosts/host-web/qualification', 'run', 'qualify', '--', '--artifacts', str(O), '--browser', 'all', '--check-matrix', '--self-test-mutations'])
    run('10-matrix-check', ['node', 'hosts/host-web/qualification/generate-matrix.mjs', '--check'])

    assert_scratch_names([PIN, RESULTS, MATRIX])
    assert (R / PIN).read_bytes() == (CANDIDATE + '\n').encode()
    final_results = json.loads((R / RESULTS).read_text())
    assert final_results == new_results
    assert normalized_matrix((R / MATRIX).read_text(), SOURCE, CANDIDATE) == normalized_matrix(old_matrix, old_commit, old_digest)
    verdict = 'PASS'
except Exception as error:
    verdict = 'FAIL: ' + str(error)
    print(verdict, flush=True)
finally:
    (E / 'final-status.txt').write_text(git('status', '--porcelain'))
    (E / 'final-overlay.diff').write_text(git('diff'))
    (E / 'README.md').write_text('# Issue627 scratch qualification\n\n' + verdict + '\n\nFrozen source ' + SOURCE + '; candidate ' + CANDIDATE + '; no retries.\n')
    files = [p for p in E.rglob('*') if p.is_file() and p.name != 'sha256sums.txt']
    (E / 'sha256sums.txt').write_text(''.join(digest(p) + '  ' + str(p.relative_to(E)) + '\n' for p in sorted(files)))
    sys.exit(0 if verdict == 'PASS' else 1)
