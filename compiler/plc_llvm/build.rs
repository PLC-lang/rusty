fn main() {
    let llvm_config = if is_os("windows") {
        "llvm-config.exe".to_string()
    } else {
        std::env::var("LLVM_CONFIG").unwrap_or_else(|_| "llvm-config-14".to_string())
    };

    let cxxflags = String::from_utf8(
        std::process::Command::new(&llvm_config)
            .arg("--cxxflags")
            .output()
            .expect("Failed to run llvm-config")
            .stdout,
    )
    .expect("Invalid UTF-8");

    let mut build = cc::Build::new();
    build.cpp(true).file("src/cpp/llvm_wrapper.cpp").flag("-std=c++14");

    for flag in cxxflags.split_whitespace() {
        if flag.starts_with("-I") {
            // Found an include path, re-add it as a system include path
            let path = flag.trim_start_matches("-I");
            build.flag(format!("-isystem{path}"));
        } else if flag.starts_with("-D") {
            // Keep definitions as they are
            build.flag(flag);
        } else {
            // For other flags (like -fno-exceptions etc.)
            build.flag(flag);
        }
    }

    build.compile("llvm_wrapper");

    println!("cargo:rerun-if-changed=src/cpp/llvm_wrapper.cpp");
}

fn is_os(os: &str) -> bool {
    match std::env::var_os("CARGO_CFG_TARGET_OS") {
        Some(target_os) => target_os == os,
        None => false,
    }
}
