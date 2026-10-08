import subprocess, sys, os, re, json
TREE = "/tmp/claude-1002/v1458/tree"
ENV = dict(os.environ, CARGO_TARGET_DIR="/tmp/claude-1002/v1458/target")
ML_MB_OLD = """                    segment.current[index] = ramp_toward(
                        segment.current[index],
                        segment.step[index],
                        segment.target[index],
                    );"""
ML_MB_NEW = """                    segment.current[index] = segment.current[index].add(segment.step[index]);"""
MUTANTS = [
 ("site2_next_value", "crates/effect-runtime/src/ramp.rs", "self.current = ramp_toward(self.current, self.step, self.target);", "self.current += self.step;", ["compressor","transient-shaper","delay"]),
 ("site2_advance_block", "crates/effect-runtime/src/ramp.rs", "_ => ramp_toward(self.current, self.step, self.target),", "_ => self.current + self.step,", ["delay","compressor","transient-shaper"]),
 ("site4_advance_where", "crates/compressor/src/kernel.rs", "            ramp_toward(self.current, self.step, self.target),", "            self.current.add(self.step),", ["compressor"]),
 ("site4_gather", "crates/compressor/src/kernel.rs", "target[lane] = ramp.target;", "target[lane] = ramps[L::WIDTH - 1 - lane].target;", ["compressor"]),
 ("site5_gate", "crates/gate-expander/src/kernel.rs", "let stepped = ramp_toward(ramp.current, ramp.step, ramp.target);", "let stepped = ramp.current.add(ramp.step);", ["gate-expander"]),
 ("site6_run_segment", "crates/multiband-compressor/src/lib.rs", ML_MB_OLD, ML_MB_NEW, ["multiband-compressor"]),
 ("site6_segment_lane", "crates/multiband-compressor/src/lib.rs", "targets[track] = self.ramps[track][index].target;", "targets[track] = self.ramps[W - 1 - track][index].target;", ["multiband-compressor"]),
 ("site7_ramp_word", "crates/delay/src/lib.rs", "        ramp_toward(value, step, target)\n", "        value + step\n", ["delay"]),
 ("site7_d5_inverted", "crates/delay/src/lib.rs", "            if ramping {", "            if !ramping {", ["delay"]),
 ("site8_drive", "crates/soft-clip/src/kernel.rs", "drive = ramp_toward(drive, c.drive_step, c.drive_target);", "drive = drive.add(c.drive_step);", ["soft-clip"]),
 ("site8_d5_inverted", "crates/soft-clip/src/kernel.rs", "if L::mask_any(L::mask_not(settled)) {", "if !L::mask_any(L::mask_not(settled)) {", ["soft-clip"]),
 ("site8_target_vector", "crates/soft-clip/src/lib.rs", "*slot = self.ramps[lane][parameter].target;", "*slot = self.ramps[L::WIDTH - 1 - lane][parameter].target;", ["soft-clip"]),
 ("site9_limiter", "crates/true-peak-limiter/src/lib.rs", "            ramp_toward(self.current, self.step, self.target),", "            self.current.add(self.step),", ["true-peak-limiter"]),
 ("site7_left_damping", "crates/delay/src/lib.rs", "gain_left = ramp_word::<RAMPING>(gain_left, left.damping);", "gain_left = gain_left + left.damping.1;", ["delay"]),
 ("site7_left_feedback", "crates/delay/src/lib.rs", "feedback_left = ramp_word::<RAMPING>(feedback_left, left.feedback);", "feedback_left = feedback_left + left.feedback.1;", ["delay"]),
 ("site7_right_damping", "crates/delay/src/lib.rs", "gain_right = ramp_word::<RAMPING>(gain_right, right.damping);", "gain_right = gain_right + right.damping.1;", ["delay"]),
 ("site7_right_feedback", "crates/delay/src/lib.rs", "feedback_right = ramp_word::<RAMPING>(feedback_right, right.feedback);", "feedback_right = feedback_right + right.feedback.1;", ["delay"]),
 ("site7_cross_position", "crates/delay/src/lib.rs", "position = ramp_word::<RAMPING>(position, cross.position);", "position = position + cross.position.1;", ["delay"]),
]
sel = sys.argv[1:] 
release = False
if sel and sel[0] == "--release":
    release = True; sel = sel[1:]
for name, path, old, new, crates in MUTANTS:
    if sel and name not in sel: continue
    full = os.path.join(TREE, path)
    src = open(full).read()
    n = src.count(old)
    if n != 1:
        print(f"{name}: PATTERN COUNT {n}", flush=True); continue
    open(full, "w").write(src.replace(old, new))
    try:
        cmd = ["cargo","test","--locked","-q"] + (["--release"] if release else []) 
        for c in crates: cmd += ["-p", c]
        cmd += ["--test","ramp_endpoint","--no-fail-fast"]
        r = subprocess.run(cmd, cwd=TREE, env=ENV, capture_output=True, text=True)
        out = r.stdout + r.stderr
        open(f"/tmp/claude-1002/v1458/mut/{name}{'.rel' if release else ''}.log","w").write(out)
        fails = re.findall(r"---- (\S+) stdout ----\n.*?panicked at [^\n]*\n(.*?)\n", out, re.S)
        compile_err = "error[" in out or "could not compile" in out
        print(f"{name}: exit {r.returncode}{' COMPILE-ERROR' if compile_err else ''}", flush=True)
        for t, msg in fails:
            print(f"    {t}: {msg[:400]}", flush=True)
        res = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed", out)
        print("    results:", res, flush=True)
    finally:
        open(full, "w").write(src)
