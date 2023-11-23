use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["log", "-1", "--pretty=%ct"])
        .output()
        .unwrap();
    let source_date_epoch = String::from_utf8(output.stdout).unwrap();

    // println!("cargo:warning={}", source_date_epoch);
    println!("cargo:rustc-env=SOURCE_DATE_EPOCH={}", source_date_epoch);
}
