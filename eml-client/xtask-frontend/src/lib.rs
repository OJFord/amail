use std::process::{Command, exit};

pub fn build() {
    Command::new("yarn")
        .status()
        .expect("Failed to install yarn deps");

    let status = Command::new("yarn")
        .args(vec![
            "run",
            &format!("vite:build:debug-{}", cfg!(debug_assertions)),
        ])
        .status()
        .expect("Failed to run vite build");

    exit(status.code().unwrap_or(1));
}
