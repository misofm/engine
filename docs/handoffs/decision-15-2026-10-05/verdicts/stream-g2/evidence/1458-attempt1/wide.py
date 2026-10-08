import subprocess, os, re, sys
TREE="/tmp/claude-1002/v1458/tree"; ENV=dict(os.environ, CARGO_TARGET_DIR="/tmp/claude-1002/v1458/target")
def run(label, mpath, mold, mnew, cmd):
    msrc=open(f"{TREE}/{mpath}").read()
    try:
        assert msrc.count(mold)==1
        open(f"{TREE}/{mpath}","w").write(msrc.replace(mold,mnew))
        r=subprocess.run(cmd,cwd=TREE,env=ENV,capture_output=True,text=True)
        out=r.stdout+r.stderr
        open(f"/tmp/claude-1002/v1458/mut/wide-{label}.log","w").write(out)
        print(label,"exit",r.returncode, flush=True)
        for line in re.findall(r"(?:Running \S+ \(|test result: \w+\. \d+ passed; \d+ failed)[^\n]*", out): print("   ",line)
        for t_,m in re.findall(r"---- (\S+) stdout ----\n.*?panicked at [^\n]*\n(.*?)\n", out, re.S): print("   FAIL",t_,m[:250])
        if "error[" in out: print("   COMPILE ERROR")
    finally:
        open(f"{TREE}/{mpath}","w").write(msrc)
which=sys.argv[1]
if which=="ab":
    run("ab-effect-runtime","crates/effect-runtime/src/ramp.rs","_ => ramp_toward(self.current, self.step, self.target),","_ => self.current + self.step,",
        ["cargo","test","--locked","-q","-p","effect-runtime","--no-fail-fast"])
if which=="d5":
    run("d5-softclip-all","crates/soft-clip/src/kernel.rs","if L::mask_any(L::mask_not(settled)) {","if !L::mask_any(L::mask_not(settled)) {",
        ["cargo","test","--locked","-q","-p","soft-clip","--all-targets","--no-fail-fast"])
