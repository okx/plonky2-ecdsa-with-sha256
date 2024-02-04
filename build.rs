use std::{env, path::Path};

fn main() {
    let directory = env::var("CARGO_MANIFEST_DIR").unwrap();

    let path = Path::new(&directory).join("plonky2"); //
    if path.exists() {
        std::process::Command::new("sh")
            .arg("-c")
            .arg("cd")
            .arg(path)
            .output()
            .expect("change directory");
        std::process::Command::new("sh")
            .arg("-c")
            .arg("cargo")
            .arg("build")
            .output()
            .expect("build dependency");
        std::process::Command::new("sh")
            .arg("-c")
            .arg("cd")
            .arg("..")
            .output()
            .expect("change directory");
    }
}
