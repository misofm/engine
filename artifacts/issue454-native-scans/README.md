# Native PCM static scan qualification (#454)

Accepted source: `7724a581e320a3e080e4d0276efb928382065e83`.
Immutable integrated workspace candidate: `8b9d0ade4a1480a2992f503c41a669ed6dd9648f`.
Only the native checker, its two policy suites and issue evidence change the product scope.
Runtime/build/fixture inputs match delivered main6589c518. Workspace qualification includes
doctests: 275 result blocks, 1,591 passed, zero failed, 24 ignored, with the same named test
population as delivered #430.

Luna1 and Sol2 failures are retained with Sol3 source PASS and focused command/status evidence.
Astra's separate unchanged-source conditional-sourcing probes are reviewer evidence; they are
not attributed to Sol or represented as committed suite cases. The two production status-loss
mutations exercise the same original/mutant/restored assertions, with97 for unexpected success
and96 for setup/wrong diagnostics. No extra mutation campaign was performed.

Manifest entries bind exact bytes and hashes. Original raw logs and commands are preserved,
including any trailing whitespace. No native PCM runner, fixture generation or timing was run.
Actual-head PR review and required CI remain delivery gates. Parents403/306/349 and siblings455/456
remain open.
