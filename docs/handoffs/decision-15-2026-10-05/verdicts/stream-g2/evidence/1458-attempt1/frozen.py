import subprocess, os, re, sys
TREE="/tmp/claude-1002/v1458/tree"; ENV=dict(os.environ, CARGO_TARGET_DIR="/tmp/claude-1002/v1458/target")
def run(label, mpath, mold, mnew, cmd):
    msrc=open(f"{TREE}/{mpath}").read()
    try:
        assert msrc.count(mold)==1, label
        open(f"{TREE}/{mpath}","w").write(msrc.replace(mold,mnew))
        r=subprocess.run(cmd,cwd=TREE,env=ENV,capture_output=True,text=True)
        out=r.stdout+r.stderr
        open(f"/tmp/claude-1002/v1458/mut/frozen-{label}.log","w").write(out)
        print(label,"exit",r.returncode, flush=True)
        for line in re.findall(r"test result: \w+\. \d+ passed; \d+ failed[^\n]*", out): 
            if "FAILED" in line: print("   ",line)
        for t_,m in re.findall(r"---- (\S+) stdout ----\n.*?panicked at [^\n]*\n(.*?)\n", out, re.S): print("   FAIL",t_,m[:250])
        if "error[" in out: print("   COMPILE ERROR"); print(out[-3000:])
    finally:
        open(f"{TREE}/{mpath}","w").write(msrc)
M={
 "gate":("crates/gate-expander/src/kernel.rs","let stepped = ramp_toward(ramp.current, ramp.step, ramp.target);","let stepped = ramp.current;","gate-expander"),
 "limiter":("crates/true-peak-limiter/src/lib.rs","            ramp_toward(self.current, self.step, self.target),\n            self.target,","            self.current,\n            self.target,","true-peak-limiter"),
 "multiband":("crates/multiband-compressor/src/lib.rs","""                    segment.current[index] = ramp_toward(
                        segment.current[index],
                        segment.step[index],
                        segment.target[index],
                    );""","""                    segment.current[index] = segment.current[index];""","multiband-compressor"),
 "compressor":("crates/compressor/src/kernel.rs","            ramp_toward(self.current, self.step, self.target),","            self.current,","compressor"),
}
for k in sys.argv[1:]:
    p,o,n,c=M[k]
    run(k,p,o,n,["cargo","test","--locked","-q","-p",c,"--all-targets","--no-fail-fast"])
if __name__=="__main__" and len(sys.argv)==1:
    pass
