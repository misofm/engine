from pathlib import Path
import hashlib, json, os, platform, shutil, subprocess, sys

repo = Path.cwd()
lease = Path('/tmp/misofm-engine-issue630-attempt1-execution.lock/owner.txt')
expected_lease = (
    b'issue=630\n'
    b'attempt=1\n'
    b'owner=/root/issue610_luna_xhigh\n'
    b'authorized_head=6b59637a70340105f5c11b9a6a335e4aed1bd129\n'
    b'scope=installed-state-preflight,sdk-check,matrix-check,cargo-fmt,diff-hygiene,workspace-policy,effect-runtime-policy\n'
)
expected_head = '4c729144a60a3b471bf5adab382365dd4ecb44f8'
expected_parent = '6b59637a70340105f5c11b9a6a335e4aed1bd129'
install_commit = '522c614df35f8b44007d7ae2853f27ad8e4add40'
output = Path('/tmp/issue627-postpin-attempt3-output-20260908')
accepted_sha = repo/'artifacts/issue627-scratch-qualification/six-file.sha256'
final_sha = repo/'artifacts/issue627-postpin-attempt3/02-output.sha256'

def sha(b): return hashlib.sha256(b).hexdigest()
def run(*args):
    return subprocess.run(args, cwd=repo, text=True, capture_output=True, check=True).stdout.strip()
def parse_sha(path):
    out={}
    for line in path.read_text().splitlines():
        if line.strip():
            h, name = line.split(None,1)
            out[Path(name).name] = h
    return out

def lease_fields():
    raw=lease.read_bytes()
    if raw != expected_lease:
        raise RuntimeError('canonical lease bytes are not the exact five expected fields')
    fields=raw.decode().splitlines()
    if len(fields) != 5 or any(not x for x in fields):
        raise RuntimeError('canonical lease does not have exactly five nonempty fields')
    return raw, dict(x.split('=',1) for x in fields)

raw, fields = lease_fields()
head=run('git','rev-parse','HEAD')
parent=run('git','rev-parse','HEAD^')
if head != expected_head or parent != expected_parent:
    raise RuntimeError(f'head relationship mismatch: {head} parent {parent}')
status_tracked=run('git','status','--porcelain=v1','--untracked-files=no')
if status_tracked:
    raise RuntimeError('tracked tree is dirty')
if run('git','diff','--name-only','HEAD') or run('git','diff','--cached','--name-only'):
    raise RuntimeError('tracked diff exists')
changed=run('git','diff-tree','--no-commit-id','--name-status','-r','HEAD').splitlines()
expected_change='M\t.github/ISSUE_SPECS/630-complete-limiter-delivery-checks-with-one-canonical-lease.md'
if changed != [expected_change]:
    raise RuntimeError(f'HEAD is not the sole docs-only lease-record commit: {changed!r}')

pkg_paths=['sdk/package.json','sdk/package-lock.json']
package_hashes={}
package_identity={}
for rel in pkg_paths:
    current=(repo/rel).read_bytes()
    at_install=subprocess.run(['git','show',f'{install_commit}:{rel}'],cwd=repo,capture_output=True,check=True).stdout
    package_hashes[rel]={'current_sha256':sha(current),'install_commit_sha256':sha(at_install),'byte_equal':current==at_install,'bytes':len(current)}
    if current != at_install:
        raise RuntimeError(f'{rel} differs from install-authorized commit')
    package_identity[rel]={'current':json.loads(current),'install_commit':json.loads(at_install)}

pkg=json.loads((repo/'sdk/package.json').read_text())
lock=json.loads((repo/'sdk/package-lock.json').read_text())
installed_path=repo/'sdk/node_modules/.package-lock.json'
installed=json.loads(installed_path.read_text())
if lock.get('name') != pkg.get('name') or lock.get('version') != pkg.get('version'):
    raise RuntimeError('package and package-lock root identity mismatch')
if installed.get('name') != pkg.get('name') or installed.get('version') != pkg.get('version'):
    raise RuntimeError('installed package-lock root identity mismatch')
