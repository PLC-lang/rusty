use core::str;
use std::process::Command;

fn main() {
    // Re-run when the workspace's HEAD moves so the embedded commit info
    // tracks reality without a full `cargo clean`.
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-env-changed=RUSTY_BUILD_INFO");

    let package_version = env!("CARGO_PKG_VERSION");
    let date_str = git_output(&["log", "-1", "--format=%cd"]);
    let hash_str = git_output(&["rev-parse", "--short", "HEAD"]);

    let build_info = match (date_str.as_deref(), hash_str.as_deref()) {
        (Some(date), Some(hash)) => format!("{package_version} ({date}, {hash})"),
        _ => package_version.to_string(),
    };
    println!("cargo:rustc-env=RUSTY_BUILD_INFO={build_info}");
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let trimmed = str::from_utf8(&output.stdout).ok()?.trim().to_string();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed)
}
