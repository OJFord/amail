use std::process::Command;
use std::process::Stdio;

fn main() {
    Command::new("yarn")
        .args(["--cwd=.."])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute yarn");

    Command::new("rm")
        .args(["-r", "../dist/build"])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to clean previous build");

    Command::new("yarn")
        .env("DEBUG", format!("{}", cfg!(debug_assertions)))
        .args(["--cwd=..", "rollup", "--config"])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute rollup");

    tauri_build::build()
}
