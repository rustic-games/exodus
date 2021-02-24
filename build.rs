use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=data/database.sqlite3");

    Command::new("cargo")
        .args(&[
            "run",
            "--target-dir",
            "./target/build",
            "--package",
            "data-manager",
        ])
        .status()
        .unwrap();
}
