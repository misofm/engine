import os,json,subprocess,pathlib,sys
wd=pathlib.Path("/home/bl/misofm/engine-475-populated-proof");source=subprocess.check_output(["git","rev-parse","HEAD"],cwd=wd,text=True).strip()
env=os.environ.copy();env["PATH"]="/home/bl/.cargo/bin:"+env["PATH"]
steps=[("workspace",["cargo","test","--locked","--workspace"],{"CARGO_TARGET_DIR":"/tmp/engine-475-workspace-qualified"}),("wasm-scalar",["cargo","check","--locked","--target","wasm32-unknown-unknown","-p","target-smoke","-p","protocol","-p","host-web","-p","builtins-compiler"],{"CARGO_TARGET_DIR":"/tmp/engine-475-wasm-scalar-qualified","RUSTFLAGS":"-C target-feature=-simd128"}),("wasm-simd",["cargo","check","--locked","--target","wasm32-unknown-unknown","-p","target-smoke","-p","protocol"],{"CARGO_TARGET_DIR":"/tmp/engine-475-wasm-simd-qualified","RUSTFLAGS":"-C target-feature=+simd128"}),("native-capi",["cargo","build","--locked","--release","-p","capi"],{"CARGO_TARGET_DIR":"/tmp/engine-475-native-qualified"}),("capi-abi",["bash","scripts/check-capi-abi.sh"],{"MISO_ENGINE_CAPI_SKIP_BUILD":"1","MISO_ENGINE_CAPI_LIBRARY":"/tmp/engine-475-native-qualified/release/libcapi.so","MISO_ENGINE_CAPI_STATIC_LIBRARY":"/tmp/engine-475-native-qualified/release/libcapi.a"})]
for label,argv,overrides in steps:
 assert subprocess.check_output(["git","rev-parse","HEAD"],cwd=wd,text=True).strip()==source
 assert not subprocess.check_output(["git","status","--porcelain"],cwd=wd,text=True)
 current=env.copy();current.update(overrides);p=pathlib.Path("/tmp/475-immutable-"+label)
 p.with_suffix(".command.json").write_text(json.dumps({"argv":argv,"cwd":str(wd),"source":source,"env_overrides":dict(overrides,PATH=current["PATH"])},indent=2)+"\n")
 with p.with_suffix(".log").open("wb") as f:r=subprocess.run(argv,cwd=wd,env=current,stdout=f,stderr=subprocess.STDOUT)
 p.with_suffix(".status").write_text(str(r.returncode)+"\n");print(label,r.returncode,flush=True)
 if r.returncode:sys.exit(r.returncode)
assert not subprocess.check_output(["git","status","--porcelain"],cwd=wd,text=True)
print("immutable source preserved",source,flush=True)
