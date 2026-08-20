use std::process::Command;

#[test]
fn all_chapter_binaries_smoke() {
    // Given: every required chapter target is represented by its Cargo-provided path.
    let targets: [(&str, Option<&str>); 17] = [
        ("chapter03", option_env!("CARGO_BIN_EXE_chapter03")),
        ("chapter04", option_env!("CARGO_BIN_EXE_chapter04")),
        ("chapter05", option_env!("CARGO_BIN_EXE_chapter05")),
        ("chapter06", option_env!("CARGO_BIN_EXE_chapter06")),
        ("chapter07", option_env!("CARGO_BIN_EXE_chapter07")),
        ("chapter08", option_env!("CARGO_BIN_EXE_chapter08")),
        ("chapter09", option_env!("CARGO_BIN_EXE_chapter09")),
        ("chapter10", option_env!("CARGO_BIN_EXE_chapter10")),
        ("chapter11", option_env!("CARGO_BIN_EXE_chapter11")),
        ("chapter12", option_env!("CARGO_BIN_EXE_chapter12")),
        ("chapter13", option_env!("CARGO_BIN_EXE_chapter13")),
        ("chapter14", option_env!("CARGO_BIN_EXE_chapter14")),
        ("chapter15", option_env!("CARGO_BIN_EXE_chapter15")),
        ("chapter16", option_env!("CARGO_BIN_EXE_chapter16")),
        ("chapter17", option_env!("CARGO_BIN_EXE_chapter17")),
        ("chapter18", option_env!("CARGO_BIN_EXE_chapter18")),
        ("chapter19", option_env!("CARGO_BIN_EXE_chapter19")),
    ];

    for (target, path) in targets {
        // When: the available target is executed as a real child process.
        let path = path.unwrap_or_else(|| panic!("missing binary target {target}"));
        let output = Command::new(path)
            .output()
            .unwrap_or_else(|error| panic!("failed to execute {target}: {error}"));

        // Then: it exits successfully and prints its chapter heading.
        assert!(
            output.status.success(),
            "{target} exited unsuccessfully: {status}",
            status = output.status
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(&format!("Chapter {}:", &target[7..])),
            "{target} stdout missing chapter heading: {stdout:?}"
        );
    }
}
