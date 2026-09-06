import os,json,subprocess,pathlib
wd=pathlib.Path("/home/bl/misofm/engine-cp4-bind-coverage")
env=os.environ.copy();env["PATH"]="/home/bl/.cargo/bin:"+env["PATH"]
source=subprocess.check_output(["git","rev-parse","HEAD"],cwd=wd,text=True).strip()
commands=[("graph-debug",["cargo","test","--locked","-p","graph","--lib"]),("graph-release",["cargo","test","--locked","--release","-p","graph","--lib"]),("clippy",["cargo","clippy","--locked","-p","graph","--all-targets","--all-features","--","-D","warnings"]),("fmt",["cargo","fmt","--all","--","--check"]),("diff",["git","diff","--check"]),("graph-policy",["bash","scripts/check-graph-policy.sh"])]
for label,argv in commands:
 p=pathlib.Path("/tmp/495-root-final-"+label)
 p.with_suffix(".command.json").write_text(json.dumps({"argv":argv,"cwd":str(wd),"source":source,"PATH":env["PATH"],"RUSTFLAGS":env.get("RUSTFLAGS")},indent=2)+"\n")
 with p.with_suffix(".log").open("wb") as f:r=subprocess.run(argv,cwd=wd,env=env,stdout=f,stderr=subprocess.STDOUT)
 p.with_suffix(".status").write_text(str(r.returncode)+"\n")
 print(label,r.returncode,flush=True)
 if r.returncode:break