if installed.get('lockfileVersion') != lock.get('lockfileVersion'):
    raise RuntimeError('installed lockfile version mismatch')
lock_pkgs=lock.get('packages',{})
installed_pkgs=installed.get('packages',{})
if '' in installed_pkgs:
    raise RuntimeError('installed package-lock unexpectedly contains a root package entry')
installed_mismatches=[]
for name, meta in installed_pkgs.items():
    if name not in lock_pkgs or meta != lock_pkgs[name]:
        installed_mismatches.append(name)
if installed_mismatches:
    raise RuntimeError(f'installed package metadata mismatches: {installed_mismatches}')

def applicable(meta):
    os_name=sys.platform
    machine=platform.machine().lower()
    cpu='x64' if machine in ('x86_64','amd64') else ('arm64' if machine in ('aarch64','arm64') else machine)
    os_values=meta.get('os')
    cpu_values=meta.get('cpu')
    def allowed(values, current):
        if not values: return True
        positive=[x for x in values if not x.startswith('!')]
        negative=[x[1:] for x in values if x.startswith('!')]
        if current in negative: return False
        return current in positive if positive else True
    return allowed(os_values,os_name) and allowed(cpu_values,cpu)
missing=[name for name in lock_pkgs if name not in installed_pkgs and name != '']
missing_non_optional=[name for name in missing if not lock_pkgs[name].get('optional')]
missing_applicable=[name for name in missing if lock_pkgs[name].get('optional') and applicable(lock_pkgs[name])]
if missing_non_optional or missing_applicable:
    raise RuntimeError(f'installed package-lock missing required/applicable packages: {missing_non_optional + missing_applicable}')

required_bins={}
for command in ['node','npm']:
    path=shutil.which(command)
    required_bins[command]={'path':path,'executable':bool(path and os.access(path,os.X_OK))}
for rel in ['sdk/node_modules/.bin/tsc','sdk/node_modules/.bin/tsserver','sdk/node_modules/.bin/esbuild']:
    p=repo/rel
    required_bins[rel]={'exists':p.exists(),'symlink':p.is_symlink(),'executable':p.exists() and os.access(p,os.X_OK),'resolved':str(p.resolve()) if p.exists() else None}
if not all(x.get('executable',False) for x in required_bins.values()):
    raise RuntimeError(f'required SDK executable missing: {required_bins}')

accepted=parse_sha(accepted_sha)
final=parse_sha(final_sha)
actual={p.name:sha(p.read_bytes()) for p in output.iterdir() if p.is_file()}
if len(actual) != 6 or set(actual) != set(accepted) or actual != accepted or final != accepted:
    raise RuntimeError('preserved six-file output/hash identity mismatch')

result={
 'status':'PASS',
 'lease':{'path':str(lease),'bytes':len(raw),'sha256':sha(raw),'fields':fields},
 'git':{'head':head,'parent':parent,'expected_head':expected_head,'reviewed_parent':expected_parent,'tracked_status':status_tracked,'head_commit_changed_paths':changed,'branch':run('git','branch','--show-current'),'upstream':run('git','rev-parse','--abbrev-ref','--symbolic-full-name','@{upstream}')},
 'sdk_manifests':{'install_authorized_commit':install_commit,'package_hashes':package_hashes,'package_name':pkg['name'],'package_version':pkg['version'],'lockfile_version':lock['lockfileVersion'],'installed_lock_sha256':sha(installed_path.read_bytes()),'installed_package_count':len(installed_pkgs),'lock_package_count':len(lock_pkgs)-1,'missing_platform_inapplicable_optional':missing,'missing_non_optional':missing_non_optional,'missing_applicable_optional':missing_applicable,'installed_metadata_mismatches':installed_mismatches},
 'required_executables':required_bins,
 'preserved_output':{'path':str(output),'is_directory':output.is_dir(),'is_symlink':output.is_symlink(),'file_count':len(actual),'accepted_record':str(accepted_sha),'final_attempt_record':str(final_sha),'accepted_hashes':accepted,'final_hashes':final,'actual_hashes':actual},
 'clean_tracked_tree':True,
}
print(json.dumps(result,indent=2,sort_keys=True))
