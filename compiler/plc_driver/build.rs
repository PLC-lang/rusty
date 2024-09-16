use core::str;
use std::process::Command;

fn main() {
    let commit_date = Command::new("git").args(&["log", "-1", "--format=%cd"]).output();
    let commit_hash = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output();
    let package_version = env!("CARGO_PKG_VERSION");

    if let (Ok(date), Ok(hash)) = (commit_date, commit_hash) {
        let date_str = str::from_utf8(&date.stdout).expect("invalid stdout output?").trim();
        let hash_str = str::from_utf8(&hash.stdout).expect("invalid stdout output?").trim();

        let build_info = format!("{package_version} ({date_str}, {hash_str})");
        println!("cargo:rustc-env=RUSTY_BUILD_INFO={build_info}");
    }
}
