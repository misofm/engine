import subprocess, os, re, shutil, sys
TREE="/tmp/claude-1002/v1458/tree"; ENV=dict(os.environ, CARGO_TARGET_DIR="/tmp/claude-1002/v1458/target")
def run(label, testfile_crate, oldtest, mpath, mold, mnew, tweak=None):
    tpath=f"{TREE}/crates/{testfile_crate}/tests/ramp_endpoint.rs"
    tsrc=open(tpath).read(); msrc=open(f"{TREE}/{mpath}").read() if mpath else None
    try:
        t=open(oldtest).read()
        if tweak: 
            assert t.count(tweak[0])==1; t=t.replace(*tweak)
        open(tpath,"w").write(t)
        if mpath:
            assert msrc.count(mold)==1; open(f"{TREE}/{mpath}","w").write(msrc.replace(mold,mnew))
        r=subprocess.run(["cargo","test","--locked","-q","-p",testfile_crate,"--test","ramp_endpoint","--no-fail-fast"],cwd=TREE,env=ENV,capture_output=True,text=True)
        out=r.stdout+r.stderr
        open(f"/tmp/claude-1002/v1458/mut/old-{label}.log","w").write(out)
        print(label, "exit", r.returncode, re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed", out), flush=True)
        for t_,m in re.findall(r"---- (\S+) stdout ----\n.*?panicked at [^\n]*\n(.*?)\n", out, re.S): print("   ",t_,m[:300])
        if "error[" in out: print("   COMPILE ERROR")
    finally:
        open(tpath,"w").write(tsrc)
        if mpath: open(f"{TREE}/{mpath}","w").write(msrc)
AB=("crates/effect-runtime/src/ramp.rs","_ => ramp_toward(self.current, self.step, self.target),","_ => self.current + self.step,")
D5=("crates/soft-clip/src/kernel.rs","if L::mask_any(L::mask_not(settled)) {","if !L::mask_any(L::mask_not(settled)) {")
run("delay-old-real","delay","cmp/delay.old.rs",None,None,None)
run("delay-old-advance_block","delay","cmp/delay.old.rs",*AB)
run("delay-old-250ms-advance_block","delay","cmp/delay.old.rs",*AB,tweak=("const DELAY_TIME_MS: f32 = 1.0;","const DELAY_TIME_MS: f32 = 250.0;"))
run("delay-old-250ms-real","delay","cmp/delay.old.rs",None,None,None,tweak=("const DELAY_TIME_MS: f32 = 1.0;","const DELAY_TIME_MS: f32 = 250.0;"))
run("softclip-old-real","soft-clip","cmp/soft-clip.old.rs",None,None,None)
run("softclip-old-d5","soft-clip","cmp/soft-clip.old.rs",*D5)
