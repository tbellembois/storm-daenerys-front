use std::process::Command;

fn main() {
    let mut mayerr_output = Command::new("git")
        .args(["log", "-1", "--pretty=%ct"])
        .output();

    if mayerr_output.is_err() {
        println!("cargo:warning=git command failed");
        mayerr_output = Command::new("date").args(["+%s"]).output();
    }

    let output = mayerr_output.unwrap();
    let source_date_epoch = String::from_utf8(output.stdout).unwrap();

    println!("cargo:warning=source_date_epoch:{}", source_date_epoch);
    println!("cargo:rustc-env=SOURCE_DATE_EPOCH={}", source_date_epoch);
}
