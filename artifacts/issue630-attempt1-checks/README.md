# Issue 630 attempt 1 checks

The canonical lease was parsed from `/tmp/misofm-engine-issue630-attempt1-execution.lock/owner.txt` before the read-only preflight and immediately before each check. Its exact five-field bytes have SHA-256 `dc84cbe3e36e8200643671d77cae6d0191b92da4fd5790f4515c0d6dfb661cc6`.

Preflight passed at exact HEAD `4c729144a60a3b471bf5adab382365dd4ecb44f8`, whose parent is the reviewed head `6b59637a70340105f5c11b9a6a335e4aed1bd129` and whose only commit path is the #630 issue spec. SDK manifests match install-authorized commit `522c614d`; the installed lock is consistent with the lockfile after allowing platform-inapplicable optional packages. Required Node/npm/SDK executables, clean tracked state, and the preserved six-file output all passed.

The six authorized checks ran once, in order, and all returned status 0. Exact commands, statuses, streams, stream hashes, and the lease record used immediately before each invocation are in `summary.json` and the numbered files.

No installation, builder, artifact/static/resource/hermetic/browser gate, benchmark, timing, compiler capture, product edit, commit, or push was performed. This evidence remains uncommitted for root checkpoint and Astra LOW review.
