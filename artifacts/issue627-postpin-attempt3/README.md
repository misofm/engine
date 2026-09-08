# Issue 627 post-pin final attempt 3 — failed

The final attempt proved a newly created empty existing non-symlink output
directory, ran the ordinary no-bypass builder once from detached promotion
commit `0bb5a820be33a49393386617358ec1db2fe7577d`, and obtained the exact qualified
six-file candidate. Those stages passed.

A coordination race then launched overlapping duplicate executions of the next
three checks. One executor wrote `03-static.*`, `04-resources.*`, and
`05-hermetic.*`; Luna wrote `03-check-web.*`, `04-expected-resources.*`, and
`05-test-web.*`. The paired static and resource results are byte-identical and
zero; Luna's hermetic status is zero. The other hermetic invocation has complete
streams but no retained numeric status. These results receive no delivery credit
because final attempt 3 required every check to run exactly once.

Execution stopped before any SDK, matrix, formatting, diff, workspace-policy,
or effect-runtime command. No install or browser command ran. This is a permanent
attempt-3 procedural failure; no fourth attempt, retry, or further rescope is
authorized. Every raw record is retained unchanged.
