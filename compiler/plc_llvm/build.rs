fn main() {
    let llvm_config = if is_os("windows") {
        "llvm-config.exe".to_string()
    } else {
        // first check for LLVM_CONFIG env var
        std::env::var("LLVM_CONFIG").unwrap_or_else(|_| {
            // Check if llvm-config exists in the path, otherwise check for llvm-config-21
            if which::which("llvm-config").is_ok() {
                "llvm-config".to_string()
            } else {
                "llvm-config-21".to_string()
            }
        })
    };

    // Fetch CXXFLAGS from llvm-config
    let cxxflags = String::from_utf8(
        std::process::Command::new(&llvm_config)
            .arg("--cxxflags")
            .output()
            .expect("Failed to run llvm-config")
            .stdout,
    )
    .expect("Invalid UTF-8");

    let mut build = cc::Build::new();
    build.cpp(true).file("src/cpp/llvm_wrapper.cpp");
    build.std("c++17");

    let is_msvc = std::env::var("TARGET").unwrap().ends_with("msvc");

    if is_msvc {
        // MSVC's way to treat included paths as system headers (suppress warnings)
        // If the LLVM includes are coming through -I, we want to convert them
        // to MSVC's equivalent.
        // This is complex, so let's simplify by using the overall warning flags:

        // This flag tells Clang/MSVC not to emit warnings from external headers
        // It should be applied to the compiler invocation overall.
        build.flag("/external:W0");
    } else {
        // Treat LLVM headers as system headers on GCC/Clang to suppress warnings.
        // This keeps third-party diagnostics out of our build output while preserving warnings
        // from our own sources.
    }

    for flag in cxxflags.split_whitespace() {
        if let Some(include_path) = flag.strip_prefix("-I") {
            if is_msvc {
                build.flag(flag);
            } else if include_path.contains("/llvm") {
                build.flag("-isystem");
                build.flag(include_path);
            } else {
                build.flag(flag);
            }
            continue;
        }
        if flag.starts_with("-D") {
            build.flag(flag);
        }
    }

    build.compile("llvm_wrapper");
    println!("cargo:rerun-if-changed=src/cpp/llvm_wrapper.cpp");
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG");
    println!("cargo:rerun-if-env-changed=PATH");
}

fn is_os(os: &str) -> bool {
    match std::env::var_os("CARGO_CFG_TARGET_OS") {
        Some(target_os) => target_os == os,
        None => false,
    }
}
