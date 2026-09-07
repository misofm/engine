//! Executable-boundary coverage for the native PCM runner.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

struct TempDir {
    path: PathBuf,
    remove_on_drop: bool,
}

impl TempDir {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir();
        let process_id = std::process::id();

        for _ in 0..100 {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let path = root.join(format!(
                "miso-native-pcm-runner-process-boundary-{process_id}-{id}"
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    return Self {
                        path,
                        remove_on_drop: true,
                    };
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("create unique temporary directory: {error}"),
            }
        }

        panic!("unable to create a unique temporary directory after 100 attempts");
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn cleanup(mut self) {
        fs::remove_dir_all(&self.path).expect("remove temporary directory");
        self.remove_on_drop = false;
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.remove_on_drop {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/native-pcm-runner/v1")
}

fn run_runner(session: &Path, source_root: &Path, frames: &str, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_native-pcm-runner"))
        .args([
            "--session",
            session.to_str().expect("session path is UTF-8"),
            "--source-root",
            source_root.to_str().expect("source root path is UTF-8"),
            "--frames",
            frames,
            "--output",
            output.to_str().expect("output path is UTF-8"),
        ])
        .output()
        .expect("spawn native-pcm-runner")
}

#[test]
fn executable_boundary_accepts_fixture_and_rejects_zero_frames() {
    let temp = TempDir::new();
    let fixtures = fixture_root();
    let session = fixtures.join("riff-48000.json");

    let accepted_output = temp.path().join("accepted.f32le");
    let accepted = run_runner(&session, &fixtures, "1024", &accepted_output);
    assert!(
        accepted.status.success(),
        "accepted status: {:?}",
        accepted.status
    );
    assert!(
        accepted.stdout.is_empty(),
        "accepted stdout: {:?}",
        accepted.stdout
    );
    assert!(
        accepted.stderr.is_empty(),
        "accepted stderr: {:?}",
        accepted.stderr
    );
    assert_eq!(
        fs::metadata(&accepted_output)
            .expect("accepted output metadata")
            .len(),
        8_192
    );
    assert!(!temp.path().join("accepted.f32le.issue073.partial").exists());

    let rejected_output = temp.path().join("rejected.f32le");
    let rejected = run_runner(&session, &fixtures, "0", &rejected_output);
    assert_eq!(rejected.status.code(), Some(2));
    assert!(
        rejected.stdout.is_empty(),
        "rejected stdout: {:?}",
        rejected.stdout
    );
    assert_eq!(rejected.stderr, b"native-pcm-runner.v1\tcli\tframes.zero\n");
    assert!(!rejected_output.exists());
    assert!(!temp.path().join("rejected.f32le.issue073.partial").exists());

    temp.cleanup();
}
