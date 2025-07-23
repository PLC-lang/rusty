use std::process::Command;

fn main() {
    // Get LLVM configuration from llvm-config
    let llvm_config_output = Command::new("llvm-config-14")
        .args(&["--cxxflags", "--ldflags", "--libs", "core", "support"])
        .output()
        .or_else(|_| {
            // Fallback to plain llvm-config if llvm-config-14 is not available
            Command::new("llvm-config")
                .args(&["--cxxflags", "--ldflags", "--libs", "core", "support"])
                .output()
        })
        .expect("Failed to run llvm-config. Make sure LLVM 14 is installed and llvm-config is in PATH");

    let llvm_config_str = String::from_utf8(llvm_config_output.stdout)
        .expect("llvm-config output is not valid UTF-8");

    // Parse the output to extract compiler flags and library paths
    let lines: Vec<&str> = llvm_config_str.trim().split('\n').collect();
    
    if lines.len() >= 3 {
        let cxxflags = lines[0];
        let ldflags = lines[1]; 
        let libs = lines[2];

        // Build the C++ wrapper using cc crate
        let mut build = cc::Build::new();
        build
            .cpp(true)
            .file("src/codegen/string_type_wrapper.cpp")
            .flag("-std=c++14");

        // Add LLVM include paths and defines from cxxflags
        for flag in cxxflags.split_whitespace() {
            if flag.starts_with("-I") {
                build.include(&flag[2..]);
            } else if flag.starts_with("-D") {
                let define = &flag[2..];
                if let Some(eq_pos) = define.find('=') {
                    build.define(&define[..eq_pos], &define[eq_pos + 1..]);
                } else {
                    build.define(define, None);
                }
            } else if flag.starts_with("-f") || flag.starts_with("-W") {
                build.flag(flag);
            }
        }

        build.compile("string_type_wrapper");

        // Add LLVM library paths from ldflags
        for flag in ldflags.split_whitespace() {
            if flag.starts_with("-L") {
                println!("cargo:rustc-link-search=native={}", &flag[2..]);
            }
        }

        // Add LLVM libraries from libs
        for flag in libs.split_whitespace() {
            if flag.starts_with("-l") {
                println!("cargo:rustc-link-lib={}", &flag[2..]);
            }
        }
    } else {
        panic!("Failed to parse llvm-config output: {}", llvm_config_str);
    }

    // Tell cargo to re-run this build script if the C++ file changes
    println!("cargo:rerun-if-changed=src/codegen/string_type_wrapper.cpp");
}
