from pathlib import Path
import subprocess,json,sys
prefix=Path(sys.argv[1]); prompt_path=Path(sys.argv[2]); effort=sys.argv[3]; worktree=sys.argv[4]
assert effort in ("low","medium")
prompt=prompt_path.read_text()
a=["/home/bl/.local/bin/codex","exec","-C",worktree,"-s","danger-full-access","-m","gpt-6-astra","-c",f'model_reasoning_effort="{effort}"',"-o",str(prefix)+".md","-"]
with open(str(prefix)+".command.json","x") as f: json.dump({"argv":a,"cwd":worktree,"prompt":str(prompt_path)},f,indent=2)
with open(str(prefix)+".stdout","x") as out,open(str(prefix)+".stderr","x") as err:r=subprocess.run(a,input=prompt,text=True,cwd=worktree,stdout=out,stderr=err)
Path(str(prefix)+".status").write_text(str(r.returncode)+"\n")
print(r.returncode)
