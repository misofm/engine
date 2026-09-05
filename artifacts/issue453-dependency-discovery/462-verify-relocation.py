import pathlib, shlex
trace = pathlib.Path(__file__).with_name("462-sol2-trace.log").read_text().splitlines()
expected = ["tree", "--locked", "--offline", "-p", "fixture", "-e", "features,no-dev", "--target", "all"]
for case in ["status-loss-cargo", "status-loss-cargo-empty", "status-loss-cargo-matching", "cargo-empty-success", "status-loss-grep"]:
    matches = []
    for line in trace:
        words = shlex.split(line)
        if words[0].startswith("cwd=") and pathlib.Path(words[0][4:]).name == case and words[1] == "args=" and words[2:] == expected:
            matches.append(line)
    assert len(matches) == 1, (case, matches)
    print("PASS:", case, "forwarded earlier fixture with exact flags")
