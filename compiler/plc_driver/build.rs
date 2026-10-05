use shadow_rs::{BuildPattern, ShadowBuilder};

fn main() {
    // Debug builds rerun only when HEAD moves. Release builds always rerun, so the build time stays current.
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    ShadowBuilder::builder()
        .build_pattern(BuildPattern::Lazy)
        .build()
        .expect("shadow-rs collects the build information");
}
