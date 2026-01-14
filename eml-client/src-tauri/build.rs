use std::env;

fn main() {
    println!("cargo::rerun-if-changed=.");
    println!("cargo::rerun-if-changed=../src");
    println!("cargo::rerun-if-changed=../index.html");
    println!("cargo::rerun-if-changed=../package.json");
    println!("cargo::rerun-if-changed=../svelte.config.js");
    println!("cargo::rerun-if-changed=../vite.config.js");

    if !env::var("TAURI").is_ok_and(|v| v == "1") {
        // regular `cargo build` (or test, etc.)
        // so need to build frontend dist
        println!("cargo::warning=Building frontend dist outside `tauri build`");
        xtask_frontend::build();
    }
    tauri_build::build()
}
