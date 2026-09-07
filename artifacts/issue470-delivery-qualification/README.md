# Issue #470 delivery qualification evidence

This directory preserves the pre-pin qualification for the nonadjacent serialized scalar pairing
candidate. The frozen product/resource source is `6d3ef49c713f059e5c60b4000889eecaa9cff3db`;
the committed browser-resource checkpoint is `d6f7880304f22ee7bd518506ed3dc1f61c66e474`.
The latter differs from the former only in the issue record and the three target-specific browser
resource expectations, neither of which is a Wasm build input.

The retained six-file artifact was built once by the ordinary no-bypass builder from a detached
exact-source worktree carrying only the provisional artifact-pin overlay. Its Wasm member is
2,747,774 bytes with SHA-256
`63dd5f8b0febf193847b697fa8e4d92e791b7e4775f3b4b6b61252783f153e9f`.
The artifact bytes remain outside Git under `/tmp/issue470-qualified-artifact`; the captured command
metadata pins their digest and source identity.

- `probe/` preserves the one exact-source repin probe.
- `artifact/` preserves the ordinary retained build, six-file/static/object/ABI checks, direct-Wasm
  oracle, independent native witness, and the expected stale-resource diagnosis.
- `full/` preserves resource, SDK generation/deletion/type/headless/package, session identity,
  generated-matrix, three-browser, mutation, and hermetic host/worklet gates.

The first `full/01-browser-resources` invocation incorrectly ran the Python checker through Bash and
returned 2 before a product gate. The corrected `python3 -B` invocation is preserved separately and
passed. The first SDK type invocation returned 2 because `sdk/node_modules` was absent; the pinned
SDK `npm ci --ignore-scripts` prerequisite and the subsequent passing type invocation are also kept
separately. Neither setup correction changed a tracked file. These records are retained as failures
and are not claimed as passing gates.

The all-browser gate passed Chromium `151.0.7922.34`, Firefox `153.0`, and WebKit `26.5`, including
36 browser mutations, 14 artifact-set mutations, and three matrix mutations. Identity, command and
observation PCM digests stayed bit-identical to their native counterparts. No benchmark or timing
claim is part of this evidence.
