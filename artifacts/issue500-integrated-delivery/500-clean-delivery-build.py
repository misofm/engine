import os,json,subprocess,pathlib
wd=pathlib.Path("/home/bl/misofm/engine-cp20-identity-plan")
out=pathlib.Path("/tmp/engine-500-clean-qualified");out.mkdir(exist_ok=False)
env=os.environ.copy();env["PATH"]="/home/bl/.cargo/bin:"+env["PATH"];env["CARGO_TARGET_DIR"]="/tmp/engine-500-artifact-target"
argv=["bash","scripts/build-web-audioworklet.sh",str(out)]
p=pathlib.Path("/tmp/500-clean-delivery-builder")
p.with_suffix(".command.json").write_text(json.dumps({"argv":argv,"cwd":str(wd),"source":subprocess.check_output(["git","rev-parse","HEAD"],cwd=wd,text=True).strip(),"PATH":env["PATH"],"CARGO_TARGET_DIR":env["CARGO_TARGET_DIR"]},indent=2)+"\n")
with p.with_suffix(".log").open("wb") as f:r=subprocess.run(argv,cwd=wd,env=env,stdout=f,stderr=subprocess.STDOUT)
p.with_suffix(".status").write_text(str(r.returncode)+"\n");print("builder",r.returncode,flush=True)
