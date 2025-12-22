use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut args = vec![
        "plc".to_owned(),
        "iec61131-st/*.st".to_owned(),
        "-c".to_owned(),
        "-o".to_owned(),
        format!("{out_dir}/st.o"),
    ];
    if let Ok(target) = env::var("TARGET") {
        args.push("--target".to_owned());
        args.push(target);
    }
    if let Ok(optimization) = env::var("PROFILE") {
        args.push("-O".to_owned());
        if optimization == "release" {
            args.push("default".to_owned());
        } else {
            args.push("none".to_owned());
        }
    }
    #[cfg(target_os = "windows")]
    args.push("--single-module".to_owned());

    plc_driver::compile(&args).unwrap();
    #[cfg(not(target_os = "windows"))]
    Command::new("ar").args(["crs", "libst.a", "st.o"]).current_dir(Path::new(&out_dir)).status().unwrap();
    #[cfg(target_os = "windows")]
    Command::new("lld-link").args(["/LIB", "st.o"]).current_dir(Path::new(&out_dir)).status().unwrap();

    //link the object file
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static=st");
    println!("cargo:rerun-if-changed=iec61131-st/");

    //We can link against the st lib generated, but this will only be reflected in static libs.
    // The shared lib still has to be generated later.
    // There is a planned feature in rust to allow whole-archive linking, but i could not get it to
    // work (should look something like this : `println!("cargo:rustc-flags=-l static:+whole-archive=st");`)
    // The following clang command is equivalent:  clang -o libiec.so --shared -Wl,--whole-archive -lst -L. -Wl,--no-whole-archive  iec.o
    // https://stackoverflow.com/questions/55886779/how-to-link-a-c-library-without-calling-one-of-its-functions
}
