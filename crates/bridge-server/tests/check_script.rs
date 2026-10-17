#![cfg(unix)]

use std::{fs, os::unix::fs::PermissionsExt, process::Command};

#[test]
fn checks_run_in_order_and_stop_at_each_failure() {
    let directory = std::env::temp_dir().join(format!("bridge-checks-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let cargo = directory.join("cargo");
    fs::write(
        &cargo,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$CHECK_LOG\"\nif [ \"$1\" = \"$FAIL_CHECK\" ]; then exit 42; fi\n",
    ).unwrap();
    fs::set_permissions(&cargo, fs::Permissions::from_mode(0o755)).unwrap();
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/check.sh");
    assert!(
        Command::new("/bin/sh")
            .arg("-n")
            .arg(&script)
            .status()
            .unwrap()
            .success()
    );
    let expected = [
        "fmt --all -- --check",
        "clippy --workspace --all-targets --locked -- -D warnings",
        "test --workspace --locked",
        "build -p bridge-server --release --locked",
    ];
    let log = directory.join("calls");
    for (fail, count) in [
        ("fmt", 1),
        ("clippy", 2),
        ("test", 3),
        ("build", 4),
        ("none", 4),
    ] {
        fs::write(&log, "").unwrap();
        let status = Command::new("/bin/sh")
            .arg(&script)
            .env("PATH", &directory)
            .env("CHECK_LOG", &log)
            .env("FAIL_CHECK", fail)
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(if fail == "none" { 0 } else { 42 }));
        assert_eq!(
            fs::read_to_string(&log)
                .unwrap()
                .lines()
                .collect::<Vec<_>>(),
            expected[..count]
        );
    }
    fs::remove_dir_all(directory).unwrap();
}
