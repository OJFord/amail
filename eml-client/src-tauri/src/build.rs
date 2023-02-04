use std::env;
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
        .args(["-r", "../dist/assets", "../dist/index.html"])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to clean previous build");

    Command::new("yarn")
        .env(
            "DEBUG",
            format!("{}", env::var("PROFILE") != Ok("release".into())),
        )
        .args(["--cwd=..", "vite", "build"])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute vite");

    tauri_build::build()
}
